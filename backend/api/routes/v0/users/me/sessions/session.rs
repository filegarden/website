//! One of a particular user's sessions.

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
    db::{self, TxError, TxResult},
    id::Id,
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
    Id(Id),
}

impl SessionQuery {
    /// The string representation of [`SessionQuery::Current`].
    pub(crate) const CURRENT_STR: &str = "current";
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
    AuthToken(token_hash): AuthToken,
) -> impl Response<DeleteResponse> {
    let is_session_deleted = db::transaction!(async |tx| -> TxResult<_, api::Error> {
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

        let session_id = match &session_query {
            SessionQuery::Current => token_hash.as_ref(),
            SessionQuery::Id(id) => id.as_slice(),
        };

        let sessions_deleted = sqlx::query!(
            "DELETE FROM sessions
                WHERE user_id = $1 AND token_hash = $2",
            user_id,
            session_id,
        )
        .execute(tx.as_mut())
        .await?
        .rows_affected();

        Ok(sessions_deleted != 0)
    })
    .await?;

    if !is_session_deleted {
        return Err(api::Error::ResourceNotFound);
    }

    let response_header = match session_query {
        // The user deleted a session other than their current one.
        SessionQuery::Id(id) if id.as_slice() != token_hash.as_ref() => None,

        // The user deleted their current session.
        _ => Some(SessionCookie::expired().to_header()),
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
