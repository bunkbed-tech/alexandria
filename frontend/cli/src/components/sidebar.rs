use std::io;

use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{List, Widget},
};

pub struct Sidebar {}

impl Sidebar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_event(&mut self, event: Event) -> io::Result<()> {
        match event {
            _ => {},
        };
        Ok(())
    }
}

impl Widget for &Sidebar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        List::new(["Explore", "Library"]).render(area, buf);
    }
}
