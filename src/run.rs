use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{Event, KeyCode, read},
};

use crate::{
    app::App,
    input::{input_controller, send_message},
    tui::{render, screen_size_warning},
};

pub fn run_app(mut app: &mut App, mut terminal: DefaultTerminal) -> Result<()> {
    app.get_layout(&terminal.get_frame());
    app.text_wrapper();
    app.scroll_bottom();

    while app.run {
        if terminal.get_frame().area().width < 80 || terminal.get_frame().area().height < 20 {
            terminal.draw(screen_size_warning)?;

            match read().unwrap() {
                Event::Key(event) => {
                    if event.code == KeyCode::Esc {
                        app.run = false;
                    }
                }
                _ => (),
            }
        } else {
            terminal.draw(|frame| render(&mut app, frame))?;

            if app.should_send_message {
                send_message(app);
            } else {
                match read()? {
                    Event::Key(event) => input_controller(event, &mut app),
                    _ => (),
                }
            }
        }
    }

    Ok(())
}
