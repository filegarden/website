//! The current authenticated user's password.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        self, Json,
        cookie::{CookieWrapper, SessionCookie},
        db_helpers::create_session,
        extract::AuthToken,
        response::Response,
        validation::{
            UserName,
            auth::{FirstFactorCredentials, VerifyCredentials},
        },
    },
    crypto::hash_with_salt,
    db::{self, TxError, TxResult},
};

/// A `PATCH` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PatchRequest {
    /// The user's credentials.
    pub credentials: FirstFactorCredentials,

    /// The user's new password.
    pub password: UserName,
}

/// Changes the current authenticated user's password.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn patch(
    AuthToken(token_hash): AuthToken,
    Json(body): Json<PatchRequest>,
) -> impl Response<PatchResponse> {
    let password_hash = hash_with_salt(&body.password);

    let session_token = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        let Some(user_id) = sqlx::query!(
            "SELECT user_id FROM sessions
                WHERE token_hash = $1",
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        .map(|session| session.user_id) else {
            return Err(TxError::Abort(api::Error::AuthFailed));
        };

        body.credentials.verify(tx, &user_id).await?;

        // Expiring all sessions is a conventionally expected security feature whenever a user's
        // password is changed.
        sqlx::query!(
            "DELETE FROM sessions
                WHERE user_id = $1 AND token_hash != $2",
            user_id,
            token_hash.as_ref(),
        )
        .execute(tx.as_mut())
        .await?;

        sqlx::query!(
            "UPDATE users
                SET password_hash = $1
                WHERE id = $2",
            password_hash,
            user_id.as_slice(),
        )
        .execute(tx.as_mut())
        .await?;

        let session_token = create_session(tx, &user_id).await?;

        Ok(session_token)
    })
    .await?;

    Ok((
        StatusCode::OK,
        [SessionCookie::new(session_token.to_string()).to_header()],
        Json(PatchResponse {}),
    ))
}

/// A `PATCH` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PatchResponse {}
