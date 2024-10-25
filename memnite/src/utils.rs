use std::collections::HashMap;
use std::env;
use std::fs::create_dir;
use std::io::Cursor;

use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;
use strfmt::strfmt;
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

pub fn format_mirror(mirror: &str, version: &str) -> Result<String> {
    let mut vars = HashMap::new();

    vars.insert("os".to_string(), env::consts::OS.to_string());
    vars.insert("arch".to_string(), env::consts::ARCH.to_string());
    vars.insert("version".to_string(), version.to_string());

    Ok(strfmt(mirror, &vars)?)
}

pub fn rfind_nth(from: &str, to_find: char, n: usize) -> Option<usize> {
    let mut count = 0;
    
    for (i, c) in from.chars().rev().enumerate() {
        if c == to_find {
            count += 1;
        }

        if count == n {
            return Some(i);
        }
    }

    None
}
