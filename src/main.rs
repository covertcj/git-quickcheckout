use clap::{Parser};

/// A tool for helping check out git branches using fuzzy search
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// If provided, remote branches will be searched instead
    #[clap(short, long)]
    remote: bool,

    /// The default search string to use
    query: Option<String>
}

fn main() {
    let cli = Cli::parse();

    if cli.remote {
        println!("Remote!");
    }

    if let Some(query) = cli.query {
        println!("{}", query);
    }

    println!("That's all!");
}
