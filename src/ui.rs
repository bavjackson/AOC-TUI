pub mod config;
pub mod events;
pub mod states;
pub mod ui;

#[derive(Debug, Clone)]
pub enum Widgets {
    Events,
    Config,
}
