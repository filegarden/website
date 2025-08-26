//! Helper functions that perform common database operations.

use sqlx::{PgTransaction, postgres::PgQueryResult};

use crate::{crypto::hash_without_salt, db::TxResult, id::Token};

/// Creates a new user session and returns its token.
///
/// # Errors
///
/// Returns an error if a database query fails.
pub(crate) async fn create_session<UserId, E>(
    tx: &mut PgTransaction<'static>,
    user_id: &UserId,
) -> TxResult<Token, E>
where
    UserId: AsRef<[u8]>,
    E: From<sqlx::Error>,
{
    let token = Token::generate();
    let token_hash = hash_without_salt(&token);

    sqlx::query!(
        "INSERT INTO sessions (token_hash, user_id)
            VALUES ($1, $2)",
        token_hash.as_ref(),
        user_id.as_ref(),
    )
    .execute(tx.as_mut())
    .await?;

    Ok(token)
}

/// Deletes all of a user's sessions. This is a conventionally expected security feature whenever a
/// user's password is changed.
///
/// # Errors
///
/// Returns an error if a database query fails.
pub(crate) async fn delete_all_sessions_for_user<UserId>(
    tx: &mut PgTransaction<'static>,
    user_id: &UserId,
) -> sqlx::Result<PgQueryResult>
where
    UserId: AsRef<[u8]>,
{
    sqlx::query!(
        "DELETE FROM sessions
            WHERE user_id = $1",
        user_id.as_ref(),
    )
    .execute(tx.as_mut())
    .await
}
