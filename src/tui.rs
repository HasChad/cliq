use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, VerticalAlignment},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, TitlePosition, Wrap},
};

use crate::{App, Popup, input::MAX_INPUT_LENGTH, popups::*};

pub fn render(app: &mut App, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(90), Constraint::Min(5)])
        .split(frame.area());

    // chat text
    let mut lines = vec![];

    for message in app.messages.iter() {
        if message.role == "user" {
            lines.push(Line::from(vec![
                Span::styled("You: ", Style::default().fg(Color::Blue)),
                Span::styled(&message.content, Style::default().fg(Color::White)),
            ]));
        } else if message.role == "assistant" {
            lines.push(Line::from(vec![
                Span::styled("AI: ", Style::default().fg(Color::Red)),
                Span::styled(&message.content, Style::default().fg(Color::White)),
            ]));
        }

        lines.push(Line::from(vec![Span::styled("", Style::default())]));
    }

    let text = Text::from(lines);
    let message_p = Paragraph::new(text);

    if app.scroll > message_p.line_count(frame.area().width) as u16 {
        app.scroll = message_p.line_count(frame.area().width) as u16;
    }

    frame.render_widget(
        message_p
            .fg(Color::White)
            .wrap(Wrap { trim: true })
            .scroll((app.scroll, 0))
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
    let lines = vec![
        Line::from(Span::styled(
            "Terminal size is too low! Width: {}, Height: {}",
            Style::default(),
        ))
        .centered(),
        Line::from(Span::styled(
            "Set your terminal size to minimum Width: 80, Height: 20",
            Style::default(),
        ))
        .centered(),
    ];
    let text = Text::from(lines);
    let p = Paragraph::new(text);

    frame.render_widget(p, frame.area());
}
