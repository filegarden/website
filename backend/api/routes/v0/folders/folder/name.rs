//! A folder's name.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        self, Json,
        extract::{AuthToken, Path},
        response::Response,
        validation::FileName,
    },
    db::{self, TxError, TxResult},
    id::Id,
};

/// A request path for this API route.
type PathParams = Path<Id>;

/// A `PUT` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PutRequest {
    /// The folder's new name.
    name: FileName,
}

/// Renames a folder.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn put(
    Path(folder_id): PathParams,
    AuthToken(token_hash): AuthToken,
    Json(body): Json<PutRequest>,
) -> impl Response<PutResponse> {
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

        let folder = match sqlx::query!(
            "UPDATE folders
                SET name = $1
                WHERE id = $2 AND owner_id = $3
                RETURNING parent_name_path, OLD.name AS old_name",
            body.name.as_str(),
            folder_id.as_slice(),
            session.user_id,
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

            Ok(None) => {
                return Err(TxError::Abort(api::Error::AccessDenied));
            }

            Ok(Some(folder)) => folder,
        };

        let mut old_folder_path = folder.parent_name_path.clone();
        old_folder_path.push(folder.old_name);

        let mut new_folder_path = folder.parent_name_path;
        new_folder_path.push(body.name.to_string());

        sqlx::query!(
            "UPDATE folders
                SET parent_name_path = $1 || parent_name_path[array_length($1::text[], 1) + 1:]
                WHERE owner_id = $2 AND parent_name_path >= $3 AND parent_name_path < $3 || NULL",
            new_folder_path.as_slice(),
            session.user_id,
            old_folder_path.as_slice(),
        )
        .execute(tx.as_mut())
        .await?;

        sqlx::query!(
            "UPDATE files
                SET parent_name_path = $1 || parent_name_path[array_length($1::text[], 1) + 1:]
                WHERE owner_id = $2 AND parent_name_path >= $3 AND parent_name_path < $3 || NULL",
            new_folder_path.as_slice(),
            session.user_id,
            old_folder_path.as_slice(),
        )
        .execute(tx.as_mut())
        .await?;

        Ok(())
    })
    .await?;

    Ok((StatusCode::OK, Json(PutResponse { name: body.name })))
}

/// A `PUT` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PutResponse {
    /// The folder's new name.
    name: FileName,
}
