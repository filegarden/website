//! A file.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::Serialize;

use crate::{
    api::{
        self, Json,
        extract::{AuthToken, Path},
        response::Response,
    },
    db::{self, TxError, TxResult},
    id::Id,
};

pub(crate) mod r#move;
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

        let Some(file) = sqlx::query!(
            r#"DELETE FROM files
                USING files AS using_files
                LEFT JOIN file_replacements ON file_replacements.id = using_files.id
                WHERE files.id = using_files.id AND files.id = $1 AND files.owner_id = $2
                RETURNING
                    files.parent_id_path,
                    files.size,
                    files.content_id,
                    file_replacements.size AS "replacement_size?",
                    file_replacements.content_id AS "replacement_content_id?""#,
            file_id.as_slice(),
            session.user_id,
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            return Err(TxError::Abort(api::Error::AccessDenied));
        };

        if !file.parent_id_path.is_empty() {
            sqlx::query!(
                "UPDATE folders
                    SET size = size - $1
                    WHERE id = ANY($2)",
                file.size + file.replacement_size.unwrap_or(0),
                file.parent_id_path.as_slice(),
            )
            .execute(tx.as_mut())
            .await?;
        }

        sqlx::query!(
            "INSERT INTO maybe_unused_file_contents (id)
                VALUES ($1)
                ON CONFLICT DO NOTHING",
            // TODO: Consider using `ON CONFLICT DO UPDATE` instead, and remove `started_checking`
            // from the primary key.
            file.content_id,
        )
        .execute(tx.as_mut())
        .await?;

        if let Some(replacement_content_id) = file.replacement_content_id
            && replacement_content_id != file.content_id
        {
            sqlx::query!(
                "INSERT INTO maybe_unused_file_contents (id)
                    VALUES ($1)
                    ON CONFLICT DO NOTHING",
                replacement_content_id,
            )
            .execute(tx.as_mut())
            .await?;
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
