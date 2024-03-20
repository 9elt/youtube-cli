// @section None
use crate::app::App;
use tui_textarea::{Input, Key};

pub fn handle_event(app: &mut App, input: &Input) -> bool {
    match input {
        // @key Ctrl+c | Quit
        Input {
            key: Key::Char('c'),
            ctrl: true,
            ..
        } => {
            app.should_quit = true;
            true
        }
        // @key Ctrl+h | Left section
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
        // @key Ctrl+j | Below section
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
        // @key Ctrl+k | Above section
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
        // @key Ctrl+l | Right section
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
        // @key Ctrl+t | Toggle keymaps
        Input {
            key: Key::Char('t'),
            ctrl: true,
            ..
        } => {
            app.show_keymaps = !app.show_keymaps;
            true
        }
        _ => false,
    }
}
