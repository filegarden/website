//! Utilities to help with API request validation.

use std::{borrow::Cow, str::FromStr};

use derive_more::derive::{AsRef, Deref, Display};
use idna::uts46::{self, Uts46};
use lettre::Address;
use regex_macro::regex;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use thiserror::Error;

pub(crate) mod auth;

/// A user's name.
pub(crate) type UserName = BoundedString<1, 64>;

/// A user's new password in plain text.
pub(crate) type NewUserPassword = BoundedString<8, 256>;

/// A user's password in plain text.
pub(crate) type UserPassword = BoundedString<0, 256>;

/// A user's TOTP or backup authentication code.
pub(crate) type Otp = BoundedString<6, 8>;

/// An unverified email's verification code.
pub(crate) type EmailVerificationCode = BoundedString<6, 6>;

/// A CAPTCHA token.
pub(crate) type CaptchaToken = BoundedString<1, 2048>;

/// A [`String`] newtype that guarantees its length is within a certain range.
#[derive(
    Deref,
    AsRef,
    Display,
    Deserialize,
    SerializeDisplay,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
)]
#[as_ref(forward)]
#[serde(try_from = "String")]
pub(crate) struct BoundedString<const MIN: usize, const MAX: usize>(String);

impl<const MIN: usize, const MAX: usize> BoundedString<MIN, MAX> {
    /// Consumes the [`BoundedString`], returning the inner [`String`].
    pub(crate) fn into_inner(self) -> String {
        self.0
    }
}

/// An error constructing a [`BoundedString`].
#[derive(Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum BoundedStringError<const MIN: usize, const MAX: usize> {
    /// The length was less than the [`BoundedString`]'s `MIN`.
    #[error("invalid length {0}, expected at least {MIN}")]
    TooShort(usize),

    /// The length was greater than the [`BoundedString`]'s `MAX`.
    #[error("invalid length {0}, expected at most {MAX}")]
    TooLong(usize),
}

impl<const MIN: usize, const MAX: usize> TryFrom<String> for BoundedString<MIN, MAX> {
    type Error = BoundedStringError<MIN, MAX>;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() < MIN {
            Err(BoundedStringError::TooShort(string.len()))
        } else if string.len() > MAX {
            Err(BoundedStringError::TooLong(string.len()))
        } else {
            Ok(Self(string))
        }
    }
}

/// A [`bool`] newtype that guarantees the Boolean is `true`.
#[derive(
    Display, Deserialize, SerializeDisplay, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
#[serde(try_from = "bool")]
pub(crate) struct True;

/// An error constructing a [`True`].
#[derive(Error, Clone, Copy, Debug)]
#[error("expected true")]
pub(crate) struct TrueError;

impl TryFrom<bool> for True {
    type Error = TrueError;

    fn try_from(value: bool) -> Result<Self, Self::Error> {
        if !value {
            return Err(TrueError);
        }

        Ok(Self)
    }
}

/// A user-inputted email address. Ensures the address uses a domain name with a TLD, and normalizes
/// the domain name (for non-ASCII characters).
#[derive(
    Deref,
    AsRef,
    Display,
    DeserializeFromStr,
    Serialize,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
)]
#[as_ref(forward)]
pub(crate) struct UserEmail(Address);

impl UserEmail {
    /// The maximum length of a [`UserEmail`].
    ///
    /// As per RFC 3696 erratum 1690, the theoretical maximum is 254.
    pub(crate) const MAX_LENGTH: usize = 254;

    /// Gets a reference to the email address string.
    pub(crate) fn as_str(&self) -> &str {
        self.as_ref()
    }

    /// Consumes the [`UserEmail`], returning the inner [`Address`].
    #[expect(dead_code, reason = "This should be part of the interface regardless")]
    pub(crate) fn into_inner(self) -> Address {
        self.0
    }
}

/// An error constructing a [`UserEmail`].
#[derive(Error, Copy, Clone, Debug)]
#[non_exhaustive]
pub(crate) enum UserEmailError {
    /// The email address was invalid.
    #[error("invalid email address")]
    Invalid,

    /// The domain part was an IP address rather than a domain name. There's no reason to let users
    /// use IP addresses in emails; strict mail agents don't even allow it.
    #[error("IP addresses not allowed in email address")]
    IpAddr,
}

