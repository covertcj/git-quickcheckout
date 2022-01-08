use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::{BranchType, Repository};
use std::{
    env,
    io::{self},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

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

struct App {
    input: String,
    pub entries: Option<Vec<String>>,
}

impl App {
    fn default() -> Result<Self> {
        Ok(Self {
            input: "".to_string(),
            entries: None,
        })
    }
}

fn load_branches(repo: &Repository, remotes: bool) -> Result<Vec<String>> {
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to attach to terminal.")?;

    let mut app = App::default().context("Failed to initialize TUI app.")?;

    let cwd = env::current_dir().context("Couldn't determine current working directory. Do you have permissions, or was the directory deleted?")?;
    let repo =
        Repository::open(cwd).context("Couldn't find a git repo in the current directory.")?;

    app.entries = Some(load_branches(&repo, cli.remote)?);

    loop {
        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (KeyCode::Esc, _) => break,
                (KeyCode::Char('c'), KeyModifiers::CONTROL)
                | (KeyCode::Char('d'), KeyModifiers::CONTROL) => break,
                (KeyCode::Char(c), _) => {
                    app.input.push(c);
                }
                (KeyCode::Backspace, _) => {
                    app.input.pop();
                }
                _ => {
                    if let Some(entries) = &mut app.entries {
                        entries.push(format!("{:?}", key))
                    }
                }
            }
        };

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                .split(f.size());

            let branch_items: Vec<ListItem> = if let Some(entries) = &app.entries {
                entries
                    .iter()
                    .map(|entry| ListItem::new(Spans::from(Span::styled(entry, Style::default()))))
                    .collect()
            } else {
                vec![]
            };

            let branches_block = Block::default().title("Branches").borders(Borders::ALL);
            let branches_list = List::new(branch_items).block(branches_block);

            let search_block = Block::default().title("Search").borders(Borders::ALL);
            let search_par = Paragraph::new(app.input.as_str()).block(search_block);

            f.render_widget(branches_list, chunks[0]);
            f.render_widget(search_par, chunks[1]);
        })?;
    }

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}
