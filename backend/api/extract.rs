//! Types and traits for extracting data from requests using `axum` extractors.

use std::str;

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::{header, request},
};
use axum_macros::FromRequestParts;

use crate::{api, id::Token};

use super::cookie::{CookieWrapper, SessionCookie};

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

/// Extractor for the user's authentication token.
pub(crate) struct AuthToken(pub Token);

impl<S> FromRequestParts<S> for AuthToken
where
    S: Sync + Send,
{
    type Rejection = api::Error;

    async fn from_request_parts(
        parts: &mut request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Some(session_cookie) = parts
            .headers
            .get(header::COOKIE)
            .and_then(|header_value| str::from_utf8(header_value.as_bytes()).ok())
            .and_then(SessionCookie::from_header)
        else {
            return Err(api::Error::AuthFailed("missing credentials".into()));
        };

        match session_cookie.as_ref().value().parse() {
            Ok(token) => Ok(Self(token)),
            Err(error) => Err(api::Error::AuthFailed(format!("invalid token: {error}"))),
        }
    }
}

impl<S> OptionalFromRequestParts<S> for AuthToken
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
