use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, List, Widget},
};

use crate::utils::area_minus_border;

enum ActivePane {
    Sidebar,
    Content,
}

pub struct Home {
    active_pane: ActivePane,
}

impl Home {
    pub fn new() -> Home {
        Home {
            active_pane: ActivePane::Sidebar,
        }
    }

    pub fn handle_event(&mut self, event: Event) -> io::Result<()> {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Tab => match self.active_pane {
                        ActivePane::Sidebar => self.active_pane = ActivePane::Content,
                        ActivePane::Content => self.active_pane = ActivePane::Sidebar,
                    },
                    _ => {},
                };
            },
            _ => {},
        };
        Ok(())
    }
}

impl Widget for &Home {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let panes = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]).split(area);

        let sidebar_block = match self.active_pane {
            ActivePane::Sidebar => Block::bordered().border_style(Style::default().fg(Color::Blue)),
            ActivePane::Content => Block::bordered(),
        };
        List::new(["Explore", "Library"]).block(sidebar_block).render(panes[0], buf);

        let content_block = match self.active_pane {
            ActivePane::Sidebar => Block::bordered(),
            ActivePane::Content => Block::bordered().border_style(Style::default().fg(Color::Blue)),
        };
        let grid_layout = Layout::vertical([Constraint::Min(1); 3])
            .split(area_minus_border(panes[1]))
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
        content_block.render(panes[1], buf);
    }
}
