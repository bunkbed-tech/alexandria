use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    style::{Color, Style},
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Widget},
};

use crate::{
    components::{
        explore::Explore,
        sidebar::Sidebar,
    },
    utils::area_minus_border,
};

#[derive(PartialEq)]
enum ActivePane {
    Sidebar,
    Explore,
}

pub struct Home {
    active_pane: ActivePane,
    explore: Explore,
    sidebar: Sidebar,
}

impl Home {
    pub fn new() -> Self {
        Self {
            active_pane: ActivePane::Explore,
            explore: Explore::new(),
            sidebar: Sidebar::new(),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> io::Result<()> {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Tab => {
                self.active_pane = match self.active_pane {
                    ActivePane::Sidebar => ActivePane::Explore,
                    ActivePane::Explore => ActivePane::Sidebar,
                };
            },
            input => match self.active_pane {
                ActivePane::Sidebar => self.sidebar.handle_event(input)?,
                ActivePane::Explore => self.explore.handle_event(input)?,
            },
        };
        Ok(())
    }
}

impl Widget for &Home {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [sidebar_area, explore_area] = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]).areas(area);

        let sidebar_color = if self.active_pane == ActivePane::Sidebar { Color::Blue } else { Color::Gray };
        let sidebar_block = Block::bordered().border_style(Style::default().fg(sidebar_color));
        sidebar_block.render(sidebar_area, buf);
        self.sidebar.render(area_minus_border(sidebar_area), buf);

        let explore_color = if self.active_pane == ActivePane::Explore { Color::Blue } else { Color::Gray };
        let explore_block = Block::bordered().border_style(Style::default().fg(explore_color));
        explore_block.render(explore_area, buf);
        self.explore.render(area_minus_border(explore_area), buf);
    }
}
