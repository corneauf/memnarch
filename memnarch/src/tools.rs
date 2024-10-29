use crate::config;
use crate::context;
use crate::context::ContextProvider;
use crate::download;
use crate::env;
use crate::target;
use crate::utils::ensure_dir;

use anyhow::{anyhow, Context, Result};

pub mod make;

fn build_target(target: &mut target::Target, config: &env::Env) -> Result<()> {
    if !target.is_present()? {
        let expander = config::Expander::new().and("install_dir", &config.installation_folder);

        target.expand_strings(expander)?;

        let temp_dir = ensure_dir(&target.name)?;
        let path = temp_dir.path().join(&target.name);

        let _c = context::ChangeCwd::with(&path);

        let build_folder = download::download(target.mirror.as_deref(), target.repo.as_ref())
            .context("Failed to download source.")?;

        let _build = context::ChangeCwd::with(&path.join(&build_folder));

        call_tool(target)?;
    } else {
        println!("{} is already present, skipping.", target.name);
    }

    Ok(())
}

pub fn call_tool(target: &target::Target) -> Result<()> {
    match target.tool.as_str() {
        "make" => make::call_make(target),
        _ => Err(anyhow!("No tool found for {}", target.name)),
    }
}

pub fn install_all(config: &mut config::Config) -> Result<()> {
    let env = &mut config.memnarch;
    env.ensure_binary_folder()?;

    for target in &mut config.target {
        build_target(target, env)?;
    }
    Ok(())
}
