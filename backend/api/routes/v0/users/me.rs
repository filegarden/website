//! The current authenticated user.

use axum::http::StatusCode;
use axum_macros::debug_handler;

use crate::{
    api::{
        self, Json,
        extract::AuthToken,
        response::{Response, body::User},
    },
    db::{self, TxResult},
};

pub(crate) mod name;
pub(crate) mod password;
pub(crate) mod sessions;
pub(crate) mod settings;
pub(crate) mod totp;

/// Gets the current authenticated user's public profile info.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(AuthToken(token_hash): AuthToken) -> impl Response<GetResponse> {
    let Some(user) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        Ok(sqlx::query!(
            "SELECT users.id, users.name FROM users
                INNER JOIN sessions ON sessions.user_id = users.id
                WHERE sessions.token_hash = $1",
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?)
    })
    .await?
    else {
        return Err(api::Error::AuthFailed);
    };

    Ok((
        StatusCode::OK,
        Json(GetResponse {
            id: user.id.into(),
            name: user.name,
        }),
    ))
}

/// A `GET` response body for this API route.
pub(crate) type GetResponse = User;
