//! The set of email verification requests for new users.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use lettre::message::Mailbox;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        self, captcha,
        extract::Query,
        response::Response,
        validation::{CaptchaToken, UserEmail},
        Json,
    },
    crypto::hash_without_salt,
    db::{self, TxError, TxResult},
    email::{MessageTemplate, PasswordResetFailedMessage, PasswordResetMessage},
    id::Token,
    WEBSITE_ORIGIN,
};

pub(crate) mod password;

/// A `GET` request query for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetQuery {
    /// The password reset token.
    token: Token,
}

/// Checks an existing password reset request.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(Query(query): Query<GetQuery>) -> impl Response<GetResponse> {
    let token_hash = hash_without_salt(&query.token);

    let Some(password_reset) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        Ok(sqlx::query!(
            "SELECT users.email
                FROM password_resets JOIN users ON users.id = password_resets.user_id
                WHERE password_resets.token_hash = $1",
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?)
    })
    .await?
    else {
        return Err(api::Error::ResourceNotFound);
    };

    Ok((
        StatusCode::OK,
        Json(GetResponse {
            email: password_reset.email,
        }),
    ))
}

/// A `GET` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetResponse {
    /// The email of the user whose password reset was requested.
    pub email: String,
}

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PostRequest {
    /// The email address of the user to request a password reset for.
    pub email: UserEmail,

    /// A token to verify this request was submitted manually.
    pub captcha_token: CaptchaToken,
}

/// Sends a password reset request to the specified email. If there is no user associated with the
/// email, a failure notification email is sent instead.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(Json(body): Json<PostRequest>) -> impl Response<PostResponse> {
    // We don't want bots spamming people with password reset emails.
    if !captcha::verify(&body.captcha_token).await? {
        return Err(api::Error::CaptchaFailed);
    }

    enum SendMessage {
        /// Sends a [`PasswordResetMessage`].
        PasswordReset {
            /// The password reset token.
            token: Token,
            /// The name of the user to send a password reset request.
            user_name: String,
        },
        /// Sends a [`PasswordResetFailedMessage`].
        PasswordResetFailed,
    }

    match db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let Some(user) = sqlx::query!(
            "SELECT id, name FROM users
                WHERE email = $1",
            body.email.as_str(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            return Ok(SendMessage::PasswordResetFailed);
        };

        // Expire any previous password reset request.
        sqlx::query!(
            "DELETE FROM password_resets
                WHERE user_id = $1",
            user.id,
        )
        .execute(tx.as_mut())
        .await?;

        let token = Token::generate();
        let token_hash = hash_without_salt(&token);

        match sqlx::query!(
            "INSERT INTO password_resets (token_hash, user_id)
                VALUES ($1, $2)",
            token_hash.as_ref(),
            user.id,
        )
        .execute(tx.as_mut())
        .await
        {
            Err(sqlx::Error::Database(error))
                if error.constraint() == Some("password_resets_pkey") =>
            {
                return Err(TxError::Retry);
            }
            result => result?,
        };

        Ok(SendMessage::PasswordReset {
            token,
            user_name: user.name,
        })
    })
    .await?
    {
        SendMessage::PasswordReset { token, user_name } => {
            PasswordResetMessage {
                email: body.email.as_str(),
                password_reset_url: &format!("{}/reset-password?token={}", *WEBSITE_ORIGIN, token),
            }
            .to(Mailbox::new(Some(user_name), (*body.email).clone()))
            .send();
        }
        SendMessage::PasswordResetFailed => {
            PasswordResetFailedMessage {
                email: body.email.as_str(),
            }
            .to(Mailbox::new(None, (*body.email).clone()))
            .send();
        }
    }

    // To prevent user enumeration, send this same successful response even if the user doesn't
    // exist.
    Ok((StatusCode::OK, Json(PostResponse {})))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {}
