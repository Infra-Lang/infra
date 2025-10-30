use anyhow::Result;
use clap::Parser;
use infra_lsp::Server;
use log::info;
use std::io;
use tower_lsp::{LspService, Server};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let log_level = if args.verbose { log::LevelFilter::Debug } else { log::LevelFilter::Info };
    env_logger::Builder()
        .filter_level(log_level)
        .init();

    info!("Starting Infra Language Server");

    let (service, socket) = LspService::new(Server::new());
    Server::new(std::io::stdin(), std::io::stdout())
        .interleave(service)
        .serve(socket)
        .await?;

    Ok(())
}
