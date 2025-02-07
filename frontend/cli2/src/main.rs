use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
    DefaultTerminal, Frame,
};

mod components;
mod utils;

use crate::components::login::Login;


fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}

struct App {
    login: Login,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            login: Login::new(),
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
        match event::read()?.into() {
            Event::Key(key) if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc => self.exit = true,
            input_event => {
                self.login.handle_event(input_event)?;
            },
        };
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.login.render(area, buf);
    }
}
