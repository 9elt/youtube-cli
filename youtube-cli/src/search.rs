use crate::select::Select;
use std::time::{Duration, SystemTime};
use tui_textarea::{CursorMove, Input, TextArea};
use youtube_api::{suggestions, SearchResult};

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub query: String,
    pub suggestions: Select<String>,
    pub textarea: TextArea<'static>,
    last_search: SystemTime,
    suggestions_synced: bool,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            query: String::new(),
            suggestions: Select::new(Vec::new()),
            textarea: TextArea::default(),
            last_search: SystemTime::now(),
            suggestions_synced: false,
        }
    }
}

impl SearchQuery {
    pub async fn input(&mut self, key: Input) {
        self.textarea.input(key);
        self.query = self.textarea.lines().join("\n");
        self.last_search = SystemTime::now();
        self.suggestions_synced = false;
    }

    pub async fn update_suggestions(&mut self) {
        if self.suggestions_synced
            || self.last_search.elapsed().unwrap_or(Duration::from_secs(0))
                < Duration::from_millis(300)
        {
            return;
        }
        self.suggestions = Select::new(suggestions(&self.query).await.unwrap_or_default());

        // NODE: insert original query as first suggestion
        if !self.suggestions.data.is_empty() {
            self.suggestions.data.insert(0, self.query.clone());
        }

        self.suggestions_synced = true;
    }

    pub fn clear_selected(&mut self) {
        self.suggestions = Select::new(Vec::new());
    }

    pub async fn load_selected(&mut self) {
        if !self.suggestions.data.is_empty() {
            self.query = self.suggestions.selected().clone();
            self.textarea = TextArea::from(vec![self.suggestions.selected().clone()]);
            self.textarea.move_cursor(CursorMove::End);
            // self.suggestions = Select::new(suggestions(&self.query).await.unwrap_or_default());
        }
    }
}

#[derive(Debug, Clone)]
pub enum Search {
    None,
    Ok(Select<SearchResult>),
    Error(String),
}
