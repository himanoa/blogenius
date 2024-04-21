use clap::Parser;

use crate::cli::{Cli, Commands};

pub mod article;
pub mod markdown;
pub mod renderer;
pub mod theme;
pub mod cli;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Build => {
            println!("exec build")
        }
    }
}
