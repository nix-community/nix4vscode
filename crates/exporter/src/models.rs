use diesel::prelude::*;

#[derive(
    Queryable,
    Selectable,
    Insertable,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Clone,
)]
#[diesel(table_name = crate::schema::marketplace)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Marketplace {
    pub name: String,
    pub publisher: String,
    pub version: String,
    pub engine: String,
    pub platform: String,
    pub is_prerelease: bool,
    pub hash: Option<String>,
    pub url: Option<String>,
}

impl Marketplace {
    pub const AARCH64_DARWIN: &str = "aarch64-darwin";
    pub const AARCH64_LINUX: &str = "aarch64-linux";
    pub const X86_64_DARWIN: &str = "x86_64-darwin";
    pub const X86_64_LINUX: &str = "x86_64-linux";
}
