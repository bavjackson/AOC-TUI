mod app;
mod ui;
mod data;

use color_eyre::Result;
use crate::app::terminal::App;

#[tokio::main]
async fn main() -> Result<()> {
    let session_token = std::env::var("AOC_SESSION_TOKEN").ok();
    let terminal = ratatui::init();
    let app_result = App::new("AOC-TUI", session_token).run(terminal).await;

    ratatui::restore();

    app_result
}
