//! An HTTP resource representing the set of all user accounts.

use argon2::{
    password_hash::{Salt, SaltString},
    Argon2, PasswordHasher,
};
use axum::http::StatusCode;
use axum_macros::debug_handler;
use lettre::Address;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use time::Date;

use crate::{
    api::{
        validate::{deserialize_date, UserName, UserPassword},
        Json, Response,
    },
    db,
    id::Id,
};

/// A `POST` request body for this API route.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PostRequest {
    /// The user's email address.
    pub email: Address,

    /// The user's name.
    pub name: UserName,

    /// The user's birthdate, from a string in ISO 8601 date format.
    #[serde(deserialize_with = "deserialize_date")]
    pub birthdate: Date,

    /// The user's password in plain text.
    pub password: UserPassword,
}

/// The type to create new user IDs with.
type NewUserId = Id<[u8; 8]>;

/// A `POST` response body for this API route.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostResponse {
    /// The user's ID.
    pub id: NewUserId,
}

/// Creates a new user and sends them an account verification email.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub async fn post(Json(body): Json<PostRequest>) -> Response<PostResponse> {
    let user_id = NewUserId::generate()?;

    let salt_string = SaltString::encode_b64(&{
        let mut salt = [0_u8; Salt::RECOMMENDED_LENGTH];
        rand::thread_rng().try_fill_bytes(&mut salt)?;
        salt
    })
    .expect("salt should be valid");

    let password_hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt_string)
        .expect("password hashing should be infallible")
        .to_string();

    sqlx::query!(
        "INSERT INTO users (id, email, name, birthdate, password_hash) VALUES ($1, $2, $3, $4, $5)",
        &*user_id,
        body.email.to_string(),
        *body.name,
        body.birthdate,
        password_hash,
    )
    .execute(db::pool())
    .await?;

    Ok((StatusCode::CREATED, Json(PostResponse { id: user_id })))
}
