//! The active email change request for the current authenticated user, if any.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use lettre::message::Mailbox;
use serde::{Deserialize, Serialize};

use crate::{
    WEBSITE_ORIGIN,
    api::{
        self, Json,
        extract::AuthToken,
        response::Response,
        validation::{
            UserEmail,
            auth::{FirstFactorCredentials, VerifyCredentials},
        },
    },
    crypto::hash_without_salt,
    db::{self, TxError, TxResult},
    email::{EmailChangeTakenMessage, EmailChangeVerificationMessage, MessageTemplate},
    id::Token,
};

// ⚠️ There are intentionally no `GET` or `DELETE` handlers here because allowing a user to tell if
// their email change request still exists may enable user enumeration. If a feature to view or
// cancel a previous email change request is added in the future, tests should be added to ensure
// the email change request initially failing (due to an existing user) or becoming invalid (due to
// a user verifying the same email for a sign-up or email change) is not revealed to the requester.

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PostRequest {
    /// The user's credentials.
    pub credentials: FirstFactorCredentials,

    /// The new email address to verify.
    pub email: UserEmail,
}

/// Sends a verification email for an email change if the new email isn't already taken by an
/// existing user.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    AuthToken(token_hash): AuthToken,
    Json(body): Json<PostRequest>,
) -> impl Response<PostResponse> {
    enum SendMessage<F: FnOnce(), G: FnOnce()> {
        /// Sends an [`EmailChangeTakenMessage`].
        EmailTaken(F),
        /// Sends an [`EmailChangeVerificationMessage`].
        Verification(G),
    }

    match db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let Some(user) = sqlx::query!(
            "SELECT users.id, users.email FROM users
                INNER JOIN sessions ON sessions.user_id = users.id
                WHERE sessions.token_hash = $1",
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            return Err(TxError::Abort(api::Error::AuthFailed));
        };

        body.credentials.verify(tx, &user.id).await?;

        let taken_user = sqlx::query!(
            "SELECT name FROM users
                WHERE email = $1",
            body.email.as_str(),
        )
        .fetch_optional(tx.as_mut())
        .await?;

        if let Some(taken_user) = taken_user {
            return Ok(SendMessage::EmailTaken(|| {
                let current_email = user.email;

                EmailChangeTakenMessage {
                    current_email: &current_email,
                    requested_email: body.email.as_str(),
                }
                .to(Mailbox::new(
                    Some(taken_user.name),
                    body.email.clone().into_inner(),
                ))
                .send();
            }));
        }

        // Expire any previous email change request from the same user.
        sqlx::query!(
            "DELETE FROM email_change_requests
                WHERE user_id = $1",
            user.id,
        )
        .execute(tx.as_mut())
        .await?;

        let token = Token::generate();
        let token_hash = hash_without_salt(&token);

        sqlx::query!(
            "INSERT INTO email_change_requests (token_hash, user_id, email)
                VALUES ($1, $2, $3)",
            token_hash.as_ref(),
            user.id,
            body.email.as_str(),
        )
        .execute(tx.as_mut())
        .await?;

        Ok(SendMessage::Verification(|| {
            let current_email = user.email;
            let token = token;

            EmailChangeVerificationMessage {
                current_email: &current_email,
                requested_email: body.email.as_str(),
                verification_url: &format!("{}/change-email?token={}", *WEBSITE_ORIGIN, token),
            }
            .to(Mailbox::new(None, body.email.clone().into_inner()))
            .send();
        }))
    })
    .await?
    {
        // Construct and send the message outside the database transaction.
        SendMessage::EmailTaken(send) => send(),
        SendMessage::Verification(send) => send(),
    }

    // To prevent user enumeration, send this same successful response even if the email is taken.
    Ok((StatusCode::OK, Json(PostResponse { email: body.email })))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {
    /// The (normalized) email address to verify.
    pub email: UserEmail,
}
