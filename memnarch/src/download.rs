use anyhow::{anyhow, Context, Result};

use crate::commands;
use crate::decoder;
use crate::target::Repo;
use crate::utils;

pub fn download_from_mirror(mirror: &str) -> Result<String> {
    let archive_name = mirror.split('/').next_back().unwrap();
    utils::download_binary_file(mirror, archive_name)?;

    decoder::decode(archive_name)
}

pub fn download_from_git(repo: &str, tag: Option<&str>) -> Result<String> {
    let mut args = vec!["clone", "--depth", "1", repo];
    let archive_name = repo.split('/').next_back();

    if let Some(tag) = tag {
        args.push("--branch");
        args.push(tag);
    }

    commands::call_with("git", &args).context("Failed to clone git repo")?;
    if let Some(name) = archive_name.unwrap().strip_suffix(".git") {
        return Ok(name.to_string());
    }

    Ok(archive_name.unwrap().to_string())
}

pub fn download(mirror: Option<&str>, repo: Option<&Repo>) -> Result<String> {
    if mirror.is_some() && repo.is_some() {
        return Err(anyhow!("Found both repo and mirror, use only one."));
    } else if mirror.is_none() && repo.is_none() {
        return Err(anyhow!("Missing repo and mirror, use at least one"));
    }

    let mut folder = String::new();

    if let Some(mirror) = mirror {
        folder = download_from_mirror(mirror)?;
    } else if let Some(repo) = repo {
        folder = download_from_git(&repo.url, repo.tag.as_deref())?;
    }

    Ok(folder)
}