impl FromStr for UserEmail {
    type Err = UserEmailError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.len() > Self::MAX_LENGTH {
            return Err(UserEmailError::Invalid);
        }

        let Some((user, domain)) = str.rsplit_once('@') else {
            return Err(UserEmailError::Invalid);
        };

        if domain.starts_with('[') {
            return Err(UserEmailError::IpAddr);
        }

        let (domain, domain_result) = Uts46::new().to_user_interface(
            // These are the recommended arguments for this function.
            domain.as_bytes(),
            uts46::AsciiDenyList::URL,
            uts46::Hyphens::Allow,
            |_, _, _| true,
        );

        if domain_result.is_err() {
            return Err(UserEmailError::Invalid);
        }

        let user = normalize_email_address_user(user);
        let domain = domain.to_lowercase();

        let Ok(address) = Address::new(user, domain) else {
            return Err(UserEmailError::Invalid);
        };

        Ok(Self(address))
    }
}

/// Normalizes an email address's user portion by removing unnecessary quotes and escapes.
fn normalize_email_address_user(user: &str) -> Cow<'_, str> {
    let Some(unquoted_user) = user
        .strip_prefix('"')
        .and_then(|user| user.strip_suffix('"'))
    else {
        return Cow::Borrowed(user);
    };

    // Remove unnecessary escapes. Only double-quotes and backslashes need to be escaped as per RFC
    // 5321 (section 4.1.2).
    let unquoted_user = regex!(r#"(\\["\\])|\\"#).replace_all(unquoted_user, "$1");

    // Remove unnecessary quotes.
    if unquoted_user.chars().all(|char| {
        matches!(
            char,
            // All characters a user portion can have unquoted as per RFC 6531 (section 3.3).
            '.' | 'A'..='Z' | 'a'..='z' | '0'..='9' | '!' | '#' | '$' | '%' | '&' | '\'' | '*' | '+'
                | '-' | '/' | '=' | '?' | '^' | '_' | '`' | '{' | '|' | '}' | '~' | '\u{0080}'..,
        )
    }) {
        return unquoted_user;
    }

    // The quotes are necessary, so re-quote it.
    if *unquoted_user == user[1..user.len() - 1] {
        Cow::Borrowed(user)
    } else {
        Cow::Owned(format!("\"{unquoted_user}\""))
    }
}

/// A TOTP secret decoded from RFC-4648-compliant Base32. Must be 160 bits as recommended by RFC
/// 4226 (section 4).
#[derive(Deref, AsRef, DeserializeFromStr, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct TotpSecret([u8; 20]);

impl TotpSecret {
    /// The number of Base32 characters needed to encode a `TotpSecret`.
    const ENCODED_LENGTH: usize = 32;
}

/// An error constructing a [`TotpSecret`].
#[derive(Error, Copy, Clone, Debug)]
#[non_exhaustive]
pub(crate) enum TotpSecretError {
    /// The Base32 encoding was invalid.
    #[error("invalid Base32 encoding (according to RFC 4648)")]
    Base32,

    /// The Base32 string length is incorrect.
    #[error("invalid encoded string length {}, expected {}", .0, TotpSecret::ENCODED_LENGTH)]
    EncodedLength(usize),

    /// The decoded number of bytes is incorrect.
    #[error("invalid decoded byte count {}, expected {}", .0, size_of::<TotpSecret>())]
    DecodedLength(usize),
}

impl FromStr for TotpSecret {
    type Err = TotpSecretError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        // Check the encoded length preemptively to avoid the cost of decoding very long inputs.
        if str.len() != Self::ENCODED_LENGTH {
            return Err(TotpSecretError::EncodedLength(str.len()));
        }

        let Some(bytes) = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, str) else {
            return Err(TotpSecretError::Base32);
        };

        match bytes.try_into() {
            Ok(bytes) => Ok(Self(bytes)),
            Err(bytes) => Err(TotpSecretError::DecodedLength(bytes.len())),
        }
    }
}

#[cfg(test)]
#[expect(clippy::missing_errors_doc, reason = "See rust-lang/rust-clippy#13391")]
mod tests {
    use super::*;

