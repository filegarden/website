//! See [`handle`].

use std::sync::LazyLock;

use axum::{
    extract::Request,
    http::{StatusCode, header::HOST},
    response::{IntoResponse, Response},
};
use axum_macros::debug_handler;

use crate::{CONTENT_ORIGIN, WEBSITE_ORIGIN, api, content, website};

/// The URI authority for user-uploaded content.
static CONTENT_AUTHORITY: LazyLock<&str> = LazyLock::new(|| authority_from_origin(&CONTENT_ORIGIN));

/// The URI authority for the website.
static WEBSITE_AUTHORITY: LazyLock<&str> = LazyLock::new(|| authority_from_origin(&WEBSITE_ORIGIN));

/// Handles all incoming requests and routes them to other services based on the request URI.
#[debug_handler]
pub(super) async fn handle(request: Request) -> Response {
    // The `:authority` pseudo-header must take priority over the `Host` header as per RFC 9113
    // (section 8.3.1).
    let authority = match request.uri().authority() {
        Some(authority) => Some(authority.as_str()),
        None => request
            .headers()
            .get(HOST)
            .and_then(|host| host.to_str().ok()),
    };

    if authority == Some(*CONTENT_AUTHORITY) {
        return content::handle(request).into_response();
    }

    if authority == Some(*WEBSITE_AUTHORITY) {
        if request.uri().path().starts_with("/api/") {
            return api::handle(request).await;
        }

        return website::handle(request).await;
    }

    StatusCode::MISDIRECTED_REQUEST.into_response()
}

/// Gets the URI authority from a URI origin string.
///
/// # Panics
///
/// Panics if the origin string doesn't contain "//".
fn authority_from_origin(origin: &str) -> &str {
    let start = origin.find("//").expect("origin should contain \"//\"") + 2;

    &origin[start..]
}
