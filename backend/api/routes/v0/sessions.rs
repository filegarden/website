//! The set of users' sign-in sessions.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        self,
        cookie::{CookieWrapper, SessionCookie},
        db_helpers::create_session,
        response::{body::User, Response},
        validation::{UserEmail, UserPassword},
        Json,
    },
    crypto::verify_hash,
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
            // To prevent user enumeration, send this same error response whether it was the email
            // or the password that was incorrect.
            return Err(TxError::Abort(api::Error::UserCredentialsWrong));
        };

        let token: Token = create_session(tx, &user.id).await?;

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
