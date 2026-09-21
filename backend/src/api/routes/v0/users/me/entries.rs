//! The current authenticated user's root files and folders.

use crate::{
    api::{
        self, Json,
        extract::AuthToken,
        response::{
            Response,
            body::{File, Folder},
        },
    },
    db::{self, TxError, TxResult},
};
use axum_macros::debug_handler;
use reqwest::StatusCode;
use serde::Serialize;

/// Gets the current authenticated user's root files and folders.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(AuthToken(token_hash): AuthToken) -> impl Response<GetResponse> {
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

        let folders = sqlx::query!(
            "SELECT created_at, id, name, browse_key, size, shared FROM folders
                WHERE owner_id = $1 AND parent_id_path = ARRAY[]::bytea[]",
            session.user_id,
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
                WHERE owner_id = $1 AND parent_id_path = ARRAY[]::bytea[]",
            session.user_id,
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
