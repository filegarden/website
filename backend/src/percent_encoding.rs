//! Module for handling the [`percent_encoding`] crate.

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};

/// All ASCII characters in the [component percent-encode
/// set](https://url.spec.whatwg.org/#component-percent-encode-set).
///
/// Using this with [`percent_encoding::utf8_percent_encode`] works the same as JavaScript's
/// [`encodeURIComponent`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent).
pub(crate) const COMPONENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'!')
    .remove(b'~')
    .remove(b'*')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')');

/// The set of [`COMPONENT`] ASCII characters, but with `/` excluded.
///
/// Using this with [`percent_encoding::utf8_percent_encode`] works the same as JavaScript's
/// [`encodeURIComponent`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent),
/// with the exception that `/` characters are left alone rather than percent-encoded.
pub(crate) const COMPONENT_IGNORING_SLASH: &AsciiSet = &COMPONENT.remove(b'/');
