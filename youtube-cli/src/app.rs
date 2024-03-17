use crate::layout::LayoutSection;
use crate::player::Player;
use crate::playlist::{Closet, Playlist};
use crate::search::{Search, SearchQuery};
use crate::select::Select;
use bincode::{deserialize, serialize};
use directories::ProjectDirs;
use std::fs;
use youtube_api::Video;

pub struct App {
    pub player: Player,
    pub section: LayoutSection,
    pub closet: Select<Playlist>,
    pub render_playlist: Select<Video>,
    pub search_query: SearchQuery,
    pub search: Search,
    pub should_quit: bool,
    pub clipboard: Vec<Video>,
    pub closet_clipboard: Vec<Playlist>,
}

impl App {
    pub async fn load() -> Self {
        let closet = App::load_playlists();

        App {
            player: Player::new().await,
            section: LayoutSection::default(),
            render_playlist: Select::<Video>::from_playlist(closet.selected()),
            closet,
            search_query: SearchQuery::default(),
            search: Search::None,
            should_quit: false,
            clipboard: Vec::new(),
            closet_clipboard: Vec::new(),
        }
    }

    pub fn dump(&mut self) {
        self.update_closet();
        self.dump_playlists().ok();
        self.player.dump_config().ok();
        self.player.timestamps.dump().ok();
    }

    pub fn update_closet(&mut self) {
        let curr_playlist = self
            .closet
            .data
            .iter_mut()
            .find(|p| p.title == self.render_playlist.title);

        if let Some(curr_playlist) = curr_playlist {
            *curr_playlist = Playlist::from_selected(&self.render_playlist);
        }
    }

    pub fn sync_player_playlist(&mut self) {
        self.player.set_playlist(
            self.render_playlist.data.clone(),
            self.render_playlist.title.clone(),
        );
    }

    pub fn sync_player_playlist_if_playing(&mut self) {
        if self.player.playlist_title == self.render_playlist.title {
            self.sync_player_playlist();
        }
    }

    pub fn load_playlists() -> Select<Playlist> {
        if let Some(dir) = ProjectDirs::from("com", "youtube-cli", "youtube-cli") {
            let mut path = dir.cache_dir().to_path_buf();
            path.push("playlists");

            if let Ok(data) = fs::read(&path) {
                if let Ok(closet) = deserialize::<Closet>(&data) {
                    if !closet.playlists.is_empty() {
                        return closet.to_select();
                    }
                }
            }
        }

        Select::new(vec![Playlist::default()])
    }

    pub fn dump_playlists(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(dir) = ProjectDirs::from("com", "youtube-cli", "youtube-cli") {
            let mut path = dir.cache_dir().to_path_buf();

            fs::create_dir_all(&path).expect("create cache directory");

            path.push("playlists");

            let data = serialize(&Closet {
                playlists: self.closet.data.clone(),
                selected: self.closet.selected,
            })?;

            fs::write(&path, data)?;

            Ok(())
        } else {
            Err("could not find cache directory".into())
        }
    }
}
