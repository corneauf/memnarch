use std::{env, fs, str};
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use flate2::read::GzDecoder;
use serde::Deserialize;
use tar::Archive;

use crate::{commands, utils, context};
use crate::tools::Buildable;
use crate::context::ContextProvider;

const ARCHIVE_NAME: &str = "archive.tar.gz";

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

    #[serde(default)]
    pub make_env_vars: Vec<(String, String)>,

    #[serde(default)]
    pub configure: bool,

    #[serde(default)]
    pub sudo_install: bool,
}

impl Target {
    fn download_from_mirror(&self, mirror: &str) -> Result<()> {
        let mirror = utils::format_mirror(mirror, &self.version)?;
        utils::download_binary_file(&mirror, ARCHIVE_NAME)?;

        let file = fs::File::open(ARCHIVE_NAME)?;

        let gz = GzDecoder::new(file);
        let mut archive = Archive::new(gz);
        archive.unpack(".")?;

        let archive_name = mirror.split('/').next_back().unwrap();

        let directory_name = archive_name.strip_suffix(".tar.gz").unwrap();

        fs::rename(directory_name, "archive").context("Failed to rename directory")?;

        Ok(())
    }
}

impl Buildable for Target {
    fn name(&self) -> &str {
        &self.name
    }

    fn is_present(&self) -> Result<bool> {
        if let Ok(make_call) = Command::new(&self.name).arg(&self.version_command).output() {
            let from = str::from_utf8(&make_call.stdout)?
                .lines()
                .next()
                .unwrap();

            utils::is_same_version(&self.version, from)
        } else {
            Ok(false)
        }
    }

    fn download(&self) -> Result<()> {
        let mirror = &self.mirror;
        let repo = &self.repo;

        if mirror.is_some() && repo.is_some() {
            return Err(anyhow!("Found both repo and mirror, use only one."));
        } else if mirror.is_none() && repo.is_none() {
            return Err(anyhow!("Missing repo and mirror, use at least one"));
        }

        if let Some(mirror) = &self.mirror {
            self.download_from_mirror(mirror)?;
        } else if let Some(_repo) = &self.repo {
        }

        Ok(())
    }

    fn build(&self) -> Result<()> {
        println!("Building {0}", self.name);

        let path = env::current_dir().unwrap().join("archive");

        let _c = context::ChangeCwd::with(&path);

        if self.configure {
            commands::call("./configure").context("Configure call failed")?;
        }

        commands::call("make").context("Make call failed.")?;

        commands::call_with("make", ["install"]).context("Make install failed")?;

        Ok(())
    }

    fn install(&self) -> Result<()> {
        println!("Installing {0}", self.name);
        Ok(())
    }
}
