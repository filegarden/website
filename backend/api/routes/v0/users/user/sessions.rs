//! The set of a particular user's sign-in sessions.

use axum_macros::debug_handler;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    api::{
        self,
        extract::{AuthToken, Path},
        response::{body::Session, Response},
        validation::UserQuery,
        Json,
    },
    crypto::hash_without_salt,
    db::{self, TxError, TxResult},
    id::Id,
};

pub(crate) mod session;

/// A request path for this API route.
type PathParams = Path<UserQuery>;

/// Lists all of a user's active sessions.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(
    Path(user_query): PathParams,
    AuthToken(token): AuthToken,
) -> impl Response<GetResponse> {
    let token_hash = hash_without_salt(&token);

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

        if let UserQuery::Id(queried_user_id) = &user_query {
            if **queried_user_id != user_id {
                return Err(TxError::Abort(api::Error::AccessDenied));
            }
        }

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
            current_session_id: Vec::from(token_hash.as_ref()).into(),
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
