//! A user.

use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::Serialize;

use crate::{
    api::{
        self,
        extract::{AuthToken, Path},
        response::Response,
        validation::UserQuery,
        Json,
    },
    crypto::hash_without_salt,
    db::{self, TxResult},
    id::Id,
};

/// A request path for this API route.
type PathParams = Path<UserQuery>;

/// Gets a user's public profile info.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(
    Path(user_query): PathParams,
    token: Option<AuthToken>,
) -> impl Response<GetResponse> {
    let (user_id, user_name) = match user_query {
        UserQuery::Me => {
            let Some(AuthToken(token)) = token else {
                return Err(api::Error::ResourceNotFound);
            };

            let token_hash = hash_without_salt(&token);

            let Some(user) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
                Ok(sqlx::query!(
                    "SELECT users.id, users.name FROM sessions
                        INNER JOIN users ON users.id = sessions.user_id
                        WHERE sessions.token_hash = $1",
                    token_hash.as_ref(),
                )
                .fetch_optional(tx.as_mut())
                .await?)
            })
            .await?
            else {
                return Err(api::Error::ResourceNotFound);
            };

            (Id::from(user.id), user.name)
        }
        UserQuery::Id(id) => {
            let Some(user) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
                Ok(sqlx::query!(
                    "SELECT name FROM users
                        WHERE id = $1",
                    id.as_slice(),
                )
                .fetch_optional(tx.as_mut())
                .await?)
            })
            .await?
            else {
                return Err(api::Error::ResourceNotFound);
            };

            (id, user.name)
        }
    };

    Ok((
        StatusCode::OK,
        Json(GetResponse {
            id: user_id,
            name: user_name,
        }),
    ))
}

/// A `GET` response body for this API route.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetResponse {
    /// The user's ID.
    pub id: Id,

    /// The user's name.
    pub name: String,
}
