use crate::app::App;
use tui_textarea::{Input, Key};

pub fn handle_event(app: &mut App, input: &Input) -> bool {
    match input {
        Input {
            key: Key::Char('c'),
            ..
        } => {
            app.should_quit = true;
            true
        }
        Input {
            key: Key::Char('h'),
            ..
        }
        | Input { key: Key::Left, .. } => {
            app.section.incr_x(-1);
            !app.section.is_search()
        }
        Input {
            key: Key::Char('j'),
            ..
        }
        | Input { key: Key::Down, .. } => {
            app.section.incr_y(1);
            true
        }
        Input {
            key: Key::Char('k'),
            ..
        }
        | Input { key: Key::Up, .. } => {
            app.section.incr_y(-1);
            true
        }
        Input {
            key: Key::Char('l'),
            ..
        }
        | Input {
            key: Key::Right, ..
        } => {
            app.section.incr_x(1);
            !app.section.is_search()
        }
        _ => false,
    }
}
