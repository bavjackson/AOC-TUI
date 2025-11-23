use crate::app::signals::AppEvent;
use crate::data::client::Client;
use crate::data::event::AOCEvent;
use crate::ui::states::LoadingState;
use crossterm::event::{Event, KeyCode};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, HighlightSpacing, Row, StatefulWidget, Table, TableState, Widget};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone)]
pub struct EventsListState {
    pub events: Vec<AOCEvent>,
    pub loading_state: LoadingState,
    pub table_state: TableState,
}
#[derive(Debug, Clone)]
pub struct EventsWidget {
    pub state: Arc<RwLock<EventsListState>>,
    pub client: Arc<Client>,
    pub sender: mpsc::UnboundedSender<AppEvent>,
}

impl EventsWidget {
    pub fn new(client: Arc<Client>, sender: UnboundedSender<AppEvent>) -> Self {
        Self {
            state: Arc::new(RwLock::new(EventsListState {
                events: Vec::new(),
                loading_state: LoadingState::Idle,
                table_state: TableState::default(),
            })),
            client,
            sender,
        }
    }
    pub fn run(&self) {
        let this = self.clone();
        tokio::spawn(this.fetch_events());
    }

    pub fn handle_event(&self, event: &Event) {
        if let Some(key) = event.as_key_press_event() {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.scroll_down();
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.scroll_up();
                }
                _ => {}
            }
        }
    }

    async fn fetch_events(self) {
        self.set_loading_state(LoadingState::Loading);
        self.sender.send(AppEvent::FetchEvents).unwrap();
    }

    pub fn set_events(&self, data: Vec<AOCEvent>) {
        let mut state = self.state.write().unwrap();
        state.events = data;
        state.loading_state = LoadingState::Loaded;
        if !state.events.is_empty() {
            state.table_state.select(Some(0));
        }
    }

    fn on_err(&self) {
        self.set_loading_state(LoadingState::Error("Failed to fetch events".to_string()));
    }

    fn set_loading_state(&self, state: LoadingState) {
        self.state.write().unwrap().loading_state = state;
    }

    fn scroll_down(&self) {
        self.state.write().unwrap().table_state.scroll_down_by(1)
    }

    fn scroll_up(&self) {
        self.state.write().unwrap().table_state.scroll_up_by(1)
    }
}

impl Widget for &EventsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.state.write().unwrap();

        let loading_state = Line::from(format!("Loading: {:?}", state.loading_state));
        let block = Block::bordered()
            .title("Events")
            .title(loading_state)
            .title_bottom("j/k to scroll, q to quit");

        let rows = state.events.iter();
        let widths = [
            Constraint::Length(7),
            Constraint::Length(4),
            Constraint::Length(5),
        ];

        let table = Table::new(rows, widths)
            .block(block)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">>")
            .row_highlight_style(Style::new());

        StatefulWidget::render(table, area, buf, &mut state.table_state)
    }
}

impl From<&AOCEvent> for Row<'_> {
    fn from(event: &AOCEvent) -> Self {
        let event = event.clone();
        let rows = vec![event.label, event.stars, event.out_of];

        Row::new(rows)
    }
}
