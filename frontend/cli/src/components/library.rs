use std::io;

use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::Widget,
};

pub struct Library {}

impl Library {
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

impl Widget for &Library {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Text::from("This is the library").render(area, buf);
    }
}
