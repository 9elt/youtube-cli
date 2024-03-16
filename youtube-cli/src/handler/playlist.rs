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
            if app.render_playlist.title == app.player.playlist_title {
                app.sync_playlists();
            }
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
            if app.render_playlist.title == app.player.playlist_title {
                app.sync_playlists();
            }
        }
        Input {
            key: Key::Char('d'),
            ..
        } => {
            app.render_playlist.cut(&mut app.clipboard);
            if app.render_playlist.title == app.player.playlist_title {
                app.sync_playlists();
            }
        }
        Input {
            key: Key::Char('p'),
            shift: false,
            ..
        } => {
            app.render_playlist.paste(&mut app.clipboard);
            if app.render_playlist.title == app.player.playlist_title {
                app.sync_playlists();
            }
        }
        Input {
            key: Key::Char('P'),
            shift: true,
            ..
        } => {
            app.render_playlist.paste_before(&mut app.clipboard);
            if app.render_playlist.title == app.player.playlist_title {
                app.sync_playlists();
            }
        }
        Input {
            key: Key::Enter, ..
        } => {
            if app.player.play(app.render_playlist.selected_mut()).await {
                app.sync_playlists();
            };
        }
        _ => {}
    }
}
