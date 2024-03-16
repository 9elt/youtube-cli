use crate::app::App;
use crate::layout::Section;
use crate::playlist::Playlist;
use crate::search::Search;
use crate::select::Select;
use tui_textarea::{Input, Key};
use youtube_api::{SearchResult, Video};

pub async fn handle_event(app: &mut App, input: &Input) {
    match input {
        Input {
            key: Key::Char('j'),
            ..
        }
        | Input { key: Key::Down, .. } => {
            if let Search::Ok(search) = &mut app.search {
                search.incr(1);
            }
        }
        Input {
            key: Key::Char('k'),
            ..
        }
        | Input { key: Key::Up, .. } => {
            if let Search::Ok(search) = &mut app.search {
                search.incr(-1);
            }
        }
        Input {
            key: Key::Enter, ..
        } => {
            let mut selected = if let Search::Ok(search) = &mut app.search {
                Some(search.selected().clone())
            } else {
                None
            };

            if let Some(selected) = &mut selected {
                match selected {
                    SearchResult::Video(video) => {
                        if app.render_playlist.data.iter().any(|v| v == video) {
                            app.player.play(video).await;
                            app.sync_playlists();
                        } else if app.player.playing.is_none() {
                            app.player.play(video).await;
                            app.sync_playlists();
                            app.render_playlist.data.push(video.clone());
                        } else {
                            app.render_playlist.data.push(video.clone());
                        }
                    }
                    SearchResult::Playlist(playlist) => {
                        let title =
                            format!("Ⓟ {}", playlist.title.clone().unwrap_or("Videos".into()));

                        if let Some(videos) = playlist.videos().await {
                            if app.render_playlist.title == title {
                                app.section.set(Section::Playlist);
                                return;
                            }

                            let found = app.closet.data.iter_mut().find(|p| p.title == title);

                            if let Some(found) = found {
                                app.render_playlist = Select::<Video>::from_playlist(found);
                                app.section.set(Section::Playlist);
                                return;
                            }

                            let create = Playlist {
                                title: title.clone(),
                                videos: videos.clone(),
                            };

                            app.render_playlist = Select::<Video>::from_playlist(&create);

                            app.closet.paste(&mut vec![create]);

                            app.section.set(Section::Playlist);
                        } else {
                            app.search = Search::Error(format!(
                                "Couldn't load '{}' videos",
                                playlist.title.clone().unwrap_or("playlist".into())
                            ));
                        }
                    }
                    SearchResult::Channel(channel) => {
                        let title =
                            format!("Ⓒ {}", channel.title.clone().unwrap_or("Videos".into()));

                        if let Some(videos) = channel.videos().await {
                            app.search =
                                Search::Ok(Select::<SearchResult>::from_videos(videos, title));
                        } else {
                            app.search = Search::Error(format!(
                                "Couldn't load '{}' videos",
                                channel.title.clone().unwrap_or("channel".into())
                            ));
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
