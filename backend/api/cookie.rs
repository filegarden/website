//! Utilities for managing cookies.

use std::borrow::Cow;

use axum::http::{header, HeaderName};
use cookie::{time::Duration, Cookie, SameSite};
use derive_more::{AsMut, AsRef, From};

use crate::WEBSITE_ORIGIN;

/// A cookie to identify a user's authentication session.
#[derive(From, AsRef, AsMut, Clone, PartialEq, Debug)]
pub(crate) struct SessionCookie<'c>(Cookie<'c>);

impl CookieWrapper for SessionCookie<'_> {
    const NAME: &'static str = "session";
    const MAX_AGE: Duration = Duration::days(60);
}

/// A trait for a type that wraps a [`Cookie`], adding convenience methods on top of it. Each type
/// implementing this represents a type of cookie with a constant name, as well as other constant
/// cookie attributes.
pub(crate) trait CookieWrapper {
    /// The name of any cookie of this type.
    const NAME: &str;

    /// Whether client scripts should have access to cookies of this type.
    const HTTP_ONLY: bool = true;

    /// How long a cookie of this type should take to expire.
    const MAX_AGE: Duration;

    /// Creates a cookie with the name and attributes of this cookie type, and sets it to the
    /// specified value.
    fn new<'c, V>(value: V) -> Self
    where
        Self: From<Cookie<'c>> + Sized,
        V: Into<Cow<'c, str>>,
    {
        let mut cookie = Cookie::new(Self::NAME, value);
        if Self::HTTP_ONLY {
            cookie.set_http_only(true);
        }
        cookie.set_max_age(Self::MAX_AGE);
        cookie.set_path("/");
        cookie.set_same_site(SameSite::Lax);
        cookie.set_secure(WEBSITE_ORIGIN.starts_with("https:"));
        cookie.into()
    }

    /// Creates an immediately expiring cookie with the name of this cookie type.
    fn expired() -> Self
    where
        Self: From<Cookie<'static>> + Sized,
    {
        let mut cookie = Cookie::new(Self::NAME, "");
        cookie.set_path("/");
        cookie.set_max_age(Duration::ZERO);
        cookie.into()
    }

    /// Finds the cookie of this type (if any) from a full HTTP `Cookie` request header string.
    fn from_header<'c, S>(header_value: S) -> Option<Self>
    where
        Self: From<Cookie<'c>> + Sized,
        S: Into<Cow<'c, str>>,
    {
        for cookie in Cookie::split_parse(header_value) {
            let Ok(cookie) = cookie else {
                continue;
            };

            if cookie.name() != Self::NAME {
                continue;
            }

            return Some(cookie.into());
        }

        None
    }

    /// Generates the name and value of the HTTP `Set-Cookie` response header corresponding to this
    /// cookie.
    fn to_header<'c>(&self) -> (HeaderName, String)
    where
        Self: AsRef<Cookie<'c>>,
    {
        (header::SET_COOKIE, self.as_ref().to_string())
    }
}
