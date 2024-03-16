use super::api::youtubei;
use super::structs::Video;
use crate::helper::jun;
use serde_json::{json, Value};

const BROWSE_CHANNEL_VIDEOS: &str = "EgZ2aWRlb3PyBgQKAjoA";

pub async fn browse_channel_videos(browse_id: &str) -> Option<Vec<Video>> {
    let any = youtubei(
        "/browse",
        json!({
            "browseId": browse_id,
            "params": BROWSE_CHANNEL_VIDEOS
        }),
    )
    .await
    .ok()?;

    jun!(any
        => "contents"
        => "twoColumnBrowseResultsRenderer"
        => "tabs"
        => flat_map
        => "tabRenderer"
        => "content"
        => "richGridRenderer"
        => "contents"
        => map
        => "richItemRenderer"
        => "content"
        => "videoRenderer"
        => to parse_video_renderer
    )
}

fn parse_video_renderer(renderer: &Value) -> Option<Video> {
    Some(Video {
        video_id: jun!(renderer
            => "videoId"
            => to_string
        )?,
        title: jun!(renderer
            => "title"
            => "runs"
            => 0
            => "text"
            => to_string
        ),
        published: jun!(renderer
            => "publishedTimeText"
            => "simpleText"
            => to_string
        ),
        views: jun!(renderer
            => "shortViewCountText"
            => "simpleText"
            => to_string
        ),
        duration: jun!(renderer
            => "lengthText"
            => "simpleText"
            => to_string
        ),
        channel: None,
        _url: None,
        _url_updated_at: None,
    })
}

pub async fn browse_playlist_videos(browse_id: &str) -> Option<Vec<Video>> {
    let any = youtubei(
        "/browse",
        json!({
            "browseId": browse_id,
        }),
    )
    .await
    .ok()?;

    jun!(any
        => "contents"
        => "twoColumnBrowseResultsRenderer"
        => "tabs"
        => flat_map
        => "tabRenderer"
        => "content"
        => "sectionListRenderer"
        => "contents"
        => flat_map
        => "itemSectionRenderer"
        => "contents"
        => 0
        => "playlistVideoListRenderer"
        => "contents"
        => map
        => "playlistVideoRenderer"
        => to parse_playlist_video_renderer
    )
}

fn parse_playlist_video_renderer(renderer: &Value) -> Option<Video> {
    Some(Video {
        video_id: jun!(renderer
            => "videoId"
            => to_string
        )?,
        title: jun!(renderer
            => "title"
            => "runs"
            => 0
            => "text"
            => to_string
        ),
        published: jun!(renderer
            => "videoInfo"
            => "runs"
            => 2
            => "text"
            => to_string
        ),
        views: jun!(renderer
            => "videoInfo"
            => "runs"
            => 0
            => "text"
            => to_string
        ),
        duration: jun!(renderer
            => "lengthText"
            => "simpleText"
            => to_string
        ),
        channel: jun!(renderer
            => "shortBylineText"
            => "runs"
            => 0
            => "text"
            => to_string
        ),
        _url: None,
        _url_updated_at: None,
    })
}
