use crate::app::App;
use crate::config::{ACTIVE_MAIN_COLOR, TEXT_MAIN_COLOR, TEXT_SECONDARY_COLOR};
use crate::layout::Section;
use ratatui::{prelude::*, widgets::*, Frame};

struct SectionKeymap {
    section: Option<Section>,
    title: &'static str,
    content: &'static str,
}

impl SectionKeymap {
    pub fn new(section: Option<Section>, content: &'static str) -> Self {
        Self {
            section,
            title: match section {
                Some(Section::PlayerControls) => " Player ",
                Some(Section::Drawer) => " Drawer ",
                Some(Section::Playlist) => " Playlist ",
                Some(Section::SearchResults) => " Results ",
                Some(Section::Search) => " Search ",
                None => " General ",
            },
            content,
        }
    }
    fn to_u8(&self) -> u8 {
        match self.section {
            None => 0,
            Some(Section::Search) => 1,
            Some(Section::SearchResults) => 2,
            Some(Section::Playlist) => 3,
            Some(Section::Drawer) => 4,
            Some(Section::PlayerControls) => 5,
        }
    }
}

pub fn render_keymaps(frame: &mut Frame, app: &mut App, layout: Rect) {
    let mut sections = [
        // @autogen
        SectionKeymap::new(
            Some(Section::Drawer),
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
        SectionKeymap::new(
            None,
            r#"Ctrl+c • Quit
Ctrl+h • Left section
Ctrl+j • Below section
Ctrl+k • Above section
Ctrl+l • Right section
Ctrl+t • Toggle keymaps
"#,
        ),
        SectionKeymap::new(
            Some(Section::PlayerControls),
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
        SectionKeymap::new(
            Some(Section::Playlist),
            r#"j • Select next
k • Select previous
J • Move selected down
K • Move selected up
D • Cut
P • Paste
P • Paste before
Enter • Play video
Ctrl+Enter • Play video from start
"#,
        ),
        SectionKeymap::new(
            Some(Section::SearchResults),
            r#"j • Select next
k • Select previous
Enter (video) • Add to playlist
Enter (video) • Play video
Ctrl+Enter (video) • Play video from start
Enter (channel) • Expand channel videos
Enter (playlist) • Load playlist
"#,
        ),
        SectionKeymap::new(
            Some(Section::Search),
            r#"Enter • Search
Tab • Select next suggestion
Shift+Tab • Select previous suggestion
Esc • Cancel suggestions selection
"#,
        ),
        // @autogen
    ];

    sections.sort_by_key(|section| section.to_u8());

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
                        .border_style(Style::default().fg(
                            if section.section == Some(app.section.id) {
                                ACTIVE_MAIN_COLOR
                            } else {
                                TEXT_SECONDARY_COLOR
                            },
                        ))
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
