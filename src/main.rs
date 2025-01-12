pub mod code;
pub mod config;
mod db;
pub mod error;
pub mod jinja;
pub mod utils;

use std::collections::HashMap;

use code::CodeNix;
use itertools::Itertools;
use jinja::NixContext;
use tokio::fs;
use tracing::*;

use clap::Parser;
use config::Config;

use crate::jinja::{Generator, GeneratorContext};

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    file: String,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(long, hide = true)]
    export: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();
    let args = Args::parse();

    let config = Config::from_file(&args.file).await?;
    debug!(?config);

    let mut generator = Generator::new();
    let mut code = CodeNix::new(config.clone());

    let ctx = code.get_extensions(generator.clone()).await;
    let mut ctx2 = HashMap::<String, NixContext>::new();
    for item in ctx {
        ctx2.insert(
            format!(
                "{}-{}-{:?}",
                item.publisher_name, item.extension_name, item.target_platform
            ),
            item,
        );
    }
    let ctx = ctx2.into_values().collect_vec();
    debug!("{ctx:#?}");

    if args.export {
        let res = serde_json::to_string_pretty(&ctx)?;
        match args.output {
            Some(filepath) => fs::write(filepath, res).await?,
            None => println!("{res}",),
        }
        return Ok(());
    }

    let res = generator.render(&GeneratorContext {
        extensions: ctx,
        config: config.clone().into(),
    })?;

    match args.output {
        Some(filepath) => tokio::fs::write(filepath, res).await.unwrap(),
        None => println!("{res}",),
    }

    Ok(())
}

fn init_logger() {
    use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};

    let log_level = std::env::var("RUST_LOG")
        .unwrap_or("INFO".into())
        .to_lowercase();

    let env_filter = EnvFilter::builder()
        .parse(format!("RUST_LOG=OFF,nix4vscode={}", log_level))
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
