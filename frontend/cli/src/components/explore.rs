use std::io;

use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Widget},
};
use tui_textarea::{Input, Key, TextArea};

use crate::utils::area_minus_border;

pub struct Explore {
    search: TextArea<'static>,
}

impl Explore {
    pub fn new() -> Self {
        Self {
            search: TextArea::default(),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> io::Result<()> {
        match event.into() {
            Input { key: Key::Char('q'), .. } => {},  // FIXME handle input types better
            input => {
                self.search.input(input);
            },
        };
        Ok(())
    }
}

impl Widget for &Explore {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, content_area] = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).areas(area);

        self.search.render(area_minus_border(header_area), buf);
        Block::bordered().title("Search").render(header_area, buf);

        let grid_layout = Layout::vertical([Constraint::Min(1); 3])
            .split(area_minus_border(content_area))
            .iter()
            .map(|&area| {
                Layout::horizontal([Constraint::Min(1); 3])
                    .split(area)
                    .to_vec()
            })
            .collect::<Vec<_>>();
        for row in grid_layout.iter() {
            for cell in row.iter() {
                Block::bordered().render(*cell, buf);
            }
        }
    }
}
