use ratatui::DefaultTerminal;
use std::sync::Arc;
use std::time::Duration;

use crate::app::signals::AppEvent;
use crate::data::client::Client;
use crate::ui::config::ConfigWidget;
use crate::ui::events::EventsWidget;
use crate::ui::{Widgets, states::InputMode, ui};
use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tui_input::Input;

#[derive(Debug)]
pub struct Config {
    pub session_token: Option<String>,
    pub input: Input,
}

#[derive(Debug)]
pub struct App<'a> {
    pub title: &'a str,
    pub config: Config,
    pub should_quit: bool,
    pub input_mode: InputMode,
    pub events_widget: EventsWidget,
    pub config_widget: ConfigWidget,
    pub selected_widget: Widgets,
    pub rx: mpsc::UnboundedReceiver<AppEvent>,
    pub tx: mpsc::UnboundedSender<AppEvent>,
}

impl<'a> App<'a> {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new(title: &'a str, session_token: Option<String>) -> Self {
        let config = Config {
            session_token,
            input: Input::default(),
        };
        let client = Arc::new(Client::new(&config.session_token).unwrap());

        let (tx, mut rx) = mpsc::unbounded_channel::<AppEvent>();

        let events_widget = EventsWidget::new(client.clone(), tx.clone());
        let config_widget = ConfigWidget::new(tx.clone());
        Self {
            title,
            should_quit: false,
            config,
            input_mode: InputMode::Normal,
            events_widget,
            config_widget,
            selected_widget: Widgets::Events,
            rx,
            tx,
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
                Some(app_evt) = self.rx.recv() => {
                    match app_evt {
                        AppEvent::SetInputMode(mode) => self.input_mode = mode,
                        AppEvent::SetSessionToken(token) => self.set_token(token),
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event) {
        if let Some(key) = event.as_key_press_event() {
            if self.input_mode == InputMode::Normal && key.code == KeyCode::Char('q')
                || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
            {
                self.should_quit = true;
            } else {
                match self.selected_widget {
                    Widgets::Config => self.config_widget.handle_event(event),
                    Widgets::Events => self.events_widget.handle_event(event),
                }
            }
            // match self.input_mode {
            //     InputMode::Normal => match key.code {
            //         KeyCode::Char('q') => {
            //             self.should_quit = true;
            //         }
            //         _ => {}
            //     },
            //     InputMode::Editing => match key.code {
            //         KeyCode::Enter => self.set_token(),
            //         _ => {
            //             self.config.input.handle_event(&event);
            //         }
            //     },
            // }
        }
        // if self.config.session_token.is_none() && self.input_mode == InputMode::Normal {
        //     self.input_mode = InputMode::Editing;
        // }
    }

    fn set_token(&mut self, token: String) {
        self.config.session_token = Some(token);
        self.input_mode = InputMode::Normal;
    }
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self::new("AOC-TUI", None)
    }
}
