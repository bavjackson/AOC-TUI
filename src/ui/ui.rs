use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Text,
    widgets::{Block, Paragraph},
};

use crate::app::terminal::App;
use crate::ui::states::{InputMode, LoadingState};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let title_block = Block::bordered();
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

    let title = Paragraph::new(app.title).block(title_block);

    frame.render_widget(title, chunks[0]);

    if app.config.session_token.is_none() {
        app.selected_widget = crate::ui::Widgets::Config;
        draw_token_input(frame, app, chunks[1]);
    } else {
        app.selected_widget = crate::ui::Widgets::Events;
        draw_events_screen(frame, app, chunks[1]);
    }
}

fn draw_token_input(frame: &mut Frame, app: &mut App, area: Rect) {
    if app.input_mode != InputMode::Editing {
        app.input_mode = InputMode::Editing;
    }
    let chunks = Layout::vertical([Constraint::Length(3)]).split(area);
    {
        let area = chunks[0];
        frame.render_widget(&app.config_widget, area);

        let x = app.config_widget.input.visual_cursor() + 1;
        frame.set_cursor_position((area.x + x as u16, area.y + 1));
    }
}

fn draw_events_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    match &app.config.session_token {
        Some(_t) => {
            if app.events_widget.state.read().unwrap().loading_state == LoadingState::Idle {
                app.events_widget.run();
            }
            frame.render_widget(&app.events_widget, area);
        }
        None => {
            frame.render_widget(Text::raw("Token not set"), area);
        }
    }
}
