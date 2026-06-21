//! A user.

use axum::http::StatusCode;
use axum_macros::debug_handler;

use crate::{
    api::{
        self, Json,
        extract::Path,
        response::{Response, body::User},
    },
    db::{self, TxResult},
    id::Id,
};

/// A request path for this API route.
type PathParams = Path<Id>;

/// Gets a user's public profile info.
///
/// # Errors
///
/// See [`crate::api::Error`].
#[debug_handler]
pub(crate) async fn get(Path(user_id): PathParams) -> impl Response<GetResponse> {
    let Some(user) = db::transaction!(async |tx| -> TxResult<_, api::Error> {
        Ok(sqlx::query!(
            "SELECT name FROM users
                WHERE id = $1",
            user_id.as_slice(),
        )
        .fetch_optional(tx.as_mut())
        .await?)
    })
    .await?
    else {
        return Err(api::Error::ResourceNotFound);
    };

    Ok((
        StatusCode::OK,
        Json(GetResponse {
            id: user_id,
            name: user.name,
        }),
    ))
}

/// A `GET` response body for this API route.
pub(crate) type GetResponse = User;
