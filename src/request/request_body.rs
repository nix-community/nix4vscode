use serde::{Deserialize, Serialize};

use crate::config::Config;

use super::*;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct ICriterium {
    pub filter_type: FilterType,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IQueryState {
    pub page_number: u64,
    pub page_size: u64,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub flags: RequestFlags,
    pub criteria: Vec<ICriterium>,
    pub asset_types: Vec<String>,
    pub source: String,
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

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub filters: Vec<IQueryState>,
    pub asset_types: Vec<String>,
    pub flags: u32,
}

impl Query {
    pub fn new(config: &Config) -> Self {
        Query {
            filters: vec![IQueryState {
                criteria: config
                    .extensions
                    .iter()
                    .map(|item| ICriterium {
                        filter_type: FilterType::EXTENSION_NAME,
                        value: format!("{}.{}", item.publisher_name, item.extension_name),
                    })
                    .intersperse(ICriterium {
                        filter_type: FilterType::TARGET,
                        value: "Microsoft.VisualStudio.Code".into(),
                    })
                    .intersperse(ICriterium {
                        filter_type: FilterType::EXCLUDE_WITH_FLAGS,
                        value: "4096".into(),
                    })
                    .collect(),
                ..Default::default()
            }],
            asset_types: Default::default(),
            flags: RequestFlags::default().bits(),
        }
    }
}
