//! A file.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::Serialize;
use sqlx::Connection;

use crate::{
    api::{
        self, Json,
        extract::{AuthToken, Path},
        response::Response,
    },
    db::{self, TxError, TxResult},
    id::Id,
};

pub(crate) mod name;
pub(crate) mod share;

/// A request path for this API route.
type PathParams = Path<Id>;

/// Deletes a file.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn delete(
    Path(file_id): PathParams,
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

        // Don't delete any incomplete replacement files for the same ID.
        let Some(file) = sqlx::query!(
            "DELETE FROM files
                WHERE id = $1 AND complete AND owner_id = $2
                RETURNING size, parent_id_path, content_id",
            file_id.as_slice(),
            session.user_id,
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            // The client shouldn't be able to tell whether a non-shared file exists by its ID.
            return Err(TxError::Abort(api::Error::AccessDenied));
        };

        if !file.parent_id_path.is_empty() {
            sqlx::query!(
                "UPDATE folders
                    SET size = size - $1
                    WHERE id = ANY($2)",
                file.size,
                &file.parent_id_path,
            )
            .execute(tx.as_mut())
            .await?;
        }

        let mut savepoint = tx.begin().await?;

        match sqlx::query!(
            "INSERT INTO maybe_unused_file_contents (id, started_checking)
                VALUES ($1, false)",
            file.content_id,
        )
        .execute(savepoint.as_mut())
        .await
        {
            Err(sqlx::Error::Database(error))
                if error.constraint() == Some("maybe_unused_file_contents_pkey") =>
            {
                // The file content is already marked maybe unused.
                savepoint.rollback().await?;
            }

            result => {
                result?;
                savepoint.commit().await?;
            }
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
