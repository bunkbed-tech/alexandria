use std::io;

use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

use crate::components::media_grid::MediaGrid;

pub struct Library {
    media_grid: MediaGrid,
}

impl Library {
    pub fn new() -> Self {
        Self {
            media_grid: MediaGrid::new(),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> io::Result<()> {
        self.media_grid.handle_event(event)?;
        Ok(())
    }
}

impl Widget for &Library {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.media_grid.render(area, buf);
    }
}
