use crate::config::{
    ACTIVE_MAIN_COLOR, ACTIVE_SECONDARY_COLOR, TEXT_MAIN_COLOR, TEXT_SECONDARY_COLOR,
};
use crate::util::h_time;
use ratatui::prelude::*;
use youtube_api::{SearchResult, Video};

pub fn playing_video_label(time: f64, duration: f64, paused: bool) -> String {
    if paused {
        format!("⏸ PAUSED   •  {} / {}", h_time(time), h_time(duration))
    } else {
        format!("▶ PLAYING  •  {} / {}", h_time(time), h_time(duration))
    }
}

pub fn render_result<'a>(playing: &Option<Video>, result: SearchResult) -> Text<'a> {
    let main = TEXT_MAIN_COLOR;
    let secondary = TEXT_SECONDARY_COLOR;

    match result {
        SearchResult::Video(video) => {
            let (main, secondary) = if playing == &Some(video.clone()) {
                (ACTIVE_MAIN_COLOR, ACTIVE_SECONDARY_COLOR)
            } else {
                (TEXT_MAIN_COLOR, TEXT_SECONDARY_COLOR)
            };

            let title = Line::styled(
                video.title.clone().unwrap_or("unknown".into()),
                Style::default().fg(main),
            );
            let channel = Line::styled(
                video.channel.clone().unwrap_or("no channel".into()),
                Style::default().fg(secondary),
            );
            let info = Line::styled(
                [&video.duration, &video.views, &video.published]
                    .iter()
                    .filter_map(|s| s.as_ref())
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" • "),
                Style::default().fg(secondary),
            );
            Text::from(vec![title, channel, info, Line::from(" ")])
        }
        SearchResult::Playlist(playlist) => {
            let title = Line::styled(
                format!("Ⓟ {}", playlist.title.unwrap_or("unknown".into()),),
                Style::default().fg(main),
            );
            let info = Line::styled(
                format!("{} videos", playlist.video_count.unwrap_or(0)),
                Style::default().fg(secondary),
            );
            Text::from(vec![title, info, Line::from(" ")])
        }
        SearchResult::Channel(channel) => {
            let title = Line::styled(
                format!("Ⓒ {}", channel.title.unwrap_or("unknown".into()),),
                Style::default().fg(main),
            );
            let channel_at = Line::styled(
                channel.at.clone().unwrap_or("@unknown".into()),
                Style::default().fg(secondary),
            );
            let info = Line::styled(
                [&channel.subs, &channel.description]
                    .iter()
                    .filter_map(|s| s.as_ref())
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" • "),
                Style::default().fg(secondary),
            );
            Text::from(vec![title, channel_at, info, Line::from(" ")])
        }
    }
}
