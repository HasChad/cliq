use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, TitlePosition},
};
use ratatui_textarea::WrapMode;

use crate::{App, Popup, input::MAX_INPUT_LENGTH, popups::*};

pub fn render(app: &mut App, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(90), Constraint::Min(5)]);

    let [top, bottom] = outer_layout.areas(frame.area());

    app.w_size = frame.area().width as usize;
    app.top_h_size = top.height as usize;
    app.bottom_h_size = bottom.height as usize;

    frame.render_widget(
        app.wrapped_msg
            .clone()
            .fg(Color::White)
            .scroll((app.scroll, 0))
            .block(
                Block::new()
                    .fg(app.settings.colors.chat_color)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .title(" Chat ")
                    .title_position(TitlePosition::Top),
            ),
        top,
    );

    app.textarea.set_block(
        Block::new()
            .fg(app.settings.colors.message_color)
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
    frame.render_widget(&app.textarea, bottom);

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
