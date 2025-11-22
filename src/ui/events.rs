use crate::data::client::Client;
use crate::data::event::AOCEvent;
use crate::ui::states::LoadingState;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, HighlightSpacing, Row, StatefulWidget, Table, TableState, Widget};
use std::sync::{Arc, RwLock};

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
}

impl EventsWidget {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            state: Arc::new(RwLock::new(EventsListState {
                events: Vec::new(),
                loading_state: LoadingState::Idle,
                table_state: TableState::default(),
            })),
            client,
        }
    }
    pub fn run(&self) {
        let this = self.clone();
        tokio::spawn(this.fetch_events());
    }
    async fn fetch_events(self) {
        self.set_loading_state(LoadingState::Loading);
        let res = self.client.get_events().await;

        match res {
            Ok(events) => self.on_load(events),
            Err(_) => self.on_err(),
        }
    }

    fn on_load(&self, data: Vec<AOCEvent>) {
        let mut state = self.state.write().unwrap();
        state.events = data;
        state.loading_state = LoadingState::Loaded;
    }

    fn on_err(&self) {
        self.set_loading_state(LoadingState::Error("Failed to fetch events".to_string()));
    }

    fn set_loading_state(&self, state: LoadingState) {
        self.state.write().unwrap().loading_state = state;
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
