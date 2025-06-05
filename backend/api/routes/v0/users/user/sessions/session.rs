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
        validation::UserQuery,
        Json,
    },
    crypto::hash_without_salt,
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
type PathParams = Path<(UserQuery, SessionQuery)>;

/// Deletes a session.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn delete(
    Path((user_query, session_query)): PathParams,
    AuthToken(token): AuthToken,
) -> impl Response<DeleteResponse> {
    let token_hash = hash_without_salt(&token);

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

        if let UserQuery::Id(queried_user_id) = &user_query {
            if **queried_user_id != user_id {
                return Err(TxError::Abort(api::Error::AccessDenied));
            }
        }

        let sessions_deleted = sqlx::query!(
            "DELETE FROM sessions
                WHERE user_id = $1 AND token_hash = $2",
            user_id,
            token_hash.as_ref(),
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
