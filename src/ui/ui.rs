use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Text,
    widgets::{Block, Paragraph},
};
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::app::terminal::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let title_block = Block::bordered();
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

    let title = Paragraph::new(app.title).block(title_block);

    frame.render_widget(title, chunks[0]);

    if app.config.session_token.is_none() {
        draw_token_input(frame, app, chunks[1]);
    } else {
        draw_home_screen(frame, app, chunks[1]);
    }
}

fn draw_token_input(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([Constraint::Length(3)]).split(area);
    {
        let area = chunks[0];
        let input = Paragraph::new(app.config.input.value())
            .block(Block::bordered().title("Enter your session token"));
        frame.render_widget(input, area);

        let x = app.config.input.visual_cursor() + 1;
        frame.set_cursor_position((area.x + x as u16, area.y + 1));
    }
}

fn draw_home_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(Text::raw("Home Screen"), area);
    match &app.config.session_token {
        Some(t) => {
            frame.render_widget(Text::raw(format!("Current Token: {}", t)), area);
        }
        None => {
            frame.render_widget(Text::raw("Token not set"), area);
        }
    }
}
