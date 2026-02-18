use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph, TitlePosition, Wrap},
};

use crate::{App, Popup, input::MAX_INPUT_LENGTH, popups::*};

pub fn render(app: &mut App, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(frame.area());

    // chat text
    let mut string_message = String::new();

    for message in app.messages.iter() {
        if message.role == "user" {
            string_message.push_str("You: ");
        } else if message.role == "assistant" {
            string_message.push_str("AI: ");
        } else {
            continue;
        }

        string_message.push_str(&message.content);
        string_message.push_str("\n\n");
    }

    frame.render_widget(
        Paragraph::new(string_message)
            .fg(Color::White)
            .wrap(Wrap { trim: true })
            // .scroll(offset)
            .block(
                Block::new()
                    .fg(Color::Blue)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .title(" Chat ")
                    .title_position(TitlePosition::Top),
            ),
        outer_layout[0],
    );

    frame.render_widget(
        Paragraph::new(app.input.clone())
            .fg(Color::White)
            .wrap(Wrap { trim: true })
            .block(
                Block::new()
                    .fg(Color::Green)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .title(format!(
                        " Message | {}/{} ",
                        app.input.len(),
                        MAX_INPUT_LENGTH,
                    ))
                    .title_position(TitlePosition::Top),
            ),
        outer_layout[1],
    );

    match &app.popup {
        Popup::Welcome => popup_welcome(frame),
        Popup::Help => popup_help(frame),
        Popup::Status => popup_status(frame, &app.messages),
        Popup::Error(msg) => popup_error(frame, msg.as_str()),
        Popup::None => {}
    }
}

pub fn screen_size_warning(frame: &mut Frame) {
    let greeting = Paragraph::new(
        "Terminal size is too low! Width: {}, Height: {}
Set your terminal size to minimum Width: 80, Height: 20",
    );
    frame.render_widget(greeting, frame.area());
}
