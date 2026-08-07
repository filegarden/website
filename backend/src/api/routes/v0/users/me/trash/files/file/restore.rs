//! See [`post`].

use axum::http::header::LOCATION;
use axum_macros::debug_handler;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        self, Json,
        db_helpers::query_folder_paths_to_modify_contents,
        extract::{AuthToken, Path},
        response::Response,
    },
    db::{self, TxError, TxResult},
    id::Id,
};

/// A request path for this API route.
type PathParams = Path<Id>;

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PostRequest {
    /// The ID of the destination folder outside the trash, or [`None`] for the root directory.
    pub parent_id: Option<Id>,
}

/// Moves a trashed file back out of the trash.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    Path(trashed_file_id): PathParams,
    AuthToken(token_hash): AuthToken,
    Json(body): Json<PostRequest>,
) -> impl Response<PostResponse> {
    let file_id = db::transaction!(async |tx| -> TxResult<_, api::Error> {
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

        let (new_parent_id_path, new_parent_name_path) = match &body.parent_id {
            Some(parent_id) => {
                query_folder_paths_to_modify_contents(tx, &session.user_id, parent_id).await?
            }
            None => (vec![], vec![]),
        };

        let file = match sqlx::query!(
            "WITH trashed_file AS (
                DELETE FROM trashed_files
                    WHERE owner_id = $1 AND id = $2
                    RETURNING created_at, modified_at, original_id, name, size, content_id, type,
                        was_shared, parent_id_path
            ),
            file AS (
                INSERT INTO files
                    (created_at, modified_at, id, complete, name, owner_id, parent_id_path,
                        parent_name_path, size, content_id, type, shared)
                    SELECT
                        created_at, modified_at, original_id, TRUE, name, $1, $3, $4, size,
                            content_id, type, was_shared
                        FROM trashed_file
            )
            SELECT original_id AS id, parent_id_path AS trashed_parent_id_path, size
                FROM trashed_file",
            session.user_id,
            trashed_file_id.as_slice(),
            new_parent_id_path.as_slice(),
            new_parent_name_path.as_slice(),
        )
        .fetch_optional(tx.as_mut())
        .await
        {
            Err(sqlx::Error::Database(error))
                if error.constraint() == Some("files_by_name_path") =>
            {
                return Err(TxError::Abort(api::Error::AlreadyExists));
            }

            Err(error) => return Err(error.into()),

            Ok(None) => return Err(TxError::Abort(api::Error::AccessDenied)),

            Ok(Some(file)) => file,
        };

        if !file.trashed_parent_id_path.is_empty() {
            sqlx::query!(
                "UPDATE trashed_folders
                    SET size = size - $1
                    WHERE id = ANY($2)",
                file.size,
                file.trashed_parent_id_path.as_slice(),
            )
            .execute(tx.as_mut())
            .await?;
        }

        if !new_parent_id_path.is_empty() {
            sqlx::query!(
                "UPDATE folders
                    SET size = size + $1
                    WHERE id = ANY($2)",
                file.size,
                new_parent_id_path.as_slice(),
            )
            .execute(tx.as_mut())
            .await?;
        }

        Ok(file.id.into())
    })
    .await?;

    Ok((
        StatusCode::CREATED,
        [(LOCATION, format!("/api/v0/files/{file_id}"))],
        Json(PostResponse { id: file_id }),
    ))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {
    /// The restored file's ID.
    id: Id,
}
