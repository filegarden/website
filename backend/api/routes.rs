//! All routes for the HTTP API.

use std::sync::LazyLock;

use axum::{
    routing::{get, post},
    Router,
};
use tower_cookies::CookieManagerLayer;

use crate::api;

mod v0 {
    //! The routes for version 1 of the HTTP API.

    pub(crate) mod email_verification;
    pub(crate) mod password_reset;
    pub(crate) mod sessions;
    pub(crate) mod users;
}

/// The API router.
pub(super) static ROUTER: LazyLock<Router> = LazyLock::new(|| {
    Router::new()
        .route(
            "/api/v0/email-verification",
            get(v0::email_verification::get).post(v0::email_verification::post),
        )
        .route(
            "/api/v0/email-verification/code",
            post(v0::email_verification::code::post),
        )
        .route(
            "/api/v0/password-reset",
            get(v0::password_reset::get).post(v0::password_reset::post),
        )
        .route(
            "/api/v0/password-reset/password",
            post(v0::password_reset::password::post),
        )
        .route("/api/v0/sessions", post(v0::sessions::post))
        .route("/api/v0/users", post(v0::users::post))
        .fallback(|| async { api::Error::RouteNotFound })
        .layer(CookieManagerLayer::new())
});
