//! Types and traits for extracting data from requests using `axum` extractors.

use axum_macros::FromRequestParts;

use crate::api;

/// Equivalent to [`axum::extract::Query`], but fails with an [`api::Error`] JSON response instead
/// of a plain text response.
#[derive(FromRequestParts, Clone, Copy, Default, Debug)]
#[from_request(via(axum::extract::Query), rejection(api::Error))]
pub(super) struct Query<T>(pub T);

/// Equivalent to [`axum::extract::Path`], but fails with an [`api::Error`] JSON response instead of
/// a plain text response.
#[derive(FromRequestParts, Debug)]
#[from_request(via(axum::extract::Path), rejection(api::Error))]
pub(super) struct Path<T>(pub T);
