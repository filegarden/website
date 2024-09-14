//! An HTTP resource representing the set of all user accounts.

use argon2::{
    password_hash::{Salt, SaltString},
    Argon2, PasswordHasher,
};
use axum::http::StatusCode;
use axum_macros::debug_handler;
use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        validate::{Birthdate, UserEmail, UserName, UserPassword},
        Json, Response,
    },
    db,
    id::Id,
};

/// The type to create new user IDs with.
///
/// Note that existing user IDs may not have the same size.
type NewUserId = Id<[u8; 8]>;

/// Hashes and salts a password using Argon2.
///
/// # Errors
///
/// Fails if the CSPRNG fails when generating salt.
fn hash_password(password: &str) -> Result<String, rand::Error> {
    let mut salt = [0; Salt::RECOMMENDED_LENGTH];
    rand::thread_rng().try_fill_bytes(&mut salt)?;

    let salt_string = SaltString::encode_b64(&salt).expect("salt should be valid");

    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt_string)
        .expect("password hashing should be infallible")
        .to_string())
}

/// A `POST` request body for this API route.
#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PostRequest {
    /// The user's email address.
    pub email: UserEmail,

    /// The user's name.
    pub name: UserName,

    /// The user's birthdate, from a string in ISO 8601 date format.
    pub birthdate: Birthdate,

    /// The user's password in plain text.
    pub password: UserPassword,
}

/// Creates a new user and sends them an account verification email.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub async fn post(Json(body): Json<PostRequest>) -> Response<PostResponse> {
    let user_id = NewUserId::generate()?;

    let password_hash = hash_password(&body.password)?;

    sqlx::query!(
        "INSERT INTO users (id, email, name, birthdate, password_hash) VALUES ($1, $2, $3, $4, $5)",
        user_id.as_slice(),
        body.email.as_str(),
        *body.name,
        *body.birthdate,
        password_hash,
    )
    .execute(db::pool())
    .await?;

    Ok((
        StatusCode::OK,
        Json(PostResponse {
            email: body.email,
            name: body.name,
        }),
    ))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostResponse {
    /// The user's email address.
    pub email: UserEmail,

    /// The user's name.
    pub name: UserName,
}
