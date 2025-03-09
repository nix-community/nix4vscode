use std::{env, pin::pin, str::FromStr};

use code_api::code::HttpClient;
use diesel::prelude::*;
use futures::StreamExt;
use models::Marketplace;

mod models;
mod schema;

use schema::marketplace;
use serde_json::Value;
use tracing::{debug, error};

// #[dotenvy::load]
#[tokio::main]
async fn main() {
    let _ = read_env().await;
    init_logger();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let client = HttpClient::new().unwrap();
    let mut iter = pin!(client.get_extension_response(vec![]));
    while let Some(item) = iter.next().await {
        let Ok(item) = item else {
            continue;
        };
        for item in item.extensions {
            for version in item.versions {
                let Ok(engne) = version.get_engine() else {
                    continue;
                };
                let Some(visix) = version.get_file(code_api::code::AssetType::Vsix) else {
                    continue;
                };
                let Some(platform) = version.target_platform.clone() else {
                    continue;
                };
                let x = Marketplace {
                    name: item.extension_name.clone(),
                    publisher: item.publisher.publisher_name.clone(),
                    version: version.version.clone(),
                    engine: engne,
                    platform,
                    assert_url: visix.source.clone(),
                    hash: None,
                };

                debug!("insert value");
                if let Err(err) = diesel::insert_into(marketplace::table)
                    .values(&x)
                    .returning(Marketplace::as_returning())
                    .get_result(&mut conn)
                {
                    error!(?err);
                }
            }
        }
    }
}

fn init_logger() {
    use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};

    let log_level = std::env::var("RUST_LOG")
        .unwrap_or("INFO".into())
        .to_lowercase();

    let env_filter = EnvFilter::builder()
        .parse(format!("RUST_LOG=OFF,exporter={}", log_level))
        .unwrap();

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_writer(std::io::stderr),
        )
        .with(env_filter)
        .init();
}

async fn read_env() -> anyhow::Result<()> {
    let content = tokio::fs::read_to_string(".env.json").await?;
    let value = serde_json::Value::from_str(&content)?;

    if let Value::Object(v) = value {
        for (k, v) in v {
            let v = match v {
                Value::String(v) => v,
                _ => v.to_string(),
            };
            if env::var(&k).is_ok() {
                continue;
            }
            unsafe { env::set_var(k, v) };
        }
    }

    Ok(())
}
