use derive::api;

use crate::config::Extension;

use super::*;

#[api]
pub struct Query {
    pub filters: Vec<IQueryState>,
    pub asset_types: Vec<String>,
    pub flags: u32,
}

impl Query {
    pub fn new(extensions: &[Extension], page_number: u64, args: IQueryState) -> Self {
        let fixed = vec![
            ICriterium {
                filter_type: FilterType::TARGET,
                value: "Microsoft.VisualStudio.Code".into(),
            },
            ICriterium {
                filter_type: FilterType::EXCLUDE_WITH_FLAGS,
                value: "4096".into(),
            },
        ];
        Query {
            filters: vec![IQueryState {
                page_number,
                criteria: extensions
                    .iter()
                    .map(|item| ICriterium {
                        filter_type: FilterType::EXTENSION_NAME,
                        value: format!("{}.{}", item.publisher_name, item.extension_name),
                    })
                    .chain(fixed)
                    .collect(),
                ..args
            }],
            asset_types: Default::default(),
            flags: RequestFlags::default().bits(),
        }
    }

    pub fn create_search(publisher_name: String, extension_name: String) -> Self {
        Query {
            filters: vec![IQueryState {
                criteria: vec![
                    ICriterium {
                        filter_type: FilterType::SEARCH_TEXT,
                        value: format!("{}.{}", publisher_name, extension_name),
                    },
                    ICriterium {
                        filter_type: FilterType::TARGET,
                        value: "Microsoft.VisualStudio.Code".into(),
                    },
                    ICriterium {
                        filter_type: FilterType::EXCLUDE_WITH_FLAGS,
                        value: "4096".into(),
                    },
                ],
                ..Default::default()
            }],
            asset_types: Default::default(),
            flags: RequestFlags::IncludeLatestVersionOnly.bits(),
        }
    }
}
