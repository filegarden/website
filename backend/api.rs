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
    // Calling the router requires either consuming it or a mutable reference to it (even though it
    // shouldn't), so the router must be either cloned on each request or restricted by a mutex. The
    // latter would allow only one request at a time, so the former is faster.
    ROUTER.clone().oneshot(request).await.into_response()
}
