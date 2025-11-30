//! The new password for a user's password reset request.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        self, Json,
        cookie::{CookieWrapper, SessionCookie},
        db_helpers::create_session,
        extract::Query,
        response::{Response, body::User},
        validation::{
            NewUserPassword,
            auth::{SecondFactorCredentials, VerifyCredentials},
        },
    },
    crypto::{hash_with_salt, hash_without_salt},
    db::{self, TxError, TxResult},
    id::Token,
};

/// A `POST` request query for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostQuery {
    /// The password reset token.
    pub token: Token,
}

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PostRequest {
    /// The user's credentials.
    pub credentials: SecondFactorCredentials,

    /// The user's new password in plain text.
    pub password: NewUserPassword,
}

/// Sets a new password to fulfill a user's password reset request. Signs in the user if successful.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn post(
    Query(query): Query<PostQuery>,
    Json(body): Json<PostRequest>,
) -> impl Response<PostResponse> {
    let token_hash = hash_without_salt(&query.token);

    let password_hash = hash_with_salt(&body.password);

    let (user_id, user_name, session_token) =
        db::transaction!(async |tx| -> TxResult<_, api::Error> {
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

            body.credentials.verify(tx, &password_reset.user_id).await?;

            let user = sqlx::query!(
                "UPDATE users
                    SET password_hash = $1
                    WHERE id = $2
                    RETURNING name",
                password_hash,
                password_reset.user_id,
            )
            .fetch_one(tx.as_mut())
            .await?;

            // Expiring all other sessions is a conventionally expected security feature whenever a
            // user's password is changed.
            sqlx::query!(
                "DELETE FROM sessions
                    WHERE user_id = $1",
                password_reset.user_id,
            )
            .execute(tx.as_mut())
            .await?;

            let session_token = create_session(tx, &password_reset.user_id).await?;

            Ok((password_reset.user_id, user.name, session_token))
        })
        .await?;

    Ok((
        StatusCode::OK,
        [SessionCookie::new(session_token.to_string()).to_header()],
        Json(PostResponse {
            user: User {
                id: user_id.into(),
                name: user_name,
            },
        }),
    ))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostResponse {
    /// The user whose password was reset.
    user: User,
}
