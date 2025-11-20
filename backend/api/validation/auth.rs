//! Utilities to help with validation of API request authentication credentials.

use serde::Deserialize;
use sqlx::PgTransaction;
use strum::IntoEnumIterator;
use strum_macros::{EnumDiscriminants, EnumIter};

use crate::{
    api::{
        self,
        validation::{Otp, UserPassword},
    },
    crypto::{verify_hash, verify_totp},
    db::{TxError, TxResult},
};

/// User credentials exclusively for first-factor authentication.
#[derive(Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub(crate) enum ExclusiveFirstFactorCredentials {
    /// Credentials for password-based authentication.
    Password {
        /// The user's password in plain text.
        password: UserPassword,
    },
}

/// User credentials exclusively for second-factor authentication.
#[derive(EnumDiscriminants, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[strum_discriminants(doc = "A 2FA method.", derive(EnumIter), name(SecondFactorAuthMethod))]
#[serde(rename_all = "camelCase", untagged)]
pub(crate) enum ExclusiveSecondFactorCredentials {
    /// Credentials for TOTP-based 2FA.
    Totp {
        /// The one-time password.
        otp: Otp,
    },
}

/// User credentials for multi-factor authentication that can't be divided into separate first
/// and second authentication factors.
#[derive(Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub(crate) enum ExclusiveMultiFactorCredentials {}

/// User credentials for first-factor authentication. Multi-factor authentication credentials are
/// also accepted because they cover the first factor.
#[derive(Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub(crate) enum FirstFactorCredentials {
    /// The user's exclusively first-factor authentication credentials.
    First(ExclusiveFirstFactorCredentials),

    /// The user's exclusively multi-factor authentication credentials, covering the first factor.
    Multi(ExclusiveMultiFactorCredentials),
}

/// User credentials where second-factor authentication is required (if enabled). Multi-factor
/// authentication credentials are also accepted because they cover the second factor, but never
/// required.
#[derive(Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub(crate) enum SecondFactorCredentials {
    /// The user's exclusively second-factor authentication credentials (if the user has 2FA
    /// enabled).
    Second(Option<ExclusiveSecondFactorCredentials>),

    /// The user's multi-factor authentication credentials, covering the second factor.
    Multi(ExclusiveMultiFactorCredentials),
}

/// User credentials where multi-factor authentication is required (if enabled for the user's chosen
/// authentication method).
#[derive(Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub(crate) enum MultiFactorCredentials {
    /// Credentials for MFA using separate first and second authentication factors.
    FirstAndSecond {
        /// The user's first-factor authentication credentials.
        #[serde(flatten)]
        first: ExclusiveFirstFactorCredentials,

        /// The user's second-factor authentication credentials (if the user has 2FA enabled).
        #[serde(flatten)]
        second: Option<ExclusiveSecondFactorCredentials>,
    },

    /// Credentials for MFA not using separate first and second authentication factors.
    Multi(ExclusiveMultiFactorCredentials),
}

impl SecondFactorAuthMethod {
    /// Checks if the 2FA method is enabled for a user.
    ///
    /// # Errors
    ///
    /// Returns an error if a database query fails.
    async fn is_enabled_for_user<UserId: AsRef<[u8]>>(
        &self,
        tx: &mut PgTransaction<'static>,
        user_id: &UserId,
    ) -> sqlx::Result<bool> {
        match self {
            Self::Totp => Ok(sqlx::query!(
                "SELECT TRUE AS exists FROM totp
                    WHERE user_id = $1",
                user_id.as_ref(),
            )
            .fetch_optional(tx.as_mut())
            .await?
            .is_some()),
        }
    }
}

pub(crate) trait VerifyCredentials {
    /// Verifies the credentials are correct.
    ///
    /// # Errors
    ///
    /// Returns an API error if verification fails. Returns a database error if a database query
    /// fails.
    async fn verify<UserId: AsRef<[u8]>>(
        &self,
        tx: &mut PgTransaction<'static>,
        user_id: &UserId,
    ) -> TxResult<(), api::Error>;
}

