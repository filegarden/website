//! The set of users' sign-in sessions.

use std::sync::LazyLock;

use axum::{extract::State, http::StatusCode};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use sqlx::Acquire;
use tower_cookies::{
    cookie::{time::Duration, SameSite},
    Cookie, Cookies,
};

use crate::{
    api::{
        self,
        validation::{UserEmail, UserPassword},
        Json, Response,
    },
    crypto::{hash_without_salt, verify_hash},
    db::{self, TxResult},
    id::Token,
    AppState, WEBSITE_ORIGIN,
};

/// The domain for the website.
static WEBSITE_DOMAIN: LazyLock<&str> = LazyLock::new(|| domain_from_origin(&WEBSITE_ORIGIN));

/// How long a session takes to expire after its creation.
const SESSION_MAX_AGE: Duration = Duration::days(60);

/// A `POST` request body for this API route.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PostRequest {
    /// The email address of the user signing in.
    pub email: UserEmail,

    /// The user's password in plain text.
    pub password: UserPassword,
}

/// Signs a user in, creating a sign-in session and returning a session cookie.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub async fn post(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(body): Json<PostRequest>,
) -> Response<PostResponse> {
    let token = db::transaction!(&state.db_pool, async |tx| -> TxResult<_, api::Error> {
        let Some(user) = sqlx::query!(
            "SELECT id, password_hash FROM users
                WHERE email = $1",
            body.email.as_str(),
        )
        .fetch_optional(tx.as_mut())
        .await?
        .filter(|user| verify_hash(&body.password, &user.password_hash)) else {
            // To prevent user enumeration, send this same error response whether or not the email
            // is correct.
            return Err(db::TxError::Abort(api::Error::UserCredentialsWrong));
        };

        let mut token = Token::generate();

        loop {
            // If this loop's query fails from a token conflict, this savepoint is rolled back to
            // rather than aborting the entire transaction.
            let mut savepoint = tx.begin().await?;

            let token_hash = hash_without_salt(&token);

            match sqlx::query!(
                "INSERT INTO sessions (token_hash, user_id)
                    VALUES ($1, $2)",
                token_hash.as_ref(),
                user.id,
            )
            .execute(savepoint.as_mut())
            .await
            {
                Err(sqlx::Error::Database(error))
                    if error.constraint() == Some("sessions_pkey") =>
                {
                    token.reroll();
                    continue;
                }
                result => result?,
            };

            savepoint.commit().await?;
            break;
        }

        Ok(token)
    })
    .await?;

    cookies.add(
        Cookie::build(("token", token.to_string()))
            .domain(*WEBSITE_DOMAIN)
            .http_only(true)
            .max_age(SESSION_MAX_AGE)
            .path("/")
            .same_site(SameSite::Lax)
            .secure(WEBSITE_ORIGIN.starts_with("https:"))
            .into(),
    );

    Ok((StatusCode::OK, Json(PostResponse {})))
}

/// A `POST` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostResponse {
    // To reduce the session token's attack surface, it isn't included in the response. It's set as
    // an `HttpOnly` cookie instead so browser scripts can't access it.
}

/// Returns the domain from an origin URI string.
///
/// # Panics
///
/// Panics if the origin string doesn't contain "//".
fn domain_from_origin(origin: &str) -> &str {
    let start = origin.find("//").expect("origin should contain \"//\"") + 2;
    let end = origin[start..]
        .find(":")
        .map(|index| index + start)
        .unwrap_or(origin.len());

    &origin[start..end]
}
