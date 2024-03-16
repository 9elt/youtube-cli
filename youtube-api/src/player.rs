use super::api::youtubei;
use crate::helper::jun;
use serde_json::{json, Value};

pub async fn get_video_url(video_id: &str) -> Option<String> {
    let any = youtubei(
        "/player",
        json!({
            "videoId": video_id
        }),
    )
    .await
    .ok()?;

    jun!(any
            => "streamingData"
            => "adaptiveFormats"
            => map
            => to collect_formats
    )?
    .into_iter()
    .find_map(|(url, audio)| {
        if audio == "AUDIO_QUALITY_MEDIUM" {
            Some(url)
        } else {
            None
        }
    })
}

fn collect_formats(value: &Value) -> Option<(String, String)> {
    Some((
        jun!(value => "url" => to_string)?,
        jun!(value => "audioQuality" => to_string)?,
    ))
}
