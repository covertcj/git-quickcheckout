use anyhow::{Context, Result};
use clap::Parser;
use git2::{BranchType, Repository};
use std::env;

/// A tool for helping check out git branches using fuzzy search
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// If provided, remote branches will be searched instead
    #[clap(short, long)]
    remote: bool,

    /// The default search string to use
    query: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cwd = env::current_dir().context("Couldn't determine current working directory. Do you have permissions, or was the directory deleted?")?;
    let repo =
        Repository::open(cwd).context("Couldn't find a git repo in the current directory.")?;

    let branch_type = if cli.remote {
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
                .context("Failed to parse a branch name into a UTF8 string")
        })
        .collect::<Result<Vec<_>, _>>()?;

    println!("{:?}", branch_names);

    // let branch_errors  = branches_result.filter_map(|res| res.err());

    // // TODO: find a better way to handle these seemingly unlikely errors
    // branch_errors.for_each(|err| println!("There was an error processing a branch: {:?}", err));

    // branches_result.for_each(|result| match result {
    //     Ok((branch, _)) => println!("{:?}", branch.name()),
    //     Err(err) => println!("ERROR: {:?}", err),
    // });

    Ok(())
}
