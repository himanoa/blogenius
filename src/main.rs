use clap::Parser;

use crate::cli::{Cli, Commands};

pub mod article;
pub mod cli;
pub mod markdown;
pub mod renderer;
pub mod theme;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Build => {
            println!("exec build")
        }
    }
}
