// @section SearchResults
use crate::app::App;
use crate::layout::Section;
use crate::playlist::Playlist;
use crate::search::Search;
use crate::select::Select;
use tui_textarea::{Input, Key};
use youtube_api::{SearchResult, Video};

pub async fn handle_event(app: &mut App, input: &Input) {
    match input {
        // @key j | Select next
        Input {
            key: Key::Char('j'),
            ..
        }
        | Input { key: Key::Down, .. } => {
            if let Search::Ok(search) = &mut app.search {
                search.incr(1);
            }
        }
        // @key k | Select previous
        Input {
            key: Key::Char('k'),
            ..
        }
        | Input { key: Key::Up, .. } => {
            if let Search::Ok(search) = &mut app.search {
                search.incr(-1);
            }
        }
        // @key Enter (video) | Add to playlist
        // @key Enter (video) | Play video
        // @key Ctrl+Enter (video) | Play video from start
        // @key Enter (channel) | Expand channel videos
        // @key Enter (playlist) | Load playlist
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
                            if input.ctrl {
                                app.player.play_from_start(video).await;
                            } else {
                                app.player.play(video).await;
                            }
                            app.sync_player_playlist();
                        } else if app.player.playing.is_none() {
                            if input.ctrl {
                                app.player.play_from_start(video).await;
                            } else {
                                app.player.play(video).await;
                            }
                            app.render_playlist.data.push(video.clone());
                            app.sync_player_playlist();
                        } else {
                            app.render_playlist.data.push(video.clone());
                            app.sync_player_playlist_if_playing();
                        }
                    }
                    SearchResult::Playlist(playlist) => {
                        let title =
                            format!("Ⓟ {}", playlist.title.clone().unwrap_or("unknown".into()));

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
                            app.search = Search::Error(format!("Couldn't load '{title}' videos"));
                        }
                    }
                    SearchResult::Channel(channel) => {
                        let title =
                            format!("Ⓒ {}", channel.title.clone().unwrap_or("unknown".into()));

                        if let Some(videos) = channel.videos().await {
                            app.search =
                                Search::Ok(Select::<SearchResult>::from_videos(videos, title));
                        } else {
                            app.search = Search::Error(format!("Couldn't load '{title}' videos"));
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
