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
        validation::{CaptchaToken, EmailVerificationCode, True, UserEmail},
        Json,
    },
    crypto::{hash_without_salt, verify_hash},
    db::{self, TxError, TxResult},
    email::{EmailTakenMessage, MessageTemplate, VerificationMessage},
    id::Token,
    WEBSITE_ORIGIN,
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
pub(crate) async fn get(Query(query): Query<GetQuery>) -> impl Response<GetResponse> {
    let email = match query {
        GetQuery::Token { token } => {
            let token_hash = hash_without_salt(&token);

            let Some(unverified_email) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
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
            let Some(unverified_email) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
                Ok(sqlx::query!(
                    r#"SELECT email, code_hash as "code_hash!" FROM unverified_emails
                        WHERE user_id IS NULL AND email = $1 AND code_hash IS NOT NULL"#,
                    email.as_str(),
                )
                .fetch_optional(tx.as_mut())
                .await?)
            })
            .await?
            .filter(|unverified_email| verify_hash(&code, &unverified_email.code_hash)) else {
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
    #[expect(dead_code, reason = "This isn't dead code; it's used for validation")]
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
pub(crate) async fn post(Json(body): Json<PostRequest>) -> impl Response<PostResponse> {
    // We don't want bots creating accounts or spamming people with verification emails.
    if !captcha::verify(&body.captcha_token).await? {
        return Err(api::Error::CaptchaFailed);
    }

    enum SendMessage {
        /// Sends an [`EmailTakenMessage`].
        EmailTaken {
            /// The name of the existing user who took the email.
            user_name: String,
        },
        /// Sends a [`VerificationMessage`].
        Verification(Token),
    }

    match db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let existing_user = sqlx::query!(
            "SELECT name FROM users
                WHERE email = $1",
            body.email.as_str(),
        )
        .fetch_optional(tx.as_mut())
        .await?;

        if let Some(user) = existing_user {
            return Ok(SendMessage::EmailTaken {
                user_name: user.name,
            });
        }

        // Expire any previous email verification request.
        sqlx::query!(
            "DELETE FROM unverified_emails
                WHERE user_id IS NULL AND email = $1",
            body.email.as_str(),
        )
        .execute(tx.as_mut())
        .await?;

        let token = Token::generate();
        let token_hash = hash_without_salt(&token);

        match sqlx::query!(
            "INSERT INTO unverified_emails (token_hash, email)
                VALUES ($1, $2)",
            token_hash.as_ref(),
            body.email.as_str(),
        )
        .execute(tx.as_mut())
        .await
        {
            Err(sqlx::Error::Database(error))
                if error.constraint() == Some("unverified_emails_pkey") =>
            {
                return Err(TxError::Retry);
            }
            result => result?,
        };

        Ok(SendMessage::Verification(token))
    })
    .await?
    {
        SendMessage::EmailTaken { user_name } => {
            EmailTakenMessage {
                email: body.email.as_str(),
            }
            .to(Mailbox::new(Some(user_name), (*body.email).clone()))
            .send();
        }
        SendMessage::Verification(token) => {
            VerificationMessage {
                email: body.email.as_str(),
                verification_url: &format!("{}/verify-email?token={}", *WEBSITE_ORIGIN, token),
            }
            .to(Mailbox::new(None, (*body.email).clone()))
            .send();
        }
    }

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
