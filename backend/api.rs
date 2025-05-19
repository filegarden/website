//! A web server for the HTTP API. File Garden exposes this via `https://filegarden.com/api/`.

use std::error::Error as _;

use axum::{
    extract::{
        rejection::{JsonRejection, PathRejection, QueryRejection},
        Request,
    },
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::FromRequest;
use routes::ROUTER;
use serde::Serialize;
use strum_macros::IntoStaticStr;
use thiserror::Error;
use tower::ServiceExt;

mod captcha;
mod extract;
mod routes;
mod validation;

/// An API error.
#[derive(Error, IntoStaticStr, Debug)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
enum Error {
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

    /// The request body doesn't match the required target type.
    #[error("Invalid request body: {0}")]
    InvalidBodyData(String),

    /// The request URI path doesn't match the required target type.
    #[error("Invalid URI path: {0}")]
    InvalidPathData(String),

    /// The request URI query doesn't match the required target type.
    #[error("Invalid URI query: {0}")]
    InvalidQueryData(String),

    /// The `Content-Type` header isn't set to `application/json`.
    #[error("Header `Content-Type: application/json` must be set.")]
    JsonContentType,

    /// The JSON syntax is incorrect.
    #[error("Invalid JSON syntax in request body: {0}")]
    JsonSyntax(String),

    /// The requested API route doesn't allow the HTTP method used.
    #[error("The requested API route doesn't allow that HTTP method.")]
    MethodNotAllowed,

    /// The requested API route exists, but the specified resource was not found.
    #[error("Resource not found.")]
    ResourceNotFound,

    /// The requested API route doesn't exist.
    #[error("The requested API route doesn't exist.")]
    RouteNotFound,

    /// Credentials specified in the request (such as email and password) don't match any user.
    #[error("The specified user credentials are incorrect.")]
    UserCredentialsWrong,
}

impl Error {
    /// Gets the HTTP response status code corresponding to the API error.
    const fn status(&self) -> StatusCode {
        match self {
            Self::BodyTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Self::CaptchaFailed => StatusCode::FORBIDDEN,
            Self::EmailVerificationCodeWrong => StatusCode::FORBIDDEN,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidBodyData(_) => StatusCode::BAD_REQUEST,
            Self::InvalidPathData(_) => StatusCode::BAD_REQUEST,
            Self::InvalidQueryData(_) => StatusCode::BAD_REQUEST,
            Self::JsonContentType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Self::JsonSyntax(_) => StatusCode::BAD_REQUEST,
            Self::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
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
                Self::InvalidPathData(match error.source() {
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
                Self::InvalidQueryData(match error.source() {
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
            JsonRejection::JsonDataError(error) => Self::InvalidBodyData(match error.source() {
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

/// Equivalent to [`axum::Json`], but fails with an [`Error`] JSON response instead of a plain text
/// response.
#[derive(FromRequest, Clone, Copy, Default, Debug)]
#[from_request(via(axum::Json), rejection(Error))]
struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

/// An API response type.
trait Response<T>: IntoResponse {}

/// Implements [`Response`] for types with various tuple lengths.
macro_rules! impl_response {
    ( $($ty:ident),* $(,)? ) => {
        // Always require an explicit status code in `Ok` responses so the most appropriate
        // successful status code is more likely to be considered each time.
        impl<R, $($ty,)*> Response<R> for std::result::Result<(StatusCode, $($ty,)* Json<R>), Error>
        where
            std::result::Result<(StatusCode, $($ty,)* Json<R>), Error>: IntoResponse,
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

/// Routes a request to an API endpoint.
pub(super) async fn handle(request: Request) -> axum::response::Response {
    // Calling the router needs a mutable reference to it (even though it shouldn't), so the router
    // must either have restricted access via a mutex or be cloned on each request. The former would
    // allow only one request at a time, so the latter is faster.
    ROUTER.clone().oneshot(request).await.into_response()
}
