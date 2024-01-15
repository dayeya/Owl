#[allow(unused_imports)]
use winsafe::prelude::*;

#[allow(unused_imports)]
use tui::{
    Frame,
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders, BorderType, Paragraph, Wrap},
    layout::{Layout, Constraint, Direction, Alignment, Margin},
    Terminal, text::{Spans, Span},
    style::{Style, Color}
};

#[allow(unused_imports)]
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

use std::io;

mod explorer;
use explorer::owl_explorer::Owl;

const OWL_BACKGROUND: Color = Color::Rgb(77, 94, 114);
const OWL_SECONDARY: Color = Color::Rgb(254, 250, 212);

#[allow(dead_code)]
enum State {
    Normal,
    OwlShell,
}

#[allow(dead_code)]
enum OwlOptions {
    Explore(String),    // Start exploring on cwd - :sc
    Quit(String),       // Quit Owl - :end
    DeleteFile(String), // Delete a file from current folder - :del
    CopyFile(String),   // Copy a specific file to clipboard - :cp 
    ShowFile(String)    // Show the file contents (based on supported formats) - :ben_dover
}

fn user_interface<B: Backend>(f: &mut Frame<B>, owl_explorer: &mut Owl) {
    let size = f.size();
    let text: Vec<Spans> = vec![
        Spans::from(Span::styled("Owl file explorer, press o", Style::default().fg(OWL_SECONDARY))),
    ];

    let layout = Layout::default()
                                .direction(Direction::Vertical)
                                .constraints([Constraint::Percentage(100)].as_ref())
                                .split(size);
    let home_style: Style = Style::default().fg(OWL_SECONDARY).bg(OWL_BACKGROUND);
    let home_block: Block<'_> = Block::default().borders(Borders::ALL);
    let home_page: Paragraph<'_> = Paragraph::new(text)
                                            .block(home_block)
                                            .style(home_style)
                                            .alignment(Alignment::Center)
                                            .wrap(Wrap { trim: true });

    if owl_explorer.inside_options {
        // TODO: Display the options table.
    }

    if owl_explorer.inside_shell {
        // TODO: Display the shell prompt.
    }

    f.render_widget(home_page, layout[0]);
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut owl_explorer: Owl = Owl::new();
    loop {
        terminal.draw(|f| user_interface(f, &mut owl_explorer))?;

        // Handle user keyboard.
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('o') => owl_explorer.inside_options = true,
                KeyCode::Char(':') => owl_explorer.inside_shell = true,
                _ => {}, 
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}