impl VerifyCredentials for ExclusiveFirstFactorCredentials {
    async fn verify<UserId: AsRef<[u8]>>(
        &self,
        tx: &mut PgTransaction<'static>,
        user_id: &UserId,
    ) -> TxResult<(), api::Error> {
        match self {
            Self::Password { password } => {
                let user = sqlx::query!(
                    "SELECT password_hash FROM users
                        WHERE id = $1",
                    user_id.as_ref(),
                )
                .fetch_one(tx.as_mut())
                .await?;

                if verify_hash(password, &user.password_hash) {
                    return Ok(());
                }
            }
        }

        Err(TxError::Abort(api::Error::FirstFactorCredentialsWrong))
    }
}

impl VerifyCredentials for ExclusiveSecondFactorCredentials {
    async fn verify<UserId: AsRef<[u8]>>(
        &self,
        tx: &mut PgTransaction<'static>,
        user_id: &UserId,
    ) -> TxResult<(), api::Error> {
        match self {
            Self::Totp { otp } => {
                if sqlx::query!(
                    "UPDATE totp
                        SET unused_backup_codes = array_remove(unused_backup_codes, $1)
                        WHERE user_id = $2 AND $1 = ANY (unused_backup_codes)",
                    otp.as_str(),
                    user_id.as_ref(),
                )
                .execute(tx.as_mut())
                .await?
                .rows_affected()
                    > 0
                {
                    return Ok(());
                }

                if let Some(totp) = sqlx::query!(
                    "UPDATE totp
                        SET otp_used_2nd_to_last = otp_used_last,
                            otp_used_last = $1
                        WHERE user_id = $2
                            AND $1 IS DISTINCT FROM otp_used_last
                            AND $1 IS DISTINCT FROM otp_used_2nd_to_last
                        RETURNING secret",
                    otp.as_str(),
                    user_id.as_ref(),
                )
                .fetch_optional(tx.as_mut())
                .await?
                    && verify_totp(otp, &totp.secret)
                {
                    return Ok(());
                }
            }
        }

        Err(TxError::Abort(api::Error::SecondFactorCredentialsWrong))
    }
}

impl VerifyCredentials for Option<ExclusiveSecondFactorCredentials> {
    async fn verify<UserId: AsRef<[u8]>>(
        &self,
        tx: &mut PgTransaction<'static>,
        user_id: &UserId,
    ) -> TxResult<(), api::Error> {
        if let Some(credentials) = self {
            return credentials.verify(tx, &user_id).await;
        }

        for auth_method in SecondFactorAuthMethod::iter() {
            if auth_method.is_enabled_for_user(tx, user_id).await? {
                return Err(TxError::Abort(api::Error::SecondFactorCredentialsWrong));
            }
        }

        Ok(())
    }
}

impl VerifyCredentials for ExclusiveMultiFactorCredentials {
    async fn verify<UserId: AsRef<[u8]>>(
        &self,
        _tx: &mut PgTransaction<'static>,
        _user_id: &UserId,
    ) -> TxResult<(), api::Error> {
        match *self {}
    }
}

impl VerifyCredentials for FirstFactorCredentials {
    async fn verify<UserId: AsRef<[u8]>>(
        &self,
        tx: &mut PgTransaction<'static>,
        user_id: &UserId,
    ) -> TxResult<(), api::Error> {
        match self {
            Self::First(first) => {
                first.verify(tx, user_id).await?;
            }
            Self::Multi(multi) => {
                multi.verify(tx, user_id).await?;
            }
        }

        Ok(())
    }
}

impl VerifyCredentials for SecondFactorCredentials {
    async fn verify<UserId: AsRef<[u8]>>(
        &self,
        tx: &mut PgTransaction<'static>,
        user_id: &UserId,
    ) -> TxResult<(), api::Error> {
        match self {
            Self::Second(second) => {
                second.verify(tx, user_id).await?;
            }
            Self::Multi(multi) => {
                multi.verify(tx, user_id).await?;
            }
        }

        Ok(())
    }
}

impl VerifyCredentials for MultiFactorCredentials {
    async fn verify<UserId: AsRef<[u8]>>(
        &self,
        tx: &mut PgTransaction<'static>,
        user_id: &UserId,
    ) -> TxResult<(), api::Error> {
        match self {
            Self::FirstAndSecond { first, second } => {
                first.verify(tx, user_id).await?;
                second.verify(tx, user_id).await?;
            }
            Self::Multi(multi) => {
                multi.verify(tx, user_id).await?;
            }
        }

        Ok(())
    }
}
