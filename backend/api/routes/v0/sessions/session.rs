//! A session.

use std::str::FromStr;

use axum::{http::StatusCode, response::AppendHeaders};
use axum_macros::debug_handler;
use derive_more::Display;
use serde::Serialize;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{
    api::{
        self,
        cookie::{CookieWrapper, SessionCookie},
        extract::{AuthToken, Path},
        response::Response,
        Json,
    },
    crypto::hash_without_salt,
    db::{self, TxResult},
    id::{Id, Token},
};

/// A value used to query a single session.
#[derive(
    Display,
    DeserializeFromStr,
    SerializeDisplay,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
)]
pub(crate) enum SessionQuery {
    /// Queries the current authenticated session.
    #[display("{}", Self::CURRENT_STR)]
    Current,

    /// Queries a session by ID.
    Id(Token),
}

impl SessionQuery {
    /// The string representation of [`SessionQuery::Current`].
    const CURRENT_STR: &str = "$current";
}

impl FromStr for SessionQuery {
    type Err = <Id as FromStr>::Err;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str == Self::CURRENT_STR {
            Ok(Self::Current)
        } else {
            let id = str.parse()?;
            Ok(Self::Id(id))
        }
    }
}

/// A request path for this API route.
type PathParams = Path<SessionQuery>;

/// Deletes a session.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn delete(
    Path(session_query): PathParams,
    token: Option<AuthToken>,
) -> impl Response<DeleteResponse> {
    #[expect(
        unused_assignments,
        reason = "This will fix itself once the TODO is resolved."
    )]
    let mut response_header = None;

    match session_query {
        SessionQuery::Current => {
            let Some(AuthToken(token)) = token else {
                return Err(api::Error::ResourceNotFound);
            };

            let token_hash = hash_without_salt(&token);

            let sessions_deleted = db::transaction!(async |tx| -> TxResult<_, api::Error> {
                Ok(sqlx::query!(
                    "DELETE FROM sessions
                        WHERE token_hash = $1",
                    token_hash.as_ref(),
                )
                .execute(tx.as_mut())
                .await?)
            })
            .await?
            .rows_affected();

            if sessions_deleted == 0 {
                return Err(api::Error::ResourceNotFound);
            }

            response_header = Some(SessionCookie::expired().to_header());
        }
        SessionQuery::Id(_) => {
            // TODO: Implement signing out specific sessions in the account settings.
            return Err(api::Error::AccessDenied);
        }
    }

    Ok((
        StatusCode::OK,
        AppendHeaders(response_header),
        Json(DeleteResponse {}),
    ))
}

/// A `DELETE` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeleteResponse {}
