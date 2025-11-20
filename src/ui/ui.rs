
use ratatui::{
    layout:: {
        Constraint, Layout, Rect
    },
    text:: {
        Text
    },
    widgets:: {
        Block,
        Paragraph,
    },
    Frame};

use crate::app::terminal::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let title_block = Block::bordered();
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

    let title = Paragraph::new(app.title).block(title_block);

    frame.render_widget(title, chunks[0]);

    draw_home_screen(frame, app, chunks[1])
}

fn draw_home_screen(frame: &mut Frame, _app: &mut App, area: Rect) {
    frame.render_widget(Text::raw("Home Screen"), area);
}