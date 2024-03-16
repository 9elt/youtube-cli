/*

[                search              ]

[             suggestions            ]

[ search results | playlist | drawer ]

[           player controls          ]

*/

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Section {
    Search,
    Playlist,
    SearchResults,
    Drawer,
    PlayerControls,
}

pub const LAYOUT_RATIO: [u16; 2] = [32, 21];

const SECTION_LAYOUT: [[Section; 3]; 3] = [
    [Section::Search, Section::Search, Section::Search],
    [Section::SearchResults, Section::Playlist, Section::Drawer],
    [
        Section::PlayerControls,
        Section::PlayerControls,
        Section::PlayerControls,
    ],
];

pub struct LayoutSection {
    pub id: Section,
    x: i8,
    y: i8,
}

impl Default for LayoutSection {
    fn default() -> Self {
        Self {
            id: Section::Search,
            x: 0,
            y: 0,
        }
    }
}

impl LayoutSection {
    pub fn is_search(&self) -> bool {
        self.id == Section::Search
    }

    pub fn is_search_results(&self) -> bool {
        self.id == Section::SearchResults
    }

    pub fn is_playlist(&self) -> bool {
        self.id == Section::Playlist
    }

    pub fn is_drawer(&self) -> bool {
        self.id == Section::Drawer
    }

    pub fn is_player_controls(&self) -> bool {
        self.id == Section::PlayerControls
    }

    pub fn key_help(&self) -> &str {
        match self.id {
            Section::Search=> "CTRL+hjkl: naviage sections • enter: search",
            Section::SearchResults => "CTRL+hjkl: naviage sections • jk: navigate • enter: add to queue / play",
            Section::Playlist=> "CTRL+hjkl: naviage sections • jk: navigate • JK: order • enter: play • d: cut • p: paste after • P: paste before",
            Section::Drawer=> "CTRL+hjkl: naviage sections • jk: navigate • JK: order • enter: select",
            Section::PlayerControls=> {
                "CTRL+hjkl: naviage sections • space: play / pause • h: seek -15s • l: seek +15s • H: seek -60s • L: seek +60s"
            }
        }
    }
}

impl LayoutSection {
    pub fn incr_x(&mut self, incr: i8) {
        self.x = (self.x + incr).max(0).min(2);
        self.update_id();
    }

    pub fn incr_y(&mut self, incr: i8) {
        self.y = (self.y + incr).max(0).min(2);
        self.update_id();
    }

    pub fn set(&mut self, id: Section) {
        self.id = id;

        for (y, row) in SECTION_LAYOUT.iter().enumerate() {
            for (x, section) in row.iter().enumerate() {
                if *section == id {
                    self.x = x as i8;
                    self.y = y as i8;
                    return;
                }
            }
        }
    }

    fn update_id(&mut self) {
        self.id = SECTION_LAYOUT[self.y as usize][self.x as usize];
    }
}
