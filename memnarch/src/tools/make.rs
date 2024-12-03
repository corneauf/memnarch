use anyhow::{anyhow, Context, Result};

use sideboard::commands;
use sideboard::closure;
use crate::target;

pub fn call_make(target: &target::Target) -> Result<()> {
    build(target).context(anyhow!("Failed to build {} with make", target.name))?;
    install(target).context(anyhow!("Failed to install {} with make", target.name))?;

    Ok(())
}

fn build(target: &target::Target) -> Result<()> {
    println!("Building {0}", target.name);

    if target.configure {
        commands::stream_with("./configure", &target.configure_args, closure::println)
            .context("Configure call failed")?;
    }

    if target.bootstrap {
        commands::call_with("./bootstrap", &target.bootstrap_args, closure::println)
            .context("Bootstrap call failed")?;
    }

    commands::stream_with("make", &target.make_args, closure::println).context("Make call failed.")?;

    Ok(())
}

fn install(_target: &target::Target) -> Result<()> {
    commands::stream_with("make", ["install"], closure::println).context("Make install failed")?;
    Ok(())
}
