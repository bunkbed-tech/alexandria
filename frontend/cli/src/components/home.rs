use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::components::{
    explore::Explore,
    sidebar::Sidebar,
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
            explore: Explore::new(true),
            sidebar: Sidebar::new(false),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> io::Result<()> {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Tab => {
                self.active_pane = match self.active_pane {
                    ActivePane::Sidebar => ActivePane::Explore,
                    ActivePane::Explore => ActivePane::Sidebar,
                };
                self.sidebar.is_active = self.active_pane == ActivePane::Sidebar;
                self.explore.is_active = self.active_pane == ActivePane::Explore;
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
        let [sidebar_area, main_area] = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]).areas(area);
        self.sidebar.render(sidebar_area, buf);
        self.explore.render(main_area, buf);
    }
}
