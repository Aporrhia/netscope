use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "netscope",
    version = "1.0",
    author = "Aporrhia",
    about = "A network scope tool"
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Dns {
        hostname: String,
    },
}
