use crate::app::App;
use tui_textarea::{Input, Key};

pub async fn handle_event(app: &mut App, input: &Input) {
    match input {
        Input {
            key: Key::Char('j'),
            shift: false,
            ..
        }
        | Input {
            key: Key::Down,
            shift: false,
            ..
        } => {
            app.render_playlist.incr(1);
        }
        Input {
            key: Key::Char('k'),
            shift: false,
            ..
        }
        | Input {
            key: Key::Up,
            shift: false,
            ..
        } => {
            app.render_playlist.incr(-1);
        }
        Input {
            key: Key::Char('J'),
            shift: true,
            ..
        }
        | Input {
            key: Key::Down,
            shift: true,
            ..
        } => {
            app.render_playlist.swap_next();
            app.sync_player_playlist_if_playing();
        }
        Input {
            key: Key::Char('K'),
            shift: true,
            ..
        }
        | Input {
            key: Key::Up,
            shift: true,
            ..
        } => {
            app.render_playlist.swap_prev();
            app.sync_player_playlist_if_playing();
        }
        Input {
            key: Key::Char('d'),
            ..
        } => {
            app.render_playlist.cut(&mut app.clipboard);
            app.sync_player_playlist_if_playing();
        }
        Input {
            key: Key::Char('p'),
            shift: false,
            ..
        } => {
            app.render_playlist.paste(&mut app.clipboard);
            app.sync_player_playlist_if_playing();
        }
        Input {
            key: Key::Char('P'),
            shift: true,
            ..
        } => {
            app.render_playlist.paste_before(&mut app.clipboard);
            app.sync_player_playlist_if_playing();
        }
        Input {
            key: Key::Enter, ..
        } => {
            if app.player.play(app.render_playlist.selected_mut()).await {
                app.sync_player_playlist();
            };
        }
        _ => {}
    }
}
