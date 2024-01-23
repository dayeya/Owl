use crate::ui::config::{OWL_FONT_COLOR, OWL_BACKGROUND};
use ratatui::{
    prelude::*, 
    widgets::*
};

pub struct Content<'a> {
    pub inner: Paragraph<'a>,
}

impl<'a> Content<'a> {
    pub fn with_text(text: Option<String>, borders: Borders) -> Self  {
        let style: Style = Style::default().fg(OWL_FONT_COLOR).bg(OWL_BACKGROUND);
        let block: Block<'_> = Block::default().borders(borders);
        let inner: Paragraph<'a> = Paragraph::new(text.unwrap_or("".to_string()))
                                            .style(style)
                                            .block(block)
                                            .alignment(Alignment::Center)
                                            .wrap(Wrap { trim: true });
        Content { inner }
    }
}