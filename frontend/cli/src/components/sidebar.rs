use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{List, ListItem, Widget},
};
use strum::{Display, EnumCount, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Clone, Copy, Default, Display, EnumCount, EnumIter, FromRepr, PartialEq)]
pub enum Page {
    #[default]
    Explore,
    Library,
}

pub struct Sidebar {
    pub page: Page,
    index: isize,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            page: Page::Explore,
            index: 0,
        }
    }

    pub async fn handle_event(&mut self, event: Event) -> color_eyre::Result<()> {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char('j') => {
                        self.index = (self.index + 1).rem_euclid(Page::COUNT as isize)
                    }
                    KeyCode::Char('k') => {
                        self.index = (self.index - 1).rem_euclid(Page::COUNT as isize)
                    }
                    KeyCode::Enter => {
                        self.page = Page::from_repr(self.index as usize).expect("Unreachable")
                    }
                    _ => {}
                };
            }
            _ => {}
        };
        Ok(())
    }
}

impl Widget for &Sidebar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lines = Page::iter().map(|page| {
            let color = if page == self.page {
                Color::Magenta
            } else {
                Color::Gray
            };
            let symbol = if page as isize == self.index {
                ">"
            } else {
                " "
            };
            let line = Line::styled(format!("{} {page}", symbol), Style::default().fg(color));
            ListItem::new(line)
        });
        List::new(lines).render(area, buf);
    }
}
