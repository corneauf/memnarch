use std::env;

use crate::config;
use crate::utils::ensure_dir;

use anyhow::Context;
use anyhow::Result;

pub mod make;

macro_rules! run_tools {
    ( $config:expr, $( $tool:ident ),+ $(,)? ) => {
        (
            ::paste::paste!({
                $(
                    let tool_config = &$config. [<$tool>];
                    for target in &tool_config.targets {
                        build_target(target)?;
                    }
                )*
            })
        )
    }
}

fn build_target<T: Buildable>(target: &T) -> Result<()> {
    if !target.is_present()? {
        let temp_dir = ensure_dir(target.name())?;
        let path = temp_dir.path().join(target.name());
        let old_cwd = env::current_dir()?;

        env::set_current_dir(path)?;

        target.download().context("Failed to download source.")?;
        target.build().context("Failed to build tool.")?;

        target.install().context("Failed to install tool")?;

        env::set_current_dir(old_cwd)?;
    }

    Ok(())
}

pub trait Buildable {
    fn name(&self) -> &str;
    fn is_present(&self) -> Result<bool>;
    fn download(&self) -> Result<()>;
    fn build(&self) -> Result<()>;
    fn install(&self) -> Result<()>;
}

pub fn install_all(config: &config::Config) -> Result<()> {
    run_tools!(config, make);

    Ok(())
}
