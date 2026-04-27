use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, TitlePosition},
};
use ratatui_textarea::WrapMode;

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
            lines.push(Line::default().spans(["━ You ━"]).fg(app.colors.user_color));

            for text in wrapped_text.iter() {
                lines.push(Line::default().spans([text.clone()]));
            }
            total_lines += wrapped_text.len() as u16 + 2
        } else if message.role == "assistant" {
            lines.push(Line::default().spans(["━ AI ━"]).fg(app.colors.ai_color));

            for text in wrapped_text.iter() {
                lines.push(Line::default().spans([text.clone()]));
            }
            total_lines += wrapped_text.len() as u16 + 2
        }

        lines.push(Line::from(vec![Span::styled("", Style::default())]));
    }

    let text = Text::from(lines);
    let message_p = Paragraph::new(text);

    app.max_scroll = total_lines.saturating_sub(outer_layout[0].height - 2);

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

    app.textarea.set_block(
        Block::new()
            .fg(app.colors.message_color)
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title(format!(
                " Message | {}/{} ",
                app.textarea.lines().join("\n").len(),
                MAX_INPUT_LENGTH,
            ))
            .title_position(TitlePosition::Top),
    );
    app.textarea.set_style(Style::default().fg(Color::White));
    app.textarea.set_wrap_mode(WrapMode::Word);
    app.textarea.set_cursor_line_style(Style::default());
    frame.render_widget(&app.textarea, outer_layout[1]);

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
        Line::default().spans(["Terminal size too small! "]),
        Line::default().spans([format!(
            "Width: {}, Height: {}",
            frame.area().width,
            frame.area().height
        )]),
        Line::default(),
        Line::default().spans(["Set your terminal size to minimum"]),
        Line::default().spans(["Width: 80, Height: 20"]),
    ];

    let text = Text::from(lines);
    let p = Paragraph::new(text).centered();

    frame.render_widget(p, frame.area());
}
