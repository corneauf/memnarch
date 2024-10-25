mod args;
mod config;
mod tools;
mod utils;

use crate::args::{Cli, Commands};
use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = config::Config::get_config(cli.config.as_deref())?;

    match &cli.command {
        Some(Commands::Install) => {
            tools::install_all(&config).with_context(|| "Failed to install tools.")
        }
        None => Ok(()),
    }
}
