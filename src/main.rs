#[allow(unused_imports)]
use winsafe::prelude::*;

#[allow(unused_imports)]
use std::{
    io, 
    thread, 
    time::Duration
};

#[allow(unused_imports)]
use tui::{
    Frame, 
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders, BorderType},
    layout::{
        Layout, 
        Constraint, 
        Direction
    },
    Terminal, text::Span,
    style::{Style, Color}
};

#[allow(unused_imports)]
use crossterm::{
    event::{
        self, 
        DisableMouseCapture, 
        EnableMouseCapture, 
        Event, 
        KeyCode
    },
    execute,
    terminal::{
        disable_raw_mode, 
        enable_raw_mode, 
        EnterAlternateScreen, 
        LeaveAlternateScreen
    }
};

const OWL_BACKGROUND: Color = Color::Rgb(77, 94, 114);
const OWL_SECONDARY: Color = Color::Rgb(254, 250, 212);

fn user_interface<B: Backend>(f: &mut Frame<B>) {
    let title: Span = Span::styled("Owl file explorer", Style::default().fg(OWL_SECONDARY));
    let size = f.size();
    let block: Block<'_> = Block::default()
        .title(title)
        .style(Style::default().bg(OWL_BACKGROUND));
    f.render_widget(block, size);
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| user_interface(f))?;

        // Handle user keyboard.
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
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