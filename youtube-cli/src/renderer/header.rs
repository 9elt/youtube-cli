use crate::app::App;
use crate::config::{ACTIVE_MAIN_COLOR, TEXT_MAIN_COLOR, TEXT_SECONDARY_COLOR};
use crate::util::version;
use ratatui::{prelude::*, symbols, widgets::*, Frame};

pub fn render_header(frame: &mut Frame, app: &mut App, header_layout: Rect) {
    let has_suggestions = app.section.is_search() && !app.search_query.suggestions.data.is_empty();

    let header_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(if has_suggestions {
            [Constraint::Min(3), Constraint::Percentage(100)]
        } else {
            [Constraint::Min(3), Constraint::Max(0)]
        })
        .split(header_layout);

    let search_layout = header_layout[0];

    let suggestions_layout = header_layout[1];

    let textarea = &mut app.search_query.textarea;

    textarea.set_block(
        Block::default()
            .borders(if has_suggestions {
                Borders::LEFT | Borders::RIGHT | Borders::TOP
            } else {
                Borders::ALL
            })
            .title(" Search ")
            .border_style(if app.section.is_search() {
                Style::default().fg(ACTIVE_MAIN_COLOR)
            } else {
                Style::default().fg(TEXT_MAIN_COLOR)
            })
            .padding(Padding::left(1)),
    );

    if has_suggestions {
        let suggestions_list = List::new(
            app.search_query
                .suggestions
                .data
                .iter()
                .enumerate()
                .map(|(i, sugg)| {
                    ListItem::new(Text::from(sugg.clone())).style(Style::default().fg(if i == 0 {
                        TEXT_SECONDARY_COLOR
                    } else {
                        TEXT_MAIN_COLOR
                    }))
                })
                .collect::<Vec<_>>(),
        );

        frame.render_stateful_widget(
            suggestions_list
                .block(
                    Block::default()
                        .border_set(symbols::border::PLAIN)
                        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                        .border_style(Style::default().fg(if app.section.is_search() {
                            ACTIVE_MAIN_COLOR
                        } else {
                            TEXT_MAIN_COLOR
                        }))
                        .padding(Padding::horizontal(1)),
                )
                .style(Style::default().fg(TEXT_MAIN_COLOR))
                .highlight_style(
                    Style::default()
                        .fg(ACTIVE_MAIN_COLOR)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("▶")
                .direction(ListDirection::TopToBottom),
            suggestions_layout,
            &mut app.search_query.suggestions.list_state,
        );
    }

    frame.render_widget(textarea.widget(), search_layout);
    frame.render_widget(Span::from(version()).to_right_aligned_line(), search_layout);
}
