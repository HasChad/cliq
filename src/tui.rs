use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, TitlePosition},
};

use crate::{App, Popup, input::MAX_INPUT_LENGTH, popups::*};

pub fn render(app: &mut App, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(90), Constraint::Min(5)])
        .split(frame.area());

    // chat text
    let mut lines = vec![];
    let mut total_lines: u16 = 0;

    for message in app.messages.iter() {
        if message.role == "system" {
            continue;
        }

        let wrapped_text = textwrap::wrap(&message.content, frame.area().width as usize - 2);

        if message.role == "user" {
            lines.push(Line::from(vec![Span::styled(
                "━ You ━",
                Style::default().fg(app.colors.user_color),
            )]));

            for text in wrapped_text.iter() {
                lines.push(Line::from(vec![Span::styled(
                    text.clone(),
                    Style::default().fg(Color::White),
                )]));
            }
            total_lines += wrapped_text.len() as u16 + 2
        } else if message.role == "assistant" {
            lines.push(Line::from(vec![Span::styled(
                "━ AI ━",
                Style::default().fg(app.colors.ai_color),
            )]));

            for text in wrapped_text.iter() {
                lines.push(Line::from(vec![Span::styled(
                    text.clone(),
                    Style::default().fg(Color::White),
                )]));
            }
            total_lines += wrapped_text.len() as u16 + 2
        }

        lines.push(Line::from(vec![Span::styled("", Style::default())]));
    }

    let text = Text::from(lines);
    let message_p = Paragraph::new(text);

    let max = total_lines.saturating_sub(outer_layout[0].height - 2);

    app.scroll = (app.scroll).min(max);

    frame.render_widget(
        message_p.fg(Color::White).scroll((app.scroll, 0)).block(
            Block::new()
                .fg(app.colors.chat_color)
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title(" Chat ")
                .title_position(TitlePosition::Top),
        ),
        outer_layout[0],
    );

    frame.render_widget(
        Paragraph::new(app.input.clone()).fg(Color::White).block(
            Block::new()
                .fg(app.colors.message_color)
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
        Popup::Quit => popup_quit(frame),
        Popup::SendingMessage => popup_sending_message(frame),
        Popup::Error(msg) => popup_error(frame, msg.as_str()),
        Popup::None => {}
    }
}

pub fn screen_size_warning(frame: &mut Frame) {
    let lines = vec![
        Line::from(Span::styled("Terminal size too small! ", Style::default())).centered(),
        Line::from(Span::styled(
            format!(
                "Width: {}, Height: {}",
                frame.area().width,
                frame.area().height
            ),
            Style::default(),
        ))
        .centered(),
        Line::from(Span::styled("", Style::default())),
        Line::from(Span::styled(
            "Set your terminal size to minimum",
            Style::default(),
        ))
        .centered(),
        Line::from(Span::styled("Width: 80, Height: 20", Style::default())).centered(),
    ];
    let text = Text::from(lines);
    let p = Paragraph::new(text);

    frame.render_widget(p, frame.area());
}
