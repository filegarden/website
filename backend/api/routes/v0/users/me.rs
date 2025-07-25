//! A user.

use axum::http::StatusCode;
use axum_macros::debug_handler;

use crate::{
    api::{
        self,
        extract::AuthToken,
        response::{body::User, Response},
        Json,
    },
    db::{self, TxResult},
};

pub(crate) mod sessions;
pub(crate) mod settings;

/// Gets a user's public profile info.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(AuthToken(token_hash): AuthToken) -> impl Response<GetResponse> {
    let Some(user) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        Ok(sqlx::query!(
            "SELECT users.id, users.name FROM sessions
                INNER JOIN users ON users.id = sessions.user_id
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
