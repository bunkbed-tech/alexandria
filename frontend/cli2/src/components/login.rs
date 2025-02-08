use std::io;

use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::Text,
    widgets::{Block, Widget},
};
use tui_textarea::{CursorMove, Input, Key, TextArea};

use crate::utils::{area_minus_border, center_widget};


fn inactivate(textarea: &mut TextArea) {
    textarea.set_cursor_style(Style::default());
}

fn activate(textarea: &mut TextArea) {
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
}

fn clear(textarea: &mut TextArea) {
    textarea.move_cursor(CursorMove::End);
    while !textarea.is_empty() {
        textarea.delete_line_by_head();
    }
}

enum ActiveInput {
    Username,
    Password,
}

#[derive(PartialEq)]
enum AuthState {
    Default,
    Failed,
    Success,
}

pub struct Login {
    username: TextArea<'static>,
    password: TextArea<'static>,
    auth_state: AuthState,
    active_input: ActiveInput,
}

impl Login {
    pub fn new() -> Login {
        let mut username = TextArea::default();
        username.set_cursor_line_style(Style::default());
        activate(&mut username);

        let mut password = TextArea::default();
        password.set_cursor_line_style(Style::default());
        password.set_mask_char('\u{1F921}');  // clown emoji
        inactivate(&mut password);

        Login {
            username,
            password,
            auth_state: AuthState::Default,
            active_input: ActiveInput::Username,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth_state == AuthState::Success
    }

    fn login(&mut self) {
        let success = self.username.lines()[0] == "test" && self.password.lines()[0] == "test";
        if !success {
            clear(&mut self.username);
            clear(&mut self.password);
            activate(&mut self.username);
            inactivate(&mut self.password);
            self.auth_state = AuthState::Failed;
            self.active_input = ActiveInput::Username;
        } else {
            self.auth_state = AuthState::Success;
        }
    }

    pub fn handle_event(&mut self, event: Event) -> io::Result<()> {
        match event.into() {
            Input { key: Key::Tab, .. } => match self.active_input {
                ActiveInput::Username => {
                    inactivate(&mut self.username);
                    activate(&mut self.password);
                    self.active_input = ActiveInput::Password;
                },
                ActiveInput::Password => {
                    activate(&mut self.username);
                    inactivate(&mut self.password);
                    self.active_input = ActiveInput::Username;
                },
            },
            Input { key: Key::Enter, .. } => self.login(),
            input => {
                let active_input = match self.active_input {
                    ActiveInput::Username => &mut self.username,
                    ActiveInput::Password => &mut self.password,
                };
                active_input.input(input);
            },
        };
        Ok(())
    }
}

impl Widget for &Login {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let centered_rect = center_widget(
            area,
            Constraint::Length(50),
            Constraint::Length(5),
        );
        let unbordered_rect = area_minus_border(centered_rect);
        // Split it into three lines
        let lines = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(unbordered_rect);

        let input_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(9),  // (len("Username") or len("Password")) + 1
                Constraint::Min(1),
            ]);

        let username_rect = input_layout.split(lines[1]);
        let password_rect = input_layout.split(lines[2]);

        let block_color = match self.auth_state {
            AuthState::Default => Color::Gray,
            AuthState::Failed => Color::Red,
            AuthState::Success => Color::Green,
        };
        Block::bordered()
            .border_set(border::THICK)
            .border_style(Style::default().fg(block_color))
            .render(centered_rect, buf);

        Text::from("Alexandria".bold()).render(lines[0], buf);
        Text::from("Username".bold()).render(username_rect[0], buf);
        Text::from("Password".bold()).render(password_rect[0], buf);

        self.username.render(username_rect[1], buf);
        self.password.render(password_rect[1], buf);
    }
}
