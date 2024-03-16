mod app;
mod config;
mod handler;
mod layout;
mod player;
mod playlist;
mod renderer;
mod search;
mod select;
mod util;

use app::App;
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use handler::handle_event;
use ratatui::prelude::*;
use renderer::render;
use std::io::{self, stdout};
use youtube_api::set_config;

#[tokio::main]
async fn main() -> io::Result<()> {
    set_config("IT", "en");

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::load().await;

    while !app.should_quit {
        app.player.update().await;

        app.search_query.update_suggestions().await;

        terminal.draw(|frame| {
            render(frame, &mut app);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                handle_event(key, &mut app).await;
            }
        }
    }

    app.dump();

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
