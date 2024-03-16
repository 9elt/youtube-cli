use crate::app::App;
use tui_textarea::{Input, Key};

pub fn handle_event(app: &mut App, input: &Input) -> bool {
    match input {
        Input {
            key: Key::Char('c'),
            ctrl: true,
            ..
        } => {
            app.should_quit = true;
            true
        }
        Input {
            key: Key::Char('h'),
            ctrl: true,
            ..
        }
        | Input {
            key: Key::Left,
            ctrl: true,
            ..
        } => {
            app.section.incr_x(-1);
            !app.section.is_search()
        }
        Input {
            key: Key::Char('j'),
            ctrl: true,
            ..
        }
        | Input {
            key: Key::Down,
            ctrl: true,
            ..
        } => {
            app.section.incr_y(1);
            true
        }
        Input {
            key: Key::Char('k'),
            ctrl: true,
            ..
        }
        | Input {
            key: Key::Up,
            ctrl: true,
            ..
        } => {
            app.section.incr_y(-1);
            true
        }
        Input {
            key: Key::Char('l'),
            ctrl: true,
            ..
        }
        | Input {
            key: Key::Right,
            ctrl: true,
            ..
        } => {
            app.section.incr_x(1);
            !app.section.is_search()
        }
        _ => false,
    }
}
