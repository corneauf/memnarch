mod args;
mod cache;
mod commands;
mod config;
mod context;
mod decoder;
mod download;
mod env;
mod target;
mod tools;
mod utils;

use crate::args::{Cli, Commands};
use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut config = config::Config::get_config(cli.config.as_deref())?;
    config.memnarch.ensure_output_folder()?;

    let mut cache = cache::Cache::new();

    match &cli.command {
        Some(Commands::Install) => tools::install_tools(&mut config, &mut cache)
            .with_context(|| "Failed to install tools.")?,
        _ => (),
    }

    Ok(())
}
