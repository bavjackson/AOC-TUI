use crate::data::event::AOCEvent;
use crate::ui::states::InputMode;

pub enum AppEvent {
    SetInputMode(InputMode),
    SetSessionToken(String),
    FetchEvents,
    FetchEventsError(String),
    SetEvents(Vec<AOCEvent>),
}
