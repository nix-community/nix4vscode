use vscode_derive::api;

use super::*;

#[api(Default)]
pub struct Query {
    pub filters: Vec<IQueryState>,
    pub asset_types: Vec<String>,
    pub flags: u32,
}
