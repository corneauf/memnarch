use std::process::Command;
use std::str;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use crate::config::Expander;
use crate::decoder;
use crate::tools::Buildable;
use crate::{commands, utils};

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub targets: Vec<Target>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Target {
    pub name: String,
    pub version_command: String,
    pub version: String,
    pub mirror: Option<String>,
    pub repo: Option<String>,
    pub tag: Option<String>,

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
    fn download_from_mirror(&self, mirror: &str) -> Result<String> {
        let archive_name = mirror.split('/').next_back().unwrap();
        utils::download_binary_file(mirror, archive_name)?;

        decoder::decode(archive_name)
    }

    fn download_from_git(&self, repo: &str, tag: Option<&str>) -> Result<String> {
        let mut args = vec!["clone", "--depth", "1", repo];
        let archive_name = repo.split('/').next_back();

        if let Some(tag) = tag {
            args.push("--branch");
            args.push(tag);
        }

        commands::call_with("git", &args).context("Failed to clone git repo")?;
        if let Some(name) = archive_name.unwrap().strip_suffix(".git") {
            return Ok(name.to_string())
        }

        Ok(archive_name.unwrap().to_string())
    }
}

impl Buildable for Target {
    fn name(&self) -> &str {
        &self.name
    }

    fn expand_strings(&mut self, expander: &mut Expander) -> Result<()> {
        let expander = expander.and("version", &self.version);

        if self.mirror.is_some() {
            self.mirror = Some(
                expander
                    .expand(self.mirror.as_ref().unwrap())
                    .context("Failed to expand version inside mirror")?,
            );
        }

        for arg in &mut self.configure_args {
            let expanded_arg = expander.expand(arg)?;
            *arg = expanded_arg;
        }

        for arg in &mut self.make_args {
            let expanded_arg = expander.expand(arg)?;
            *arg = expanded_arg;
        }

        for arg in &mut self.bootstrap_args {
            let expanded_arg = expander.expand(arg)?;
            *arg = expanded_arg;
        }

        Ok(())
    }

    fn is_present(&self) -> Result<bool> {
        if let Ok(make_call) = Command::new(&self.name).arg(&self.version_command).output() {
            let from = str::from_utf8(&make_call.stdout)?.lines().next().unwrap();

            utils::is_same_version(&self.version, from)
        } else {
            Ok(false)
        }
    }

    fn download(&self) -> Result<String> {
        let mirror = &self.mirror;
        let repo = &self.repo;

        if mirror.is_some() && repo.is_some() {
            return Err(anyhow!("Found both repo and mirror, use only one."));
        } else if mirror.is_none() && repo.is_none() {
            return Err(anyhow!("Missing repo and mirror, use at least one"));
        }

        let mut folder = String::new();

        if let Some(mirror) = &self.mirror {
            folder = self.download_from_mirror(mirror)?;
        } else if let Some(repo) = &self.repo {
            folder = self.download_from_git(repo, self.tag.as_deref())?;
        }

        Ok(folder)
    }

    fn build(&self) -> Result<()> {
        println!("Building {0}", self.name);

        if self.configure {
            commands::call_with("./configure", &self.configure_args)
                .context("Configure call failed")?;
        }

        if self.bootstrap {
            commands::call_with("./bootstrap", &self.bootstrap_args)
                .context("Bootstrap call failed")?;
        }

        commands::call_with("make", &self.make_args).context("Make call failed.")?;

        Ok(())
    }

    fn install(&self) -> Result<()> {
        commands::call_with("make", ["install"]).context("Make install failed")?;
        Ok(())
    }
}
