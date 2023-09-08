use serde::{Deserialize, Serialize};

use super::*;
use crate::{config::Config, data};
use log::*;

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
                criteria: config
                    .extensions
                    .iter()
                    .map(|item| ICriterium {
                        filter_type: FilterType::EXTENSION_NAME,
                        value: format!("{}.{}", item.publisher_name, item.extension_name),
                    })
                    .chain(fixed.into_iter())
                    .collect(),
                ..Default::default()
            }],
            asset_types: Default::default(),
            flags: RequestFlags::default().bits(),
        }
    }

    pub async fn get_response(
        &self,
        client: &reqwest::Client,
    ) -> anyhow::Result<data::IRawGalleryQueryResult> {
        let body = serde_json::to_string(&self)?;
        debug!("{body}");
        Ok(client
            .post("https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery")
            .header(
                "Accept",
                "Application/json; charset=utf-8; api-version=7.2-preview.1",
            )
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?
            .json::<data::IRawGalleryQueryResult>()
            .await?)
    }
}
