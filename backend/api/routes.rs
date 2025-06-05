//! All routes for the HTTP API.

use std::sync::LazyLock;

use axum::{
    routing::{delete, get, post},
    Router,
};

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
    let v0_router = Router::new()
        .route(
            "/email-verification",
            get(v0::email_verification::get).post(v0::email_verification::post),
        )
        .route(
            "/email-verification/code",
            post(v0::email_verification::code::post),
        )
        .route(
            "/password-reset",
            get(v0::password_reset::get).post(v0::password_reset::post),
        )
        .route(
            "/password-reset/password",
            post(v0::password_reset::password::post),
        )
        .route("/sessions", post(v0::sessions::post))
        .route("/users", post(v0::users::post))
        .route("/users/{user_id}", get(v0::users::user::get))
        .route(
            "/users/{user_id}/sessions",
            get(v0::users::user::sessions::get),
        )
        .route(
            "/users/{user_id}/sessions/{session_id}",
            delete(v0::users::user::sessions::session::delete),
        )
        .fallback(|| async { api::Error::RouteNotFound })
        .method_not_allowed_fallback(|| async { api::Error::MethodNotAllowed });

    Router::new().nest("/api/v0", v0_router)
});
