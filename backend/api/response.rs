//! Types and traits for API responses.

use std::error::Error as _;

use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
use strum_macros::IntoStaticStr;
use thiserror::Error;

use super::Json;

pub(crate) mod body;

/// An API error.
#[derive(Error, IntoStaticStr, Debug)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub(crate) enum Error {
    /// The user is authenticated but has insufficient permission to access the requested resource
    /// (and possibly to even know whether it exists).
    #[error("You don't have permission to access the requested resource (if it exists).")]
    #[expect(dead_code, reason = "I'll use it in the future.")]
    AccessDenied,

    /// Authentication credentials are required but either unspecified, invalid, or don't match any
    /// user.
    #[error("You must be signed in to access the requested resource.")]
    AuthFailed,

    /// The request body doesn't match the required target type.
    #[error("Invalid request body: {0}")]
    BodyDataInvalid(String),

    /// The request body is too large.
    #[error("The request body is too large.")]
    BodyTooLarge,

    /// CAPTCHA verification failed.
    #[error("CAPTCHA verification failed.")]
    CaptchaFailed,

    /// An email verification code specified in the request is incorrect.
    #[error("Incorrect email verification code.")]
    EmailVerificationCodeWrong,

    /// An internal error occurred on the server which is unknown or expected never to happen.
    ///
    /// For security, this must not expose error details to clients since there's no way to tell if
    /// an arbitrary error is safe to expose.
    #[error("An unexpected internal server error occurred. Please try again.")]
    Internal(#[source] Box<dyn std::error::Error>),

    /// The `Content-Type` header isn't set to `application/json`.
    #[error("Header `Content-Type: application/json` must be set.")]
    JsonContentType,

    /// The JSON syntax is incorrect.
    #[error("Invalid JSON syntax in request body: {0}")]
    JsonSyntax(String),

    /// The requested API route doesn't allow the HTTP method used.
    #[error("The requested API route doesn't allow that HTTP method.")]
    MethodNotAllowed,

    /// The request URI path doesn't match the required target type.
    #[error("Invalid URI path: {0}")]
    PathDataInvalid(String),

    /// The request URI query doesn't match the required target type.
    #[error("Invalid URI query: {0}")]
    QueryDataInvalid(String),

    /// The requested API route exists, but the specified resource was not found.
    #[error("Resource not found.")]
    ResourceNotFound,

    /// The requested API route doesn't exist.
    #[error("The requested API route doesn't exist.")]
    RouteNotFound,

    /// User credentials (such as email and password, not session credentials) specified in the
    /// request don't match any user.
    #[error("The specified user credentials are incorrect.")]
    UserCredentialsWrong,
}

impl Error {
    /// Gets the HTTP response status code corresponding to the API error.
    const fn status(&self) -> StatusCode {
        match self {
            Self::AccessDenied => StatusCode::FORBIDDEN,
            Self::AuthFailed => StatusCode::UNAUTHORIZED,
            Self::BodyDataInvalid(_) => StatusCode::BAD_REQUEST,
            Self::BodyTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Self::CaptchaFailed => StatusCode::FORBIDDEN,
            Self::EmailVerificationCodeWrong => StatusCode::FORBIDDEN,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::JsonContentType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Self::JsonSyntax(_) => StatusCode::BAD_REQUEST,
            Self::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Self::PathDataInvalid(_) => StatusCode::BAD_REQUEST,
            Self::QueryDataInvalid(_) => StatusCode::BAD_REQUEST,
            Self::ResourceNotFound => StatusCode::NOT_FOUND,
            Self::RouteNotFound => StatusCode::NOT_FOUND,
            Self::UserCredentialsWrong => StatusCode::FORBIDDEN,
        }
    }

    /// Gets the API error's code in `SCREAMING_SNAKE_CASE`.
    fn code(&self) -> &'static str {
        self.into()
    }
}

impl From<PathRejection> for Error {
    fn from(error: PathRejection) -> Self {
        match error {
            PathRejection::FailedToDeserializePathParams(_) => {
                Self::PathDataInvalid(match error.source() {
                    Some(source) => source.to_string(),
                    None => error.body_text(),
                })
            }
            error => Self::Internal(error.into()),
        }
    }
}

impl From<QueryRejection> for Error {
    fn from(error: QueryRejection) -> Self {
        match error {
            QueryRejection::FailedToDeserializeQueryString(_) => {
                Self::QueryDataInvalid(match error.source() {
                    Some(source) => source.to_string(),
                    None => error.body_text(),
                })
            }
            error => Self::Internal(error.into()),
        }
    }
}

impl From<JsonRejection> for Error {
    fn from(error: JsonRejection) -> Self {
        if error.status() == StatusCode::PAYLOAD_TOO_LARGE {
            return Self::BodyTooLarge;
        }

        match error {
            JsonRejection::JsonDataError(error) => Self::BodyDataInvalid(match error.source() {
                Some(source) => source.to_string(),
                None => error.body_text(),
            }),
            JsonRejection::JsonSyntaxError(error) => Self::JsonSyntax(match error.source() {
                Some(source) => source.to_string(),
                None => error.body_text(),
            }),
            JsonRejection::MissingJsonContentType(_) => Self::JsonContentType,
            error => Self::Internal(error.into()),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Internal(error.into())
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Internal(error.into())
    }
}

/// An API error's response body.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ErrorBody {
    /// The computer-friendly error code in `SCREAMING_SNAKE_CASE`. See [`Error`] for error codes.
    pub code: &'static str,

    /// The human-friendly error message.
    pub message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let body = ErrorBody {
            code: self.code(),
            message: self.to_string(),
        };

        (self.status(), Json(body)).into_response()
    }
}

/// An API response type.
pub(crate) trait Response<T>: IntoResponse {}

/// Implements [`Response`] for types with various tuple lengths.
macro_rules! impl_response {
    ($($ty:ident),* $(,)?) => {
        // Always require an explicit status code in `Ok` responses so the most appropriate
        // successful status code is more likely to be considered each time.
        impl<R, $($ty,)*> Response<R> for Result<(StatusCode, $($ty,)* Json<R>), Error>
        where
            Result<(StatusCode, $($ty,)* Json<R>), Error>: IntoResponse,
        {
        }
    }
}

impl_response!();
impl_response!(T1);
impl_response!(T1, T2);
impl_response!(T1, T2, T3);
impl_response!(T1, T2, T3, T4);
impl_response!(T1, T2, T3, T4, T5);
impl_response!(T1, T2, T3, T4, T5, T6);
impl_response!(T1, T2, T3, T4, T5, T6, T7);
impl_response!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_response!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
