use super::util::render_result;
use crate::app::App;
use crate::config::{ACTIVE_MAIN_COLOR, TEXT_MAIN_COLOR, TEXT_SECONDARY_COLOR};
use crate::layout::LAYOUT_RATIO;
use crate::search::Search;
use ratatui::{prelude::*, symbols, widgets::*, Frame};
use youtube_api::SearchResult;

pub fn render_body(frame: &mut Frame, app: &mut App, body_layout: Rect) {
    let lists_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(if app.section.is_search_results() {
                LAYOUT_RATIO[0]
            } else {
                LAYOUT_RATIO[1]
            }),
            Constraint::Fill(if app.section.is_playlist() {
                LAYOUT_RATIO[0]
            } else {
                LAYOUT_RATIO[1]
            }),
            Constraint::Fill(if app.section.is_drawer() {
                LAYOUT_RATIO[0]
            } else {
                LAYOUT_RATIO[1]
            }),
        ])
        .split(body_layout);

    let search_layout = lists_layout[0];

    let playlist_layout = lists_layout[1];

    let drawer_layout = lists_layout[2];

    let search_title = match &app.search {
        Search::Ok(search) => search.title.clone(),
        Search::Error(error) => error.clone(),
        Search::None => "Search Results".to_string(),
    };

    let search_block = Block::default()
        .title(format!(" {} ", search_title))
        .border_set(symbols::border::PLAIN)
        .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
        .border_style(Style::default().fg(if app.section.is_search_results() {
            ACTIVE_MAIN_COLOR
        } else {
            TEXT_SECONDARY_COLOR
        }))
        .padding(Padding::proportional(1));

    if let Search::Ok(search) = &mut app.search {
        let list = List::new(
            search
                .data
                .iter()
                .map(|result| {
                    ListItem::new(render_result(&app.player.playing, result.clone())).style(
                        Style::default().fg(
                            if app.player.playing.is_some() && app.player.playing == result.video()
                            {
                                ACTIVE_MAIN_COLOR
                            } else {
                                TEXT_MAIN_COLOR
                            },
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        );

        frame.render_stateful_widget(
            list.block(search_block)
                .style(Style::default().fg(TEXT_MAIN_COLOR))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("▶")
                .direction(ListDirection::TopToBottom),
            search_layout,
            &mut search.list_state,
        );
    } else if let Search::Error(error) = &app.search {
        frame.render_widget(
            Paragraph::new(Text::from(error.clone()))
                .block(search_block)
                .style(Style::default().fg(TEXT_MAIN_COLOR)),
            search_layout,
        );
    } else if let Search::None = app.search {
        frame.render_widget(
            Paragraph::new(Text::from(""))
                .block(search_block)
                .style(Style::default().fg(TEXT_MAIN_COLOR)),
            search_layout,
        );
    }

    let playlist_block = Block::default()
        .title(format!(" {} ", app.render_playlist.title))
        .border_set(symbols::border::PLAIN)
        .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
        .border_style(Style::default().fg(if app.section.is_playlist() {
            ACTIVE_MAIN_COLOR
        } else {
            TEXT_SECONDARY_COLOR
        }))
        .padding(Padding::proportional(1));

    let list = List::new(
        app.render_playlist
            .data
            .iter()
            .map(|video| {
                ListItem::new(render_result(
                    &app.player.playing,
                    SearchResult::Video(video.clone()),
                ))
                .style(Style::default().fg(
                    if app.player.playing == Some(video.clone()) {
                        ACTIVE_MAIN_COLOR
                    } else {
                        TEXT_MAIN_COLOR
                    },
                ))
            })
            .collect::<Vec<_>>(),
    );

    frame.render_stateful_widget(
        list.block(playlist_block)
            .style(Style::default().fg(TEXT_MAIN_COLOR))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("▶")
            .direction(ListDirection::TopToBottom),
        playlist_layout,
        &mut app.render_playlist.list_state,
    );

    let drawer_block = Block::default()
        .title(" Drawer ")
        .border_set(symbols::border::PLAIN)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if app.section.is_drawer() {
            ACTIVE_MAIN_COLOR
        } else {
            TEXT_SECONDARY_COLOR
        }))
        .padding(Padding::proportional(1));

    let list = List::new(
        app.closet
            .data
            .iter()
            .map(|playlist| {
                ListItem::new(Text::from(vec![
                    Line::from(playlist.title.clone()),
                    Line::from(""),
                ]))
                .style(Style::default().fg(
                    if app.player.playlist_title == playlist.title {
                        ACTIVE_MAIN_COLOR
                    } else {
                        TEXT_MAIN_COLOR
                    },
                ))
            })
            .collect::<Vec<_>>(),
    );

    frame.render_stateful_widget(
        list.block(drawer_block)
            .style(Style::default().fg(TEXT_MAIN_COLOR))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("▶")
            .direction(ListDirection::TopToBottom),
        drawer_layout,
        &mut app.closet.list_state,
    );
}
