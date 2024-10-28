use std::fs::create_dir;
use std::io::Cursor;

use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;
use tempfile::{tempdir_in, TempDir};

const VERSION_REGEX: &str = "[0-9]\\.[0-9]*(.[0-9])?";

pub fn download_binary_file(url: &str, filename: &str) -> Result<()> {
    println!("Downloading {url}");
    let response = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(filename)?;
    let mut content = Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
}

pub fn ensure_dir(name: &str) -> Result<TempDir> {
    let path = home::home_dir().unwrap();
    let temp_dir = tempdir_in(path)?;

    create_dir(temp_dir.path().join(name))?;

    Ok(temp_dir)
}

pub fn is_same_version(version: &str, from: &str) -> Result<bool> {
    let re = Regex::new(VERSION_REGEX)?;
    let captures = re
        .captures(from)
        .ok_or(anyhow!("Could not find version inside output {from}"))?;

    Ok(captures[0] == *version)
}
