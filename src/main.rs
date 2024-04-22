use clap::Parser;
use config::Config;

use crate::cli::{Cli, Commands};

pub mod article;
pub mod cli;
pub mod command;
pub mod config;
pub mod distributor;
pub mod markdown;
pub mod renderer;
pub mod theme;

fn main() {
    let cli = Cli::parse();
    let config = cli
        .config
        .and_then(|p| Config::load(p).ok())
        .unwrap_or(Config::default());

    match &cli.command {
        Commands::Build => {
            println!("{:?}", config);
            println!("exec build")
        }
    }
}
