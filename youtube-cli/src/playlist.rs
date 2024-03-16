use crate::select::Select;
use serde::{Deserialize, Serialize};
use youtube_api::Video;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub title: String,
    pub videos: Vec<Video>,
}

impl Default for Playlist {
    fn default() -> Self {
        Self {
            title: "Queue".to_string(),
            videos: Vec::new(),
        }
    }
}

impl Playlist {
    pub fn from_selected(selected: &Select<Video>) -> Self {
        Self {
            title: selected.title.clone(),
            videos: selected.data.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Closet {
    pub playlists: Vec<Playlist>,
    pub selected: usize,
}

impl Closet {
    pub fn to_select(&self) -> Select<Playlist> {
        let mut select = Select::new(self.playlists.clone());
        select.set_selected(self.selected);
        select
    }
}
