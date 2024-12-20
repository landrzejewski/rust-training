use axum::body::Body;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

const CACHE_CONTROL_HEADER_VALUE: &str = "public, max-age=300, s-maxage=300, stale-while-revalidate=300, stale-if-error=300";
const RESPONSE_BUILD_FAILED: &str = "Response build failed";

pub fn created<T: Serialize>(location_uri: String, body: T) -> Response {
    Response::builder()
        .status(StatusCode::CREATED)
        .header(header::LOCATION, location_uri)
        .body(to_json(body))
        .expect(RESPONSE_BUILD_FAILED)
}

pub fn ok<T: Serialize>(body: T) -> Response {
    Json(body).into_response()
}

pub fn png_image(data: Vec<u8>) -> Response {
    ([(header::CONTENT_TYPE, "image/png")], data).into_response()
}

pub fn no_content() -> Response {
    with_status(StatusCode::NO_CONTENT)
}

pub fn internal_server_error() -> Response {
    with_status(StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn temporary_redirect(target_url: &str) -> Response {
    Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header(header::LOCATION, target_url)
        .header(header::CACHE_CONTROL, CACHE_CONTROL_HEADER_VALUE)
        .body(Body::empty())
        .expect(RESPONSE_BUILD_FAILED)
}

pub fn with_status(status_code: StatusCode) -> Response {
    Response::builder().status(status_code).body(Body::empty()).expect(RESPONSE_BUILD_FAILED)
}

pub fn with_status_and_error(status_code: StatusCode, message: &str) -> Response {
    let body = ErrorBody::new(message);
    Response::builder().status(status_code).body(to_json(body)).expect(RESPONSE_BUILD_FAILED)
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub message: String,
}

impl ErrorBody {
    fn new(message: &str) -> Self {
        Self { message: message.into() }
    }
}

fn to_json<T: Serialize>(body: T) -> Body {
    Json(body).into_response().into_body()
}
