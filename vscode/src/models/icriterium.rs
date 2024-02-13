use super::*;
use vscode_derive::api;

#[api(Default)]
pub struct ICriterium {
    filter_type: FilterType,
    value: String,
}
