use std::process::Command;
use std::str;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::config::Expander;
use crate::utils;

#[derive(Debug, Deserialize, Default)]
pub struct Repo {
    pub url: String,
    pub tag: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Target {
    pub name: String,
    pub version_command: String,
    pub version: String,

    pub mirror: Option<String>,
    pub repo: Option<Repo>,
    pub tool: String,

    #[serde(default)]
    pub make_args: Vec<String>,

    #[serde(default)]
    pub configure: bool,

    #[serde(default)]
    pub configure_args: Vec<String>,

    #[serde(default)]
    pub bootstrap: bool,

    #[serde(default)]
    pub bootstrap_args: Vec<String>,
}

impl Target {
    fn expand_vec(expander: &Expander, container: &mut [String]) -> Result<()> {
        for i in 0..container.len() {
            let expanded_arg = expander.expand(&container[i])?;
            container[i] = expanded_arg;
        }

        Ok(())
    }

    pub fn expand_strings(&mut self, expander: Expander) -> Result<()> {
        let expander = expander.and("version", &self.version);

        if self.mirror.is_some() {
            self.mirror = Some(
                expander
                    .expand(self.mirror.as_ref().unwrap())
                    .context("Failed to expand version inside mirror")?,
            );
        }

        Target::expand_vec(&expander, &mut self.configure_args)?;
        Target::expand_vec(&expander, &mut self.make_args)?;
        Target::expand_vec(&expander, &mut self.bootstrap_args)?;

        Ok(())
    }

    pub fn is_present(&self) -> Result<bool> {
        if let Ok(call) = Command::new(&self.name).arg(&self.version_command).output() {
            let from = str::from_utf8(&call.stdout)?.lines().next().unwrap();

            utils::is_same_version(&self.version, from)
        } else {
            Ok(false)
        }
    }
}
