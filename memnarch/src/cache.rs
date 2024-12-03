use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::target::Target;

pub struct Cache {
    cache_path: PathBuf,
    cache: HashSet<PathBuf>,
}

fn get_cache_dir() -> PathBuf {
    let mut path = PathBuf::from(
        env::var_os("XDG_CACHE_HOME")
            .or_else(|| format!("{}/.cache", env::var_os("HOME").unwrap())),
    );
    path.push("memnite");

    Ok(path)
}

fn fill_cache(cache_path: &Path, cache: &mut HashSet<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(cache_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            cache.insert(path);
        }
    }

    Ok(())
}

impl Cache {
    pub fn new() -> Result<Self> {
        let mut path = get_cache_dir();

        let _ = std::fs::create_dir_all(&path);

        let mut cache = HashSet::new();

        fill_cache(&path, &mut cache)?;

        Ok(Cache {
            cache_path: path,
            cache: cache,
        })
    }

    pub fn cache(&self, target: &Target) -> Result<&Path> {
        let directory_name = format!("{}_{}", &target.name, &target.version);
        let path = self.cache_path.as_path().join(directory_name);

        if !self.cache.contains(&path) {
            fs::create_dir(&path)?
        }

        Ok(&self.cache.get(&path).unwrap().as_path())
    }
}
