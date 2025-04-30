//! General database handling.

use std::error::Error;

use castaway::cast;
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgPoolOptions, Executor, PgPool};

/// Initializes the SQLx database pool, runs pending database migrations, and performs some initial
/// data population. Returns the pool once complete.
///
/// # Errors
///
/// Returns an error if the database connection or initial database operations fail.
///
/// # Panics
///
/// May panic if the database is already initialized.
pub(super) async fn initialize(db_url: &str) -> sqlx::Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .after_connect(|conn, _| {
            Box::pin(async move {
                conn.execute("SET default_transaction_isolation TO 'serializable';")
                    .await?;

                Ok(())
            })
        })
        .connect(db_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    sync_terms_version_to_db(&pool).await?;

    Ok(pool)
}

/// Checks for any updates to the terms of service and privacy notice and updates the database
/// accordingly.
///
/// # Errors
///
/// Returns an error if the database operations fail.
async fn sync_terms_version_to_db(db_pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut hasher = Sha256::new();
    hasher.update(include_bytes!("../frontend/components/TermsOfService.md"));
    let terms_hash = hasher.finalize();
    let terms_hash = terms_hash.as_slice();

    let mut hasher = Sha256::new();
    hasher.update(include_bytes!("../frontend/components/PrivacyNotice.md"));
    let privacy_hash = hasher.finalize();
    let privacy_hash = privacy_hash.as_slice();

    transaction!(&db_pool, async |tx| -> TxResult<_, sqlx::Error> {
        let Some(user_agreement) =
            sqlx::query!("SELECT terms_hash, privacy_hash FROM user_agreement")
                .fetch_optional(tx.as_mut())
                .await?
        else {
            sqlx::query!(
                "INSERT INTO user_agreement (terms_hash, privacy_hash)
                    VALUES ($1, $2)",
                terms_hash,
                privacy_hash,
            )
            .execute(tx.as_mut())
            .await?;

            return Ok(());
        };

        if user_agreement.terms_hash != terms_hash {
            sqlx::query!(
                "UPDATE user_agreement
                    SET terms_hash = $1,
                        terms_updated_at = now()",
                terms_hash,
            )
            .execute(tx.as_mut())
            .await?;
        }

        if user_agreement.privacy_hash != privacy_hash {
            sqlx::query!(
                "UPDATE user_agreement
                    SET privacy_hash = $1,
                        privacy_updated_at = now()",
                privacy_hash,
            )
            .execute(tx.as_mut())
            .await?;
        }

        Ok(())
    })
    .await?;

    Ok(())
}

/// The error result of a database transaction.
///
/// Doesn't implement [`Error`] to prevent an impl conflict.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum TxError<E> {
    /// Aborts the transaction and returns the wrapped error.
    Abort(E),

    /// Aborts the transaction and runs the `transaction!` callback again.
    Retry,
}

/// The SQLSTATE code for serialization failures.
const SERIALIZATION_FAILURE: &str = "40001";

impl<S, E> From<S> for TxError<E>
where
    // This `Error` bound may be overly restrictive but prevents an impl conflict.
    S: Error + 'static,
    E: From<S>,
{
    fn from(source: S) -> Self {
        match cast!(&source, &sqlx::Error) {
            Ok(sqlx::Error::Database(source))
                if source
                    .code()
                    .is_some_and(|code| code == SERIALIZATION_FAILURE) =>
            {
                Self::Retry
            }
            _ => Self::Abort(source.into()),
        }
    }
}

/// The result of a database transaction.
pub(crate) type TxResult<T, E> = Result<T, TxError<E>>;

/// Begins a database transaction with the maximum isolation level (`SERIALIZABLE`), retrying if the
/// database detects a race condition (serialization failure).
///
/// Maximum isolation is used to minimize the possibility of data races. This generally greatly
/// simplifies database operations and reduces the mental overhead of working with them.
macro_rules! transaction {
    ($db_pool:expr, $($ident:ident)* |$tx:ident| $(-> $Return:ty)? $block:block$(,)?) => {
        $crate::db::transaction!(
            $db_pool,
            $($ident)* |$tx: &mut ::sqlx::PgTransaction<'static>| $(-> $Return)? {
                $block
            },
        )
    };

    ($db_pool:expr, $callback:expr$(,)?) => {
        async {
            let db_pool: &::sqlx::PgPool = $db_pool;

            #[expect(clippy::allow_attributes, reason = "`unused_mut` isn't always expected")]
            #[allow(unused_mut, reason = "some callers need this to be `mut`")]
            let mut callback = $callback;

            #[expect(clippy::allow_attributes, reason = "`unused_mut` isn't always expected")]
            #[allow(unused_mut, reason = "some callers need this to be `mut`")]
            let mut callback = async || -> $crate::db::TxResult<_, _> {
                let mut tx = db_pool.begin().await?;

                let return_value = match callback(&mut tx).await {
                    Ok(value) => value,
                    Err(error) => return Err(error),
                };

                tx.commit().await?;
                Ok(return_value)
            };

            loop {
                match callback().await {
                    Ok(value) => break Ok(value),
                    Err($crate::db::TxError::Abort(error)) => break Err(error),
                    Err($crate::db::TxError::Retry) => {}
                }
            }
        }
    };
}

pub(crate) use transaction;
