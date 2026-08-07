//! See [`post`].

use std::iter;

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

/// Moves a folder to the current authenticated user's trash.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    Path(folder_id): PathParams,
    AuthToken(token_hash): AuthToken,
) -> impl Response<PostResponse> {
    let trashed_at = db::transaction!(async |tx| -> TxResult<_, api::Error> {
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

        let Some(trashed_folder) = sqlx::query!(
            r#"WITH folder AS (
                DELETE FROM folders
                    WHERE owner_id = $1 AND id = $2
                    RETURNING created_at, name, parent_id_path, browse_key, size, shared
            )
            INSERT INTO trashed_folders
                (created_at, id, name, owner_id, parent_id_path, original_parent_id_path,
                    browse_key, size, was_shared)
                SELECT
                    created_at, $2, name, $1, ARRAY[]::bytea[], parent_id_path, browse_key, size,
                        shared
                    FROM folder
                RETURNING trashed_at, original_parent_id_path, size"#,
            session.user_id,
            folder_id.as_slice(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            return Err(TxError::Abort(api::Error::AccessDenied));
        };

        let original_parent_id_path = trashed_folder
            .original_parent_id_path
            .expect("root trashed folders should have `original_parent_id_path`");

        if !original_parent_id_path.is_empty() {
            sqlx::query!(
                "UPDATE folders
                    SET size = size - $1
                    WHERE id = ANY($2)",
                trashed_folder.size,
                original_parent_id_path.as_slice()
            )
            .execute(tx.as_mut())
            .await?;
        }

        let original_depth = original_parent_id_path.len();

        let mut original_id_path = original_parent_id_path;
        original_id_path.push(folder_id.to_vec());

        sqlx::query!(
            "WITH sub_folders AS (
                DELETE FROM folders
                    WHERE
                        owner_id = $1
                        AND parent_id_path >= $2
                        AND parent_id_path < $2 || NULL::bytea
                    RETURNING created_at, id, name, parent_id_path, parent_name_path, browse_key,
                        size, shared
            )
            INSERT INTO trashed_folders
                (trashed_at, owner_id, created_at, id, name, parent_id_path, parent_name_path,
                    browse_key, size, was_shared)
                SELECT
                    NULL, $1, created_at, id, name, parent_id_path[$3 + 1:],
                        parent_name_path[$3 + 1:], browse_key, size, shared
                    FROM sub_folders",
            session.user_id,
            original_id_path.as_slice(),
            original_depth as i32,
        )
        .execute(tx.as_mut())
        .await?;

        let sub_files = sqlx::query!(
            r#"SELECT COUNT(*) AS "count!" FROM files
                WHERE
                    owner_id = $1 AND
                    parent_id_path >= $2 AND
                    parent_id_path < $2 || NULL::bytea"#,
            session.user_id,
            original_id_path.as_slice(),
        )
        .fetch_one(tx.as_mut())
        .await?;

        // TODO: Fix race condition from adding a file under the trashed folder after this point.

        if sub_files.count != 0 {
            // TODO: Also cancel replacement file uploads.

            let trashed_file_ids: Vec<_> = iter::repeat_with(NewFileId::generate)
                // TODO: If transact-rs/sqlx#4357 is completed, replace `to_vec` with `into_inner`.
                .map(|id| id.to_vec())
                .take(sub_files.count as usize)
                .collect();

            match sqlx::query!(
                "WITH sub_files AS (
                    DELETE FROM files
                        WHERE
                            owner_id = $1
                            AND parent_id_path >= $2
                            AND parent_id_path < $2 || NULL::bytea
                        RETURNING created_at, modified_at, id, name, parent_id_path,
                            parent_name_path, size, content_id, type, shared
                )
                INSERT INTO trashed_files
                    (trashed_at, id, created_at, modified_at, name, owner_id, original_id,
                        parent_id_path, parent_name_path, size, content_id, type, was_shared)
                    SELECT
                        NULL, ($3::bytea[])[row_number() OVER ()], created_at, modified_at, name,
                            $1, id, parent_id_path[$4 + 1:], parent_name_path[$4 + 1:], size,
                            content_id, type, shared
                        FROM sub_files",
                session.user_id,
                original_id_path.as_slice(),
                trashed_file_ids.as_slice(),
                original_depth as i32,
            )
            .execute(tx.as_mut())
            .await
            {
                Err(sqlx::Error::Database(error))
                    if error.constraint() == Some("trashed_files_pkey") =>
                {
                    return Err(TxError::Retry);
                }

                trashed_file => trashed_file?,
            };
        }

        Ok(trashed_folder
            .trashed_at
            .expect("root trashed folders should have `trashed_at`"))
    })
    .await?;

    Ok((
        StatusCode::CREATED,
        [(
            LOCATION,
            format!("/api/v0/users/me/trash/folders/{folder_id}"),
        )],
        Json(PostResponse {
            id: folder_id,
            trashed_at: trashed_at.timestamp_millis(),
        }),
    ))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {
    /// The new trashed folder's ID.
    id: Id,

    /// The new trashed folder's creation timestamp in Unix milliseconds.
    trashed_at: i64,
}
