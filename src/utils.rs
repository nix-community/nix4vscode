mod cacher;

pub use cacher::*;

use crate::error::Error;

pub async fn get_sha256(url: &str) -> anyhow::Result<String> {
    if let Ok(val) = GLOBAL_CACHER.get(CacheType::Cache256, url) {
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

    let _ = GLOBAL_CACHER.insert(CacheType::Cache256, url, &sha256);
    Ok(sha256)
}
