//! The set of all users.

use axum::{extract::State, http::StatusCode};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use sqlx::Acquire;

use crate::{
    api::{
        self,
        validation::{EmailVerificationCode, NewUserPassword, UserEmail, UserName},
        Json, Response,
    },
    crypto::{hash_with_salt, verify_hash},
    db::{self, TxError, TxResult},
    id::NewUserId,
    AppState,
};

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PostRequest {
    /// The user's email address.
    pub email: UserEmail,

    /// The verification code for the user's email address.
    pub email_verification_code: EmailVerificationCode,

    /// The user's name.
    pub name: UserName,

    /// The user's new password in plain text.
    pub password: NewUserPassword,
}

/// Creates a new user.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub async fn post(
    State(state): State<AppState>,
    Json(body): Json<PostRequest>,
) -> Response<PostResponse> {
    let mut user_id = NewUserId::generate();

    let password_hash = hash_with_salt(&body.password);

    db::transaction!(&state.db_pool, async |tx| -> TxResult<_, api::Error> {
        let does_code_match = sqlx::query!(
            "DELETE FROM unverified_emails
                WHERE user_id IS NULL AND email = $1
                RETURNING code_hash",
            body.email.as_str(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        .and_then(|unverified_email| unverified_email.code_hash)
        .is_some_and(|code_hash| verify_hash(&body.email_verification_code, &code_hash));

        if !does_code_match {
            return Err(TxError::Abort(api::Error::EmailVerificationCodeWrong));
        }

        loop {
            // If this loop's query fails from an ID conflict, this savepoint is rolled back to
            // rather than aborting the entire transaction.
            let mut savepoint = tx.begin().await?;

            match sqlx::query!(
                "INSERT INTO users (id, email, name, password_hash)
                    VALUES ($1, $2, $3, $4)",
                user_id.as_slice(),
                body.email.as_str(),
                *body.name,
                password_hash,
            )
            .execute(savepoint.as_mut())
            .await
            {
                Err(sqlx::Error::Database(error)) if error.constraint() == Some("users_pkey") => {
                    user_id.reroll();
                    continue;
                }
                result => result?,
            };

            savepoint.commit().await?;
            break;
        }

        Ok(())
    })
    .await?;

    // TODO: Set `Location` header.
    Ok((StatusCode::CREATED, Json(PostResponse { id: user_id })))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostResponse {
    /// The user's ID.
    pub id: NewUserId,
}
