use anyhow::{anyhow, Context, Result};

use crate::commands;
use crate::decoder;
use crate::target::Repo;

pub fn download(mirror: Option<&str>, repo: Option<&Repo>, cache: &mut cache::Cache) -> Result<String> {
    if mirror.is_some() && repo.is_some() {
        return Err(anyhow!("Found both repo and mirror, use only one."));
    } else if mirror.is_none() && repo.is_none() {
        return Err(anyhow!("Missing repo and mirror, use at least one"));
    }

    let mut folder = String::new();

    if let Some(mirror) = mirror {
        return download_from_mirror(mirror);
    } else if let Some(repo) = repo {
        return download_from_git(&repo.url, repo.tag.as_deref());
    }

    std::unreachable("Download should have proceeded or failed");
}
