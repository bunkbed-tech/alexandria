use std::io;

use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, List, Widget},
};
use tui_textarea::{Input, Key, TextArea};

use crate::utils::area_minus_border;

enum ActivePane {
    Sidebar,
    Content,
}

pub struct Home {
    active_pane: ActivePane,
    search: TextArea<'static>,
}

impl Home {
    pub fn new() -> Home {
        Home {
            active_pane: ActivePane::Sidebar,
            search: TextArea::default(),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> io::Result<()> {
        match event.into() {
            Input { key: Key::Tab, .. } => match self.active_pane {
                ActivePane::Sidebar => self.active_pane = ActivePane::Content,
                ActivePane::Content => self.active_pane = ActivePane::Sidebar,
            },
            input => match self.active_pane {
                ActivePane::Sidebar => {},
                ActivePane::Content => {
                    self.search.input(input);
                },
            },
        };
        Ok(())
    }
}

impl Widget for &Home {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [sidebar_area, main_area] = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]).areas(area);

        let sidebar_block = match self.active_pane {
            ActivePane::Sidebar => Block::bordered().border_style(Style::default().fg(Color::Blue)),
            ActivePane::Content => Block::bordered(),
        };
        List::new(["Explore", "Library"]).block(sidebar_block).render(sidebar_area, buf);

        let main_block = match self.active_pane {
            ActivePane::Sidebar => Block::bordered(),
            ActivePane::Content => Block::bordered().border_style(Style::default().fg(Color::Blue)),
        };
        main_block.render(main_area, buf);

        let [header_area, content_area] = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).areas(area_minus_border(main_area));

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
