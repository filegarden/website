//! A user's private settings.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::Serialize;

use crate::{
    api::{self, extract::AuthToken, response::Response, Json},
    crypto::hash_without_salt,
    db::{self, TxResult},
};

/// Gets information about a user's private settings.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(AuthToken(token): AuthToken) -> impl Response<GetResponse> {
    let token_hash = hash_without_salt(&token);

    let Some(user) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        Ok(sqlx::query!(
            r#"SELECT users.email, users.totp_secret IS NOT NULL AS "totp_enabled!" FROM sessions
                INNER JOIN users ON users.id = sessions.user_id
                WHERE sessions.token_hash = $1"#,
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?)
    })
    .await?
    else {
        return Err(api::Error::AuthFailed);
    };

    Ok((
        StatusCode::OK,
        Json(GetResponse {
            email: user.email,
            totp_enabled: user.totp_enabled,
        }),
    ))
}

/// A `GET` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetResponse {
    /// The user's email.
    email: String,

    /// Whether the user has TOTP authentication enabled.
    totp_enabled: bool,
}
