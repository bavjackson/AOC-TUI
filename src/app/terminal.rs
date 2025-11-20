use std::time::Duration;
use ratatui::DefaultTerminal;

use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode};
use tokio_stream::StreamExt;
use crate::ui::ui;

#[derive(Debug)]
pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new(title: &'a str) -> Self {
        Self { title, should_quit: false }
    }
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| ui::draw(frame, &mut self))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event) {
        if let Some(key) = event.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                }
                _ => {}
            }
        }
    }
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self::new("AOC-TUI")
    }
}