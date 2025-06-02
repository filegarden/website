//! Helper functions that perform common database operations.

use sqlx::PgTransaction;

use crate::{
    crypto::hash_without_salt,
    db::{TxError, TxResult},
    id::Token,
};

/// Creates a new user session and returns its token.
///
/// # Errors
///
/// Returns a transaction error if the database query fails.
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

    match sqlx::query!(
        "INSERT INTO sessions (token_hash, user_id)
            VALUES ($1, $2)",
        token_hash.as_ref(),
        user_id.as_ref(),
    )
    .execute(tx.as_mut())
    .await
    {
        Err(sqlx::Error::Database(error)) if error.constraint() == Some("sessions_pkey") => {
            return Err(TxError::Retry);
        }
        result => result?,
    };

    Ok(token)
}
