//! A TOTP verification request for the current authenticated user.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use percent_encoding::utf8_percent_encode;
use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::{
    api::{self, Json, extract::AuthToken, response::Response, validation::UserPassword},
    crypto::verify_hash,
    db::{self, TxError, TxResult},
    percent_encoding::COMPONENT,
};

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PostRequest {
    /// The user's password in plain text.
    pub password: UserPassword,
}

/// Submits a request to enable TOTP for the current authenticated user.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    AuthToken(token_hash): AuthToken,
    Json(body): Json<PostRequest>,
) -> impl Response<PostResponse> {
    let (email, secret) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let Some(user) = sqlx::query!(
            "SELECT users.id, users.password_hash, users.email FROM users
                INNER JOIN sessions ON sessions.user_id = users.id
                WHERE sessions.token_hash = $1",
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            return Err(TxError::Abort(api::Error::AuthFailed));
        };

        if !verify_hash(&body.password, &user.password_hash) {
            return Err(TxError::Abort(api::Error::UserCredentialsWrong));
        }

        sqlx::query!(
            "DELETE FROM totp
                WHERE user_id = $1 AND unverified_secret IS NOT NULL",
            user.id,
        )
        .execute(tx.as_mut())
        .await?;

        // RFC 4226 (section 4) recommends TOTP secrets be 160 bits.
        let mut secret = [0; 20];
        rand::rng().fill_bytes(secret.as_mut());

        match sqlx::query!(
            "INSERT INTO totp (user_id, unverified_secret)
                VALUES ($1, $2)",
            user.id,
            &secret,
        )
        .execute(tx.as_mut())
        .await
        {
            Err(sqlx::Error::Database(error)) if error.constraint() == Some("totp_pkey") => {
                // The user already has a verified TOTP configuration.
                return Err(TxError::Abort(api::Error::AlreadyExists));
            }
            result => result?,
        };

        Ok((user.email, secret))
    })
    .await?;

    let issuer = utf8_percent_encode("File Garden", COMPONENT);
    let account_name = utf8_percent_encode(&email, COMPONENT);
    let secret = base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &secret);

    let otpauth_uri =
        format!("otpauth://totp/{issuer}:{account_name}?secret={secret}&issuer={issuer}");

    Ok((StatusCode::CREATED, Json(PostResponse { otpauth_uri })))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {
    /// The new TOTP key URI using the `otpauth` scheme.
    otpauth_uri: String,
}
