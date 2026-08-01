//! See [`post`].

use axum::http::header::LOCATION;
use axum_macros::debug_handler;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    api::{
        self, Json,
        extract::{AuthToken, Path},
        response::Response,
    },
    db::{self, TxError, TxResult},
    id::{Id, NewFileId},
};

/// A request path for this API route.
type PathParams = Path<Id>;

/// Moves a file to the current authenticated user's trash.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    Path(file_id): PathParams,
    AuthToken(token_hash): AuthToken,
) -> impl Response<PostResponse> {
    let (trashed_file_id, trashed_at) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
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

        let trashed_file_id = NewFileId::generate();

        let trashed_file = match sqlx::query!(
            "WITH file AS (
                DELETE FROM files
                    WHERE owner_id = $1 AND id = $2
                    RETURNING created_at, modified_at, name, parent_id_path, size, content_id, type,
                        shared
            )
            INSERT INTO trashed_files 
                (created_at, modified_at, id, name, owner_id, parent_id_path,
                    original_parent_id_path, original_id, size, content_id, type, was_shared)
                SELECT
                    created_at, modified_at, $3, name, $1, ARRAY[]::bytea[], parent_id_path, $2,
                        size, content_id, type, shared
                    FROM file
                RETURNING trashed_at, original_parent_id_path, size",
            session.user_id,
            file_id.as_slice(),
            trashed_file_id.as_slice(),
        )
        .fetch_optional(tx.as_mut())
        .await
        {
            Err(sqlx::Error::Database(error))
                if error.constraint() == Some("trashed_files_pkey") =>
            {
                return Err(TxError::Retry);
            }

            Err(error) => return Err(error.into()),

            Ok(None) => return Err(TxError::Abort(api::Error::AccessDenied)),

            Ok(Some(trashed_file)) => trashed_file,
        };

        let original_parent_id_path = trashed_file
            .original_parent_id_path
            .expect("root trashed files should have `original_parent_id_path`");

        // TODO: Also cancel replacement file uploads and subtract their size from ancestors.
        if !original_parent_id_path.is_empty() {
            sqlx::query!(
                "UPDATE folders
                    SET size = size - $1
                    WHERE id = ANY($2)",
                trashed_file.size,
                original_parent_id_path.as_slice()
            )
            .execute(tx.as_mut())
            .await?;
        }

        Ok((
            trashed_file_id,
            trashed_file
                .trashed_at
                .expect("root trashed files should have `trashed_at`"),
        ))
    })
    .await?;

    Ok((
        StatusCode::CREATED,
        [(LOCATION, format!("/api/v0/TODO/{trashed_file_id}"))],
        Json(PostResponse {
            id: trashed_file_id,
            trashed_at: trashed_at.timestamp_millis(),
        }),
    ))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {
    /// The new trashed file's ID.
    id: NewFileId,

    /// The new trashed file's creation timestamp in Unix milliseconds.
    trashed_at: i64,
}
