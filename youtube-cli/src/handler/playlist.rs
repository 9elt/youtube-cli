// @section Playlist
use crate::app::App;
use tui_textarea::{Input, Key};

pub async fn handle_event(app: &mut App, input: &Input) {
    match input {
        // @key j | Select next
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
        // @key k | Select previous
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
        // @key J | Move selected down
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
        // @key K | Move selected up
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
        // @key D | Cut
        Input {
            key: Key::Char('d'),
            ..
        } => {
            app.render_playlist.cut(&mut app.clipboard);
            app.sync_player_playlist_if_playing();
        }
        // @key P | Paste
        Input {
            key: Key::Char('p'),
            shift: false,
            ..
        } => {
            app.render_playlist.paste(&mut app.clipboard);
            app.sync_player_playlist_if_playing();
        }
        // @key P | Paste before
        Input {
            key: Key::Char('P'),
            shift: true,
            ..
        } => {
            app.render_playlist.paste_before(&mut app.clipboard);
            app.sync_player_playlist_if_playing();
        }
        // @key Enter | Play video
        Input {
            key: Key::Enter,
            ctrl: false,
            ..
        } => {
            if app.player.play(app.render_playlist.selected_mut()).await {
                app.sync_player_playlist();
            };
        }
        // @key Ctrl+Enter | Play video from start
        Input {
            key: Key::Enter,
            ctrl: true,
            ..
        } => {
            if app
                .player
                .play_from_start(app.render_playlist.selected_mut())
                .await
            {
                app.sync_player_playlist();
            };
        }
        _ => {}
    }
}
