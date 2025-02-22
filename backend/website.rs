//! A web server for the website. File Garden exposes this via `https://filegarden.com/`.

use std::sync::LazyLock;

use axum::{
    body::Body,
    extract::Request,
    http::{
        uri::{Authority, Scheme},
        StatusCode,
    },
    response::{IntoResponse, Response},
};

/// The local address of the internal server for the website.
static INTERNAL_ADDRESS: LazyLock<Authority> = LazyLock::new(|| {
    dotenvy::var("INTERNAL_WEBSITE_ADDRESS")
        .expect("environment variable `INTERNAL_WEBSITE_ADDRESS` should be a valid string")
        .parse()
        .expect("environment variable `INTERNAL_WEBSITE_ADDRESS` should be a valid URI authority")
});

/// The client for connecting to the internal server for the website.
static INTERNAL_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("internal website client should build")
});

/// The service function to handle incoming requests for the website, proxying them to the website's
/// internal server.
pub(super) async fn handle(request: Request) -> Response {
    let (mut request_parts, request_body) = request.into_parts();

    let mut uri_parts = request_parts.uri.into_parts();
    uri_parts.scheme = Some(Scheme::HTTP);
    uri_parts.authority = Some(INTERNAL_ADDRESS.clone());

    request_parts.uri = uri_parts
        .try_into()
        .expect("URI should still be valid after changing scheme and authority");

    let request: reqwest::Request = Request::from_parts(
        request_parts,
        reqwest::Body::wrap_stream(request_body.into_data_stream()),
    )
    .try_into()
    .expect("internal website request should be valid");

    let Ok(response) = INTERNAL_CLIENT.execute(request).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let mut response_builder = Response::builder().status(response.status());
    *response_builder
        .headers_mut()
        .expect("builder should be ok") = response.headers().clone();
    response_builder
        .body(Body::from_stream(response.bytes_stream()))
        .expect("internal website response should be valid")
}
