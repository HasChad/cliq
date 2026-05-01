use color_eyre::{Result, eyre::Context};

mod ai_logic;
mod app;
mod input;
mod popups;
mod run;
mod settings;
mod tui;

use crate::{app::App, run::run_app};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    let terminal = ratatui::init();

    let app_result = run_app(&mut app, terminal).context("App loop failed");

    ratatui::restore();
    app_result
}
