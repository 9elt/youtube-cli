use crate::app::App;
use tui_textarea::{Input, Key};

pub fn handle_event(app: &mut App, input: &Input) {
    match input {
        Input {
            key: Key::Char(' '),
            ..
        } => {
            app.player.toggle_pause();
        }
        Input {
            key: Key::Char('h'),
            ..
        }
        | Input {
            key: Key::Left,
            shift: false,
            ..
        } => {
            app.player.seek_relative(-5.0);
        }
        Input {
            key: Key::Char('l'),
            ..
        }
        | Input {
            key: Key::Right,
            shift: false,
            ..
        } => {
            app.player.seek_relative(5.0);
        }
        Input {
            key: Key::Char('H'),
            ..
        }
        | Input {
            key: Key::Left,
            shift: true,
            ..
        } => {
            app.player.seek_relative(-15.0);
        }
        Input {
            key: Key::Char('L'),
            ..
        }
        | Input {
            key: Key::Right,
            shift: true,
            ..
        } => {
            app.player.seek_relative(15.0);
        }
        Input {
            key: Key::Char('9'),
            ..
        } => {
            app.player.set_volume(app.player.volume - 10.0);
        }
        Input {
            key: Key::Char('0'),
            ..
        } => {
            app.player.set_volume(app.player.volume + 10.0);
        }
        Input {
            key: Key::Char('o'),
            ..
        } => {
            app.player.set_speed(app.player.speed - 0.25);
        }
        Input {
            key: Key::Char('p'),
            ..
        } => {
            app.player.set_speed(app.player.speed + 0.25);
        }
        Input {
            key: Key::Char('m'),
            ..
        } => {
            app.player.toggle_mute();
        }
        _ => {}
    }
}
