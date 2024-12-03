use crate::cache;
use crate::config;
use crate::context;
use crate::context::ContextProvider;
use crate::download;
use crate::env;
use crate::target;
use crate::utils::ensure_dir;

use anyhow::{anyhow, Context, Result};

pub mod make;

fn build_target(target: &mut target::Target, &mut cache: cache::Cache, out_folder: &str) -> Result<()> {
    if !target.is_present()? {
        let expander = config::Expander::new().and("out_folder", out_folder);

        target.expand_strings(expander)?;

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

pub fn install_tools(config: config::Config, cache: &mut cache::Cache) -> Result<()> {
    for target in &mut config.target {
        build_target(target, cache, config.env.out_folder)?;
    }
    Ok(())
}
