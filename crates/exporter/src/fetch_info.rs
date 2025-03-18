use std::pin::pin;

use crate::models::Marketplace;
use code_api::code::{HttpClient, SortBy};
use diesel::prelude::*;
use futures::StreamExt;

use crate::schema::marketplace;
use tracing::{error, trace};

pub async fn fetch_marketplace(conn: &mut PgConnection) -> anyhow::Result<()> {
    let client = HttpClient::new().unwrap();
    let mut iter = pin!(client.get_extension_response(
        vec![],
        code_api::code::IQueryState {
            page_size: u16::MAX as u64,
            sort_by: SortBy::PUBLISHED_DATE,
            sort_order: code_api::code::SortOrder::DESCENDING,
            ..Default::default()
        }
    ));
    while let Some(item) = iter.next().await {
        let Ok(item) = item else {
            continue;
        };

        let values: Vec<_> = item
            .extensions
            .iter()
            .flat_map(|item| {
                item.versions.iter().filter_map(|version| {
                    let Ok(engne) = version.get_engine() else {
                        return None;
                    };
                    let visix = version.get_file(code_api::code::AssetType::Vsix)?;
                    let platform = version.target_platform.clone()?;
                    Some(Marketplace {
                        name: item.extension_name.clone(),
                        publisher: item.publisher.publisher_name.clone(),
                        version: version.version.clone(),
                        engine: engne,
                        platform,
                        assert_url: visix.source.clone(),
                        hash: None,
                        is_prerelease: Some(version.is_pre_release_version()),
                    })
                })
            })
            .collect();

        if let Err(err) = diesel::insert_into(marketplace::table)
            .values(&values)
            .on_conflict_do_nothing()
            .returning(Marketplace::as_returning())
            .get_result(conn)
        {
            error!(?err);
        } else {
            trace!("insert value");
        }
    }

    Ok(())
}
