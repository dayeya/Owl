use std::rc::Rc;

use crate::ui::config::{OWL_FONT_COLOR, OWL_SECONDARY_BG, OPTIONS};
use ratatui::{
    prelude::*, 
    widgets::*
};

pub struct Options<'a> {
    pub inner: List<'a>,
    pub rect: Rect
}

impl<'a> Options<'a> {
    pub fn with_items(w: u16, h: u16, parent: Rect, title: String) -> Self {
        let height_constraints = [Constraint::Percentage(100 - h), Constraint::Percentage(h), ];
        let options_layout: Rc<[Rect]> = Layout::new(Direction::Vertical, height_constraints).split(parent);
        let width_constraints = [Constraint::Percentage(100 - w), Constraint::Percentage(w), ];
        let options_area: Rect = Layout::new(Direction::Horizontal, width_constraints).split(options_layout[1])[1];

        let options_style: Style = Style::default().fg(OWL_FONT_COLOR).bg(OWL_SECONDARY_BG);
        let options_block: Block<'_> = Block::default().title(title).borders(Borders::ALL);
        let options: List<'a> = List::new(OPTIONS)
            .block(options_block)
            .style(options_style)
            .highlight_symbol(">")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        Options {
            inner: options,
            rect: options_area
        }
    }
}