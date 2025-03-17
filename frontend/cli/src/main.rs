use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
    DefaultTerminal, Frame,
};
use tokio::time::{sleep, Duration};

mod components;
mod utils;

use crate::components::{
    home::Home,
    login::Login,
};


#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal).await;
    ratatui::restore();
    result
}

struct App {
    login: Login,
    home: Home,
    exit: bool,
    event_stream: EventStream,
}

impl App {
    pub fn new() -> App {
        App {
            login: Login::new(),
            home: Home::new(),
            exit: false,
            event_stream: EventStream::default(),
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    async fn handle_events(&mut self) -> color_eyre::Result<()> {
        tokio::select! {
            event = self.event_stream.next().fuse() => {
                match event {
                    Some(Ok(evt)) => {
                        match evt {
                            Event::Key(key) if (
                                key.kind == KeyEventKind::Press
                                && key.code == KeyCode::Char('c')
                                && key.modifiers.contains(KeyModifiers::CONTROL)
                            ) => self.exit = true,
                            input_event => {
                                if self.login.is_authenticated() {
                                    self.home.handle_event(input_event).await?;
                                } else {
                                    self.login.handle_event(input_event).await?;
                                }
                            },
                        }
                    }
                    _ => {}
                }
            }
            // Sleep for a short duration to avoid busy waiting.
            _ = sleep(Duration::from_millis(100)) => {}
        }
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
