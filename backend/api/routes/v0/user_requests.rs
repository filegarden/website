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
        validation::{CaptchaToken, EmailVerificationCode, True, UserEmail},
    },
    crypto::{hash_without_salt, verify_hash},
    db::{self, TxResult},
    email::{MessageTemplate, SignUpEmailTakenMessage, SignUpVerificationMessage},
    id::Token,
};

pub(crate) mod user_request;

/// A `GET` request query for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetQuery {
    /// The email address to verify.
    email: UserEmail,

    /// The email verification code.
    code: EmailVerificationCode,
}

/// Checks an existing email verification request.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(Query(query): Query<GetQuery>) -> impl Response<GetResponse> {
    let Some(unverified_user) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        Ok(sqlx::query!(
            r#"SELECT email, code_hash as "code_hash!" FROM unverified_users
                WHERE email = $1 AND code_hash IS NOT NULL"#,
            query.email.as_str(),
        )
        .fetch_optional(tx.as_mut())
        .await?)
    })
    .await?
    .filter(|unverified_user| verify_hash(&query.code, &unverified_user.code_hash)) else {
        return Err(api::Error::ResourceNotFound);
    };

    Ok((
        StatusCode::OK,
        Json(GetResponse {
            email: unverified_user.email,
        }),
    ))
}

/// A `GET` response body for this API route.
pub(crate) type GetResponse = user_request::GetResponse;

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

    enum SendMessage<F: FnOnce(), G: FnOnce()> {
        /// Sends a [`SignUpEmailTakenMessage`].
        EmailTaken(F),
        /// Sends a [`SignUpVerificationMessage`].
        Verification(G),
    }

    match db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let taken_user = sqlx::query!(
            "SELECT name FROM users
                WHERE email = $1",
            body.email.as_str(),
        )
        .fetch_optional(tx.as_mut())
        .await?;

        if let Some(taken_user) = taken_user {
            return Ok(SendMessage::EmailTaken(|| {
                SignUpEmailTakenMessage {
                    email: body.email.as_str(),
                }
                .to(Mailbox::new(
                    Some(taken_user.name),
                    body.email.clone().into_inner(),
                ))
                .send();
            }));
        }

        // Expire any previous email verification request.
        sqlx::query!(
            "DELETE FROM unverified_users
                WHERE email = $1",
            body.email.as_str(),
        )
        .execute(tx.as_mut())
        .await?;

        let token = Token::generate();
        let token_hash = hash_without_salt(&token);

        sqlx::query!(
            "INSERT INTO unverified_users (token_hash, email)
                VALUES ($1, $2)",
            token_hash.as_ref(),
            body.email.as_str(),
        )
        .execute(tx.as_mut())
        .await?;

        Ok(SendMessage::Verification(|| {
            let token = token;

            SignUpVerificationMessage {
                email: body.email.as_str(),
                verification_url: &format!("{}/verify-email?token={}", *WEBSITE_ORIGIN, token),
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
