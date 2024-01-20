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
        Direction::Vertical, [
            Constraint::Percentage(96), // Normal block
            Constraint::Percentage(2),  // State block
            Constraint::Percentage(2)   // Shell block
        ], ).split(size);

    let home_style: Style = Style::default().fg(OWL_SECONDARY).bg(OWL_BACKGROUND);
    let home_block: Block<'_> = Block::default();
    let home_frame: Paragraph<'_> = Paragraph::new(text)
                                            .style(home_style)
                                            .block(home_block)
                                            .alignment(Alignment::Center)
                                            .wrap(wrap_trim);

    let current_state: String = owl_explorer.get_state_desc().unwrap();
    let state_style: Style = Style::default().fg(OWL_BACKGROUND).bg(OWL_SECONDARY);
    let state_block: Block<'_> = Block::default();
    let state_frame: Paragraph<'_> = Paragraph::new(current_state)
                                .style(state_style)
                                .block(state_block);

    let current_shell_input: &str = owl_explorer.shell.input.as_str();
    let shell_style: Style = Style::default().fg(OWL_SECONDARY).bg(SHELL_BACKGROUND);
    let shell_block: Block<'_> = Block::default();
    let shell_frame: Paragraph<'_> = Paragraph::new(current_shell_input)
                                .style(shell_style)
                                .block(shell_block)
                                .wrap(wrap_trim);

    f.render_widget(home_frame, layout[0]);
    f.render_widget(state_frame, layout[1]);
    f.render_widget(shell_frame, layout[2]);

    match owl_explorer.state {
        OwlState::OwlOptions => {
            let options_style: Style = Style::default().fg(Color::White);
            let options_block: Block<'_> = Block::default().title("OwlOptions").borders(Borders::ALL);
            let options = List::new(owl_explorer.options.items)
                .block(options_block)
                .style(options_style)
                .highlight_symbol(">")
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom);
    
            f.render_widget(options, layout[0]);
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
                    KeyCode::Esc => owl_explorer.state = OwlState::Normal,
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

// TODO: Fix bug that makes the shell not delete chars from the end.

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