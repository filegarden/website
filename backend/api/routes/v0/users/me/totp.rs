//! The current authenticated user's TOTP configuration.

use std::array;

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        self, Json,
        extract::AuthToken,
        response::Response,
        validation::{Otp, UserPassword},
    },
    crypto::{generate_short_code, verify_hash, verify_totp},
    db::{self, TxError, TxResult},
};

/// The length of a new backup authentication code.
const BACKUP_CODE_LENGTH: usize = 8;

/// The number of backup authentication codes to generate for a user.
const BACKUP_CODE_COUNT: usize = 10;

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PostRequest {
    /// The user's password in plain text.
    pub password: UserPassword,

    /// The user's TOTP authentication code.
    pub otp: Otp,
}

/// Completes a TOTP verification request, enabling TOTP for the current authenticated user.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    AuthToken(token_hash): AuthToken,
    Json(body): Json<PostRequest>,
) -> impl Response<PostResponse> {
    let backup_codes = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let Some(user) = sqlx::query!(
            "SELECT users.id, users.password_hash, totp.unverified_secret as totp_unverified_secret
                FROM users
                INNER JOIN sessions ON sessions.user_id = users.id
                LEFT JOIN totp ON totp.user_id = users.id
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

        let Some(unverified_secret) = user.totp_unverified_secret else {
            return Err(TxError::Abort(api::Error::ResourceNotFound));
        };

        if !verify_totp(&body.otp, &unverified_secret) {
            return Err(TxError::Abort(api::Error::OtpWrong));
        }

        let backup_codes: [String; BACKUP_CODE_COUNT] =
            array::from_fn(|_| generate_short_code(BACKUP_CODE_LENGTH));

        sqlx::query!(
            "UPDATE totp
                SET secret = unverified_secret,
                    code_used_last = $1,
                    unused_backup_codes = $2,
                    unverified_secret = NULL",
            *body.otp,
            &backup_codes,
        )
        .execute(tx.as_mut())
        .await?;

        Ok(backup_codes)
    })
    .await?;

    Ok((StatusCode::CREATED, Json(PostResponse { backup_codes })))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {
    /// The TOTP backup authentication codes.
    backup_codes: [String; BACKUP_CODE_COUNT],
}
