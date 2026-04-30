use color_eyre::{Result, eyre::Context};

mod ai_logic;
mod app;
mod input;
mod message_controller;
mod popups;
mod run;
mod settings;
mod tui;

use crate::run::run_app;
use app::*;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    app.text_wrapper();

    let terminal = ratatui::init();

    let app_result = run_app(&mut app, terminal).context("App loop failed");

    ratatui::restore();
    app_result
}
