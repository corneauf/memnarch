use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::str;

use anyhow::anyhow;
use anyhow::Result;
use flate2::read::GzDecoder;
use serde::Deserialize;
use tar::Archive;

use crate::tools::Buildable;
use crate::utils;

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
        Ok(())
    }

    fn configure(&self) -> Result<()> {
        let mut command = Command::new("./configure").stdout(Stdio::piped()).spawn()?;

        {
            let stdout = command.stdout.take().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            for line in stdout_lines {
                println!("{}", line?);
            }
        }

        command.wait().unwrap();

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
                .split('\n')
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

        let old_cwd = env::current_dir()?;
        let mut directory_name = self.name.to_owned();
        directory_name.push('-');
        directory_name.push_str(&self.version);
        let path = old_cwd.join(&directory_name);

        env::set_current_dir(path)?;

        if self.configure {
            self.configure()?;
        }

        env::set_current_dir(old_cwd)?;
        Ok(())
    }

    fn install(&self) -> Result<()> {
        println!("Installing {0}", self.name);
        Ok(())
    }
}
