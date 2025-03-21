use crate::mini_toml;
use crate::models::*;
use crate::schema::marketplace::dsl::*;
use diesel::SqliteConnection;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

pub async fn export_toml(conn: &mut SqliteConnection, target: &str) -> anyhow::Result<()> {
    let mut record: Vec<Marketplace> = marketplace
        .filter(platform.not_like("win32%"))
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

    tokio::fs::write(target, mini_toml::to_string(&record)).await?;

    Ok(())
}
