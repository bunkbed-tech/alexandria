use std::io;

use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, List, Widget},
};

pub struct Sidebar {
    pub is_active: bool,
}

impl Sidebar {
    pub fn new(is_active: bool) -> Self {
        Self {
            is_active
        }
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
        let color = if self.is_active { Color::Blue } else { Color::Gray };
        let block = Block::bordered().border_style(Style::default().fg(color));
        List::new(["Explore", "Library"]).block(block).render(area, buf);
    }
}
