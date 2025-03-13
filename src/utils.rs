mod cacher;

pub use cacher::*;

use crate::error::Error;
use tracing::*;

pub async fn get_sha256(url: &str) -> anyhow::Result<String> {
    trace!("get sha256 of {url}");
    if let Ok(val) = GLOBAL_CACHER.get(CacheType::Cache256, url) {
        return Ok(val);
    }

    let sha256 = tokio::process::Command::new("nix-prefetch-url")
        .arg(url)
        .output()
        .await?
        .stdout;
    let _ = nix_gc().await;

    let sha256 = String::from_utf8(sha256).unwrap().trim().to_owned();
    if sha256.is_empty() {
        return Err(Error::Sha256Error(url.into()).into());
    }

    let _ = GLOBAL_CACHER.insert(CacheType::Cache256, url, &sha256);
    Ok(sha256)
}

pub async fn nix_gc() {
    let _ = tokio::process::Command::new("nix")
        .arg("store")
        .arg("gc")
        .output()
        .await;
}
