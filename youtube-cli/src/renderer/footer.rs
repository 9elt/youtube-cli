use super::util::{playing_video_label, render_result};
use crate::app::App;
use crate::config::{ACTIVE_MAIN_COLOR, TEXT_MAIN_COLOR, TEXT_SECONDARY_COLOR};
use ratatui::{prelude::*, symbols, widgets::*, Frame};
use youtube_api::SearchResult;

pub fn render_footer(frame: &mut Frame, app: &mut App, footer_layout: Rect) {
    let footer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Min(7), Constraint::Min(1)])
        .split(footer_layout);

    let gauge_layout = footer_layout[0];

    let player_layout = footer_layout[1];

    let key_help_layout = footer_layout[2];

    if let Some(video) = app.player.playing.clone() {
        frame.render_widget(
            Gauge::default()
                .block(Block::default())
                .gauge_style(Style::default().fg(ACTIVE_MAIN_COLOR))
                .label(playing_video_label(
                    app.player.time,
                    app.player.duration,
                    app.player.is_paused,
                ))
                .ratio(app.player.ratio()),
            gauge_layout,
        );

        let info_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100), Constraint::Min(20)])
            .split(player_layout);

        let video_layout = info_layout[0];

        let controls_layout = info_layout[1];

        frame.render_widget(
            Paragraph::new(render_result(&None, SearchResult::Video(video))).block(
                Block::default()
                    .border_set(symbols::border::PLAIN)
                    .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                    .border_style(Style::default().fg(if app.section.is_player_controls() {
                        ACTIVE_MAIN_COLOR
                    } else {
                        TEXT_MAIN_COLOR
                    }))
                    .title(if app.player.is_paused {
                        " ⏸ PAUSED "
                    } else {
                        " ▶ PLAYING "
                    })
                    .padding(Padding::proportional(1)),
            ),
            video_layout,
        );

        frame.render_widget(
            Paragraph::new(
                Text::from(vec![
                    Line::from(if app.player.is_paused {
                        "⏸ paused"
                    } else {
                        "▶ playing"
                    }),
                    Line::from(format!(
                        "volume • {:>5}",
                        if app.player.is_muted {
                            "MUTED".to_string()
                        } else {
                            app.player.volume.to_string()
                        }
                    )),
                    Line::from(format!("speed • {:>4.2}x", app.player.speed)),
                ])
                .style(Style::default().fg(TEXT_SECONDARY_COLOR)),
            )
            .block(
                Block::default()
                    .border_set(symbols::border::PLAIN)
                    .borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT)
                    .border_style(Style::default().fg(if app.section.is_player_controls() {
                        ACTIVE_MAIN_COLOR
                    } else {
                        TEXT_MAIN_COLOR
                    }))
                    .title(" Controls ")
                    .padding(Padding::proportional(1)),
            )
            .alignment(Alignment::Right),
            controls_layout,
        );
    } else {
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if app.section.is_player_controls() {
                    ACTIVE_MAIN_COLOR
                } else {
                    TEXT_MAIN_COLOR
                }))
                .border_set(symbols::border::PLAIN)
                .title(" Player ")
                .padding(Padding::proportional(1)),
            player_layout,
        );
    }

    frame.render_widget(
        Paragraph::new(app.section.key_help())
            .block(Block::default().style(Style::default().fg(TEXT_SECONDARY_COLOR)))
            .alignment(Alignment::Center),
        key_help_layout,
    );
}
