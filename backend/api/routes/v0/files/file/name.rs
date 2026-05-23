//! A file's name.

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
    /// The file's new name.
    name: FileName,
}

/// Renames a file.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn put(
    Path(file_id): PathParams,
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

        let is_name_updated = sqlx::query!(
            "UPDATE files
                SET name = $1
                WHERE id = $2 AND owner_id = $3",
            body.name.as_str(),
            file_id.as_slice(),
            session.user_id,
        )
        .execute(tx.as_mut())
        .await?
        .rows_affected()
            != 0;

        if !is_name_updated {
            // The client shouldn't be able to tell whether a non-shared file exists by its ID.
            return Err(TxError::Abort(api::Error::AccessDenied));
        }

        Ok(())
    })
    .await?;

    Ok((StatusCode::OK, Json(PutResponse { name: body.name })))
}

/// A `PUT` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PutResponse {
    /// The file's new name.
    name: FileName,
}
