mod export;
mod fetch_hash;
mod fetch_info;
mod models;
mod schema;
mod utils;

use std::env;

use clap::Parser;
use diesel::prelude::*;

use export::export_toml;
use fetch_info::fetch_marketplace;
use tracing::error;
use utils::init_logger;

#[derive(Debug, Parser)]
struct Args {
    /// Fetch extension info from vscode marketplace.
    #[clap(short, long, default_value_t = false)]
    fetch: bool,

    /// Update hash of extension.
    #[clap(long, default_value_t = false)]
    hash: bool,

    /// Batch size for coroutine pool
    #[clap(long, default_value_t = 4)]
    batch_size: usize,

    /// Export toml path
    #[clap(short, long)]
    output: Option<String>,
}

// #[dotenvy::load]
#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    init_logger();
    let args = Args::parse();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    if args.fetch {
        if let Err(err) = fetch_marketplace(&mut conn).await {
            error!(?err)
        }
    }

    if args.hash {
        if let Err(err) = fetch_hash::fetch_hash(&mut conn, args.batch_size).await {
            error!(?err)
        }
    }

    if let Some(target) = args.output {
        if let Err(err) = export_toml(&mut conn, &target).await {
            error!(?err);
        }
    }
}
