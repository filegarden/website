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

/// Moves a trashed folder back out of the trash.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    Path(trashed_folder_id): PathParams,
    AuthToken(token_hash): AuthToken,
    Json(body): Json<PostRequest>,
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

        let (new_parent_id_path, new_parent_name_path) = match &body.parent_id {
            Some(parent_id) => {
                query_folder_paths_to_modify_contents(tx, &session.user_id, parent_id).await?
            }
            None => (vec![], vec![]),
        };

        let folder = match sqlx::query!(
            "WITH trashed_folder AS (
                DELETE FROM trashed_folders
                    WHERE owner_id = $1 AND id = $2
                    RETURNING created_at, name, browse_key, size, was_shared, parent_id_path
            ),
            folder AS (
                INSERT INTO folders
                    (created_at, id, name, owner_id, parent_id_path, parent_name_path, browse_key,
                        size, shared)
                    SELECT
                        created_at, $2, name, $1, $3, $4, browse_key, size, was_shared
                        FROM trashed_folder
            )
            SELECT name, parent_id_path AS trashed_parent_id_path, size
                FROM trashed_folder",
            session.user_id,
            trashed_folder_id.as_slice(),
            new_parent_id_path.as_slice(),
            new_parent_name_path.as_slice(),
        )
        .fetch_optional(tx.as_mut())
        .await
        {
            Err(sqlx::Error::Database(error))
                if error.constraint() == Some("folders_by_name_path") =>
            {
                return Err(TxError::Abort(api::Error::AlreadyExists));
            }

            Err(error) => return Err(error.into()),

            Ok(None) => return Err(TxError::Abort(api::Error::AccessDenied)),

            Ok(Some(folder)) => folder,
        };

        if !folder.trashed_parent_id_path.is_empty() {
            sqlx::query!(
                "UPDATE trashed_folders
                    SET size = size - $1
                    WHERE id = ANY($2)",
                folder.size,
                folder.trashed_parent_id_path.as_slice(),
            )
            .execute(tx.as_mut())
            .await?;
        }

        if !new_parent_id_path.is_empty() {
            sqlx::query!(
                "UPDATE folders
                    SET size = size + $1
                    WHERE id = ANY($2)",
                folder.size,
                new_parent_id_path.as_slice(),
            )
            .execute(tx.as_mut())
            .await?;
        }

        let mut trashed_folder_id_path = folder.trashed_parent_id_path;
        trashed_folder_id_path.push(trashed_folder_id.to_vec());

        let mut new_folder_id_path = new_parent_id_path;
        new_folder_id_path.push(trashed_folder_id.to_vec());

        let mut new_folder_name_path = new_parent_name_path;
        new_folder_name_path.push(folder.name);

        sqlx::query!(
            "WITH sub_trashed_folders AS (
                DELETE FROM trashed_folders
                    WHERE
                        owner_id = $1
                        AND parent_id_path >= $2
                        AND parent_id_path < $2 || NULL::bytea
                    RETURNING created_at, id, name, parent_id_path, parent_name_path, browse_key,
                        size, was_shared
            )
            INSERT INTO folders
                (owner_id, created_at, id, name, parent_id_path, parent_name_path, browse_key, size,
                    shared)
                SELECT
                    $1, created_at, id, name,
                        $3 || parent_id_path[array_length($2::bytea[], 1) + 1:],
                        $4 || parent_name_path[array_length($2::bytea[], 1) + 1:], browse_key, size,
                        was_shared
                    FROM sub_trashed_folders",
            session.user_id,
            trashed_folder_id_path.as_slice(),
            new_folder_id_path.as_slice(),
            new_folder_name_path.as_slice(),
        )
        .execute(tx.as_mut())
        .await?;

        sqlx::query!(
            "WITH sub_trashed_files AS (
                DELETE FROM trashed_files
                    WHERE
                        owner_id = $1
                        AND parent_id_path >= $2
                        AND parent_id_path < $2 || NULL::bytea
                    RETURNING created_at, modified_at, original_id, name, parent_id_path,
                        parent_name_path, size, content_id, type, was_shared
            )
            INSERT INTO files
                (created_at, modified_at, id, name, owner_id, parent_id_path, parent_name_path,
                    size, content_id, type, shared)
                SELECT
                    created_at, modified_at, original_id, name, $1,
                        $3 || parent_id_path[array_length($2::bytea[], 1) + 1:],
                        $4 || parent_name_path[array_length($2::bytea[], 1) + 1:], size, content_id,
                        type, was_shared
                    FROM sub_trashed_files",
            session.user_id,
            trashed_folder_id_path.as_slice(),
            new_folder_id_path.as_slice(),
            new_folder_name_path.as_slice(),
        )
        .execute(tx.as_mut())
        .await?;

        Ok(())
    })
    .await?;

    Ok((
        StatusCode::CREATED,
        [(LOCATION, format!("/api/v0/folders/{trashed_folder_id}"))],
        Json(PostResponse {
            id: trashed_folder_id,
        }),
    ))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {
    /// The restored folder's ID.
    id: Id,
}
