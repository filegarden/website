//! The set of all users.

use axum::http::{StatusCode, header::LOCATION};
use axum_macros::debug_handler;
use serde::Deserialize;

use crate::{
    api::{
        self, Json,
        cookie::{CookieWrapper, SessionCookie},
        db_helpers::create_session,
        response::{Response, body::User},
        validation::{EmailVerificationCode, NewUserPassword, UserEmail, UserName},
    },
    crypto::{hash_with_salt, hash_without_salt, verify_hash},
    db::{self, TxError, TxResult},
    id::{NewUserId, Token},
};

pub(crate) mod me;
pub(crate) mod user;

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostRequest {
    /// Information that verifies the user's email address.
    #[serde(flatten)]
    pub email_verification: EmailVerification,

    /// The user's name.
    pub name: UserName,

    /// The user's new password in plain text.
    pub password: NewUserPassword,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub(crate) enum EmailVerification {
    /// Email verification using a code.
    Code {
        /// The user's email address.
        email: UserEmail,

        /// The verification code for the user's email address.
        #[serde(rename = "emailVerificationCode")]
        code: EmailVerificationCode,
    },

    /// Email verification using a token.
    Token {
        /// The verification token for the user's email address.
        #[serde(rename = "emailVerificationToken")]
        token: Token,
    },
}

/// Creates a new user. Signs in the user if successful.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(Json(body): Json<PostRequest>) -> impl Response<PostResponse> {
    let password_hash = hash_with_salt(&body.password);

    let (user_id, session_token) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let (accepted_terms_at, email) = match &body.email_verification {
            EmailVerification::Code { email, code } => {
                let Some(unverified_user) = sqlx::query!(
                    "DELETE FROM unverified_users
                        WHERE email = $1
                        RETURNING accepted_terms_at, code_hash",
                    email.as_str(),
                )
                .fetch_optional(tx.as_mut())
                .await?
                else {
                    return Err(TxError::Abort(api::Error::EmailVerificationWrong));
                };

                let does_code_match = unverified_user
                    .code_hash
                    .is_some_and(|code_hash| verify_hash(&code, &code_hash));

                if !does_code_match {
                    return Err(TxError::Abort(api::Error::EmailVerificationWrong));
                }

                (unverified_user.accepted_terms_at, email.to_string())
            }
            EmailVerification::Token { token } => {
                let token_hash = hash_without_salt(&token);

                let Some(unverified_user) = sqlx::query!(
                    "DELETE FROM unverified_users
                        WHERE token_hash = $1
                        RETURNING accepted_terms_at, email",
                    token_hash.as_ref(),
                )
                .fetch_optional(tx.as_mut())
                .await?
                else {
                    return Err(TxError::Abort(api::Error::EmailVerificationWrong));
                };

                (unverified_user.accepted_terms_at, unverified_user.email)
            }
        };

        sqlx::query!(
            "DELETE FROM email_change_requests
                WHERE email = $1",
            email,
        )
        .execute(tx.as_mut())
        .await?;

        let user_id = NewUserId::generate();

        match sqlx::query!(
            "INSERT INTO users (accepted_terms_at, id, email, name, password_hash)
                VALUES ($1, $2, $3, $4, $5)",
            accepted_terms_at,
            user_id.as_slice(),
            email.as_str(),
            *body.name,
            password_hash,
        )
        .execute(tx.as_mut())
        .await
        {
            Err(sqlx::Error::Database(error)) if error.constraint() == Some("users_pkey") => {
                return Err(TxError::Retry);
            }
            result => result?,
        };

        let session_token = create_session(tx, &user_id).await?;

        Ok((user_id, session_token))
    })
    .await?;

    Ok((
        StatusCode::CREATED,
        [
            (LOCATION, format!("/api/v0/users/{user_id}")),
            SessionCookie::new(session_token.to_string()).to_header(),
        ],
        Json(PostResponse {
            id: user_id,
            name: body.name.into_inner(),
        }),
    ))
}

/// A `POST` response body for this API route.
pub(crate) type PostResponse = User<[u8; 8]>;
