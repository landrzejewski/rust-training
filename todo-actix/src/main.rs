use std::{env, io};

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::http::header;
use actix_web::middleware::Logger;

use data::AppState;

mod data;
mod handler;

#[actix_web::main]
async fn main() -> io::Result<()> {
    const ACTIX_LOGGER: &str = "RUST_LOG";
    if env::var_os(ACTIX_LOGGER).is_none() {
        unsafe { env::set_var(ACTIX_LOGGER, "actix_web=info"); }
    }
    env_logger::init();
    let app_data = web::Data::new(AppState::new());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(app_data.clone())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
