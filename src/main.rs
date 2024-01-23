mod ui;
mod explorer;
mod internal;

use std::io;
use std::rc::Rc;
use crossterm::{
    execute,
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, 
};
use ratatui::{prelude::*, widgets::*, layout::Layout};
use explorer::{Owl, OwlState, CursorDirection};
use crate::ui::content::Content;
use crate::ui::mode_bar::ModeBar;
use crate::ui::shell::Shell;
use crate::ui::options::Options;
use crate::ui::tree::UiTree;


fn user_interface(f: &mut Frame, owl_explorer: &mut Owl) {
    let size: Rect = f.size();
    let layout: Rc<[Rect]> = Layout::new(
        Direction::Vertical, [
            Constraint::Percentage(96), // Normal block
            Constraint::Percentage(2),  // State block
            Constraint::Percentage(2)   // Shell block
        ], ).split(size);

    let second_layout: Rc<[Rect]> = Layout::new(
        Direction::Horizontal, [
            Constraint::Percentage(50), // Tree pane
            Constraint::Percentage(50), // Preview pane
        ], ).split(layout[0]);

    let preview: Content = Content::with_text(Some("".to_string()), Borders::ALL);
    let mode_bar: ModeBar = ModeBar::with_text(owl_explorer.format_mode());
    let shell_input: String = (&owl_explorer.shell.input).to_owned();
    let shell: Shell = Shell::with_text(shell_input);
    let options: Options<'_> = Options::with_items(40, 27, layout[0], String::from("Options"));

    let cwd: Vec<[String; 4]> = owl_explorer.walk();
    let tree_title: String = format!("Walk through {}", owl_explorer.cwd.display());
    let items = UiTree::new(tree_title, cwd);

    f.render_widget(items.render(), second_layout[0]);
    
    f.render_widget(preview.inner, second_layout[1]);
    f.render_widget(mode_bar.inner, layout[1]);
    f.render_widget(shell.inner, layout[2]);

    match owl_explorer.state {
        OwlState::OwlOptions => {
            f.render_widget(Clear, options.rect);
            f.render_widget(options.inner, options.rect);
        },
        _ => {},
    }

}

fn handle_events(owl_explorer: &mut Owl) -> Result<bool, io::Error> {
    if let Event::Key(key) = event::read()? {
        match owl_explorer.state {
            OwlState::Ended => { 
                return Ok(true) 
            },
            OwlState::Normal => match key.code {
                    KeyCode::Char('o') => owl_explorer.state = OwlState::OwlOptions, 
                    KeyCode::Char(':') => owl_explorer.state = OwlState::OwlShell,
                    KeyCode::Char('f') => {}, // TODO: Move into previous cwd [inner].
                    KeyCode::Char('g') => {}, // TODO: Move into previous cwd [outer].
                    _ => {}, 
            },
            OwlState::OwlShell => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Enter => owl_explorer.execute_shell(),
                        KeyCode::Char(pressed) => owl_explorer.append_to_shell(pressed), 
                        KeyCode::Backspace => owl_explorer.delete_from_shell(),
                        KeyCode::Right => owl_explorer.move_cursor(CursorDirection::Right),
                        KeyCode::Left => owl_explorer.move_cursor(CursorDirection::Left),
                        KeyCode::Esc => {
                            owl_explorer.state = OwlState::Normal;
                            owl_explorer.reset_shell();
                        },
                        _ => {},
                    }
                }
            },
            OwlState::OwlOptions => match key.code {
                    KeyCode::Char(':') => owl_explorer.state = OwlState::OwlShell,
                    KeyCode::Esc => owl_explorer.state = OwlState::Normal,
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
    let mut owl_explorer: Owl = Owl::new().unwrap(); // TODO: HANDLE DRIVELOADINGERROR.

    while !should_quit {
        terminal.draw(|f: &mut Frame<'_>| user_interface(f, &mut owl_explorer))?;
        should_quit = handle_events(&mut owl_explorer).unwrap();
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    Ok(())
}