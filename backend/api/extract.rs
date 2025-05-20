//! Types and traits for extracting data from requests using `axum` extractors.

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::request,
};
use axum_macros::FromRequestParts;
use tower_cookies::Cookies;

use crate::{api, id::Token};

/// Equivalent to [`axum::extract::Query`], but fails with an [`api::Error`] JSON response instead
/// of a plain text response.
#[derive(FromRequestParts, Clone, Copy, Default, Debug)]
#[from_request(via(axum::extract::Query), rejection(api::Error))]
pub(crate) struct Query<T>(pub T);

/// Equivalent to [`axum::extract::Path`], but fails with an [`api::Error`] JSON response instead of
/// a plain text response.
#[derive(FromRequestParts, Debug)]
#[from_request(via(axum::extract::Path), rejection(api::Error))]
pub(crate) struct Path<T>(pub T);

impl<S> FromRequestParts<S> for Token
where
    S: Sync + Send,
{
    type Rejection = api::Error;

    async fn from_request_parts(
        parts: &mut request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .expect("cookies should be extractable");

        let Some(token) = cookies.get("token") else {
            return Err(api::Error::AuthFailed("missing credentials".into()));
        };

        match token.value().parse() {
            Ok(token) => Ok(token),
            Err(error) => Err(api::Error::AuthFailed(format!("invalid token: {error}"))),
        }
    }
}

impl<S> OptionalFromRequestParts<S> for Token
where
    S: Sync + Send,
{
    type Rejection = api::Error;

    async fn from_request_parts(
        parts: &mut request::Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        match <Self as FromRequestParts<_>>::from_request_parts(parts, state).await {
            Ok(token) => Ok(Some(token)),
            Err(api::Error::AuthFailed(_)) => Ok(None),
            Err(error) => Err(error),
        }
    }
}
