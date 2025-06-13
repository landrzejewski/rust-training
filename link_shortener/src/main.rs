pub mod responses;
pub mod persistence;
pub mod rest;

use persistence::SqlxLinkRepository;
use axum::http::{HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post};
use axum::{serve, Router};
use chrono::Local;
use dotenvy::dotenv;
use link_shortener::{delete_expired_links, delete_orphaned_tags};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;
use tokio_cron_scheduler::{Job, JobScheduler};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use crate::rest::{create_link_handler, delete_link_handler, find_links_handler, generate_qr_code_handler, redirect_handler, update_link_handler};

#[tokio::main]
async fn main() {
    _ = dotenv();
    configure_tracing();
    let db_connection_pool = create_db_connection_pool().await;
    configure_scheduler(db_connection_pool.clone()).await;
    let server_address = get_env("SERVER_ADDRESS");
    let listener = create_listener(&server_address).await;
    let router = create_router(db_connection_pool.clone()).await;
    tracing::info!("Listening on address: {server_address}");
    serve(listener, router).await.expect("Server failed to start");
}

fn configure_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_env("TRACING_LEVEL"))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn create_db_connection_pool() -> PgPool {
    let database_url = get_env("DATABASE_URL");
    let max_connections: u32 = get_env("DATABASE_MAX_POOL_SIZE").parse().expect("invalid DATABASE_MAX_POOL_SIZE value");
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool")
}

async fn configure_scheduler(db_connection_pool: PgPool) {
    let scheduler = JobScheduler::new().await.expect("Creating scheduler failed");
    let cron_expression = get_env("CLEANING_JOB_CRON_EXPRESSION");
    scheduler
        .add(create_cleaning_job(&cron_expression, db_connection_pool))
        .await
        .expect("Adding cleaning job to scheduler failed");
    scheduler.start().await.expect("Starting scheduler failed");
}

fn create_cleaning_job(cron_expression: &str, db_connection_pool: PgPool) -> Job {
    Job::new_cron_job_async_tz(cron_expression, Local, move |_, _| {
        let connection_pool = db_connection_pool.clone();
        Box::pin(async move {
            tracing::info!("Executing cleaning job");
            let repository = SqlxLinkRepository::new(connection_pool);
            if let Err(error) = delete_expired_links(&repository).await {
                tracing::error!("Failed to delete expired links: {error:?}");
            }
            if let Err(error) = delete_orphaned_tags(&repository).await {
                tracing::error!("Failed to delete orphaned tags: {error:?}");
            }
        })
    })
    .expect("Creating cleaning job failed")
}

async fn create_listener(server_address: &str) -> TcpListener {
    TcpListener::bind(&server_address).await.expect("Creating tcp listener failed")
}

async fn create_router(db_connection_pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/:shortened_path", get(redirect_handler))
        .route("/:shortened_path/qrcode", get(generate_qr_code_handler))
        .route("/api/links", post(create_link_handler))
        .route("/api/links", get(find_links_handler))
        .route("/api/links/:id", patch(update_link_handler))
        .route("/api/links/:id", delete(delete_link_handler))
        .layer(TraceLayer::new_for_http())
        .layer(create_cors_layer())
        .layer(CompressionLayer::new())
        .with_state(create_app_state(db_connection_pool).await)
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

fn create_cors_layer() -> CorsLayer {
    let allowed_origin = get_env("CORS_ALLOWED_ORIGIN").parse::<HeaderValue>().unwrap();
    CorsLayer::new().allow_origin(allowed_origin).allow_methods(Any).allow_headers(Any)
}

async fn create_app_state(db_connection_pool: PgPool) -> AppState {
    let link_repository = SqlxLinkRepository::new(db_connection_pool.clone());
    let redirect_base_url = get_env("DEFAULT_REDIRECT_BASE_URL");
    AppState {
        link_repository,
        redirect_base_url,
    }
}

#[derive(Clone)]
pub struct AppState {
    link_repository: SqlxLinkRepository,
    redirect_base_url: String,
}

fn get_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("Environment variable {} is required", name))
}
