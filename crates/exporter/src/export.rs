mod data;
use std::collections::BTreeMap;

use crate::mini_json;
use crate::models::*;
use crate::schema::marketplace::dsl::*;
use data::ExportedData;
use diesel::prelude::*;
use diesel::SqliteConnection;
use itertools::Itertools;
use lazy_regex::regex;
use rayon::prelude::*;

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
        .filter(is_prerelease.eq(false))
        .select(Marketplace::as_select())
        .load(conn)?;

    record.par_iter_mut().for_each(|item| {
        item.name = format!("{}.{}", item.publisher, item.name).to_lowercase();
        if let Some(url) = minilizer_url(&item.assert_url) {
            item.assert_url = url;
        }
    });

    let mut data = BTreeMap::<String, Vec<ExportedData>>::new();

    for (key, chunk) in &record.into_iter().chunk_by(|item| item.name.to_string()) {
        let mut chunk: Vec<ExportedData> = chunk.map(Into::into).collect();
        chunk.sort();
        data.entry(key.clone()).or_default();
        data.entry(key).and_modify(|v| {
            v.extend(chunk);
        });
    }

    tokio::fs::write(target, mini_json::to_string(&data)).await?;

    Ok(())
}

fn minilizer_url(url: &str) -> Option<String> {
    let re = regex!(
        r#"https://.*.gallerycdn.vsassets.io/extensions/.*/.*/.*/(\d+)/Microsoft.VisualStudio.Services.VSIXPackage"#
    );
    let captures = re.captures(url)?;
    if captures.len() != 2 {
        return None;
    }
    Some(captures[1].to_string())
}
