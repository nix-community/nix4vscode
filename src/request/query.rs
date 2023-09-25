use serde::{Deserialize, Serialize};

use crate::config::Extension;

use super::*;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub filters: Vec<IQueryState>,
    pub asset_types: Vec<String>,
    pub flags: u32,
}

impl Query {
    pub fn new(extensions: &[Extension]) -> Self {
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
                criteria: extensions
                    .iter()
                    .map(|item| ICriterium {
                        filter_type: FilterType::SEARCH_TEXT,
                        value: format!("{}.{}", item.publisher_name, item.extension_name),
                    })
                    .chain(fixed)
                    .collect(),
                ..Default::default()
            }],
            asset_types: Default::default(),
            flags: RequestFlags::default().bits(),
        }
    }
}
