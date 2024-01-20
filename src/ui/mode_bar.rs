use crate::ui::config::{OWL_FONT_COLOR, OWL_BACKGROUND};
use ratatui::{
    prelude::*, 
    widgets::*
};

pub struct ModeBar<'a> {
    pub inner: Paragraph<'a>,
}

impl<'a> ModeBar<'a> {
    pub fn with_text(text: String) -> Self  {
        let style: Style = Style::default().fg(OWL_BACKGROUND).bg(OWL_FONT_COLOR);
        let block: Block<'_> = Block::default();
        let inner: Paragraph<'a> = Paragraph::new(text)
                                            .style(style)
                                            .block(block);
        ModeBar { inner }
    }
}