use super::browse::{browse_channel_videos, browse_playlist_videos};
use super::player::get_video_url;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub enum SearchResult {
    Video(Video),
    Channel(Channel),
    Playlist(Playlist),
}

impl SearchResult {
    pub fn is_video(&self) -> bool {
        if let SearchResult::Video(_) = self {
            return true;
        }
        false
    }

    pub fn video(&self) -> Option<Video> {
        if let SearchResult::Video(v) = self {
            return Some(v.clone());
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub video_id: String,
    pub title: Option<String>,
    pub channel: Option<String>,
    pub published: Option<String>,
    pub views: Option<String>,
    pub duration: Option<String>,

    pub _url: Option<String>,
    pub _url_updated_at: Option<SystemTime>,
}

impl PartialEq for Video {
    fn eq(&self, other: &Self) -> bool {
        self.video_id == other.video_id
    }
}

impl Video {
    fn is_url_valid(&self) -> bool {
        if self._url.is_some() {
            if let Some(at) = &self._url_updated_at {
                return at.elapsed().unwrap_or(Duration::from_secs(3601)).as_secs() < 3600;
            }
        }
        false
    }

    pub async fn url(&mut self) -> &Option<String> {
        if self.is_url_valid() {
            return &self._url;
        }

        self._url = get_video_url(&self.video_id).await;
        self._url_updated_at = Some(SystemTime::now());

        &self._url
    }
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub channel_id: String,
    pub at: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub subs: Option<String>,

    pub _videos: Option<Vec<Video>>,
}

impl Channel {
    pub async fn videos(&mut self) -> &mut Option<Vec<Video>> {
        self._videos = browse_channel_videos(&self.channel_id).await;
        // channel is none in browse channel videos
        if let Some(videos) = &mut self._videos {
            for v in videos {
                v.channel = self.title.clone();
            }
        }
        &mut self._videos
    }
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub playlist_id: String,
    pub browse_id: String,
    pub title: Option<String>,
    pub video_count: Option<i64>,

    pub _videos: Option<Vec<Video>>,
}

impl Playlist {
    pub async fn videos(&mut self) -> &mut Option<Vec<Video>> {
        self._videos = browse_playlist_videos(&self.browse_id).await;
        &mut self._videos
    }
}
