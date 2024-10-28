use crate::config;
use crate::context;
use crate::context::ContextProvider;
use crate::env;
use crate::utils::ensure_dir;

use anyhow::Context;
use anyhow::Result;

pub mod make;

macro_rules! run_tools {
    ( $config:expr, $( $tool:ident ),+ $(,)? ) => {
        (
            ::paste::paste!({
                $(
                    let tool_config = &mut $config. [<$tool>];
                    for target in &mut tool_config.targets {
                        build_target(target, &$config.memnarch)?;
                    }
                )*
            })
        )
    }
}

fn build_target<T: Buildable>(target: &mut T, config: &env::Config) -> Result<()> {
    if !target.is_present()? {
        let mut expander = config::Expander::new();
        expander.and("install_dir", &config.installation_folder);

        target.expand_strings(&mut expander)?;

        let temp_dir = ensure_dir(target.name())?;
        let path = temp_dir.path().join(target.name());

        let _c = context::ChangeCwd::with(&path);

        let build_folder = target.download().context("Failed to download source.")?;

        let _build = context::ChangeCwd::with(&path.join(&build_folder));
        target.build().context("Failed to build tool.")?;

        target.install().context("Failed to install tool")?;
    } else {
        println!("{} is already present, skipping.", target.name());
    }

    Ok(())
}

pub trait Buildable {
    fn expand_strings(&mut self, expander: &mut config::Expander) -> Result<()>;
    fn name(&self) -> &str;
    fn is_present(&self) -> Result<bool>;
    fn download(&self) -> Result<String>;
    fn build(&self) -> Result<()>;
    fn install(&self) -> Result<()>;
}

pub fn install_all(config: &mut config::Config) -> Result<()> {
    let env = &mut config.memnarch;
    env.ensure_binary_folder()?;

    run_tools!(config, make);

    Ok(())
}
