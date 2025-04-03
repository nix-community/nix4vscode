use std::collections::HashSet;
use std::sync::Arc;

use crate::schema::marketplace::dsl::*;
use anyhow::bail;
use diesel::prelude::*;
use diesel::SqliteConnection;
use futures::stream;
use futures::StreamExt;
use tokio::sync::Mutex;
use tracing::info;
use tracing::*;

pub async fn fetch_hash(conn: &mut SqliteConnection, batch_size: usize) -> anyhow::Result<()> {
    let urls: HashSet<String> = marketplace
        .filter(hash.is_null().or(hash.eq("")))
        .select(assert_url)
        .load(conn)?
        .into_iter()
        .collect();
    info!("count: {}", urls.len());

    let conn = Arc::new(Mutex::new(conn));

    let _: Vec<_> = stream::iter(urls)
        .enumerate()
        .map(|(idx, url)| {
            let conn = conn.clone();
            async move {
                let now = tokio::time::Instant::now();
                let _ = nix_gc().await;
                let Ok(file_hash) = compute_hash(&url).await else {
                    return;
                };
                let escaped = now.elapsed().as_secs();
                debug!("[{idx}] compute hash: {file_hash} of {url:?}, costs {escaped} sec.");

                let mut conn = conn.lock().await;
                let conn: &mut SqliteConnection = &mut conn;
                let now = tokio::time::Instant::now();
                if let Err(err) = diesel::update(marketplace)
                    .filter(assert_url.eq(&url))
                    .set(hash.eq(file_hash))
                    .execute(conn)
                {
                    error!(?err);
                }
                let sec = now.elapsed().as_secs();
                debug!("update db cost {sec} seconds");
            }
        })
        .buffer_unordered(batch_size)
        .collect()
        .await;

    Ok(())
}

pub async fn compute_hash(url: &str) -> anyhow::Result<String> {
    let sha256 = tokio::process::Command::new("nix-prefetch-url")
        .arg(url)
        .output()
        .await?
        .stdout;

    let h = String::from_utf8(sha256)?;
    let h = h.trim();

    if h.is_empty() {
        bail!("hash is invalid");
    }

    Ok(h.to_string())
}

pub async fn nix_gc() -> anyhow::Result<()> {
    let _ = tokio::process::Command::new("nix")
        .arg("store")
        .arg("gc")
        .output()
        .await;
    Ok(())
}
