use crate::mini_json;
use crate::models::*;
use crate::schema::marketplace::dsl::*;
use diesel::prelude::*;
use diesel::SqliteConnection;
use lazy_regex::regex;
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
    record.iter_mut().for_each(|item| {
        item.name = format!("{}.{}", item.publisher, item.name).to_lowercase();
        let re  = regex!(r#"https://.*.gallerycdn.vsassets.io/extensions/.*/.*/.*/(\d+)/Microsoft.VisualStudio.Services.VSIXPackage"#);
        let Some(captures) = re.captures(&item.assert_url) else {
            return;
        };
        if captures.len() != 2 {
            return;
        }
        item.assert_url=captures[1].to_string();
    });

    #[derive(Serialize, Deserialize)]
    struct Extension {
        extension: Vec<Marketplace>,
    }

    let record = Extension { extension: record };

    tokio::fs::write(target, mini_json::to_string(&record)).await?;

    Ok(())
}
