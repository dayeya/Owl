use crate::ui::config::{OWL_FONT_COLOR, OWL_BACKGROUND};
use ratatui::{
    prelude::*, 
    widgets::*
};

pub struct Shell<'a> {
    pub inner: Paragraph<'a>,
}

impl<'a> Shell<'a> {
    pub fn with_text(text: String) -> Self  {
        let style: Style = Style::default().fg(OWL_FONT_COLOR).bg(OWL_BACKGROUND);
        let block: Block<'_> = Block::default();
        let inner: Paragraph<'a> = Paragraph::new(text)
                                            .style(style)
                                            .block(block)
                                            .wrap(Wrap { trim: true });
        Shell { inner }
    }
}