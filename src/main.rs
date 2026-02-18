mod ai_logic;
mod app;
mod input;
mod popups;
mod tui;

use app::*;
use input::*;
use tui::*;

use color_eyre::{Result, eyre::Context};
use dotenvy::dotenv;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{Event, read},
};

fn main() -> Result<()> {
    dotenv().ok();
    let mut app = App::init();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = run(&mut app, terminal).context("app loop failed");
    ratatui::restore();
    app_result
}

fn run(mut app: &mut App, mut terminal: DefaultTerminal) -> Result<()> {
    while app.run {
        terminal.draw(|frame| render(&mut app, frame))?;

        match read().unwrap() {
            Event::Key(event) => input_controller(event, &mut app),
            _ => (),
        }

        // if app.size.0 < 80 || app.size.1 < 20 {
        //     terminal.draw(screen_size_warning)?;
        //     continue;
        // }
    }

    Ok(())
}
