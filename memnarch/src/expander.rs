use std::collections::HashMap;

use anyhow::Result;
use strfmt::strfmt;

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

    pub fn and(mut self, key: &str, value: &str) -> Self {
        self.mapping.insert(key.to_string(), value.to_string());

        self
    }

    pub fn expand(&self, from: &str) -> Result<String> {
        Ok(strfmt(from, &self.mapping)?)
    }
}
