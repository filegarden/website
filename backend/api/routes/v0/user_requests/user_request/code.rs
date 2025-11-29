//! The verification code of a new user's email verification request.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::Serialize;

use crate::{
    api::{self, Json, extract::Path, response::Response},
    crypto::{generate_short_code, hash_with_salt, hash_without_salt},
    db::{self, TxResult},
    id::Token,
};

/// The length of a new email verification code.
const EMAIL_VERIFICATION_CODE_LENGTH: usize = 6;

/// A request path for this API route.
type PathParams = Path<Token>;

/// Generates a new verification code for an email verification request.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(Path(token): PathParams) -> impl Response<PostResponse> {
    let token_hash = hash_without_salt(&token);

    let code = generate_short_code(EMAIL_VERIFICATION_CODE_LENGTH);
    let code_hash = hash_with_salt(&code);

    let Some(unverified_user) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        Ok(sqlx::query!(
            "UPDATE unverified_users
                SET code_hash = $1
                WHERE token_hash = $2
                RETURNING email",
            code_hash,
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
        StatusCode::CREATED,
        Json(PostResponse {
            email: unverified_user.email,
            code,
        }),
    ))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {
    /// The email address to verify.
    pub email: String,

    /// The new email verification code.
    pub code: String,
}
