use crate::config::{TEXT_MAIN_COLOR, TEXT_SECONDARY_COLOR};
use ratatui::{prelude::*, widgets::*, Frame};

struct Section {
    title: &'static str,
    content: &'static str,
}

impl Section {
    pub fn new(title: &'static str, content: &'static str) -> Self {
        Self { title, content }
    }
}

pub fn render_help(frame: &mut Frame, layout: Rect) {
    let sections = [
        // @autogen
        Section::new(
            " Drawer ",
            r#"j • Select next
k • Select previous
J • Move selected down
K • Move selected up
d • Cut
p • Paste
P • Paste before
Enter • Open playlist
"#,
        ),
        Section::new(
            " Generic ",
            r#"Ctrl + c • Quit
Ctrl + h • Left section
Ctrl + j • Below section
Ctrl + k • Above section
Ctrl + l • Right section
"#,
        ),
        Section::new(
            " PlayerControls ",
            r#"Space • Toggle pause/play
h • Seek backward 5s
l • Seek forward 5s
H • Seek backward 15s
L • Seek forward 15s
9 • Volume down
0 • Volume up
o • Speed down
p • Speed up
m • Toggle mute
"#,
        ),
        Section::new(
            " Playlist ",
            r#"j • Select next
k • Select previous
J • Move selected down
K • Move selected up
D • Cut
P • Paste
P • Paste before
Enter • Play video
Ctrl + Enter • Play video from start
"#,
        ),
        Section::new(
            " SearchResults ",
            r#"j • Select next
k • Select previous
Enter (video) • Add to playlist
Enter x2 (video) • Play video
Ctrl + Enter x2 (video) • Play video from start
Enter (channel) • Expand channel videos
Enter (playlist) • Load playlist
"#,
        ),
        Section::new(
            " Search ",
            r#"Enter • Search
Tab • Select next suggestion
Shift + Tab • Select previous suggestion
Esc • Cancel suggestions selection
"#,
        ),
        // @autogen
    ];

    let sections_size = sections.len();

    let top = sections_size / 2;

    let bottom = sections_size - top;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layout);

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints((0..top).map(|_| Constraint::Fill(1)).collect::<Vec<_>>())
        .split(layout[0]);

    let bottom_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints((0..bottom).map(|_| Constraint::Fill(1)).collect::<Vec<_>>())
        .split(layout[1]);

    sections.iter().enumerate().for_each(|(i, section)| {
        frame.render_widget(
            Paragraph::new(section.content)
                .style(Style::default().fg(TEXT_MAIN_COLOR))
                .block(
                    Block::default()
                        .title(section.title)
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(TEXT_SECONDARY_COLOR))
                        .padding(Padding::proportional(1)),
                )
                .alignment(Alignment::Left),
            if i < top {
                top_layout[i]
            } else {
                bottom_layout[i - top]
            },
        );
    });
}
