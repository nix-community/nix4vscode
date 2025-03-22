use crate::models::*;
use crate::schema::marketplace::dsl::*;
use diesel::prelude::*;
use diesel::SqliteConnection;
use serde::Deserialize;
use serde::Serialize;

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

    record.sort();

    #[derive(Serialize, Deserialize)]
    struct Extension {
        extension: Vec<Marketplace>,
    }

    let record = Extension { extension: record };

    tokio::fs::write(target, serde_json::to_string_pretty(&record)?).await?;

    Ok(())
}
