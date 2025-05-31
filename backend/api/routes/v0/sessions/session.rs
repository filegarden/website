//! A session.

use std::str::FromStr;

use axum::http::StatusCode;
use axum_macros::debug_handler;
use derive_more::Display;
use serde::Serialize;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

use crate::{
    api::{self, extract::Path, Json, Response},
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
    token: Option<Token>,
    cookies: Cookies,
) -> impl Response<DeleteResponse> {
    match session_query {
        SessionQuery::Current => {
            let Some(token) = token else {
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

            cookies.add(Cookie::build("token").max_age(Duration::seconds(0)).into());
        }
        SessionQuery::Id(_) => {
            return Err(api::Error::AccessDenied);
        }
    }

    Ok((StatusCode::OK, Json(DeleteResponse {})))
}

/// A `DELETE` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeleteResponse {}
