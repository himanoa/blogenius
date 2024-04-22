use std::process;

use clap::Parser;
use command::build::build;
use config::Config;
use kernel::Kernel;

use crate::cli::{Cli, Commands};

pub mod article;
pub mod cli;
pub mod command;
pub mod config;
pub mod distributor;
pub mod markdown;
pub mod renderer;
pub mod theme;
pub mod kernel;

fn main() {
    let cli = Cli::parse();
    let config = cli
        .config
        .and_then(|p| Config::load(p).ok())
        .unwrap_or(Config::default());
    let kernel = Kernel::new(config.clone());

    match &cli.command {
        Commands::Build => {
            build(&kernel, config).map_err(|e| {
                eprintln!("{}", e);
                process::exit(1)
            }).unwrap();
        }
    }
}
