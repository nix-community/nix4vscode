use std::sync::Arc;

use crate::models::*;
use crate::schema::marketplace::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
use scopeguard::defer;
use tokio::select;
use tokio::sync::mpsc::unbounded_channel;
use tracing::debug;
use tracing::error;
use waitgroup::WaitGroup;

pub async fn fetch_hash(conn: &mut PgConnection, batch_size: usize) -> anyhow::Result<()> {
    let record = marketplace
        .filter(hash.is_null())
        .select(Marketplace::as_select())
        .load(conn)?;

    let sem = Arc::new(tokio::sync::Semaphore::new(batch_size));
    let wg = WaitGroup::new();
    let (tx, mut rx) = unbounded_channel::<Marketplace>();

    let db_task = async move {
        loop {
            let Some(record) = rx.recv().await else {
                break;
            };

            let Some(file_hash) = record.hash else {
                error!("unreachable");
                continue;
            };
            let _ = diesel::update(marketplace)
                .filter(name.eq(&record.name))
                .filter(publisher.eq(&record.publisher))
                .filter(version.eq(&record.version))
                .filter(engine.eq(&record.engine))
                .filter(platform.eq(&record.platform))
                .filter(assert_url.eq(&record.assert_url))
                .set(hash.eq(file_hash))
                .execute(conn);
        }
    };

    let w = wg.worker();

    let task2 = async move {
        for mut record in record {
            let t = sem.clone().acquire_owned().await.unwrap();
            debug!("create task");
            let tx = tx.clone();
            let w = w.clone();
            tokio::spawn(async move {
                defer! {
                    drop(t);
                    drop(w);
                }

                if let Ok(file_hash) = compute_hash(&record.assert_url).await {
                    debug!("compute hash: {file_hash} of {record:?}");
                    record.hash = Some(file_hash);
                    tx.send(record).unwrap();
                }
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
