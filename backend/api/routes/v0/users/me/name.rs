//! The current authenticated user's display name.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{
    api::{self, extract::AuthToken, response::Response, validation::UserName, Json},
    db::{self, TxError, TxResult},
};

/// A `PUT` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PutRequest {
    /// The user's new display name.
    pub name: UserName,
}

/// Changes the current authenticated user's display name.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn put(
    AuthToken(token_hash): AuthToken,
    Json(body): Json<PutRequest>,
) -> impl Response<PutResponse> {
    db::transaction!(async |tx| -> TxResult<_, api::Error> {
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

        sqlx::query!(
            "UPDATE users
                SET name = $1
                WHERE id = $2",
            body.name.as_str(),
            user_id.as_slice(),
        )
        .execute(tx.as_mut())
        .await?;

        Ok(())
    })
    .await?;

    Ok((StatusCode::OK, Json(PutResponse { name: body.name })))
}

/// A `PUT` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PutResponse {
    /// The user's new display name.
    pub name: UserName,
}
