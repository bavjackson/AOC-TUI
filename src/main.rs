mod app;
mod ui;
mod data;

use color_eyre::Result;
use crate::app::terminal::App;

#[tokio::main]
async fn main() -> Result<()> {
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal).await;

    ratatui::restore();

    app_result
}
