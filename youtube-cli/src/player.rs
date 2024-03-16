use bincode::{deserialize, serialize};
use directories::ProjectDirs;
use libmpv::{FileState, Mpv};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fs;
use youtube_api::Video;

use crate::util::hash;

const TIMESTAMPS_CAP: usize = 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reload {
    pub playing: Option<Video>,
    pub playing_id: Option<u64>,
    pub playlist: Vec<Video>,
    pub playlist_title: String,
    pub volume: f64,
    pub speed: f64,
    pub is_paused: bool,
}

pub struct Player {
    client: Mpv,
    pub playlist: Vec<Video>,
    pub playlist_title: String,
    pub playing: Option<Video>,
    pub playing_id: Option<u64>,
    pub timestamps: Timestamps,
    pub time: f64,
    pub duration: f64,
    pub volume: f64,
    pub speed: f64,
    pub is_muted: bool,
    pub is_loaded: bool,
    pub is_loading: bool,
    pub is_paused: bool,
    pub is_done: bool,
}

impl Player {
    pub async fn new() -> Self {
        let client = Mpv::new().expect("could not start mpv");

        let timestamps = Timestamps::load();

        let config = Self::load_config();

        let mut player = Player {
            client,
            timestamps,
            playlist: config.playlist,
            playlist_title: config.playlist_title,
            playing_id: config.playing_id,
            playing: config.playing.clone(),
            time: 0.0,
            duration: 0.0,
            volume: config.volume,
            speed: config.speed,
            is_muted: false,
            is_loaded: false,
            is_loading: false,
            is_paused: config.is_paused,
            is_done: false,
        };

        player.reload().await;

        player
    }

    pub fn set_playlist(&mut self, playlist: Vec<Video>, title: String) {
        self.playlist = playlist;
        self.playlist_title = title;
    }

    pub async fn reload(&mut self) -> bool {
        if let Some(video) = &mut self.playing {
            let args = format!(
                "start={}",
                self.timestamps.get(self.playing_id.unwrap_or(0)) as u64,
            );

            let url = match video.url().await {
                Some(url) => url,
                None => {
                    self.playing_id = None;
                    self.playing = None;
                    return false;
                }
            };

            if self
                .client
                .playlist_load_files(&[(url, FileState::Replace, Some(&args))])
                .is_err()
            {
                self.playing_id = None;
                self.playing = None;
                return false;
            }

            self.is_loaded = false;

            if self.is_paused {
                self.pause();
            }

            return true;
        }

        false
    }

    pub async fn play(&mut self, video: &mut Video) -> bool {
        let id = hash(&video.video_id);

        if self.playing_id == Some(id) && !self.is_done {
            return true;
        }

        let url = match video.url().await {
            Some(url) => url,
            None => return false,
        };

        let args = format!("start={}", self.timestamps.get(id) as u64);

        if self.is_paused {
            self.resume();
            self.is_paused = false;
        }

        self.is_loaded = false;
        self.is_done = false;

        self.time = 0.0;
        self.duration = 0.0;

        if self
            .client
            .playlist_load_files(&[(url, FileState::Replace, Some(&args))])
            .is_err()
        {
            self.playing_id = None;
            self.playing = None;
            false
        } else {
            self.playing_id = Some(id);
            self.playing = Some(video.clone());
            true
        }
    }

    pub fn set_volume_t(&mut self, volume: f64) -> bool {
        let volume = volume.min(100.0).max(0.0);

        self.client.set_property("volume", volume).is_ok()
    }

    pub fn set_volume(&mut self, volume: f64) -> bool {
        let volume = volume.min(100.0).max(0.0);

        if self.client.set_property("volume", volume).is_ok() {
            self.volume = volume;
            true
        } else {
            false
        }
    }

    pub fn set_speed(&mut self, speed: f64) -> bool {
        let speed = speed.min(2.5).max(0.25);

        if self.client.set_property("speed", speed).is_ok() {
            self.speed = speed;
            true
        } else {
            false
        }
    }

    pub fn toggle_mute(&mut self) -> bool {
        self.is_muted = !self.is_muted;
        self.set_volume_t(if self.is_muted { 0.0 } else { self.volume })
    }

    pub fn pause(&mut self) -> bool {
        self.is_paused = self.client.pause().is_ok();
        self.is_paused
    }

    pub fn resume(&mut self) -> bool {
        self.is_paused = self.client.unpause().is_err();
        !self.is_paused
    }

    pub fn toggle_pause(&mut self) -> bool {
        if self.is_paused {
            self.resume()
        } else {
            self.pause()
        }
    }

