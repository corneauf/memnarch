use std::env;
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

use crate::tools::make;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub make: make::Config,
}

impl Config {
    fn parse_from_file(config_path: &Path) -> Result<Config> {
        let content = fs::read_to_string(config_path)?;
        toml::from_str(&content).or(Ok(Config::default()))
    }

    fn from_path(config_path: Option<&Path>) -> Result<Config> {
        if let Some(path) = config_path {
            Config::parse_from_file(path)
        } else {
            Ok(Config::default())
        }
    }

    fn from_env() -> Result<Config> {
        Config::from_path(env::var_os("MEMNITE_PATH").as_deref().map(Path::new))
    }

    pub fn get_config(config_path: Option<&Path>) -> Result<Config> {
        if let Some(path) = config_path {
            Config::from_path(Some(path))
        } else {
            Config::from_env()
        }
    }
}
