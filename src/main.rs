mod cli;
mod error;
mod modules;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dns { hostname } => {
            modules::dns::lookup(&hostname).await?;
        }
    }

    Ok(())
}