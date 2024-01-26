mod ui;
mod app;
mod config;
mod internal;

use std::io;
use ratatui::prelude::*;
use crossterm::{
    execute,
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, 
};
use app::{
    App, 
    Mode, 
    CursorDirection
};

fn handle_events(explorer: &mut App) -> Result<bool, io::Error> {
    if let Event::Key(key) = event::read()? {
        match explorer.mode {
            Mode::Ended => { 
                return Ok(true)
            },
            Mode::Normal => match key.code {
                    KeyCode::Char(':') => explorer.mode = Mode::InsideShell,
                    KeyCode::Char('o') => explorer.mode = Mode::InsideOptions,
                    KeyCode::Char('f') => {},
                    KeyCode::Char('g') => {},
                    KeyCode::Char('j') => {}, // TODO: Display tree of selected child. [inner]
                    KeyCode::Char('h') => {}, // TODO: Return back into parent. [outer]
                    _ => {}, 
            },
            Mode::InsideShell => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Enter => explorer.execute_shell(),
                        KeyCode::Char(pressed) => explorer.append_to_shell(pressed),
                        KeyCode::Backspace => explorer.delete_from_shell(),
                        KeyCode::Right => explorer.move_cursor(CursorDirection::Right),
                        KeyCode::Left => explorer.move_cursor(CursorDirection::Left),
                        KeyCode::Esc => {
                            explorer.mode = Mode::Normal;
                            explorer.reset_shell();
                        },
                        _ => {},
                    }
                }
            },
            Mode::InsideOptions => match key.code {
                    KeyCode::Char(':') => explorer.mode = Mode::InsideShell,
                    KeyCode::Esc => explorer.mode = Mode::Normal,
                    _ => {}, 
                },
            }
        }
    Ok(false)
}

// TODO: Make options height fit the actual content.

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout: io::Stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

    let mut should_quit: bool = false;
    let mut explorer: App = App::new().unwrap(); // TODO: HANDLE DRIVELOADINGERROR.

    while !should_quit {
        terminal.draw(|f: &mut Frame<'_>| ui::user_interface(f, &mut explorer))?;
        should_quit = handle_events(&mut explorer).unwrap();
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    Ok(())
}
