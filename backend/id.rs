//! See [`Id`].

use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use derive_more::derive::{AsMut, AsRef, Deref, DerefMut};
use rand::RngCore;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use thiserror::Error;

/// The type to create new user IDs with. Note that existing user IDs may not fit this type.
pub(crate) type NewUserId = Id<[u8; 8]>;

/// A 1024-bit token.
pub(crate) type Token = Id<[u8; 128]>;

/// An ID that can be deserialized from and serialized to `base64url` (without padding).
#[derive(
    Deref,
    DerefMut,
    AsRef,
    AsMut,
    DeserializeFromStr,
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
#[as_mut(forward)]
pub(crate) struct Id<T = Vec<u8>>(T);

impl<const N: usize> Id<[u8; N]> {
    /// Generates a cryptographically secure pseudorandom ID.
    ///
    /// Cryptographic security is important so that IDs are unguessable.
    pub(crate) fn generate() -> Self {
        let mut id = Self([0; N]);
        id.reroll();
        id
    }
}

impl<T: AsMut<[u8]>> Id<T> {
    /// Overwrites this ID with a new cryptographically secure pseudorandom ID, reusing the existing
    /// memory.
    pub(crate) fn reroll(&mut self) {
        rand::rng().fill_bytes(self.as_mut());
    }
}

impl<T: AsRef<[u8]>> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", URL_SAFE_NO_PAD.encode(self))
    }
}

impl<T> From<T> for Id<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

/// An error constructing an [`Id`].
#[derive(Error, Clone, Debug)]
#[non_exhaustive]
pub(crate) enum Error {
    /// The ID isn't valid Base64.
    #[error("failed to construct ID from Base64: {0}")]
    Base64(#[from] base64::DecodeError),

    /// The size of the decoded bytes doesn't match the expected size of the ID's type.
    #[error("expected ID to be {expected} bytes, found {found} bytes")]
    Size {
        /// The expected size of the ID's type.
        expected: usize,

        /// The size of the decoded bytes.
        found: usize,
    },
}

impl FromStr for Id<Vec<u8>> {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let bytes = URL_SAFE_NO_PAD.decode(str)?;
        Ok(Self(bytes))
    }
}

impl<const N: usize> FromStr for Id<[u8; N]> {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let bytes: Vec<u8> = URL_SAFE_NO_PAD.decode(str)?;
        let bytes: [u8; N] = bytes.try_into().map_err(|bytes: Vec<u8>| Error::Size {
            expected: N,
            found: bytes.len(),
        })?;

        Ok(Self(bytes))
    }
}
