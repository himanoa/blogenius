use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command[version, about, long_about = None]]
#[command(propagate_version = true)]
pub struct Cli {
    #[command[subcommand]]
    pub(crate) command: Commands,

    #[arg(short, long)]
    pub config: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Build,
}
