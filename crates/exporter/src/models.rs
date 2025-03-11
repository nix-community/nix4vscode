use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::marketplace)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Marketplace {
    pub name: String,
    pub publisher: String,
    pub version: String,
    pub engine: String,
    pub platform: String,
    pub assert_url: String,
    pub hash: Option<String>,
}
