use std::fmt;
use std::rc::Rc;
use ratatui::{prelude::*, widgets::*};
use crate::app::{App, Mode};
use crate::config::Config;
use crate::internal::Directory;

pub struct ModeBar;
pub struct Shell;
pub struct Options;

#[derive(Debug, Clone)]
pub enum ParseError {
    ParseColorError(String)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::ParseColorError(s) => write!(f, "Could not parse {s} to Color")
        }
    }
}

fn parse_to_color(s: &String) -> Result<Color, ParseError> {
    let parts: Vec<&str> = s.split(",").collect();
    let (r, g, b): (u8, u8, u8) = {
        let parsed: Vec<u8> = parts.into_iter().map(|val| {
            let trimmed = val.trim();
            trimmed.parse().unwrap()
        }).collect();
        (parsed[0], parsed[1], parsed[2])
    };
    Ok(Color::Rgb(r, g, b))
}

impl ModeBar {
    pub fn new(mode: String, conf: &Config) -> Paragraph {
        let style: Style = Style::default()
            .fg(parse_to_color(&conf.color_schemes.bg).unwrap())
            .bg(parse_to_color(&conf.color_schemes.fg).unwrap());
        let block: Block<'_> = Block::default();
        let mode_bar: Paragraph<'_> = Paragraph::new(mode)
            .style(style)
            .block(block);
        mode_bar
    }
}

impl Shell {
    pub fn new(input: String, conf: &Config) -> Paragraph {
        let style: Style = Style::default()
            .fg(parse_to_color(&conf.color_schemes.fg).unwrap())
            .bg(parse_to_color(&conf.color_schemes.bg).unwrap());
        let block: Block<'_> = Block::default();
        let shell: Paragraph<'_> = Paragraph::new(input)
            .style(style)
            .block(block)
            .wrap(Wrap { trim: true });
        shell
    }
}

impl Options {
    pub fn new(title: String, conf: &Config) -> List {
        let options_style: Style = Style::default()
            .fg(parse_to_color(&conf.color_schemes.fg).unwrap())
            .bg(parse_to_color(&conf.color_schemes.bg).unwrap());
        let options_block: Block<'_> = Block::default()
            .title(title)
            .borders(Borders::ALL);

        let items: Vec<ListItem> = (&conf.options.ops).into_iter().map(|op| {
            ListItem::from(op.as_str())
        }).collect();
        let options: List<'_> = List::new(items)
            .block(options_block)
            .style(options_style)
            .highlight_symbol(">")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
        options
    }
}

pub struct UiTree<'a> {
    pub bg: Color,
    pub fg: Color,
    pub title: String,
    pub state: TableState,
    pub headers: Row<'a>,
    pub items: Vec<Row<'a>>,
    pub highlight_sym: &'a str,
}

impl<'a> Clone for UiTree<'a> {
    fn clone(&self) -> Self {
        Self {
            bg: self.bg.clone(),
            fg: self.fg.clone(),
            title: self.title.clone(),
            state: self.state.clone(),
            headers: self.headers.clone(),
            items: self.items.clone(),
            highlight_sym: self.highlight_sym.clone(),

        }
    }
}

impl<'a> UiTree<'a> {
    pub fn new(dir: &mut Directory, conf: &Config) -> Self {
        Self {
            bg: parse_to_color(&conf.color_schemes.bg).unwrap(),
            fg: parse_to_color(&conf.color_schemes.fg).unwrap(),
            title: format!("Walk through {}", dir.display()),
            state: TableState::default().with_selected(Some(0)),
            headers: Row::new(vec!["Name", "Date modified", "Type", "Size"]),
            items: {
                let items = dir.walk();
                let rows = items.iter().map(|r| Row::new(r.to_vec())).collect::<Vec<Row>>();
                rows
            },
            highlight_sym: " > "
        }
    }

    pub fn render(&self) -> Table<'_> {
        let items = (&self.items).to_owned();
        let title = (&self).title.to_owned();
        let headers = (&self).headers.to_owned();
        let widths = [
            Constraint::Length(25),
            Constraint::Length(20),
            Constraint::Length(10),
            Constraint::Length(10)
        ];
        let tree_block = Block::default().title(title).borders(Borders::ALL);
        let tree_style = Style::default()
            .fg(self.fg)
            .bg(self.bg);
        let tree = Table::new(items, widths)
            .style(tree_style)
            .block(tree_block)
            .header(headers)
            .highlight_symbol(self.highlight_sym).add_modifier(Modifier::BOLD)
            .highlight_spacing(HighlightSpacing::Always);

        tree
    }

    pub fn move_next(&mut self) {
        let i = match self.state.selected() {
            Some(k) => {
                if k < self.items.len() {
                    k + 1
                } else {
                    0
                }
            }
            None => 0
        };
        self.state.select(Some(i))
    }

    pub fn move_previous(&mut self) {
        let i = match self.state.selected() {
            Some(k) => {
                if k > 0 {
                    k - 1
                } else {
                    self.items.len() - 1
                }
            }
            None => 0
        };
        self.state.select(Some(i))
    }
}

