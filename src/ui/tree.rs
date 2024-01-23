use crate::ui::config::{OWL_FONT_COLOR, OWL_BACKGROUND};
use ratatui::{layout::Constraint, prelude::Style, widgets::{Block, Borders, Row, Table, TableState}};

pub struct UiTree<'a> {
    pub title: String, 
    pub state: TableState,
    pub headers: Row<'a>,
    pub items: Vec<Row<'a>>,
    pub highlight_sym: &'a str,
}

impl<'a> UiTree<'a> {
    pub fn new(title: String, items: Vec<[String; 4]>) -> Self {
        let state = TableState::default();
        let rows = items.iter().map(|r| Row::new(r.to_vec())).collect::<Vec<Row>>();
        let headers = Row::new(vec!["Name", "Date modified", "Type", "Size"]);
        let highlight_symbol = ">";

        Self {
            title: title,
            state: state,
            headers: headers,
            items: rows,
            highlight_sym: highlight_symbol
        }
    }

    pub fn render(&self) -> Table<'a> {
        let items = (&self.items).to_owned();
        let title = (&self).title.to_owned();
        let headers = (&self).headers.to_owned();
        let widths = [
            Constraint::Length(25), 
            Constraint::Length(20), 
            Constraint::Length(10), 
            Constraint::Length(10)
        ];
        let tree_style = Style::default().fg(OWL_FONT_COLOR).bg(OWL_BACKGROUND);
        let tree = Table::new(items, widths)
        .style(tree_style)
        .block(Block::default().title(title).borders(Borders::ALL))
        .header(headers)
        .highlight_symbol(self.highlight_sym);

        tree
    }
}