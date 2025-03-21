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
)]
#[diesel(table_name = crate::schema::marketplace)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Marketplace {
    pub name: String,
    pub publisher: String,
    pub version: String,
    pub engine: String,
    #[serde(skip_serializing_if = "is_universal")]
    pub platform: String,
    pub assert_url: String,
    #[serde(skip)]
    pub is_prerelease: bool,
    pub hash: Option<String>,
}

fn is_universal(name: &str) -> bool {
    name == "universal"
}
