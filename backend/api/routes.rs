//! All routes for the HTTP API.

use std::sync::LazyLock;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::api;

mod v0 {
    //! The routes for version 1 of the HTTP API.

    pub(crate) mod password_reset;
    pub(crate) mod sessions;
    pub(crate) mod user_requests;
    pub(crate) mod users;
}

/// The API router.
pub(super) static ROUTER: LazyLock<Router> = LazyLock::new(|| {
    let v0_router = Router::new()
        .route(
            "/password-reset",
            get(v0::password_reset::get).post(v0::password_reset::post),
        )
        .route(
            "/password-reset/password",
            post(v0::password_reset::password::post),
        )
        .route("/sessions", post(v0::sessions::post))
        .route(
            "/user-requests",
            get(v0::user_requests::get).post(v0::user_requests::post),
        )
        .route(
            "/user-requests/{token}",
            get(v0::user_requests::user_request::get),
        )
        .route(
            "/user-requests/{token}/code",
            post(v0::user_requests::user_request::code::post),
        )
        .route("/users", post(v0::users::post))
        .route("/users/me", get(v0::users::me::get))
        .route("/users/me/name", put(v0::users::me::name::put))
        .route(
            "/users/me/password/verify",
            post(v0::users::me::password::verify::post),
        )
        .route("/users/me/sessions", get(v0::users::me::sessions::get))
        .route(
            "/users/me/sessions/{session_id}",
            delete(v0::users::me::sessions::session::delete),
        )
        .route("/users/me/settings", get(v0::users::me::settings::get))
        .route(
            "/users/me/totp",
            delete(v0::users::me::totp::delete).post(v0::users::me::totp::post),
        )
        .route(
            "/users/me/totp-request",
            post(v0::users::me::totp_request::post),
        )
        .route("/users/{user_id}", get(v0::users::user::get))
        .fallback(|| async { api::Error::RouteNotFound })
        .method_not_allowed_fallback(|| async { api::Error::MethodNotAllowed });

    Router::new().nest("/api/v0", v0_router)
});
