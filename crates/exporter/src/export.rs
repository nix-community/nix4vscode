mod data;
use std::collections::BTreeMap;

use crate::mini_json;
use crate::models::*;
use crate::schema::marketplace::dsl::*;
use crate::utils::version_compare;
use data::ExportedData;
use diesel::SqliteConnection;
use diesel::prelude::*;
use itertools::Itertools;

pub async fn export_toml(conn: &mut SqliteConnection, target: &str) -> anyhow::Result<()> {
    let mut record: Vec<Marketplace> = marketplace
        .filter(
            platform
                .eq("linux-x64")
                .or(platform.eq("linux-arm64"))
                .or(platform.eq("linux-armhf"))
                .or(platform.eq("darwin-x64"))
                .or(platform.eq("darwin-arm64"))
                .or(platform.eq("universal")),
        )
        .filter(hash.is_not_null())
        .filter(hash.is_not(""))
        .select(Marketplace::as_select())
        .load(conn)?;

    record
        .iter_mut()
        .for_each(|item| item.name = format!("{}.{}", item.publisher, item.name).to_lowercase());

    let mut data = BTreeMap::<String, Vec<ExportedData>>::new();

    for (key, chunk) in &record.into_iter().chunk_by(|item| item.name.to_string()) {
        let mut chunk: Vec<ExportedData> = chunk.map(Into::into).collect();
        chunk.sort();
        data.entry(key.clone()).or_default();
        data.entry(key).and_modify(|v| {
            v.extend(chunk);
        });
    }

    // sort by desc
    data.iter_mut().for_each(|(_, v)| {
        v.sort_by(|a, b| version_compare(&b.v, &a.v));
    });

    tokio::fs::write(target, mini_json::to_string(&data)).await?;

    Ok(())
}
