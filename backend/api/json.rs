//! See [`Json`].

use axum::response::IntoResponse;
use axum_macros::FromRequest;
use serde::Serialize;

use crate::api;

/// Equivalent to [`axum::Json`], but fails with an [`api::Error`] JSON response instead of a plain
/// text response.
#[derive(FromRequest, Clone, Copy, Default, Debug)]
#[from_request(via(axum::Json), rejection(api::Error))]
pub(crate) struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}
