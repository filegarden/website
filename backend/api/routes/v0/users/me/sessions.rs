//! The set of a particular user's sign-in sessions.

use axum_macros::debug_handler;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    api::{
        self,
        extract::AuthToken,
        response::{body::Session, Response},
        Json,
    },
    db::{self, TxError, TxResult},
    id::Id,
};

pub(crate) mod session;

/// Lists all of a user's active sessions.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(AuthToken(token_hash): AuthToken) -> impl Response<GetResponse> {
    let sessions = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let Some(user_id) = sqlx::query!(
            "SELECT user_id FROM sessions
                WHERE token_hash = $1",
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        .map(|session| session.user_id) else {
            return Err(TxError::Abort(api::Error::AuthFailed));
        };

        Ok(sqlx::query_as!(
            Session,
            "SELECT token_hash AS id, created_at, accessed_at FROM sessions
                WHERE user_id = $1",
            user_id,
        )
        .fetch_all(tx.as_mut())
        .await?)
    })
    .await?;

    Ok((
        StatusCode::OK,
        Json(GetResponse {
            current_session_id: token_hash.as_ref().to_owned().into(),
            values: sessions,
        }),
    ))
}

/// A `GET` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetResponse {
    /// The ID of the user's current session.
    current_session_id: Id,

    /// All of the user's active sessions.
    values: Vec<Session>,
}
