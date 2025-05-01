//! All routes for the HTTP API.

use std::sync::LazyLock;

use axum::{
    routing::{get, post},
    Router,
};
use tower_cookies::CookieManagerLayer;

use crate::api;

mod v1 {
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
            "/api/v1/email-verification",
            get(v1::email_verification::get).post(v1::email_verification::post),
        )
        .route(
            "/api/v1/email-verification/code",
            post(v1::email_verification::code::post),
        )
        .route(
            "/api/v1/password-reset",
            get(v1::password_reset::get).post(v1::password_reset::post),
        )
        .route(
            "/api/v1/password-reset/password",
            post(v1::password_reset::password::post),
        )
        .route("/api/v1/sessions", post(v1::sessions::post))
        .route("/api/v1/users", post(v1::users::post))
        .fallback(|| async { api::Error::RouteNotFound })
        .layer(CookieManagerLayer::new())
});
