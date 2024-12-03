use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

use crate::env;
use crate::target;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub target: Vec<target::Target>,
    #[serde(default)]
    pub memnarch: env::Env,
}

impl Config {
    fn parse_from_file(config_path: &Path) -> Result<Config> {
        let content = fs::read_to_string(config_path)?;
        let config: Config = match toml::from_str(&content) {
            Ok(config) => config,
            Err(error) => {
                println!("Could not load config from file {error:?}");
                Config::default()
            }
        };

        Ok(config)
    }

    fn from_path(config_path: Option<&Path>) -> Result<Config> {
        if let Some(path) = config_path {
            Config::parse_from_file(path)
        } else {
            Ok(Config::default())
        }
    }

    fn from_env() -> Result<Config> {
        Config::from_path(std::env::var_os("MEMNITE_PATH").as_deref().map(Path::new))
    }

    pub fn get_config(config_path: Option<&Path>) -> Result<Config> {
        if let Some(path) = config_path {
            Config::from_path(Some(path))
        } else {
            Config::from_env()
        }
    }
}
