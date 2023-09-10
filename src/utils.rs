use redb::{ReadableTable, TableDefinition};
use standard_paths::{LocationType, StandardPaths};

static CACHER: std::sync::LazyLock<redb::Database> = std::sync::LazyLock::new(|| {
    let path = StandardPaths::new("nix4vscode", "cathaysia");
    let path = path
        .writable_location(LocationType::GenericCacheLocation)
        .unwrap();
    let path = path.join("nix4vscode");
    if !path.exists() {
        std::fs::create_dir_all(path.clone()).unwrap();
    }
    let path = path.join("cache.redb");

    redb::Database::builder().create(path).unwrap()
});

static TABLE_SHA256: TableDefinition<&str, &str> = TableDefinition::new("SHA256");

pub async fn get_sha256(url: &str) -> anyhow::Result<String> {
    let value = (|| -> anyhow::Result<String> {
        let r_txn = CACHER.begin_read()?;
        let table = r_txn.open_table(TABLE_SHA256)?;
        let value = table
            .get(url)?
            .ok_or_else(|| redb::Error::InvalidSavepoint)?
            .value()
            .to_string();

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
    let _ = (|| -> anyhow::Result<()> {
        let wt = CACHER.begin_write()?;
        let mut table = wt.open_table(TABLE_SHA256)?;
        table.insert(url, sha256.as_str())?;

        Ok(())
    })();

    Ok(sha256)
}
