use log::*;
use std::path::PathBuf;

use redb::{ReadableTable, TableDefinition};

use crate::error::Error;

pub static CACHER: std::sync::LazyLock<redb::Database> = std::sync::LazyLock::new(|| {
    let path = format!(
        "{}/{}/{}",
        std::env::var("HOME").unwrap(),
        ".cache",
        "nix4vscode"
    );
    let path = PathBuf::from(path);
    if !path.exists() {
        std::fs::create_dir_all(path.clone()).unwrap();
    }
    let path = path.join("cache.redb");

    redb::Database::builder().create(path).unwrap()
});

static TABLE_SHA256: TableDefinition<&str, &str> = TableDefinition::new("SHA256");
pub static TABLE_HTTP_CLIENT: TableDefinition<&str, &str> = TableDefinition::new("HTTP_CLIENT");

pub async fn get_sha256(url: &str) -> anyhow::Result<String> {
    let value = (|| -> anyhow::Result<String> {
        let r_txn = CACHER.begin_read()?;
        let table = r_txn.open_table(TABLE_SHA256)?;
        let value = table
            .get(url)?
            .ok_or_else(|| Error::CacheMissing(url.into()))?
            .value()
            .to_string();
        if value.is_empty() {
            return Err(Error::CacheMissing(url.into()).into());
        }
        trace!("Cache hint: {} -> {}", url, value);

        Ok(value)
    })();

    if let Ok(val) = value {
        return Ok(val);
    }

    let sha256 = tokio::process::Command::new("nix-prefetch-url")
        .arg(url.clone())
        .output()
        .await?
        .stdout;

    let sha256 = String::from_utf8(sha256).unwrap().trim().to_owned();
    if sha256.is_empty() {
        return Err(Error::Sha256Error(url.into()).into());
    }
    let _ = (|| -> anyhow::Result<()> {
        let wt = CACHER.begin_write()?;
        {
            let mut table = wt.open_table(TABLE_SHA256)?;
            table.insert(url, sha256.as_str())?;
        }
        wt.commit()?;

        Ok(())
    })();

    Ok(sha256)
}
