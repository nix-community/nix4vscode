use std::sync::Arc;

use crate::models::Marketplace;
use crate::schema::marketplace::dsl::*;
use crate::utils::get_assert_url;
use anyhow::bail;
use code_api::code::ApiEndpoint;
use diesel::SqliteConnection;
use diesel::prelude::*;
use futures::StreamExt;
use futures::stream;
use tokio::sync::Mutex;
use tracing::info;
use tracing::*;

pub async fn fetch_hash(
    conn: &mut SqliteConnection,
    batch_size: usize,
    endpoint: ApiEndpoint,
) -> anyhow::Result<()> {
    let is_open_vsx = matches!(endpoint, ApiEndpoint::OpenVsx);
    let urls: Vec<Marketplace> = marketplace
        .filter(hash.is_null().or(hash.eq("")))
        .select(Marketplace::as_select())
        .load(conn)?;
    info!("count: {}", urls.len());

    let conn = Arc::new(Mutex::new(conn));

    let _: Vec<_> = stream::iter(urls)
        .enumerate()
        .map(|(idx, item)| {
            let conn = conn.clone();
            async move {
                let now = tokio::time::Instant::now();
                let _ = nix_gc().await;
                let url = get_assert_url(
                    is_open_vsx,
                    &item.publisher,
                    &item.name,
                    &item.version,
                    if item.platform == "universal" {
                        None
                    } else {
                        Some(&item.platform)
                    },
                );
                let Ok(file_hash) = compute_hash(&url).await.inspect_err(|err| error!(?err)) else {
                    return;
                };
                let escaped = now.elapsed().as_secs();
                debug!("[{idx}] compute hash: {file_hash} of {url:?}, costs {escaped} sec.");

                let mut conn = conn.lock().await;
                let conn: &mut SqliteConnection = &mut conn;
                let now = tokio::time::Instant::now();
                if let Err(err) = diesel::update(marketplace)
                    .filter(publisher.eq(item.publisher))
                    .filter(name.eq(item.name))
                    .filter(version.eq(item.version))
                    .filter(platform.eq(item.platform))
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
        bail!("hash is invalid of {url}");
    }

    Ok(h.to_string())
}

pub async fn nix_gc() -> anyhow::Result<()> {
    let _ = tokio::process::Command::new("nix")
        .arg("store")
        .arg("gc")
        .output()
        .await?;
    Ok(())
}
