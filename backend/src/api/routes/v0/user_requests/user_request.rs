//! An email verification request for a new user.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::Serialize;

use crate::{
    api::{self, Json, extract::Path, response::Response},
    crypto::hash_without_salt,
    db::{self, TxResult},
    id::Token,
};

pub(crate) mod code;

/// A request path for this API route.
type PathParams = Path<Token>;

/// Checks an existing email verification request.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(Path(token): PathParams) -> impl Response<GetResponse> {
    let token_hash = hash_without_salt(&token);

    let Some(unverified_user) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        Ok(sqlx::query!(
            "SELECT email FROM unverified_users
                WHERE token_hash = $1",
            token_hash.as_ref(),
        )
        .fetch_optional(tx.as_mut())
        .await?)
    })
    .await?
    else {
        return Err(api::Error::ResourceNotFound);
    };

    Ok((
        StatusCode::OK,
        Json(GetResponse {
            email: unverified_user.email,
        }),
    ))
}

/// A `GET` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetResponse {
    /// The email address to verify.
    pub email: String,
}
