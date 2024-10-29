use anyhow::{anyhow, Context, Result};

use crate::commands;
use crate::target;

pub fn call_make(target: &target::Target) -> Result<()> {
    build(target).context(anyhow!("Failed to build {} with make", target.name))?;
    install(target).context(anyhow!("Failed to install {} with make", target.name))?;

    Ok(())
}

fn build(target: &target::Target) -> Result<()> {
    println!("Building {0}", target.name);

    if target.configure {
        commands::call_with("./configure", &target.configure_args)
            .context("Configure call failed")?;
    }

    if target.bootstrap {
        commands::call_with("./bootstrap", &target.bootstrap_args)
            .context("Bootstrap call failed")?;
    }

    commands::call_with("make", &target.make_args).context("Make call failed.")?;

    Ok(())
}

fn install(_target: &target::Target) -> Result<()> {
    commands::call_with("make", ["install"]).context("Make install failed")?;
    Ok(())
}
