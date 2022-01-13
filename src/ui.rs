use std::io::{self, Stdout};

use anyhow::{Context, Result};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use crate::state::State;

pub struct View {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl View {
    pub fn default() -> Result<Self> {
        enable_raw_mode().context("Failed to enter terminal's raw mode")?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).context("Failed to attach to terminal.")?;

        Ok(Self { terminal })
    }

    pub fn draw(&mut self, state: &State) -> Result<()> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                .split(f.size());

            let branch_items: Vec<ListItem> = if let Some(entries) = &state.entries {
                entries
                    .iter()
                    .enumerate()
                    .map(|(idx, entry)| {
                        let gutter = if idx == state.selected_idx {
                            "> "
                        } else {
                            "  "
                        };
                        ListItem::new(Spans::from(vec![
                            Span::styled(gutter, Style::default()),
                            Span::raw(entry),
                        ]))
                    })
                    .collect()
            } else {
                vec![]
            };

            let branches_block = Block::default().title("Branches").borders(Borders::ALL);
            let branches_list = List::new(branch_items)
                .start_corner(tui::layout::Corner::BottomLeft)
                .block(branches_block);

            let search_block = Block::default().title("Search").borders(Borders::ALL);
            let search_par = Paragraph::new(state.input.as_str()).block(search_block);

            f.render_widget(branches_list, chunks[0]);
            f.render_widget(search_par, chunks[1]);
        })?;

        Ok(())
    }
}

impl Drop for View {
    fn drop(&mut self) {
        disable_raw_mode()
            .context("Failed to disable terminal's raw mode!")
            .unwrap();

        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .context("Failed to detach terminal!")
        .unwrap();
    }
}
