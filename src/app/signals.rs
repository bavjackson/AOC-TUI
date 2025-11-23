use crate::ui::states::InputMode;

pub enum AppEvent {
    SetInputMode(InputMode),
    SetSessionToken(String),
}
