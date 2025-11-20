use ratatui::DefaultTerminal;
use std::time::Duration;

use crate::ui::ui;
use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode};
use tokio_stream::StreamExt;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

#[derive(Debug)]
pub struct Config {
    pub session_token: Option<String>,
    pub input: Input,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Debug)]
pub struct App<'a> {
    pub title: &'a str,
    pub config: Config,
    pub should_quit: bool,
    pub input_mode: InputMode,
}

impl<'a> App<'a> {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new(title: &'a str, session_token: Option<String>) -> Self {
        Self {
            title,
            should_quit: false,
            config: Config {
                session_token,
                input: Input::default(),
            },
            input_mode: InputMode::Normal,
        }
    }
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| ui::draw(frame, &mut self))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event) {
        if let Some(key) = event.as_key_press_event() {
            match self.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => self.set_token(),
                    _ => {
                        self.config.input.handle_event(&event);
                    }
                },
            }
        }
        if self.config.session_token.is_none() && self.input_mode == InputMode::Normal {
            self.input_mode = InputMode::Editing;
        }
    }

    fn set_token(&mut self) {
        self.config.session_token = Some(self.config.input.value().to_string());
        self.input_mode = InputMode::Normal;
    }
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self::new("AOC-TUI", None)
    }
}
