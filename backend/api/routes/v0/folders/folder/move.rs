//! See [`post`].

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
    /// The new parent folder's ID, or [`None`] for the root directory.
    pub parent_id: Option<Id>,
}

/// Changes a folder's parent folder.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    Path(folder_id): PathParams,
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
            "UPDATE folders
                SET parent_id_path = $1,
                    parent_name_path = $2
                WHERE owner_id = $3 AND id = $4
                RETURNING
                    name,
                    size,
                    OLD.parent_id_path AS old_parent_id_path",
            new_parent_id_path.as_slice(),
            new_parent_name_path.as_slice(),
            session.user_id,
            folder_id.as_slice(),
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

        if !folder.old_parent_id_path.is_empty() {
            sqlx::query!(
                "UPDATE folders
                    SET size = size - $1
                    WHERE id = ANY($2)",
                folder.size,
                folder.old_parent_id_path.as_slice(),
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

        let mut old_folder_id_path = folder.old_parent_id_path;
        old_folder_id_path.push(folder_id.to_vec());

        let mut new_folder_id_path = new_parent_id_path;
        new_folder_id_path.push(folder_id.to_vec());

        let mut new_folder_name_path = new_parent_name_path;
        new_folder_name_path.push(folder.name);

        sqlx::query!(
            "UPDATE folders
                SET parent_id_path = $1 || parent_id_path[array_length($2::bytea[], 1) + 1:],
                    parent_name_path = $3 || parent_name_path[array_length($2::text[], 1) + 1:]
                WHERE owner_id = $4 AND parent_id_path >= $2 AND parent_id_path < $2 || NULL",
            new_folder_id_path.as_slice(),
            old_folder_id_path.as_slice(),
            new_folder_name_path.as_slice(),
            session.user_id,
        )
        .execute(tx.as_mut())
        .await?;

        sqlx::query!(
            "UPDATE files
                SET parent_id_path = $1 || parent_id_path[array_length($2::bytea[], 1) + 1:],
                    parent_name_path = $3 || parent_name_path[array_length($2::text[], 1) + 1:]
                WHERE owner_id = $4 AND parent_id_path >= $2 AND parent_id_path < $2 || NULL",
            new_folder_id_path.as_slice(),
            old_folder_id_path.as_slice(),
            new_folder_name_path.as_slice(),
            session.user_id,
        )
        .execute(tx.as_mut())
        .await?;

        Ok(())
    })
    .await?;

    Ok((StatusCode::OK, Json(PostResponse {})))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {}
