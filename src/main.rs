use std::io;
use std::rc::Rc;

#[allow(unused_imports)]
use winsafe::prelude::*;

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
};

use ratatui::{prelude::*, widgets::*, layout::Layout};

mod explorer;
use explorer::owl_explorer::{Owl, OwlState, CursorDirection};

const SHELL_BACKGROUND: Color = Color::Rgb(53, 80, 112);
const OWL_BACKGROUND: Color = Color::Rgb(77, 94, 114);
const OWL_SECONDARY: Color = Color::Rgb(254, 250, 212);


fn user_interface(f: &mut Frame, owl_explorer: &mut Owl) {
    let size: Rect = f.size();
    let text: Vec<Line> = vec![
        Line::from(Span::styled("Owl file explorer, PRESS O", Style::default().fg(OWL_SECONDARY))),
    ];

    let wrap_trim: Wrap = Wrap { trim: true };
    let layout: Rc<[Rect]> = Layout::new(
        Direction::Vertical, [Constraint::Percentage(98), Constraint::Percentage(2)], ).split(size);

    let home_style: Style = Style::default().fg(OWL_SECONDARY).bg(OWL_BACKGROUND);
    let home_block: Block<'_> = Block::default().borders(Borders::ALL);
    let home_page: Paragraph<'_> = Paragraph::new(text)
                                            .style(home_style)
                                            .block(home_block)
                                            .alignment(Alignment::Center)
                                            .wrap(wrap_trim);

    let current_shell_input: &str = owl_explorer.shell.input.as_str();
    let shell_style: Style = Style::default().fg(OWL_SECONDARY).bg(SHELL_BACKGROUND);
    let shell_block: Block<'_> = Block::default();
    let shell: Paragraph<'_> = Paragraph::new(current_shell_input)
                                .style(shell_style)
                                .block(shell_block)
                                .wrap(wrap_trim);

    f.render_widget(home_page, layout[0]);
    f.render_widget(shell, layout[1]);
}

fn handle_events(owl_explorer: &mut Owl) -> Result<bool, io::Error> {
    if let Event::Key(key) = event::read()? {
        match owl_explorer.state {
            OwlState::Ended => { return Ok(true); }
            OwlState::Normal => match key.code {
                    KeyCode::Char('o') => owl_explorer.state = OwlState::OwlOptions,
                    KeyCode::Char(':') => owl_explorer.state = OwlState::OwlShell,
                    _ => {}, 
            }
            OwlState::OwlShell => {
                    if key.kind == KeyEventKind::Press { 
                        match key.code {
                            KeyCode::Enter => {
                                /*
                                1. BackEnd: Get the shell command.
                                2. Backend: Execute.
                                3. FrontEnd: Execute.
                                */
                            },
                            KeyCode::Char(pressed) => owl_explorer.append_to_shell(pressed), 
                            KeyCode::Backspace => owl_explorer.delete_from_shell(),
                            KeyCode::Right => owl_explorer.move_cursor(CursorDirection::Right),
                            KeyCode::Left => owl_explorer.move_cursor(CursorDirection::Left),
                            KeyCode::Esc => owl_explorer.state = OwlState::Normal,
                            _ => {},
                        }
                    }
                },
            OwlState::OwlOptions => todo!(),
        }
    }
    Ok(false)
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout: io::Stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

    let mut should_quit: bool = false;
    let mut owl_explorer: Owl = Owl::new();

    while !should_quit {
        terminal.draw(|f: &mut Frame<'_>| user_interface(f, &mut owl_explorer))?;
        should_quit = handle_events(&mut owl_explorer).unwrap();
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