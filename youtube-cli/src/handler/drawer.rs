use crate::app::App;
use crate::layout::Section;
use crate::select::Select;
use tui_textarea::{Input, Key};
use youtube_api::Video;

pub fn handle_event(app: &mut App, input: &Input) {
    match input {
        Input {
            key: Key::Char('j'),
            ..
        }
        | Input {
            key: Key::Down,
            shift: false,
            ..
        } => {
            app.closet.incr(1);
        }
        Input {
            key: Key::Char('k'),
            ..
        }
        | Input {
            key: Key::Up,
            shift: false,
            ..
        } => {
            app.closet.incr(-1);
        }
        Input {
            key: Key::Char('J'),
            ..
        }
        | Input {
            key: Key::Down,
            shift: true,
            ..
        } => {
            app.closet.swap_next();
        }
        Input {
            key: Key::Char('K'),
            ..
        }
        | Input {
            key: Key::Up,
            shift: true,
            ..
        } => {
            app.closet.swap_prev();
        }
        Input {
            key: Key::Char('d'),
            ..
        } => {
            if app.closet.selected().title != app.player.playlist_title {
                app.closet.cut(&mut app.closet_clipboard);
            }
        }
        Input {
            key: Key::Char('p'),
            ..
        } => {
            app.closet.paste(&mut app.closet_clipboard);
        }
        Input {
            key: Key::Char('P'),
            ..
        } => {
            app.closet.paste_before(&mut app.closet_clipboard);
        }
        Input {
            key: Key::Enter, ..
        } => {
            app.update_closet();

            app.render_playlist = Select::<Video>::from_playlist(app.closet.selected());
            app.section.set(Section::Playlist);
        }
        _ => {}
    }
}
