use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::ai_logic::Message;

fn popup_area(area: Rect, length_x: u16, length_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(length_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(length_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

pub fn popup_welcome(frame: &mut Frame) {
    let lines = vec![
        Line::default()
            .spans(["Welcome to Cliq"])
            .fg(Color::Yellow)
            .centered(),
        Line::default()
            .spans(["────────────────────────────────────"])
            .fg(Color::Yellow)
            .centered(),
        Line::default()
            .spans(["- Type '/help' for available commands"])
            .fg(Color::Blue),
        Line::default()
            .spans(["- Press 'q' to close pop-ups"])
            .fg(Color::Blue),
    ];

    let area = popup_area(frame.area(), 40, 6);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(lines).block(Block::new().fg(Color::Green).borders(Borders::ALL)),
        area,
    );
}

pub fn popup_help(frame: &mut Frame) {
    let lines = vec![
        Line::default()
            .spans(["Available Commands"])
            .fg(Color::Yellow)
            .centered(),
        Line::default()
            .spans(["────────────────────"])
            .fg(Color::Yellow)
            .centered(),
        Line::default()
            .spans(["/exit | /quit  - Quit"])
            .fg(Color::Blue),
        Line::default()
            .spans(["/help          - Show this help message"])
            .fg(Color::Blue),
        Line::default()
            .spans(["/status        - Show current conversation status"])
            .fg(Color::Blue),
        Line::default()
            .spans(["/clear         - Clear chat history"])
            .fg(Color::Blue),
    ];

    let p_height = lines.len() as u16 + 2;

    let area = popup_area(frame.area(), 51, p_height);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(lines).block(
            Block::new()
                .fg(Color::Green)
                .borders(Borders::ALL)
                .title(" Help "),
        ),
        area,
    );
}

pub fn popup_status(frame: &mut Frame, messages: &[Message]) {
    let user_messages = messages.iter().filter(|m| m.role == "user").count();
    let assistant_messages = messages.iter().filter(|m| m.role == "assistant").count();

    let lines = vec![
        Line::default()
            .spans(["Conversation Status"])
            .fg(Color::Yellow)
            .centered(),
        Line::default()
            .spans(["────────────────────"])
            .fg(Color::Yellow)
            .centered(),
        Line::default()
            .spans([format!("- Messages in history: {}", messages.len() - 1,)])
            .fg(Color::Blue),
        Line::default()
            .spans([format!("- User messages: {}", user_messages)])
            .fg(Color::Blue),
        Line::default()
            .spans([format!("- AI responses: {}", assistant_messages)])
            .fg(Color::Blue),
    ];

    let p_height = lines.len() as u16 + 2;

    let area = popup_area(frame.area(), 35, p_height);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(lines).block(
            Block::new()
                .fg(Color::Green)
                .borders(Borders::ALL)
                .title(" Status "),
        ),
        area,
    );
}

pub fn popup_sending_message(frame: &mut Frame) {
    let line = Line::default()
        .spans([" Sending message..."])
        .fg(Color::Yellow);

    let area = popup_area(frame.area(), 22, 3);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(line).block(Block::new().fg(Color::Blue).borders(Borders::ALL)),
        area,
    );
}

// pub fn popup_info(frame: &mut Frame, msg: &str) {
//     let line = Line::default().spans([msg]).fg(Color::White).centered();

//     let area = popup_area(frame.area(), msg.len() as u16 + 4, 3);
//     frame.render_widget(Clear, area);
//     frame.render_widget(
//         Paragraph::new(line).wrap(Wrap { trim: true }).block(
//             Block::new()
//                 .fg(Color::Green)
//                 .borders(Borders::ALL)
//                 .title(" Info "),
//         ),
//         area,
//     );
// }

pub fn popup_quit(frame: &mut Frame) {
    let lines = vec![
        Line::default()
            .spans(["Are you sure want to quit?"])
            .fg(Color::Yellow)
            .centered(),
        Line::default(),
        Line::default()
            .spans(["ESC: Quit"])
            .fg(Color::Blue)
            .centered(),
    ];

    let area = popup_area(frame.area(), 30, 5);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(lines).block(Block::new().fg(Color::LightRed).borders(Borders::ALL)),
        area,
    );
}

pub fn popup_error(frame: &mut Frame, error_msg: &str) {
    let line = Line::default()
        .spans([error_msg])
        .fg(Color::Yellow)
        .centered();

    let area = popup_area(frame.area(), error_msg.len() as u16 + 4, 3);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(line).wrap(Wrap { trim: true }).block(
            Block::new()
                .fg(Color::LightRed)
                .borders(Borders::ALL)
                .title(" Error "),
        ),
        area,
    );
}
