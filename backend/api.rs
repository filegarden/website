//! A web server for the HTTP API. File Garden exposes this via `https://filegarden.com/api/`.

use axum::{extract::Request, response::IntoResponse};
use tower::ServiceExt;

pub(crate) use json::Json;
pub(crate) use response::Error;
use routes::ROUTER;

mod captcha;
mod cookie;
mod db_helpers;
mod extract;
mod json;
mod response;
mod routes;
mod validation;

/// Routes a request to an API endpoint.
pub(super) async fn handle(request: Request) -> axum::response::Response {
    // Calling the router requires consuming it (even though it shouldn't), so the router must be
    // cloned on each request.
    ROUTER.clone().oneshot(request).await.into_response()
}
