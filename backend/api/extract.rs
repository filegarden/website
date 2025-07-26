//! Types and traits for extracting data from requests using `axum` extractors.

use std::str;

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::{header::COOKIE, request},
};
use axum_macros::FromRequestParts;
use ring::digest::Digest;

use crate::{api, crypto::hash_without_salt, id::Token};

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

/// Extractor for the user's hashed authentication token.
pub(crate) struct AuthToken(pub Digest);

impl<S> FromRequestParts<S> for AuthToken
where
    S: Sync + Send,
{
    type Rejection = api::Error;

    async fn from_request_parts(
        parts: &mut request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Some(token) = parts
            .headers
            .get(COOKIE)
            .and_then(|header_value| str::from_utf8(header_value.as_bytes()).ok())
            .and_then(SessionCookie::from_header)
            .and_then(|session_cookie| session_cookie.as_ref().value().parse::<Token>().ok())
        else {
            return Err(api::Error::AuthFailed);
        };

        let token_hash = hash_without_salt(&token);

        Ok(Self(token_hash))
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
            Err(api::Error::AuthFailed) => Ok(None),
            Err(error) => Err(error),
        }
    }
}
