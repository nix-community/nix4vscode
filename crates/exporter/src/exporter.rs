mod data;
mod mini_json;

use std::collections::BTreeMap;

use crate::models::*;
use crate::schema::marketplace::dsl::*;
use crate::utils::version_compare;
use data::ExportedData;
use diesel::SqliteConnection;
use diesel::prelude::*;
use itertools::Itertools;

pub async fn export_data(conn: &mut SqliteConnection, target: &str) -> anyhow::Result<()> {
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

    let mut out_files: Vec<BTreeMap<String, Vec<ExportedData>>> = vec![];

    for _ in 0..128 {
        out_files.push(Default::default());
    }

    for (key, data) in data {
        let hkey = crown::hash::blake2b::sum256(key.as_bytes());
        let idx = u32::from_le_bytes(hkey[0..4].try_into().unwrap()) as usize;
        let idx = idx % out_files.len();
        let v = out_files.get_mut(idx).unwrap();
        v.insert(key, data);
    }

    if let Err(err) = std::fs::create_dir_all(target) {
        tracing::error!("create {target} failed: {err}");
    }

    for (idx, data) in out_files.into_iter().enumerate() {
        let serialized = mini_json::to_string(&data);

        tokio::fs::write(format!("{target}/data_{idx}.json"), serialized).await?;
    }

    Ok(())
}
