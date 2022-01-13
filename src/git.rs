use std::path::PathBuf;

use anyhow::{Context, Result};
use git2::{BranchType, Repository};

pub fn load_branches(directory: PathBuf, remotes: bool) -> Result<Vec<String>> {
    let repo = Repository::open(directory)
        .context("Couldn't find a git repo in the current directory.")?;

    let branch_type = if remotes {
        BranchType::Remote
    } else {
        BranchType::Local
    };

    let branches = repo
        .branches(Some(branch_type))
        .context("Error listing branches in the git repo")?
        .collect::<Result<Vec<_>, _>>()
        .context("Processing one of the repo's branches caused an error")?;

    let branch_names = branches
        .iter()
        .map(|(branch, _)| {
            branch
                .name()
                .unwrap_or(None)
                .map(|s| s.to_string())
                .context("Failed to parse a branch name into a UTF8 string")
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(branch_names)
}
