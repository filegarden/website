//! General types for use in API response body types.

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::id::{Id, IdInner as IdInnerTrait};

/// A reference to a user.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "IdInner: AsRef<[u8]>")]
pub(crate) struct User<IdInner = <Id as IdInnerTrait>::Inner> {
    /// The user's ID.
    pub id: Id<IdInner>,

    /// The user's name.
    pub name: String,
}

/// A reference to a session.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "IdInner: AsRef<[u8]>")]
pub(crate) struct Session<IdInner = <Id as IdInnerTrait>::Inner> {
    /// The session's ID.
    pub id: Id<IdInner>,

    /// The timestamp this session was first created.
    pub created_at: DateTime<Utc>,

    /// The timestamp this session was last used.
    pub accessed_at: DateTime<Utc>,
}
