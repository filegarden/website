//! The set of all files and folders.

use axum_macros::debug_handler;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        self, Json,
        extract::{AuthToken, Query},
        response::{
            Response,
            body::{File, Folder},
        },
    },
    db::{self, TxError, TxResult},
    id::FolderBrowseKey,
};

/// A `GET` request query for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetQuery {
    /// The parent folder's browse key.
    pub parent_browse_key: FolderBrowseKey,
}

/// Gets a folder's children.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(
    AuthToken(token_hash): AuthToken,
    Query(query): Query<GetQuery>,
) -> impl Response<GetResponse> {
    let (files, folders) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
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

        let Some(parent) = sqlx::query!(
            "SELECT id, parent_id_path FROM folders
                WHERE owner_id = $1 AND browse_key = $2",
            session.user_id,
            query.parent_browse_key.as_slice(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            return Err(TxError::Abort(api::Error::AccessDenied));
        };

        let mut parent_id_path = parent.parent_id_path;
        parent_id_path.push(parent.id);

        let folders = sqlx::query!(
            "SELECT created_at, id, name, browse_key, size, shared FROM folders
                WHERE owner_id = $1 AND parent_id_path = $2",
            session.user_id,
            parent_id_path.as_slice(),
        )
        .map(|folder| Folder {
            created_at: folder.created_at.timestamp_millis(),
            id: folder.id.into(),
            name: folder.name,
            browse_key: folder.browse_key.into(),
            size: folder
                .size
                .try_into()
                .expect("folder size should be positive"),
            shared: folder.shared,
        })
        .fetch_all(tx.as_mut())
        .await?;

        let files = sqlx::query!(
            "SELECT created_at, modified_at, complete, id, name, size, type, shared FROM files
                WHERE owner_id = $1 AND parent_id_path = $2",
            session.user_id,
            parent_id_path.as_slice(),
        )
        .map(|file| File {
            created_at: file.created_at.timestamp_millis(),
            modified_at: file.modified_at.timestamp_millis(),
            complete: file.complete,
            id: file.id.into(),
            name: file.name,
            size: file.size.try_into().expect("file size should be positive"),
            r#type: file.r#type,
            shared: file.shared,
        })
        .fetch_all(tx.as_mut())
        .await?;

        Ok((files, folders))
    })
    .await?;

    Ok((StatusCode::OK, Json(GetResponse { files, folders })))
}

/// A `GET` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetResponse {
    /// The file entries.
    files: Vec<File>,

    /// The folder entries.
    folders: Vec<Folder>,
}
