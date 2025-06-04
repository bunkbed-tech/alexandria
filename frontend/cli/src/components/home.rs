use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Widget},
};

use crate::{
    components::{
        explore::Explore,
        library::Library,
        sidebar::{Page, Sidebar},
    },
    utils::area_minus_border,
};

#[derive(PartialEq)]
enum ActivePane {
    Sidebar,
    Page,
}

pub struct Home {
    active_pane: ActivePane,
    explore: Explore,
    library: Library,
    sidebar: Sidebar,
}

impl Home {
    pub fn new() -> Self {
        Self {
            active_pane: ActivePane::Page,
            explore: Explore::new(),
            library: Library::new(),
            sidebar: Sidebar::new(),
        }
    }

    pub async fn handle_event(&mut self, event: Event) -> color_eyre::Result<()> {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Tab => {
                self.active_pane = match self.active_pane {
                    ActivePane::Sidebar => ActivePane::Page,
                    ActivePane::Page => ActivePane::Sidebar,
                };
            }
            input => match self.active_pane {
                ActivePane::Sidebar => self.sidebar.handle_event(input).await?,
                ActivePane::Page => match self.sidebar.page {
                    Page::Explore => self.explore.handle_event(input).await?,
                    Page::Library => self.library.handle_event(input).await?,
                },
            },
        };
        Ok(())
    }
}

impl Widget for &Home {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [sidebar_area, page_area] =
            Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
                .areas(area);

        let sidebar_color = if self.active_pane == ActivePane::Sidebar {
            Color::Blue
        } else {
            Color::Gray
        };
        let sidebar_block = Block::bordered().border_style(Style::default().fg(sidebar_color));
        sidebar_block.render(sidebar_area, buf);
        self.sidebar.render(area_minus_border(sidebar_area), buf);

        let page_color = if self.active_pane == ActivePane::Page {
            Color::Blue
        } else {
            Color::Gray
        };
        let page_block = Block::bordered()
            .border_style(Style::default().fg(page_color))
            .title(self.sidebar.page.to_string());
        page_block.render(page_area, buf);
        let page_area_inner = area_minus_border(page_area);
        match self.sidebar.page {
            Page::Explore => self.explore.render(page_area_inner, buf),
            Page::Library => self.library.render(page_area_inner, buf),
        };
    }
}
