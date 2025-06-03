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
    pub(crate) const CURRENT_STR: &str = "$current";
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
    AuthToken(token): AuthToken,
) -> impl Response<DeleteResponse> {
    let token_hash = hash_without_salt(&token);

    let response_header = match session_query {
        // The user requested deletion of a session other than their current one.
        SessionQuery::Id(id) if id.as_ref() != token_hash.as_ref() => {
            // TODO: Implement signing out specific sessions in the account settings.
            return Err(api::Error::AccessDenied);
        }

        // The user requested deletion of their current session.
        _ => {
            let sessions_deleted = db::transaction!(async |tx| -> TxResult<_, api::Error> {
                Ok(sqlx::query!(
                    "DELETE FROM sessions
                        WHERE token_hash = $1",
                    token_hash.as_ref(),
                )
                .execute(tx.as_mut())
                .await?
                .rows_affected())
            })
            .await?;

            if sessions_deleted == 0 {
                return Err(api::Error::ResourceNotFound);
            }

            Some(SessionCookie::expired().to_header())
        }
    };

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
