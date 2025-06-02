//! The set of users' sign-in sessions.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        self,
        cookie::{CookieWrapper, SessionCookie},
        response::{body::User, Response},
        validation::{UserEmail, UserPassword},
        Json,
    },
    crypto::{hash_without_salt, verify_hash},
    db::{self, TxError, TxResult},
    id::Token,
};

pub(crate) mod session;

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PostRequest {
    /// The email address of the user signing in.
    pub email: UserEmail,

    /// The user's password in plain text.
    pub password: UserPassword,
}

/// Signs a user in, creating a sign-in session and returning a session cookie.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(Json(body): Json<PostRequest>) -> impl Response<PostResponse> {
    let (token, user_id, user_name) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let Some(user) = sqlx::query!(
            "SELECT id, password_hash, name FROM users
                WHERE email = $1",
            body.email.as_str(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        .filter(|user| verify_hash(&body.password, &user.password_hash)) else {
            // To prevent user enumeration, send this same error response whether or not the email
            // is correct.
            return Err(TxError::Abort(api::Error::UserCredentialsWrong));
        };

        let mut token = Token::generate();
        let token_hash = hash_without_salt(&token);

        match sqlx::query!(
            "INSERT INTO sessions (token_hash, user_id)
                VALUES ($1, $2)",
            token_hash.as_ref(),
            user.id,
        )
        .execute(tx.as_mut())
        .await
        {
            Err(sqlx::Error::Database(error)) if error.constraint() == Some("sessions_pkey") => {
                return Err(TxError::Retry);
            }
            result => result?,
        };

        Ok((token, user.id, user.name))
    })
    .await?;

    // To reduce the session token's attack surface, it isn't included in the response. It's set as
    // an `HttpOnly` cookie instead so browser scripts can't access it.
    Ok((
        StatusCode::OK,
        [SessionCookie::new(token.to_string()).to_header()],
        Json(PostResponse {
            user: User {
                id: user_id.into(),
                name: user_name,
            },
        }),
    ))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {
    /// The newly authenticated user.
    user: User,
}
