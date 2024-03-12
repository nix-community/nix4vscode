pub mod code;
pub mod config;
pub mod error;
pub mod jinja;
pub mod openvsx_ext;
pub mod utils;

use tracing::*;
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};

use clap::Parser;
use config::Config;

use crate::{
    code::CodeNix,
    jinja::{Generator, GeneratorContext},
};

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    file: String,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(long, hide = true)]
    export: bool,
    #[arg(long, hide = true)]
    dump: bool,
    #[arg(long)]
    openvsx: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    init_logger();

    let config = Config::new(tokio::fs::read_to_string(&args.file).await?.as_str())?;
    debug!("request: {config:?}");
    let mut generator = Generator::new();
    let mut code = CodeNix::new(config.clone());

    let res = code.get_extensions(generator.clone()).await;

    debug!("{res:#?}");
    if args.export {
        let res = serde_json::to_string(&res).unwrap();
        match args.output {
            Some(filepath) => tokio::fs::write(filepath, res).await.unwrap(),
            None => println!("{res}",),
        }
        return Ok(());
    }

    let res = generator.render(&GeneratorContext {
        extensions: res,
        config: config.clone().into(),
    })?;

    match args.output {
        Some(filepath) => tokio::fs::write(filepath, res).await.unwrap(),
        None => println!("{res}",),
    }

    Ok(())
}

fn init_logger() {
    let log_level = std::env::var("RUST_LOG")
        .unwrap_or("WARN".into())
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
