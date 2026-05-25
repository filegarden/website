//! A folder's shared state.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::Serialize;

use crate::{
    api::{
        self, Json,
        extract::{AuthToken, Path},
        response::Response,
    },
    db::{self, TxError, TxResult},
    id::Id,
};

/// A request path for this API route.
type PathParams = Path<Id>;

/// Shares a folder.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    Path(folder_id): PathParams,
    AuthToken(token_hash): AuthToken,
) -> impl Response<PostResponse> {
    db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let Some(session) = sqlx::query!(
            "SELECT user_id FROM sessions
                WHERE token_hash = $1",
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            return Err(TxError::Abort(api::Error::AuthFailed));
        };

        let is_shared_updated = sqlx::query!(
            "UPDATE folders
                SET shared = true
                WHERE id = $1 AND owner_id = $2",
            folder_id.as_slice(),
            session.user_id,
        )
        .execute(tx.as_mut())
        .await?
        .rows_affected()
            != 0;

        if !is_shared_updated {
            // The client shouldn't be able to tell whether a non-shared folder exists by its ID.
            return Err(TxError::Abort(api::Error::AccessDenied));
        }

        Ok(())
    })
    .await?;

    Ok((StatusCode::OK, Json(PostResponse {})))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {}

/// Unshares a folder.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn delete(
    Path(folder_id): PathParams,
    AuthToken(token_hash): AuthToken,
) -> impl Response<DeleteResponse> {
    db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let Some(session) = sqlx::query!(
            "SELECT user_id FROM sessions
                WHERE token_hash = $1",
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            return Err(TxError::Abort(api::Error::AuthFailed));
        };

        let is_shared_updated = sqlx::query!(
            "UPDATE folders
                SET shared = false
                WHERE id = $1 AND owner_id = $2",
            folder_id.as_slice(),
            session.user_id,
        )
        .execute(tx.as_mut())
        .await?
        .rows_affected()
            != 0;

        if !is_shared_updated {
            // The client shouldn't be able to tell whether a non-shared folder exists by its ID.
            return Err(TxError::Abort(api::Error::AccessDenied));
        }

        Ok(())
    })
    .await?;

    Ok((StatusCode::OK, Json(DeleteResponse {})))
}

/// A `DELETE` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeleteResponse {}
