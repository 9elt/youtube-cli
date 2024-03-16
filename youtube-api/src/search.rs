use super::api::youtubei;
use super::structs::{Channel, Playlist, SearchResult, Video};
use crate::helper::jun;
use serde_json::{json, Value};

pub enum SearchFilter {
    Video,
    Channel,
    Playlist,
}

impl SearchFilter {
    fn as_str(&self) -> &str {
        match self {
            SearchFilter::Video => "EgIQAQ%3D%3D",
            SearchFilter::Channel => "EgIQAg%3D%3D",
            SearchFilter::Playlist => "EgIQAw%3D%3D",
        }
    }
}

pub async fn search(query: &str, filter: Option<SearchFilter>) -> Option<Vec<SearchResult>> {
    let any = youtubei(
        "/search",
        if let Some(filter) = filter {
            json!({
                "query": query,
                "params": filter.as_str()
            })
        } else {
            json!({ "query": query })
        },
    )
    .await
    .ok()?;

    jun!(any
        => "contents"
        => "twoColumnSearchResultsRenderer"
        => "primaryContents"
        => "sectionListRenderer"
        => "contents"
        => flat_map
        => "itemSectionRenderer"
        => "contents"
        => map
        => to parse_search_contents
    )
}

fn parse_search_contents(value: &Value) -> Option<SearchResult> {
    jun!(value
        => "videoRenderer"
        => to parse_video_renderer
    )
    .or(jun!(value
        => "channelRenderer"
        => to parse_channel_renderer
    ))
    .or(jun!(value
        => "playlistRenderer"
        => to parse_playlist_renderer
    ))
}

fn parse_video_renderer(value: &Value) -> Option<SearchResult> {
    Some(SearchResult::Video(Video {
        video_id: jun!(value
            => "videoId"
            => to_string
        )?,
        title: jun!(value
            => "title"
            => "runs"
            => 0
            => "text"
            => to_string
        ),
        channel: jun!(value
            => "longBylineText"
            => "runs"
            => 0
            => "text"
            => to_string
        ),
        published: jun!(value
            => "publishedTimeText"
            => "simpleText"
            => to_string
        ),
        views: jun!(value
            => "shortViewCountText"
            => "simpleText"
            => to_string
        ),
        duration: jun!(value
            => "lengthText"
            => "simpleText"
            => to_string
        ),
        _url: None,
        _url_updated_at: None,
    }))
}

fn parse_channel_renderer(value: &Value) -> Option<SearchResult> {
    Some(SearchResult::Channel(Channel {
        channel_id: jun!(value
            => "channelId"
            => to_string
        )?,
        at: jun!(value
            => "subscriberCountText"
            => "simpleText"
            => to_string
        ),
        title: jun!(value
            => "title"
            => "simpleText"
            => to_string
        ),
        description: jun!(value
            => "descriptionSnippet"
            => "runs"
            => 0
            => "text"
            => to_string
        ),
        subs: jun!(value
            => "videoCountText"
            => "simpleText"
            => to_string
        ),
        _videos: None,
    }))
}

fn parse_playlist_renderer(value: &Value) -> Option<SearchResult> {
    Some(SearchResult::Playlist(Playlist {
        playlist_id: jun!(value
            => "playlistId"
            => to_string
        )?,
        browse_id: jun!(value
            => "viewPlaylistText"
            => "runs"
            => 0
            => "navigationEndpoint"
            => "browseEndpoint"
            => "browseId"
            => to_string
        )?,
        title: jun!(value
            => "title"
            => "simpleText"
            => to_string
        ),
        video_count: jun!(value
            => "videoCount"
        )
        .and_then(|value| match value {
            Value::String(s) => Some(s.parse().ok()?),
            Value::Number(n) => Some(n.as_i64()?),
            _ => None,
        }),
        _videos: None,
    }))
}
