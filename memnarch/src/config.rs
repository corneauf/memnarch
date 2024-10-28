use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;
use strfmt::strfmt;

use crate::env;
use crate::tools::make;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub make: make::Config,
    #[serde(default)]
    pub memnarch: env::Config,
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

pub struct Expander {
    pub mapping: HashMap<String, String>,
}

impl Expander {
    pub fn new() -> Self {
        Expander {
            mapping: [
                ("os".to_string(), std::env::consts::OS.to_string()),
                ("arch".to_string(), std::env::consts::ARCH.to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    pub fn and(&mut self, key: &str, value: &str) -> &mut Self {
        self.mapping.insert(key.to_string(), value.to_string());

        self
    }

    pub fn expand(&self, from: &str) -> Result<String> {
        Ok(strfmt(from, &self.mapping)?)
    }
}
