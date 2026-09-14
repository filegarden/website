//! Serves files under the `.well-known` directory as defined in RFC 8615.

use askama::Template;
use axum::{
    extract::Request,
    http::{Method, header},
    response::{IntoResponse, Response},
};
use chrono::{Duration, NaiveTime, SecondsFormat, Utc};
use reqwest::StatusCode;

use crate::{CONTENT_ORIGIN, WEBSITE_ORIGIN};

/// The template for our `security.txt` file.
#[derive(Template, Debug)]
#[template(path = "well-known/security.txt")]
pub(crate) struct SecurityTxt {
    /// See RFC 9116, section 2.5.5.
    pub(crate) expires: String,
}

/// Serves the `/.well-known/security.txt` file as defined in RFC 9116.
pub(crate) fn security_txt(request: &Request) -> Response {
    let headers = [(header::CONTENT_TYPE, "text/plain; charset=utf-8")];
    let allow_header = (header::ALLOW, "GET, HEAD, OPTIONS");

    match *request.method() {
        Method::GET => {}
        Method::HEAD => return headers.into_response(),
        Method::OPTIONS => return (StatusCode::NO_CONTENT, [allow_header]).into_response(),
        _ => return (StatusCode::METHOD_NOT_ALLOWED, [allow_header]).into_response(),
    }

    let expires = (Utc::now() + Duration::days(14))
        .with_time(NaiveTime::MIN)
        .single()
        .expect("the datetime should be in range");

    let content = SecurityTxt {
        expires: expires.to_rfc3339_opts(SecondsFormat::Secs, true),
    }
    .to_string();

    (headers, content).into_response()
}