#[derive(Clone)]
pub struct FileSystemUi<'a> {
    pub tree: UiTree<'a>,
    pub preview: Paragraph<'a>
}

impl<'a> FileSystemUi<'a> {
    pub fn new(dir: &mut Directory, conf: &Config) -> Self {
        let tree = UiTree::new(dir, conf);
        let style = Style::default()
            .fg(parse_to_color(&conf.color_schemes.fg).unwrap())
            .bg(parse_to_color(&conf.color_schemes.bg).unwrap());
        let block = Block::default().borders(Borders::ALL);
        let preview = Paragraph::new("").style(style).block(block);

        Self {
            tree,
            preview
        }
    }
}

pub enum LayoutOps {
    App,
    View,
    Options(u16, u16)
}

pub enum Layouts {
    App {
        rects: Rc<[Rect]>
    },
    View {
        rects: Rc<[Rect]>
    },
    Options {
        rects: Rc<[Rect]>
    },
}

impl Layouts {
    fn new(layout_type: LayoutOps, parent: Rect) -> Self {
        match layout_type {
            LayoutOps::App => {
                let application_layout = Layout::new(
                    Direction::Vertical, [
                        Constraint::Max(98),    // Normal block
                        Constraint::Length(1),  // State block
                        Constraint::Length(1)   // Shell block
                    ], ).split(parent);
                Layouts::App {rects: application_layout}
            },
            LayoutOps::View => {
                let view_layout = Layout::new(
                    Direction::Horizontal, [
                        Constraint::Percentage(55), // Tree pane
                        Constraint::Percentage(45), // Preview pane
                    ], ).split(parent);
                Layouts::View {rects: view_layout}
            },
            LayoutOps::Options(width, height) => {
                let vertical_layout = Layout::new(
                    Direction::Vertical, [
                        Constraint::Percentage(100 - height),
                        Constraint::Percentage(height),
                    ]).split(parent);
                let horizontal_layout = Layout::new(
                    Direction::Horizontal,[
                        Constraint::Percentage(100 - width),
                        Constraint::Percentage(width),
                    ]).split(vertical_layout[1]);

                Layouts::Options {rects: horizontal_layout}
            }
        }
    }

    fn rects(&self) -> &Rc<[Rect]> {
        match self {
            Layouts::App { rects } => rects,
            Layouts::View { rects } => rects,
            Layouts::Options { rects } => rects,
        }
    }
}

fn draw_bars(f: &mut Frame, app: &mut App, area: &Rc<[Rect]>) {
    let mode_bar = ModeBar::new(app.format_mode(), &app.config);
    let shell = Shell::new((&app.shell.input).to_owned(), &app.config);
    f.render_widget(mode_bar, area[1]);
    f.render_widget(shell, area[2]);
}

fn draw_main(f: &mut Frame, app: &mut App, area: &Rc<[Rect]>) {
    let mut main_view = FileSystemUi::new(&mut app.cwd, &app.config);
    let mut tree = main_view.tree;
    f.render_stateful_widget(tree.clone().render(), area[0], &mut tree.state);
    f.render_widget(main_view.preview, area[1]);
}

fn draw_options(f: &mut Frame, app: &mut App, area: &Rc<[Rect]>) {
    let options_list = Options::new("Available cmds".to_string(), &app.config);
    f.render_widget(options_list, area[1])
}

pub(crate) fn user_interface(f: &mut Frame, app: &mut App) {
    let screen: Rect = f.size();
    let root = Layouts::new(LayoutOps::App, screen);
    let root_rects = root.rects();
    let secondary_area = Layouts::new(LayoutOps::View, root_rects[0]);
    let secondary_rects = secondary_area.rects();
    let options_area = Layouts::new(LayoutOps::Options(40, 27), root_rects[0]);
    let options_rects = options_area.rects();

    // Draw all layouts.
    draw_main(f, app, secondary_rects);
    draw_bars(f, app, root_rects);
    match app.mode {
        Mode::InsideOptions => draw_options(f, app, options_rects),
        _ => {}
    }
}

/*

    let second_layout: Rc<[Rect]> = Layout::new(
        Direction::Horizontal, [
            Constraint::Percentage(55), // Tree pane
            Constraint::Percentage(45), // Preview pane
        ], ).split(layout[0]);

    let preview: Content = Content::with_text(Some("".to_string()), Borders::ALL);
    let mode_bar: ModeBar = ModeBar::with_text(app.format_mode());
    let shell_input: String = (&app.shell.input).to_owned();
    let shell: Shell = Shell::with_text(shell_input);
    let options: Options<'_> = Options::with_items(40, 27, layout[0], String::from("Options"));
    let ui_tree: Table<'_> = app.tree.render();

    f.render_stateful_widget(ui_tree, second_layout[0], &mut app.tree.state);
    f.render_widget(preview.inner, second_layout[1]);
    f.render_widget(mode_bar.inner, layout[1]);
    f.render_widget(shell.inner, layout[2]);

    match app.mode {
        Mode::InsideOptions => {
            f.render_widget(Clear, options.rect);
            f.render_widget(options.inner, options.rect);
        },
        _ => {},
    }*/