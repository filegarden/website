//! See [`Response`].

use axum::{
    body::Body,
    http::{
        self,
        header::{CONTENT_TYPE, LOCATION},
        HeaderName, HeaderValue, StatusCode,
    },
};

/// A wrapper for [`axum::response::Response`] with a simpler API.
pub(crate) struct Response {
    /// The [`axum::response::Response`] value being wrapped.
    inner: axum::response::Response,
}

impl Response {
    /// Constructs a new [`Response`].
    pub(crate) fn new() -> Self {
        Self {
            inner: axum::response::Response::new(Body::empty()),
        }
    }

    /// Sets a [`StatusCode`] on the response.
    pub(crate) fn status(&mut self, status: StatusCode) -> &mut Self {
        *self.inner.status_mut() = status;

        self
    }

    /// Sets a header on the response.
    pub(crate) fn header(&mut self, name: HeaderName, value: HeaderValue) -> &mut Self {
        self.inner.headers_mut().insert(name, value);

        self
    }

    /// Sets a header on the response, panicking if the header value is invalid.
    ///
    /// # Panics
    ///
    /// Panics if the header value isn't valid. For example, passing a string panics if it contains
    /// a character that isn't visible ASCII (32-127).
    pub(crate) fn header_valid<V>(&mut self, name: HeaderName, value: V) -> &mut Self
    where
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        let value = <HeaderValue as TryFrom<V>>::try_from(value)
            .map_err(Into::into)
            .expect("header value should be valid");

        self.header(name, value)
    }

    /// Sets a [`Body`] on the response.
    pub(crate) fn body<T: Into<Body>>(mut self, body: T) -> Self {
        *self.inner.body_mut() = body.into();

        self
    }

    /// Sets the response to a [`308 Permanent
    /// Redirect`](https://developer.mozilla.org/docs/Web/HTTP/Status/308).
    ///
    /// # Panics
    ///
    /// Panics if the location isn't a valid header value. See "Panics" section of
    /// [`Response::header_valid`].
    pub(crate) fn permanent_redirect(mut self, location: &str) -> Self {
        self.status(StatusCode::PERMANENT_REDIRECT)
            .header_valid(LOCATION, location);

        self
    }

    /// Sets a [`StatusCode`], and sets it along with its canonical reason text (e.g. `404 Not
    /// Found`) as a `text/plain` body on the response.
    pub(crate) fn plain_error(mut self, status: StatusCode) -> Self {
        self.status(status).header_valid(CONTENT_TYPE, "text/plain");

        self.body(status.to_string())
    }
}

impl axum::response::IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        self.inner
    }
}
