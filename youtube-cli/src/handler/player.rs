// @section PlayerControls
use crate::app::App;
use tui_textarea::{Input, Key};

pub fn handle_event(app: &mut App, input: &Input) {
    match input {
        // @key Space | Toggle pause/play
        Input {
            key: Key::Char(' '),
            ..
        } => {
            app.player.toggle_pause();
        }
        // @key h | Seek backward 5s
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
        // @key l | Seek forward 5s
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
        // @key H | Seek backward 15s
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
        // @key L | Seek forward 15s
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
        // @key 9 | Volume down
        Input {
            key: Key::Char('9'),
            ..
        } => {
            app.player.set_volume(app.player.volume - 10.0);
        }
        // @key 0 | Volume up
        Input {
            key: Key::Char('0'),
            ..
        } => {
            app.player.set_volume(app.player.volume + 10.0);
        }
        // @key o | Speed down
        Input {
            key: Key::Char('o'),
            ..
        } => {
            app.player.set_speed(app.player.speed - 0.25);
        }
        // @key p | Speed up
        Input {
            key: Key::Char('p'),
            ..
        } => {
            app.player.set_speed(app.player.speed + 0.25);
        }
        // @key m | Toggle mute
        Input {
            key: Key::Char('m'),
            ..
        } => {
            app.player.toggle_mute();
        }
        _ => {}
    }
}
