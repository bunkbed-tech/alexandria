use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Widget},
};
use tui_textarea::{Input, Key, TextArea};

use crate::utils::area_minus_border;

const GRID_ROWS: usize = 3;

enum State {
    Searching,
    Navigating,
}

pub struct MediaGrid {
    search: TextArea<'static>,
    state: State,
    focused: Option<(usize, usize)>,
}

fn render_search_state(search: &mut TextArea, state: &State) {
    let cursor_modifier = match state {
        State::Searching => Modifier::REVERSED,
        State::Navigating => Modifier::HIDDEN,
    };
    search.set_cursor_style(Style::default().add_modifier(cursor_modifier));
}

impl MediaGrid {
    pub fn new() -> Self {
        let mut search = TextArea::default();
        let state = State::Searching;
        render_search_state(&mut search, &state);

        Self {
            search,
            state,
            focused: Some((0, 0)),
        }
    }

    pub async fn handle_event(&mut self, event: Event) -> color_eyre::Result<()> {
        match self.state {
            State::Searching => match event.into() {
                Input { key: Key::Esc, .. } => self.state = State::Navigating,
                input => { self.search.input(input); },
            },
            State::Navigating => match event.into() {
                Input { key: Key::Char('/'), .. } => self.state = State::Searching,
                input => self.focused = self.focused.map(|position| match input {
                    Input { key: Key::Char('h'), .. } => (position.0.saturating_sub(1), position.1),
                    Input { key: Key::Char('j'), .. } => (position.0, (GRID_ROWS - 1).min(position.1 + 1)),
                    Input { key: Key::Char('k'), .. } => (position.0, position.1.saturating_sub(1)),
                    Input { key: Key::Char('l'), .. } => ((GRID_ROWS - 1).min(position.0 + 1), position.1),
                    _ => position,
                }),
            },
        };
        render_search_state(&mut self.search, &self.state);
        Ok(())
    }
}

impl Widget for &MediaGrid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, content_area] = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).areas(area);

        self.search.render(area_minus_border(header_area), buf);
        Block::bordered().title("Search").render(header_area, buf);

        let grid_layout = Layout::vertical([Constraint::Min(1); GRID_ROWS])
            .split(area_minus_border(content_area))
            .iter()
            .map(|&area| {
                Layout::horizontal([Constraint::Min(1); 3])
                    .split(area)
                    .to_vec()
            })
            .collect::<Vec<_>>();
        for (y, row) in grid_layout.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let color = if self.focused == Some((x, y)) { Color::Green } else { Color::Gray };
                Block::bordered().border_style(Style::default().fg(color)).render(*cell, buf);
            }
        }
    }
}
