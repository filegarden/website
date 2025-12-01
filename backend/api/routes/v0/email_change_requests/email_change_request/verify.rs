//! See [`post`].

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::Serialize;

use crate::{
    api::{self, Json, extract::Path, response::Response},
    crypto::hash_without_salt,
    db::{self, TxError, TxResult},
    id::Token,
};

/// A request path for this API route.
type PathParams = Path<Token>;

/// Performs the email change from an email change request.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(Path(token): PathParams) -> impl Response<PostResponse> {
    let token_hash = hash_without_salt(&token);

    db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let Some(email_change_request) = sqlx::query!(
            "DELETE FROM email_change_requests
                WHERE token_hash = $1
                RETURNING user_id, email",
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            return Err(TxError::Abort(api::Error::ResourceNotFound));
        };

        sqlx::query!(
            "DELETE FROM unverified_users
                WHERE email = $1",
            email_change_request.email,
        )
        .execute(tx.as_mut())
        .await?;

        sqlx::query!(
            "DELETE FROM email_change_requests
                WHERE email = $1",
            email_change_request.email,
        )
        .execute(tx.as_mut())
        .await?;

        sqlx::query!(
            "UPDATE users
                SET email = $1
                WHERE id = $2",
            email_change_request.email,
            email_change_request.user_id,
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
