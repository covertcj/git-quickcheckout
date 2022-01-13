use anyhow::{Context, Result};
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use git::load_branches;

use state::{process_action, Action, State};
use std::env;

use ui::View;

mod git;
mod state;
mod ui;

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

    let mut view = View::default().context("Failed to create UI View")?;
    let mut state = State::default();

    let cwd = env::current_dir().context("Couldn't determine current working directory. Do you have permissions, or was the directory deleted?")?;
    let entries = load_branches(cwd, cli.remote)?;
    process_action(&mut state, Action::EntriesLoaded(entries));

    loop {
        view.draw(&state).context("Failed to draw frame.")?;

        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (KeyCode::Esc, _) => break,
                (KeyCode::Char('c'), KeyModifiers::CONTROL)
                | (KeyCode::Char('d'), KeyModifiers::CONTROL) => break,
                (KeyCode::Char(c), _) => {
                    // TODO: replace with an action
                    state.input.push(c);
                }
                (KeyCode::Backspace, _) => {
                    state.input.pop();
                }
                (KeyCode::Up, _) => process_action(&mut state, Action::SelectedIndexIncreased),
                (KeyCode::Down, _) => process_action(&mut state, Action::SelectedIndexDecreased),
                (KeyCode::Enter, _) => {}
                _ => {}
            }
        };
    }

    Ok(())
}
