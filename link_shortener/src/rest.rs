use crate::responses::{created, internal_server_error, no_content, ok, png_image, temporary_redirect, with_status_and_error};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::Json;
use axum_extra::extract::Query;
use common::generators::generate_qr_code;
use common::id::Id;
use common::pagination::PageRequest;
use link_shortener::{
    create_link, create_link_with_generated_path, delete_link, find_link_by_shortened_path, find_links, find_links_by_tags, update_link, CharSet, Error, Link, LinkPatch,
};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use url::Url;

const DEFAULT_SHORTENED_PATH_LENGTH: usize = 6;
const MAX_SHORTENED_PATH_LENGTH: usize = 30;
const DEFAULT_PAGE_INDEX: usize = 0;
const DEFAULT_PAGE_SIZE: usize = 10;
const MAX_TAG_COUNT: usize = 16;

pub async fn create_link_handler(State(state): State<AppState>, Query(params): Query<CreateLinkRequestParams>, Json(request): Json<CreateLinkRequest>) -> Response {
    let is_active = request.is_active.unwrap_or(true);
    match request.shortened_path {
        Some(shortened_path) if shortened_path.len() > MAX_SHORTENED_PATH_LENGTH => return with_status_and_error(StatusCode::BAD_REQUEST, "Shortened path is too long"),
        Some(shortened_path) => create_link(request.original_url, shortened_path, is_active, request.expires_at, &state.link_repository).await,
        None => {
            let shortened_path_length = params.length.unwrap_or(DEFAULT_SHORTENED_PATH_LENGTH);
            if shortened_path_length > MAX_SHORTENED_PATH_LENGTH {
                return with_status_and_error(StatusCode::BAD_REQUEST, "Shortened path is too long");
            }
            let char_set = CharSet::try_from(params.char_set.unwrap_or_default()).unwrap_or(CharSet::Digits);
            create_link_with_generated_path(request.original_url, char_set, shortened_path_length, is_active, request.expires_at, &state.link_repository).await
        }
    }
    .map(|link| {
        let location_uri = format!("links/{}", &link.id.to_str());
        let shortened_path = link.shortened_path;
        created(location_uri, CreateLinksResponse { shortened_path })
    })
    .unwrap_or_else(|error| match error {
        Error::NonUniqueShortenedPath => with_status_and_error(StatusCode::BAD_REQUEST, "Non-unique shortened path"),
        _ => internal_server_error(),
    })
}

pub async fn find_links_handler(State(state): State<AppState>, Query(params): Query<FindLinksRequestParams>) -> Response {
    let index = params.page_number.unwrap_or(DEFAULT_PAGE_INDEX);
    let size = params.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
    let page_request = PageRequest { index, size };
    let tags = params.tags.unwrap_or_default();
    if tags.is_empty() {
        find_links(&page_request, &state.link_repository).await
    } else {
        find_links_by_tags(&tags, &page_request, &state.link_repository).await
    }
    .map(|page| ok(page.map(FindLinksResponse::from)))
    .unwrap_or_else(|_| internal_server_error())
}

pub async fn update_link_handler(State(state): State<AppState>, Path(id): Path<String>, Json(request): Json<UpdateLinkRequest>) -> Response {
    let tags = request.tags.unwrap_or_default();
    if tags.len() > MAX_TAG_COUNT {
        return with_status_and_error(StatusCode::BAD_REQUEST, "Too many tags");
    }
    let patch = LinkPatch {
        original_url: request.original_url,
        is_active: request.is_active,
        tags,
        expires_at: request.expires_at,
    };
    update_link(&Id::from(id), &patch, &state.link_repository)
        .await
        .map(|_| no_content())
        .unwrap_or_else(|error| match error {
            Error::LinkNotFound => with_status_and_error(StatusCode::BAD_REQUEST, "Link not found"),
            _ => internal_server_error(),
        })
}

pub async fn delete_link_handler(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    delete_link(&Id::from(id), &state.link_repository)
        .await
        .map(|_| no_content())
        .unwrap_or_else(|_| internal_server_error())
}

pub async fn redirect_handler(State(state): State<AppState>, Path(shortened_path): Path<String>) -> Response {
    match find_link_by_shortened_path(&shortened_path, &state.link_repository).await {
        Ok(Some(link)) => temporary_redirect(link.original_url.as_str()),
        Ok(None) => temporary_redirect(format!("{}/{}", state.redirect_base_url, shortened_path).as_str()),
        Err(_) => internal_server_error(),
    }
}

pub async fn generate_qr_code_handler(State(state): State<AppState>, Path(shortened_path): Path<String>) -> Response {
    let link = match find_link_by_shortened_path(&shortened_path, &state.link_repository).await {
        Ok(Some(link)) => link,
        Ok(None) => return with_status_and_error(StatusCode::BAD_REQUEST, "Link not found"),
        Err(_) => return internal_server_error(),
    };
    generate_qr_code(link.original_url.as_str(), 600, 600)
        .map(|qr_code| png_image(qr_code))
        .unwrap_or_else(|_| internal_server_error())
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateLinkRequest {
    original_url: Url,
    shortened_path: Option<String>,
    is_active: Option<bool>,
    expires_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateLinkRequestParams {
    char_set: Option<String>,
    length: Option<usize>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLinksResponse {
    pub shortened_path: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindLinksRequestParams {
    page_number: Option<usize>,
    page_size: Option<usize>,
    tags: Option<Vec<String>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindLinksResponse {
    pub id: String,
    pub original_url: String,
    pub shortened_path: String,
    pub is_active: bool,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl From<Link> for FindLinksResponse {
    fn from(link: Link) -> Self {
        FindLinksResponse {
            id: link.id.to_str().to_string(),
            original_url: link.original_url.to_string(),
            shortened_path: link.shortened_path,
            is_active: link.is_active,
            tags: link.tags,
            created_at: link.created_at,
            expires_at: link.expires_at,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLinkRequest {
    original_url: Option<Url>,
    is_active: Option<bool>,
    tags: Option<Vec<String>>,
    expires_at: Option<DateTime<Utc>>,
}
