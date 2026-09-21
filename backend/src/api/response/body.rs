//! General types for use in API response body types.

use serde::Serialize;

use crate::id::{Id, IdInner as IdInnerTrait};

/// A reference to a user creation request.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UserRequest {
    /// The email address to verify.
    pub email: String,
}

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

    /// The timestamp this session was first created in Unix milliseconds.
    pub created_at: i64,

    /// The timestamp this session was last used in Unix milliseconds.
    pub accessed_at: i64,
}

/// A reference to a file.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "IdInner: AsRef<[u8]>")]
pub(crate) struct File<IdInner = <Id as IdInnerTrait>::Inner> {
    /// The file's creation timestamp in Unix milliseconds.
    pub created_at: i64,

    /// The file's modified timestamp in Unix milliseconds.
    pub modified_at: i64,

    /// Whether the file is complete.
    pub complete: bool,

    /// The file's ID.
    pub id: Id<IdInner>,

    /// The file's name.
    pub name: String,

    /// The file's final size in bytes.
    pub size: u64,

    /// The file's media type.
    pub r#type: String,

    /// Whether the file is shared.
    pub shared: bool,
}

/// A reference to a folder.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "IdInner: AsRef<[u8]>, BrowseKeyInner: AsRef<[u8]>")]
pub(crate) struct Folder<
    IdInner = <Id as IdInnerTrait>::Inner,
    BrowseKeyInner = <Id as IdInnerTrait>::Inner,
> {
    /// The folder's creation timestamp in Unix milliseconds.
    pub created_at: i64,

    /// The folder's ID.
    pub id: Id<IdInner>,

    /// The folder's name.
    pub name: String,

    /// The folder's browse key.
    pub browse_key: Id<BrowseKeyInner>,

    /// The total size in bytes of all of the folder's descendants.
    pub size: u64,

    /// Whether the folder is shared.
    pub shared: bool,
}
