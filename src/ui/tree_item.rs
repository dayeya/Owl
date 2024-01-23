use crate::ui::config::{OWL_FONT_COLOR, OWL_BACKGROUND};
use ratatui::{prelude::Style, widgets::ListItem};

pub fn list_item(path: &str) -> ListItem<'_> {
    let item_style: Style = Style::default().fg(OWL_FONT_COLOR).bg(OWL_BACKGROUND);
    ListItem::new(path).style(item_style)
}