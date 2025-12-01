//! An email change request for an existing user.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::Serialize;

use crate::{
    api::{self, Json, extract::Path, response::Response},
    crypto::hash_without_salt,
    db::{self, TxResult},
    id::Token,
};

pub(crate) mod verify;

/// A request path for this API route.
type PathParams = Path<Token>;

/// Checks an existing email change request.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(Path(token): PathParams) -> impl Response<GetResponse> {
    let token_hash = hash_without_salt(&token);

    let Some(email_change_request) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        Ok(sqlx::query!(
            "SELECT email_change_requests.email, users.email AS user_email
                FROM email_change_requests
                INNER JOIN users ON users.id = email_change_requests.user_id
                WHERE email_change_requests.token_hash = $1",
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
            current_email: email_change_request.user_email,
            requested_email: email_change_request.email,
        }),
    ))
}

/// A `GET` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetResponse {
    /// The user's current email address.
    pub current_email: String,

    /// The email address requested for the email change.
    pub requested_email: String,
}
