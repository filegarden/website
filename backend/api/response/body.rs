//! General types for use in API response body types.

use serde::Serialize;

use crate::id::Id;

/// A reference to a user.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "IdType: AsRef<[u8]>")]
pub(crate) struct User<IdType = Vec<u8>> {
    /// The user's ID.
    pub id: Id<IdType>,

    /// The user's name.
    pub name: String,
}
