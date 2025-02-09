use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
    DefaultTerminal, Frame,
};

mod components;
mod utils;

use crate::components::{
    home::Home,
    login::Login,
};


fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}

struct App {
    login: Login,
    home: Home,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            login: Login::new(),
            home: Home::new(),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) if (
                key.kind == KeyEventKind::Press
                && key.code == KeyCode::Char('c')
                && key.modifiers.contains(KeyModifiers::CONTROL)
            ) => self.exit = true,
            input_event => {
                if self.login.is_authenticated() {
                    self.home.handle_event(input_event)?;
                } else {
                    self.login.handle_event(input_event)?;
                }
            },
        };
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.login.is_authenticated() {
            self.home.render(area, buf);
        } else {
            self.login.render(area, buf);
        }
    }
}
