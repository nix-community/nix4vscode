use redb::{ReadableTable, TableDefinition};
use std::{path::PathBuf, sync::LazyLock};
use tracing::*;

use crate::error::Error;

pub static GLOBAL_CACHER: LazyLock<Cacher> = LazyLock::new(|| {
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

    Cacher::new(path)
});

static TABLE_SHA256: TableDefinition<&str, &str> = TableDefinition::new("SHA256");
static TABLE_HTTP_CLIENT: TableDefinition<&str, &str> = TableDefinition::new("HTTP_CLIENT");

#[derive(Debug)]
pub enum CacheType {
    Cache256,
    HttpClient,
}

impl From<CacheType> for TableDefinition<'static, &str, &str> {
    fn from(value: CacheType) -> Self {
        match value {
            CacheType::Cache256 => TABLE_SHA256,
            CacheType::HttpClient => TABLE_HTTP_CLIENT,
        }
    }
}

pub struct Cacher(redb::Database);

impl Cacher {
    fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self(redb::Database::builder().create(path).unwrap())
    }
}

impl Cacher {
    pub fn get(&self, cache_type: CacheType, key: &str) -> anyhow::Result<String> {
        let r_txn = self.0.begin_read()?;
        let table = r_txn.open_table(cache_type.into())?;
        let value = table
            .get(key)?
            .ok_or_else(|| Error::CacheMissing(key.into()))?
            .value()
            .to_string();
        if value.is_empty() {
            return Err(Error::CacheMissing(key.into()).into());
        }
        trace!("Cache hint: {} -> {}", key, value);

        Ok(value)
    }

    pub fn insert(&self, cache_type: CacheType, key: &str, value: &str) -> anyhow::Result<()> {
        if value.is_empty() {
            // Clean cache
            return Ok(());
        }
        let wt = self.0.begin_write()?;
        {
            let mut table = wt.open_table(TABLE_SHA256)?;
            table.insert(key, value)?;
        }
        wt.commit()?;

        Ok(())
    }
}
