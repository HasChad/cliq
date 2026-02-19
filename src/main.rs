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
        if terminal.get_frame().area().width < 80 || terminal.get_frame().area().height < 20 {
            terminal.draw(screen_size_warning)?;
        } else {
            terminal.draw(|frame| render(&mut app, frame))?;

            match read().unwrap() {
                Event::Key(event) => input_controller(event, &mut app),
                _ => (),
            }
        }
    }

    Ok(())
}
