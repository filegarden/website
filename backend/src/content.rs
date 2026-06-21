//! A web server for user-uploaded content. File Garden exposes this via `https://file.garden/`.

use std::borrow::Cow;

use axum::{
    extract::Request,
    http::{
        Method, StatusCode,
        header::{ACCESS_CONTROL_ALLOW_ORIGIN, ALLOW, CONTENT_SECURITY_POLICY},
    },
};
use percent_encoding::{percent_decode_str, utf8_percent_encode};

use crate::{WEBSITE_ORIGIN, percent_encoding::COMPONENT_IGNORING_SLASH, response::Response};

/// The start of a file ID query parameter.
const FILE_ID_QUERY_PREFIX: &str = "_id=";

/// The service function to handle incoming requests for user-uploaded content.
pub(super) fn handle(request: Request) -> Response {
    let (request, _body) = request.into_parts();
    let mut response = Response::new();

    response
        .header_valid(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header_valid(
            CONTENT_SECURITY_POLICY,
            "default-src 'self' 'unsafe-eval' 'unsafe-inline' blob: data: mediastream:",
        );

    if !(request.method == Method::GET || request.method == Method::HEAD) {
        let status = if request.method == Method::OPTIONS {
            StatusCode::NO_CONTENT
        } else {
            StatusCode::METHOD_NOT_ALLOWED
        };

        response
            .status(status)
            .header_valid(ALLOW, "GET, HEAD, OPTIONS");

        return response;
    }

    let encoded_path = request.uri.path();

    if encoded_path == "/" {
        return response.permanent_redirect(format!("{}/", *WEBSITE_ORIGIN).as_str());
    }

    let Ok(path) = percent_decode_str(encoded_path).decode_utf8() else {
        return response.plain_error(StatusCode::BAD_REQUEST);
    };

    // The above can decode `%00` into a null byte, so disallow null bytes as a defensive measure.
    if path.contains('\x00') {
        return response.plain_error(StatusCode::BAD_REQUEST);
    }

    let normalized_encoded_path: Cow<str> =
        utf8_percent_encode(&path, COMPONENT_IGNORING_SLASH).into();

    let query = request.uri.query();

    if encoded_path != normalized_encoded_path {
        // Redirect to the same URI with normalized path encoding. This reduces how many URLs must
        // be purged from the CDN's cache when a file changes. It's impossible to purge every
        // possible variation of encoding for a URL.

        let normalized_uri = concat_path_and_query(&normalized_encoded_path, query);

        return response.permanent_redirect(&normalized_uri);
    }

    let Some((user_identifier, file_path)) = path
        .strip_prefix('/')
        .expect("path should start with `/`")
        .split_once('/')
    else {
        return response.plain_error(StatusCode::BAD_REQUEST);
    };

    let file_id = match query {
        Some(query) => query
            .split('&')
            .find_map(|param| param.strip_prefix(FILE_ID_QUERY_PREFIX)),
        None => None,
    };

    // response
    //     .header_valid(CONTENT_LENGTH, 0)
    //     .header_valid(CONTENT_TYPE, "")
    //     .header_valid(LAST_MODIFIED, "");

    if request.method == Method::HEAD {
        return response;
    }

    response.body(format!(
        "{user_identifier} - {file_path} - {}",
        file_id.unwrap_or("None")
    ))
}

/// Joins a path and a query into one string, separated by a `?` if there exists a query.
fn concat_path_and_query<'a>(path: &'a str, query: Option<&'a str>) -> Cow<'a, str> {
    let mut path_and_query = Cow::from(path);

    if let Some(query) = query {
        path_and_query.to_mut().push('?');
        path_and_query.to_mut().push_str(query);
    }

    path_and_query
}
