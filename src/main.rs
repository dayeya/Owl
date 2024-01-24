mod ui;
mod explorer;
mod internal;

use std::io;
use std::rc::Rc;
use ratatui::{prelude::*, widgets::*, layout::Layout};
use crossterm::{
    execute,
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, 
};

use explorer::{App, OwlState, CursorDirection};
use crate::ui::content::Content;
use crate::ui::mode_bar::ModeBar;
use crate::ui::shell::Shell;
use crate::ui::options::Options;

fn user_interface(f: &mut Frame, explorer: &mut App) {
    let size: Rect = f.size();
    let layout: Rc<[Rect]> = Layout::new(
        Direction::Vertical, [
            Constraint::Max(98),    // Normal block
            Constraint::Length(1),  // State block
            Constraint::Length(1)   // Shell block
        ], ).split(size);

    let second_layout: Rc<[Rect]> = Layout::new(
        Direction::Horizontal, [
            Constraint::Percentage(50), // Tree pane
            Constraint::Percentage(50), // Preview pane
        ], ).split(layout[0]);

    let preview: Content = Content::with_text(Some("".to_string()), Borders::ALL);
    let mode_bar: ModeBar = ModeBar::with_text(explorer.format_mode());
    let shell_input: String = (&explorer.shell.input).to_owned();
    let shell: Shell = Shell::with_text(shell_input);
    let options: Options<'_> = Options::with_items(40, 27, layout[0], String::from("Options"));
    let ui_tree: Table<'_> = explorer.tree.render();

    f.render_stateful_widget(ui_tree, second_layout[0], &mut explorer.tree.state);
    f.render_widget(preview.inner, second_layout[1]);
    f.render_widget(mode_bar.inner, layout[1]);
    f.render_widget(shell.inner, layout[2]);

    match explorer.state {
        OwlState::OwlOptions => {
            f.render_widget(Clear, options.rect);
            f.render_widget(options.inner, options.rect);
        },
        _ => {},
    }

}

fn handle_events(explorer: &mut App) -> Result<bool, io::Error> {
    if let Event::Key(key) = event::read()? {
        match explorer.state {
            OwlState::Ended => { 
                return Ok(true) 
            },
            OwlState::Normal => match key.code {
                    KeyCode::Char(':') => explorer.state = OwlState::OwlShell,
                    KeyCode::Char('o') => explorer.state = OwlState::OwlOptions, 
                    KeyCode::Char('j') => explorer.tree.move_previous(),
                    KeyCode::Char('k') => explorer.tree.move_next(),
                    KeyCode::Char('f') => {}, // TODO: Move into previous cwd [inner].
                    KeyCode::Char('g') => {}, // TODO: Move into previous cwd [outer].
                    _ => {}, 
            },
            OwlState::OwlShell => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Enter => explorer.execute_shell(),
                        KeyCode::Char(pressed) => explorer.append_to_shell(pressed), 
                        KeyCode::Backspace => explorer.delete_from_shell(),
                        KeyCode::Right => explorer.move_cursor(CursorDirection::Right),
                        KeyCode::Left => explorer.move_cursor(CursorDirection::Left),
                        KeyCode::Esc => {
                            explorer.state = OwlState::Normal;
                            explorer.reset_shell();
                        },
                        _ => {},
                    }
                }
            },
            OwlState::OwlOptions => match key.code {
                    KeyCode::Char(':') => explorer.state = OwlState::OwlShell,
                    KeyCode::Esc => explorer.state = OwlState::Normal,
                    _ => {}, 
                },
            }
        }
    Ok(false)
}

// TODO: define UI constants in a more convenient way.
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
        terminal.draw(|f: &mut Frame<'_>| user_interface(f, &mut explorer))?;
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