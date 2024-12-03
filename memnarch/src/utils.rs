use std::fs::create_dir;
use std::io::Cursor;

use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;
use tempfile::{tempdir_in, TempDir};

const VERSION_REGEX: &str = "[0-9]\\.[0-9]*(.[0-9])?";


pub fn is_same_version(version: &str, from: &str) -> Result<bool> {
    let re = Regex::new(VERSION_REGEX)?;
    let captures = re
        .captures(from)
        .ok_or(anyhow!("Could not find version inside output {from}"))?;

    Ok(captures[0] == *version)
}
