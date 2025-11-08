//! See [`post`].

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        self, Json,
        extract::AuthToken,
        response::Response,
        validation::auth::{MultiFactorCredentials, VerifyCredentials},
    },
    db::{self, TxError, TxResult},
};

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PostRequest {
    /// The user's credentials.
    pub credentials: MultiFactorCredentials,
}

/// Verifies that the submitted credentials are correct for the current authenticated user. Responds
/// with an HTTP error if not.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    AuthToken(token_hash): AuthToken,
    Json(body): Json<PostRequest>,
) -> impl Response<PostResponse> {
    db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let Some(user) = sqlx::query!(
            "SELECT users.id FROM users
                INNER JOIN sessions ON sessions.user_id = users.id
                WHERE sessions.token_hash = $1",
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            return Err(TxError::Abort(api::Error::AuthFailed));
        };

        body.credentials.verify(tx, &user.id).await?;

        Ok(())
    })
    .await?;

    Ok((StatusCode::OK, Json(PostResponse {})))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {}
