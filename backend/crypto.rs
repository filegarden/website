//! Utilities for cryptographic operations.

use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{Salt, SaltString},
};
use rand::{
    RngCore,
    distr::{Distribution, Uniform},
};
use ring::digest::{Digest, SHA256, digest};
use totp_lite::{Sha1, totp_custom};

/// Hashes the input using SHA-256.
///
/// Salt is necessary for secrets that may be short or guessable, so use [`hash_with_salt`] instead
/// for such inputs.
pub(crate) fn hash_without_salt<T: AsRef<[u8]>>(bytes: &T) -> Digest {
    digest(&SHA256, bytes.as_ref())
}

/// Salts and hashes the input using Argon2, returning a hash in PHC string format.
///
/// Salt is necessary for secrets that may be short or guessable, but it has a drawback: a database
/// can't index salted hashes, since salting and hashing the same input produces a different output
/// each time. If the input can't be a short or guessable secret, use [`hash_without_salt`] instead.
pub(crate) fn hash_with_salt<T: AsRef<[u8]>>(bytes: &T) -> String {
    // `SaltString::generate` would be simpler, but `argon2`'s `rand_core` version is outdated
    // compared to `rand`'s, so that wouldn't accept `rand::rng()`. It accepts its own version of
    // `rand_core::OsRng`, but then `SaltString::generate` would panic if `OsRng` fails, which is
    // much more likely than with `rand::rng()`.
    let mut salt = [0; Salt::RECOMMENDED_LENGTH];
    rand::rng().fill_bytes(&mut salt);
    let salt = SaltString::encode_b64(&salt).expect("salt should be valid");

    Argon2::default()
        .hash_password(bytes.as_ref(), &salt)
        .expect("password hashing should be infallible")
        .to_string()
}

/// Checks if the input bytes match the Argon2 hash specified in PHC string format (as outputted by
/// [`hash_with_salt`]).
///
/// If the hash string is invalid, returns `false`.
pub(crate) fn verify_hash<T: AsRef<[u8]>>(bytes: &T, hash_phc_format: &str) -> bool {
    let Ok(hash) = PasswordHash::new(hash_phc_format) else {
        return false;
    };

    Argon2::default()
        .verify_password(bytes.as_ref(), &hash)
        .is_ok()
}

/// Checks if a TOTP code matches a TOTP secret.
///
/// # Panics
///
/// Panics if the system time is earlier than the Unix epoch.
pub(crate) fn verify_totp<T: AsRef<[u8]>>(totp: &str, secret: &T) -> bool {
    const STEP: u64 = 30;
    const DIGITS: u32 = 6;

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time shouldn't be earlier than Unix epoch")
        .as_secs();

    totp == totp_custom::<Sha1>(STEP, DIGITS, secret.as_ref(), time)
        || totp == totp_custom::<Sha1>(STEP, DIGITS, secret.as_ref(), time - STEP)
}

/// Generates a cryptographically secure pseudorandom string that should be short and easy to type.
pub(crate) fn generate_short_code(length: usize) -> String {
    // `O` is excluded because it's often mistaken for `0`.
    const CHARS: [char; 35] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    Uniform::try_from(0..CHARS.len())
        .expect("`CHARS` should be nonempty and finite")
        .sample_iter(rand::rng())
        .take(length)
        .map(|i| CHARS[i])
        .collect()
}
