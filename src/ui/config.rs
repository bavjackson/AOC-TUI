use crate::app::signals::AppEvent;
use crossterm::event::{Event, KeyCode};
use ratatui::widgets::{Block, Paragraph, Widget};
use tokio::sync::mpsc::UnboundedSender;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

#[derive(Debug, Clone)]
pub struct ConfigWidget {
    pub input: Input,
    pub sender: UnboundedSender<AppEvent>,
}

impl ConfigWidget {
    pub fn new(sender: UnboundedSender<AppEvent>) -> Self {
        Self {
            input: Input::default(),
            sender,
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        if let Some(key) = event.as_key_press_event() {
            match key.code {
                KeyCode::Enter => {
                    self.sender
                        .send(AppEvent::SetSessionToken(self.input.value().to_string()))
                        .unwrap();
                }
                _ => {
                    self.input.handle_event(&event);
                }
            }
        }
    }
}

impl Widget for &ConfigWidget {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let paragraph = Paragraph::new(self.input.value())
            .block(Block::bordered().title("Enter your session token"));
        paragraph.render(area, buf);
    }
}
