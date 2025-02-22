//! The new password for a user's password reset request.

use axum::{extract::State, http::StatusCode};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{
    api::{self, validation::NewUserPassword, Json, Query, Response},
    crypto::{hash_with_salt, hash_without_salt},
    db::{self, TxError, TxResult},
    id::Token,
    AppState,
};

/// A `POST` request query for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery {
    /// The password reset token.
    pub token: Token,
}

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PostRequest {
    /// The user's new password in plain text.
    pub password: NewUserPassword,
}

/// Sets a new password to fulfill a user's password reset request.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub async fn post(
    State(state): State<AppState>,
    Query(query): Query<PostQuery>,
    Json(body): Json<PostRequest>,
) -> Response<PostResponse> {
    let token_hash = hash_without_salt(&query.token);

    let password_hash = hash_with_salt(&body.password);

    db::transaction!(&state.db_pool, async |tx| -> TxResult<_, api::Error> {
        let Some(password_reset) = sqlx::query!(
            "DELETE FROM password_resets
                WHERE token_hash = $1
                RETURNING user_id",
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        else {
            return Err(TxError::Abort(api::Error::ResourceNotFound));
        };

        sqlx::query!(
            "UPDATE users
                SET password_hash = $1
                WHERE id = $2",
            password_hash,
            password_reset.user_id,
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
pub struct PostResponse {}
