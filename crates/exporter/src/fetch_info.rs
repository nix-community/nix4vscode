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
                item.versions.iter().filter_map(|v| {
                    let Ok(engne) = v.get_engine() else {
                        return None;
                    };
                    let visix = v.get_file(code_api::code::AssetType::Vsix)?;
                    let platform = v.target_platform.clone()?;
                    Some(Marketplace {
                        name: item.extension_name.clone(),
                        publisher: item.publisher.publisher_name.clone(),
                        version: v.version.clone(),
                        engine: engne,
                        platform,
                        assert_url: visix.source.clone(),
                        hash: None,
                        is_prerelease: Some(v.is_pre_release_version()),
                    })
                })
            })
            .collect();

        for i in &values {
            use crate::schema::marketplace::dsl::*;
            if let Err(err) = diesel::update(marketplace)
                .filter(publisher.eq(&i.publisher))
                .filter(name.eq(&i.name))
                .filter(version.eq(&i.version))
                .filter(engine.eq(&i.engine))
                .filter(platform.eq(&i.platform))
                .filter(assert_url.eq(&i.assert_url))
                .set(is_prerelease.eq(i.is_prerelease))
                .execute(conn)
            {
                error!(?err);
            }
        }

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
