use derive::api;

use super::*;

#[api]
pub struct ICriterium {
    pub filter_type: FilterType,
    pub value: String,
}

#[api(nodefault)]
pub struct IQueryState {
    pub page_number: u64,
    #[serde(skip_serializing_if = "is_u64_max")]
    pub page_size: u64,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub flags: RequestFlags,
    pub criteria: Vec<ICriterium>,
    pub asset_types: Vec<String>,
    pub source: String,
}

fn is_u64_max(v: &u64) -> bool {
    *v == u64::MAX
}

impl IQueryState {
    pub const DEFAULT_PAGE_SIZE: u64 = 10;
}

impl Default for IQueryState {
    fn default() -> Self {
        Self {
            page_number: 1,
            page_size: Self::DEFAULT_PAGE_SIZE,
            sort_by: SortBy::NONE_OR_RELEVANCE,
            sort_order: SortOrder::DEFAULT,
            flags: RequestFlags::None,
            criteria: Default::default(),
            asset_types: Default::default(),
            source: Default::default(),
        }
    }
}
