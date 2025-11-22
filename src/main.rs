mod app;
mod data;
mod ui;

use crate::app::terminal::App;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let session_token = std::env::var("AOC_SESSION_TOKEN").ok();

    // let client = data::client::Client::new(&session_token)?;
    //
    // let res = client.get_events().await?;
    //
    // println!("{:?}", res);

    let terminal = ratatui::init();
    let app_result = App::new("AOC-TUI", session_token).run(terminal).await;

    ratatui::restore();

    app_result

    // Ok(())
}
