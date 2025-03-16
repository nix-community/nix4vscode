use std::collections::HashSet;
use std::process::exit;
use std::sync::Arc;

use crate::schema::marketplace::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
use scopeguard::defer;
use tokio::select;
use tokio::sync::mpsc::unbounded_channel;
use tracing::info;
use tracing::*;
use waitgroup::WaitGroup;

pub async fn fetch_hash(conn: &mut PgConnection, batch_size: usize) -> anyhow::Result<()> {
    let urls: HashSet<String> = marketplace
        .filter(hash.is_null())
        .select(assert_url)
        .load(conn)?
        .into_iter()
        .collect();
    info!("count: {}", urls.len());

    let sem = Arc::new(tokio::sync::Semaphore::new(batch_size));
    let wg = WaitGroup::new();
    let (tx, mut rx) = unbounded_channel::<(String, String)>();

    let db_task = async move {
        loop {
            let Some((url, file_hash)) = rx.recv().await else {
                error!("channel closed!");
                exit(-1);
            };

            if let Err(err) = diesel::update(marketplace)
                .filter(assert_url.eq(&url))
                .set(hash.eq(file_hash))
                .execute(conn)
            {
                error!(?err);
            }
        }
    };

    let w = wg.worker();

    let task2 = async move {
        for url in urls {
            let t = sem.clone().acquire_owned().await.unwrap();
            trace!("create task");
            let tx = tx.clone();
            let w = w.clone();
            tokio::spawn(async move {
                defer! {
                    drop(t);
                    drop(w);
                }

                if let Ok(file_hash) = compute_hash(&url).await {
                    debug!("compute hash: {file_hash} of {url:?}");
                    tx.send((url, file_hash)).unwrap();
                }
                nix_gc().await;
            });
        }
        drop(w);

        loop {
            tokio::task::yield_now().await;
        }
    };

    select! {
        _ = db_task => {}
        _ = task2 => {}
        _ = wg.wait() => {}
    }
    Ok(())
}

pub async fn compute_hash(url: &str) -> anyhow::Result<String> {
    let sha256 = tokio::process::Command::new("nix-prefetch-url")
        .arg(url)
        .output()
        .await?
        .stdout;

    Ok(String::from_utf8(sha256)?.trim().to_owned())
}

pub async fn nix_gc() {
    let _ = tokio::process::Command::new("nix")
        .arg("store")
        .arg("gc")
        .output()
        .await;
}
