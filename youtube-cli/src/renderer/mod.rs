mod body;
mod footer;
mod header;
mod util;

use crate::app::App;
use body::render_body;
use footer::render_footer;
use header::render_header;
use ratatui::{prelude::*, Frame};

pub fn render(frame: &mut Frame, app: &mut App) {
    let has_suggestions = app.section.is_search() && !app.search_query.suggestions.data.is_empty();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            if has_suggestions {
                Constraint::Length((app.search_query.suggestions.data.len() + 3).min(18) as u16)
            } else {
                Constraint::Length(3)
            },
            Constraint::Percentage(100),
            Constraint::Min(9),
        ])
        .split(frame.size());

    render_header(frame, app, layout[0]);

    render_body(frame, app, layout[1]);

    render_footer(frame, app, layout[2]);
}
