use crate::playlist::Playlist;
use ratatui::widgets::ListState;
use youtube_api::{SearchResult, Video};

#[derive(Debug, Clone)]
pub struct Select<T> {
    pub title: String,
    pub data: Vec<T>,
    pub selected: usize,
    pub list_state: ListState,
}

impl Select<SearchResult> {
    pub fn with_title(results: Vec<SearchResult>, title: String) -> Self {
        let mut select = Select::new(results);
        select.title = title;
        select
    }

    pub fn from_videos(videos: &[Video], title: String) -> Self {
        let mut select = Select::new(videos.iter().cloned().map(SearchResult::Video).collect());
        select.title = title;
        select
    }
}

impl Select<Video> {
    pub fn from_playlist(playlist: &Playlist) -> Self {
        let mut select = Select::new(playlist.videos.clone());
        select.title = playlist.title.clone();
        select
    }
}

impl<T> Select<T> {
    pub fn new(data: Vec<T>) -> Self {
        let mut select = Self {
            title: String::new(),
            data,
            selected: 0,
            list_state: ListState::default(),
        };
        select.list_state.select(Some(0));
        select
    }

    pub fn set_selected(&mut self, selected: usize) {
        self.selected = selected;
        self.list_state.select(Some(self.selected));
    }

    pub fn selected(&self) -> &T {
        &self.data[self.selected]
    }

    pub fn selected_mut(&mut self) -> &mut T {
        &mut self.data[self.selected]
    }

    pub fn incr(&mut self, incr: isize) {
        if self.data.len() > 1 {
            self.set_selected(
                (self.selected as isize + incr)
                    .max(0)
                    .min(self.data.len() as isize - 1) as usize,
            );
        }
    }

    pub fn swap_next(&mut self) {
        if self.data.len() > 1 && self.selected < self.data.len() - 1 {
            self.data.swap(self.selected, self.selected + 1);
            self.incr(1);
        }
    }

    pub fn swap_prev(&mut self) {
        if self.data.len() > 1 && self.selected > 0 {
            self.data.swap(self.selected, self.selected - 1);
            self.incr(-1);
        }
    }

    pub fn cut(&mut self, clipboard: &mut Vec<T>) {
        if !self.data.is_empty() {
            clipboard.push(self.data.remove(self.selected));

            if self.selected == self.data.len() && self.selected > 0 {
                self.set_selected(self.selected - 1);
            }
        }
    }

    pub fn paste(&mut self, clipboard: &mut Vec<T>) {
        if let Some(item) = clipboard.pop() {
            if self.data.is_empty() {
                self.data.push(item);
            } else {
                self.data.insert(self.selected + 1, item);
                self.incr(1);
            }
        }
    }

    pub fn paste_before(&mut self, clipboard: &mut Vec<T>) {
        if let Some(item) = clipboard.pop() {
            if self.data.is_empty() {
                self.data.push(item);
            } else {
                self.data.insert(self.selected, item);
            }
        }
    }
}
