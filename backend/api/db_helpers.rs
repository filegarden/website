//! Helper functions that perform common database operations.

use sqlx::PgTransaction;

use crate::{
    api,
    crypto::hash_without_salt,
    db::{TxError, TxResult},
    id::Token,
};

/// Creates a new user session and returns its token.
///
/// # Errors
///
/// Returns an error if a database query fails.
pub(crate) async fn create_session<E>(
    tx: &mut PgTransaction<'static>,
    user_id: &[u8],
) -> TxResult<Token, E>
where
    E: From<sqlx::Error>,
{
    let token = Token::generate();
    let token_hash = hash_without_salt(&token);

    sqlx::query!(
        "INSERT INTO sessions (token_hash, user_id)
            VALUES ($1, $2)",
        token_hash.as_ref(),
        user_id,
    )
    .execute(tx.as_mut())
    .await?;

    Ok(token)
}

/// `SELECT`s a folder's ID and name paths if the user has permission to modify the folder's
/// contents.
///
/// # Errors
///
/// Returns an error if a database query fails, or if the user doesn't have access to the folder.
pub(crate) async fn query_folder_paths_to_modify_contents(
    tx: &mut PgTransaction<'static>,
    user_id: &[u8],
    folder_id: &[u8],
) -> TxResult<(Vec<Vec<u8>>, Vec<String>), api::Error> {
    let Some(folder) = sqlx::query!(
        "SELECT name, parent_id_path, parent_name_path FROM folders
            WHERE id = $1 AND owner_id = $2",
        folder_id,
        user_id,
    )
    .fetch_optional(tx.as_mut())
    .await?
    else {
        return Err(TxError::Abort(api::Error::AccessDenied));
    };

    let mut id_path = folder.parent_id_path;
    id_path.push(folder_id.to_vec());

    let mut name_path = folder.parent_name_path;
    name_path.push(folder.name);

    Ok((id_path, name_path))
}
