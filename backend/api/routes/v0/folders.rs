//! The set of all folders.

pub(crate) mod folder;

use axum::http::header::LOCATION;
use axum_macros::debug_handler;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        self, Json, db_helpers::query_folder_paths_to_modify_contents, extract::AuthToken,
        response::Response, validation::FileName,
    },
    db::{self, TxError, TxResult},
    id::{FolderBrowseKey, Id, NewFolderId},
};

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PostRequest {
    /// The ID of the folder to create a folder under, or [`None`] for the root directory.
    parent_id: Option<Id>,

    /// The folder's name.
    name: FileName,
}

/// Creates a folder.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    AuthToken(token_hash): AuthToken,
    Json(body): Json<PostRequest>,
) -> impl Response<PostResponse> {
    let (folder_id, browse_key, created_at) =
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

            let (parent_id_path, parent_name_path) = match &body.parent_id {
                Some(parent_id) => {
                    query_folder_paths_to_modify_contents(tx, &session.user_id, parent_id).await?
                }
                None => (vec![], vec![]),
            };

            let folder_id = NewFolderId::generate();
            let browse_key = FolderBrowseKey::generate();

            let folder = match sqlx::query!(
                "INSERT INTO folders
                    (id, name, owner_id, parent_id_path, parent_name_path, browse_key)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING created_at",
                folder_id.as_slice(),
                body.name.as_str(),
                session.user_id,
                parent_id_path.as_slice(),
                parent_name_path.as_slice(),
                browse_key.as_slice(),
            )
            .fetch_one(tx.as_mut())
            .await
            {
                Err(sqlx::Error::Database(error))
                    if error.constraint() == Some("folders_by_name_path") =>
                {
                    return Err(TxError::Abort(api::Error::AlreadyExists));
                }

                Err(sqlx::Error::Database(error)) if error.constraint() == Some("folders_pkey") => {
                    return Err(TxError::Retry);
                }

                result => result?,
            };

            Ok((folder_id, browse_key, folder.created_at))
        })
        .await?;

    Ok((
        StatusCode::CREATED,
        [(LOCATION, format!("/api/v0/folders/{folder_id}"))],
        Json(PostResponse {
            id: folder_id,
            name: body.name,
            browse_key,
            created_at: created_at.timestamp_millis(),
        }),
    ))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {
    /// The new folder's ID.
    id: NewFolderId,

    /// The new folder's name.
    name: FileName,

    /// The new folder's browse key.
    browse_key: FolderBrowseKey,

    /// The new folder's creation timestamp in Unix milliseconds.
    created_at: i64,
}
