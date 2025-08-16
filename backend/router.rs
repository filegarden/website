//! See [`handle`].

use std::sync::LazyLock;

use axum::{
    extract::Request,
    http::{StatusCode, header::HOST},
    response::{IntoResponse, Response},
};
use axum_macros::debug_handler;

use crate::{CONTENT_ORIGIN, WEBSITE_ORIGIN, api, content, website};

/// The URI host for user-uploaded content.
static CONTENT_HOST: LazyLock<&str> = LazyLock::new(|| host_from_origin(&CONTENT_ORIGIN));

/// The URI host for the website.
static WEBSITE_HOST: LazyLock<&str> = LazyLock::new(|| host_from_origin(&WEBSITE_ORIGIN));

/// Handles all incoming requests and routes them to other services based on the request URI.
#[debug_handler]
pub(super) async fn handle(request: Request) -> Response {
    let host = request
        .headers()
        .get(HOST)
        .and_then(|host| host.to_str().ok());

    if host == Some(*CONTENT_HOST) {
        return content::handle(request).into_response();
    }

    if host == Some(*WEBSITE_HOST) {
        if request.uri().path().starts_with("/api/") {
            return api::handle(request).await;
        }

        return website::handle(request).await;
    }

    StatusCode::MISDIRECTED_REQUEST.into_response()
}

/// Returns the host from an origin URI string.
///
/// # Panics
///
/// Panics if the origin string doesn't contain "//".
fn host_from_origin(origin: &str) -> &str {
    let start = origin.find("//").expect("origin should contain \"//\"") + 2;

    &origin[start..]
}
