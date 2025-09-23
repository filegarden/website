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
        validation::{Otp, TotpSecret, UserPassword},
    },
    crypto::{generate_short_code, verify_hash, verify_totp},
    db::{self, TxError, TxResult},
};

/// The length of a new backup authentication code.
const BACKUP_CODE_LENGTH: usize = 8;

/// The number of backup authentication codes to generate for a user.
const BACKUP_CODE_COUNT: usize = 10;

/// A `DELETE` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DeleteRequest {
    /// The user's password in plain text.
    pub password: UserPassword,
    // `otp` is notably absent here. Requiring 2FA to disable 2FA would mitigate attacks where your
    // session token and password are both compromised (e.g., by malware that steals browser cookies
    // and autofill data), but it would also lock you out if you lose your 2FA device and backup
    // codes. Both are common, but a lockout is far riskier:
    //
    // - On one hand, being unable to disable 2FA after losing your 2FA device would require proving
    //   account ownership to a support admin, which is prone to social engineering, and the
    //   worst-case scenario is having insufficient proof and losing access permanently.
    // - On the other hand, an attacker disabling 2FA with your session token and password allows
    //   them to take over your account, but malicious changes can be reverted by a support admin
    //   with little risk and no need for proof of ownership.
}

/// Disables TOTP for the current authenticated user.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn delete(
    AuthToken(token_hash): AuthToken,
    Json(body): Json<DeleteRequest>,
) -> impl Response<DeleteResponse> {
    let is_totp_deleted = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let Some(user) = sqlx::query!(
            "SELECT users.id, users.password_hash FROM users
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

        Ok(sqlx::query!(
            "DELETE FROM totp
                WHERE user_id = $1",
            user.id,
        )
        .execute(tx.as_mut())
        .await?
        .rows_affected()
            != 0)
    })
    .await?;

    if !is_totp_deleted {
        return Err(api::Error::ResourceNotFound);
    }

    Ok((StatusCode::OK, Json(DeleteResponse {})))
}

/// A `DELETE` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeleteResponse {}

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PostRequest {
    /// The user's password in plain text.
    pub password: UserPassword,

    /// The user's new TOTP secret. Generated client-side to significantly reduce backend
    /// complexity. A client should ensure it generates this using a CSPRNG.
    pub secret: TotpSecret,

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
            "SELECT users.id, users.password_hash FROM users
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

        if !verify_totp(&body.otp, body.secret.as_ref()) {
            return Err(TxError::Abort(api::Error::OtpWrong));
        }

        let backup_codes: [String; BACKUP_CODE_COUNT] =
            array::from_fn(|_| generate_short_code(BACKUP_CODE_LENGTH));

        match sqlx::query!(
            "INSERT INTO totp (user_id, secret, otp_used_last, unused_backup_codes)
                VALUES ($1, $2, $3, $4)",
            user.id,
            body.secret.as_ref(),
            *body.otp,
            &backup_codes,
        )
        .execute(tx.as_mut())
        .await
        {
            Err(sqlx::Error::Database(error)) if error.constraint() == Some("totp_pkey") => {
                // The user already has a TOTP configuration.
                return Err(TxError::Abort(api::Error::AlreadyExists));
            }
            result => result?,
        };

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
