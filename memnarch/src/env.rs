use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub installation_folder: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            installation_folder: "~/.memnarch/".to_owned(),
        }
    }
}

impl Config {
    pub fn ensure_binary_folder(&mut self) -> Result<()> {
        let path = self.installation_folder.to_string();
        self.installation_folder = String::from(
            shellexpand::full(&path)
                .context("Failed to expand shell variable when creating bin folder")?,
        );

        if std::path::Path::new(&self.installation_folder).exists() {
            return Ok(());
        }

        println!("Creating {} folder", self.installation_folder);

        fs::DirBuilder::new()
            .recursive(true)
            .create(&self.installation_folder)
            .context("Failed to create bin folder")?;

        Ok(())
    }
}
