//! The set of email verification requests for new users.

use axum::{extract::State, http::StatusCode};
use axum_macros::debug_handler;
use lettre::message::Mailbox;
use serde::{Deserialize, Serialize};
use sqlx::Acquire;

use crate::{
    api::{
        self, captcha,
        validation::{CaptchaToken, EmailVerificationCode, True, UserEmail},
        Json, Query, Response,
    },
    crypto::{hash_without_salt, verify_hash},
    db::{self, TxResult},
    email::{EmailTakenMessage, MessageTemplate, SendMessage, VerificationMessage},
    id::Token,
    AppState, WEBSITE_ORIGIN,
};

pub(crate) mod code;

/// A `GET` request query for this API route.
#[derive(Deserialize, Debug)]
#[serde(untagged, rename_all = "camelCase")]
pub(crate) enum GetQuery {
    /// Identifies an email verification request by its verification token.
    Token {
        /// The email verification token.
        token: Token,
    },

    /// Identifies an email verification request by its email and verification code.
    EmailAndCode {
        /// The email address to verify.
        email: UserEmail,

        /// The email verification code.
        code: EmailVerificationCode,
    },
}

/// Checks an existing email verification request.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(
    State(state): State<AppState>,
    Query(query): Query<GetQuery>,
) -> Response<GetResponse> {
    let email = match query {
        GetQuery::Token { token } => {
            let token_hash = hash_without_salt(&token);

            let Some(unverified_email) =
                db::transaction!(&state.db_pool, async |tx| -> TxResult<_, api::Error> {
                    Ok(sqlx::query!(
                        "SELECT email FROM unverified_emails
                            WHERE token_hash = $1 AND user_id IS NULL",
                        token_hash.as_ref(),
                    )
                    .fetch_optional(tx.as_mut())
                    .await?)
                })
                .await?
            else {
                return Err(api::Error::ResourceNotFound);
            };

            unverified_email.email
        }

        GetQuery::EmailAndCode { email, code } => {
            let Some(unverified_email) =
                db::transaction!(&state.db_pool, async |tx| -> TxResult<_, api::Error> {
                    Ok(sqlx::query!(
                        r#"SELECT email, code_hash as "code_hash!" FROM unverified_emails
                            WHERE user_id IS NULL AND email = $1 AND code_hash IS NOT NULL"#,
                        email.as_str(),
                    )
                    .fetch_optional(tx.as_mut())
                    .await?)
                })
                .await?
                .filter(|unverified_email| verify_hash(&code, &unverified_email.code_hash))
            else {
                return Err(api::Error::ResourceNotFound);
            };

            unverified_email.email
        }
    };

    Ok((StatusCode::OK, Json(GetResponse { email })))
}

/// A `GET` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetResponse {
    /// The email address to verify.
    pub email: String,
}

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PostRequest {
    /// Whether the user agrees to our terms of service and privacy notice.
    ///
    /// This is part of the API so users can't use the API to get around accepting the terms.
    #[expect(dead_code, reason = "This isn't dead code; it's used for validation.")]
    pub accept_terms: True,

    /// The email address to verify.
    pub email: UserEmail,

    /// A token to verify this request was submitted manually.
    pub captcha_token: CaptchaToken,
}

/// Sends a verification email for a new user if the email isn't already taken by an existing user.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    State(state): State<AppState>,
    Json(body): Json<PostRequest>,
) -> Response<PostResponse> {
    // We don't want bots creating accounts or spamming people with verification emails.
    if !captcha::verify(&body.captcha_token).await? {
        return Err(api::Error::CaptchaFailed);
    }

    db::transaction!(&state.db_pool, async |tx| -> TxResult<_, api::Error> {
        let existing_user = sqlx::query!(
            "SELECT name FROM users
                WHERE email = $1",
            body.email.as_str(),
        )
        .fetch_optional(tx.as_mut())
        .await?;

        if let Some(user) = existing_user {
            EmailTakenMessage {
                email: body.email.as_str(),
            }
            .to(Mailbox::new(Some(user.name), (*body.email).clone()))
            .send();

            return Ok(());
        }

        // Expire any previous email verification request.
        sqlx::query!(
            "DELETE FROM unverified_emails
                WHERE user_id IS NULL AND email = $1",
            body.email.as_str(),
        )
        .execute(tx.as_mut())
        .await?;

        let mut token = Token::generate();

        loop {
            // If this loop's query fails from a token conflict, this savepoint is rolled back to
            // rather than aborting the entire transaction.
            let mut savepoint = tx.begin().await?;

            let token_hash = hash_without_salt(&token);

            match sqlx::query!(
                "INSERT INTO unverified_emails (token_hash, email)
                    VALUES ($1, $2)",
                token_hash.as_ref(),
                body.email.as_str(),
            )
            .execute(savepoint.as_mut())
            .await
            {
                Err(sqlx::Error::Database(error))
                    if error.constraint() == Some("unverified_emails_pkey") =>
                {
                    token.reroll();
                    continue;
                }
                result => result?,
            };

            savepoint.commit().await?;
            break;
        }

        VerificationMessage {
            email: body.email.as_str(),
            verification_url: &format!("{}/verify-email?token={}", *WEBSITE_ORIGIN, token),
        }
        .to(Mailbox::new(None, (*body.email).clone()))
        .send();

        Ok(())
    })
    .await?;

    // To prevent user enumeration, send this same successful response even if the email is taken.
    Ok((StatusCode::OK, Json(PostResponse { email: body.email })))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {
    /// The email address to verify.
    pub email: UserEmail,
}