    pub fn seek_relative(&self, time: f64) -> bool {
        self.client
            .command("seek", &[&format!("{}", time), "relative"])
            .is_ok()
    }

    pub fn next(&mut self) -> Option<Video> {
        let found_index = self
            .playlist
            .iter()
            .position(|v| v == self.playing.as_ref().unwrap());

        if let Some(index) = found_index {
            if index < self.playlist.len() - 2 {
                return Some(self.playlist[index + 1].clone());
            }
        }

        None
    }

    pub async fn update(&mut self) {
        if let Some(playing_id) = self.playing_id {
            let time = self.client.get_property("time-pos").unwrap_or(0.0);
            let duration = self.client.get_property("duration").unwrap_or(0.0);

            self.is_loading = time == self.time && !self.is_paused;

            self.time = time;
            self.duration = duration;

            self.timestamps.set(playing_id, self.time);

            if self.duration > 0.0 && self.time > 0.0 {
                self.is_loaded = true;
            }

            if self.is_loaded && self.duration == 0.0 {
                self.is_loaded = false;
                self.is_done = true;
            }

            if self.is_done && self.playing.is_some() {
                if let Some(mut video) = self.next() {
                    self.play(&mut video).await;
                }
            }
        }
    }

    pub fn ratio(&self) -> f64 {
        if self.duration > 0.0 {
            (self.time / self.duration).min(1.0).max(0.0)
        } else {
            0.0
        }
    }

    fn load_config() -> Reload {
        if let Some(dir) = ProjectDirs::from("com", "youtube-cli", "youtube-cli") {
            let mut path = dir.cache_dir().to_path_buf();
            path.push("player");

            if let Ok(data) = fs::read(&path) {
                if let Ok(player_reload) = deserialize::<Reload>(&data) {
                    return player_reload;
                }
            }
        }

        Reload {
            playing: None,
            playing_id: None,
            playlist: Vec::new(),
            playlist_title: String::new(),
            volume: 100.0,
            speed: 1.0,
            is_paused: false,
        }
    }

    pub fn dump_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(dir) = ProjectDirs::from("com", "youtube-cli", "youtube-cli") {
            let mut path = dir.cache_dir().to_path_buf();

            fs::create_dir_all(&path).expect("create cache directory");

            path.push("player");

            let data = serialize(&Reload {
                playing: self.playing.clone(),
                playing_id: self.playing_id,
                playlist: self.playlist.clone(),
                playlist_title: self.playlist_title.clone(),
                volume: self.volume,
                speed: self.speed,
                is_paused: self.is_paused || self.is_done,
            })?;

            fs::write(&path, data)?;

            Ok(())
        } else {
            Err("could not find cache directory".into())
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamps {
    flag: usize,
    #[serde_as(as = "[_; TIMESTAMPS_CAP]")]
    buffer: [Timestamp; TIMESTAMPS_CAP],
}

impl Timestamps {
    fn load() -> Self {
        if let Some(dir) = ProjectDirs::from("com", "youtube-cli", "youtube-cli") {
            let mut path = dir.cache_dir().to_path_buf();
            path.push("timestamps");

            if let Ok(data) = fs::read(&path) {
                if let Ok(timestamps) = deserialize(&data) {
                    return timestamps;
                }
            }
        }

        Timestamps {
            flag: 0,
            buffer: [Timestamp::default(); TIMESTAMPS_CAP],
        }
    }

    pub fn dump(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(dir) = ProjectDirs::from("com", "youtube-cli", "youtube-cli") {
            let mut path = dir.cache_dir().to_path_buf();

            fs::create_dir_all(&path).expect("create cache directory");

            path.push("timestamps");

            let data = serialize(&self)?;

            fs::write(&path, data)?;

            Ok(())
        } else {
            Err("could not find cache directory".into())
        }
    }

    fn get(&self, id: u64) -> f64 {
        self.buffer
            .iter()
            .find(|t| t.id == id && t.timestamp > 0.0)
            .map(|t| t.timestamp)
            .unwrap_or(0.0)
    }

    fn set(&mut self, id: u64, timestamp: f64) {
        if let Some(t) = self.buffer.iter_mut().find(|t| t.id == id) {
            t.timestamp = timestamp;
        } else {
            self.buffer[self.flag] = Timestamp { id, timestamp };

            if self.flag == TIMESTAMPS_CAP - 1 {
                self.flag = 0;
            } else {
                self.flag += 1;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Timestamp {
    id: u64,
    timestamp: f64,
}

impl Copy for Timestamp {}

impl Default for Timestamp {
    fn default() -> Self {
        Timestamp {
            id: 0,
            timestamp: -1.0,
        }
    }
}
