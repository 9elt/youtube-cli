mod drawer;
mod generic;
mod player;
mod playlist;
mod results;
mod search;

use crate::app::App;
use crate::layout::Section;
use crossterm::event::{self, KeyEvent};
use tui_textarea::Input;

pub async fn handle_event(key: KeyEvent, app: &mut App) {
    if key.kind != event::KeyEventKind::Press {
        return;
    }

    let input: Input = key.into();

    if key.modifiers.contains(event::KeyModifiers::CONTROL) && generic::handle_event(app, &input) {
        return;
    }

    match app.section.id {
        Section::Search => search::handle_event(app, &input).await,
        Section::Playlist => playlist::handle_event(app, &input).await,
        Section::SearchResults => results::handle_event(app, &input).await,
        Section::Drawer => drawer::handle_event(app, &input),
        Section::PlayerControls => player::handle_event(app, &input),
    }
}
