//! The set of email verification requests for new users.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use lettre::message::Mailbox;
use serde::{Deserialize, Serialize};

use crate::{
    WEBSITE_ORIGIN,
    api::{
        self, Json, captcha,
        extract::Query,
        response::Response,
        validation::{CaptchaToken, UserEmail},
    },
    crypto::hash_without_salt,
    db::{self, TxResult},
    email::{MessageTemplate, PasswordResetFailedMessage, PasswordResetMessage},
    id::Token,
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
            "SELECT users.email FROM users
                INNER JOIN password_resets ON password_resets.user_id = users.id
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

    enum SendMessage<F: FnOnce(), G: FnOnce()> {
        /// Sends a [`PasswordResetFailedMessage`].
        PasswordResetFailed(G),
        /// Sends a [`PasswordResetMessage`].
        PasswordReset(F),
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
            return Ok(SendMessage::PasswordResetFailed(|| {
                PasswordResetFailedMessage {
                    email: body.email.as_str(),
                }
                .to(Mailbox::new(None, body.email.clone().into_inner()))
                .send();
            }));
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

        sqlx::query!(
            "INSERT INTO password_resets (token_hash, user_id)
                VALUES ($1, $2)",
            token_hash.as_ref(),
            user.id,
        )
        .execute(tx.as_mut())
        .await?;

        Ok(SendMessage::PasswordReset(|| {
            let user_name = user.name;
            let token = token;

            PasswordResetMessage {
                email: body.email.as_str(),
                password_reset_url: &format!("{}/reset-password?token={}", *WEBSITE_ORIGIN, token),
            }
            .to(Mailbox::new(
                Some(user_name),
                body.email.clone().into_inner(),
            ))
            .send();
        }))
    })
    .await?
    {
        // Construct and send the message outside the database transaction.
        SendMessage::PasswordResetFailed(send) => send(),
        SendMessage::PasswordReset(send) => send(),
    }

    // To prevent user enumeration, send this same successful response even if the user doesn't
    // exist.
    Ok((StatusCode::OK, Json(PostResponse {})))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {}
