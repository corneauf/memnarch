use std::fs;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Env {
    pub out_folder: String,
}

impl Default for Env {
    fn default() -> Self {
        Env {
            out_folder: "$HOME/.memnarch/".to_owned(),
        }
    }
}

impl Env {
    pub fn ensure_out_folder(&mut self) -> Result<()> {
        let path = self.out_folder.to_string();
        self.out_folder = String::from(
            shellexpand::full(&path)
                .context("Failed to expand shell variable when creating bin folder")?,
        );

        let path = std::path::Path::new(&self.out_folder);

        if !path.is_absolute() {
            return Err(anyhow!(
                "Installation directory must be an absolute path, got {}",
                path.display()
            ));
        }

        if path.exists() {
            return Ok(());
        }

        println!("Creating {} folder", self.out_folder);

        fs::create_dir_all(&path).context("Failed to create bin folder")?;

        Ok(())
    }
}
