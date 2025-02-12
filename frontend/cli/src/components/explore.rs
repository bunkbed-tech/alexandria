use std::io;

use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

use crate::components::media_grid::MediaGrid;

pub struct Explore {
    media_grid: MediaGrid,
}

impl Explore {
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

impl Widget for &Explore {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.media_grid.render(area, buf);
    }
}
