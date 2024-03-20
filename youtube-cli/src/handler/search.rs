// @section Search
use crate::app::App;
use crate::layout::Section;
use crate::search::Search;
use crate::select::Select;
use tui_textarea::{Input, Key};
use youtube_api::{search, SearchResult};

pub async fn handle_event(app: &mut App, input: &Input) {
    match input {
        // @key Enter | Search
        Input {
            key: Key::Enter, ..
        } => {
            if app.search_query.query.trim().is_empty() {
                return;
            }

            if let Some(results) = search(&app.search_query.query, None).await {
                app.search = Search::Ok(Select::<SearchResult>::with_title(
                    results,
                    "Search Results".into(),
                ));
                app.search_query.clear_selected();
                app.section.set(Section::SearchResults);
            } else {
                app.search = Search::Error(format!("No results for '{}'", app.search_query.query));
            }
        }
        // @key Tab | Select next suggestion
        Input {
            key: Key::Tab,
            shift: false,
            ..
        }
        | Input {
            key: Key::Char('n'),
            ctrl: true,
            ..
        }
        | Input { key: Key::Down, .. } => {
            if !app.search_query.suggestions.data.is_empty() {
                app.search_query.suggestions.incr(1);
            }
            app.search_query.load_selected().await;
        }
        // @key Shift+Tab | Select previous suggestion
        Input {
            key: Key::Tab,
            shift: true,
            ..
        }
        | Input {
            key: Key::Char('N'),
            ctrl: true,
            ..
        }
        | Input { key: Key::Up, .. } => {
            if !app.search_query.suggestions.data.is_empty() {
                app.search_query.suggestions.incr(-1);
            }
            app.search_query.load_selected().await;
        }
        // @key Esc | Cancel suggestions selection
        Input { key: Key::Esc, .. }
        | Input {
            key: Key::Char('q'),
            ctrl: true,
            ..
        } => {
            if !app.search_query.suggestions.data.is_empty() {
                app.search_query.suggestions.set_selected(0);
                app.search_query.load_selected().await;
            }
        }
        _ => {
            app.search_query.input(input.clone()).await;
        }
    }
}