    #[test]
    fn invalid_user_emails() {
        let invalid_emails = [
            "invalid",
            "invalid@@example.com",
            "invalid@invalid@example.com",
            "user@example-.com",
            "more-than-64-characters-in-the-local-part-is-toooooooooooooo-long@example.com",
            "more-than-254-characters-total-is-tooo-long@example.example.example.example.example.example.example.example.example.example.example.example.example.example.example.example.example.example.example.example.example.example.example.example.example.example.com",
            "user with spaces@example.com",
            ".leading-dot@example.com",
            "trailing-dot.@example.com",
            "adjacent..dots@example.com",
            "\"\"@example.com",
            "\"user-with\nline-break\"@example.com",
            "\"user-with-unbalanced\"quotes\"@example.com",
            "user-with-\"balanced-quotes\"-in-middle@example.com",
            "user-with\\backslash@example.com",
            "user-with-unquoted-escaped\\ special-character@example.com",
            // While technically allowed by RFC 5322, the following forms aren't allowed by RFC 5321
            // and aren't required by any standards-compliant mail server, so there's no reason to
            // allow them. Users submitting these are most likely trying to break things.
            "\"user\".name@example.com",
            "user.\"name\"@example.com",
            "\"user\".\"name\"@example.com",
            " user.name@example.com",
            "user .name@example.com",
            "user. name@example.com",
            "user.name @example.com",
            "user.name@ example.com",
            "user.name@example .com",
            "user.name@example. com",
            "user.name@example.com ",
            "(comment)user@example.com",
            "user(comment)@example.com",
            "user@(comment)example.com",
            "user@example(comment).com",
            "user@example.(comment)com",
            "user@example.com(comment)",
            // The following forms using IP addresses in brackets are valid, but we disallow users
            // from using IP addresses to reduce our attack surface.
            "user@[127.0.0.1]",
            "user@[::1]",
        ];

        for email in invalid_emails {
            assert!(
                email.parse::<UserEmail>().is_err(),
                "user email {email:?} should be invalid",
            );
        }
    }

    #[test]
    fn valid_user_emails() {
        let valid_emails = [
            "user-of-a-mail-server-on-a-tld@com",
            "64-characters-in-the-local-part-is-fiiiiiiiiiiiiiiiiiiiiiiiiiine@example.com",
            "\"unnecessarily.quoted.user\"@example.com",
            "\"quoted @ sign\"@example.com",
            "\"quoted user with unnecessary \\escapes\"@example.com",
            "\"quoted user with unnecessary\\ escapes on special characters\"@example.com",
            "\"quoted user with spaces and \\\" escapes\"@example.com",
            // While not allowed by RFC 5321 or RFC 5322, the following forms are allowed with the
            // SMTPUTF8 extension specified by RFC 6531.
            "non-ASCII-in-user-📂🌱@example.com",
            "non-ASCII-in-domain@📂🌱.example.com",
        ];

        for email in valid_emails {
            email
                .parse::<UserEmail>()
                .unwrap_or_else(|error| panic!("user email {email:?} should be valid: {error}"));
        }
    }

    /// Ensures users can't sign up multiple times with different forms of the same email.
    #[test]
    fn user_email_normalization() -> anyhow::Result<()> {
        // The user portion isn't all lowercase or all uppercase when normalized because RFC 5321
        // (section 2.3.11) lets mail servers treat the user portion case-sensitively.
        let normalized_email = "USER.name@examplé.com";

        let equivalent_emails = [
            normalized_email,
            "USER.name@example\u{0301}.com",
            "USER.name@EXAMPLÉ.COM",
            "USER.name@EXAMPLE\u{0301}.COM",
            "USER.name@xn--exampl-gva.com",
            "USER.name@xN--eXaMpL-gVa.CoM",
            r#""USER.name"@examplé.com"#,
            r#""\USER\.name"@examplé.com"#,
        ];

        for email in equivalent_emails {
            assert_eq!(
                normalized_email,
                email.parse::<UserEmail>()?.as_str(),
                "normalizing {email:?}",
            );
        }

        let normalized_email = r#""a \\\" b\\."@example.com"#;

        let equivalent_emails = [
            normalized_email,
            r#""\a \\\" b\\."@example.com"#,
            r#""\a\ \\\" b\\\."@example.com"#,
        ];

        for email in equivalent_emails {
            assert_eq!(
                normalized_email,
                email.parse::<UserEmail>()?.as_str(),
                "normalizing {email:?}",
            );
        }

        Ok(())
    }
}
