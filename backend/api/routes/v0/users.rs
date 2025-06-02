//! The set of all users.

use axum::http::{header, StatusCode};
use axum_macros::debug_handler;
use serde::Deserialize;

use crate::{
    api::{
        self,
        cookie::{CookieWrapper, SessionCookie},
        db_helpers::create_session,
        response::{body::User, Response},
        validation::{EmailVerificationCode, NewUserPassword, UserEmail, UserName},
        Json,
    },
    crypto::{hash_with_salt, verify_hash},
    db::{self, TxError, TxResult},
    id::NewUserId,
};

pub(crate) mod user;

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PostRequest {
    /// The user's email address.
    pub email: UserEmail,

    /// The verification code for the user's email address.
    pub email_verification_code: EmailVerificationCode,

    /// The user's name.
    pub name: UserName,

    /// The user's new password in plain text.
    pub password: NewUserPassword,
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
        let Some(unverified_email) = sqlx::query!(
            "DELETE FROM unverified_emails
                WHERE user_id IS NULL AND email = $1
                RETURNING user_accepted_terms_at, code_hash",
            body.email.as_str(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            return Err(TxError::Abort(api::Error::EmailVerificationCodeWrong));
        };

        let does_code_match = unverified_email
            .code_hash
            .is_some_and(|code_hash| verify_hash(&body.email_verification_code, &code_hash));

        if !does_code_match {
            return Err(TxError::Abort(api::Error::EmailVerificationCodeWrong));
        }

        let user_id = NewUserId::generate();

        match sqlx::query!(
            "INSERT INTO users (accepted_terms_at, id, email, name, password_hash)
                VALUES ($1, $2, $3, $4, $5)",
            unverified_email.user_accepted_terms_at,
            user_id.as_slice(),
            body.email.as_str(),
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
            (header::LOCATION, format!("/api/v0/users/{user_id}")),
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
