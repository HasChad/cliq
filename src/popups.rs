use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, TitlePosition},
};

use crate::ai_logic::Message;

pub fn popup_welcome(frame: &mut Frame) {
    let lines = vec![
        Line::from(Span::styled(
            "Welcome to Cliq",
            Style::default().fg(Color::Yellow),
        ))
        .alignment(Alignment::Center),
        Line::from(Span::styled(
            "────────────────────────────────────",
            Style::default().fg(Color::Yellow),
        ))
        .alignment(Alignment::Center),
        Line::from(Span::styled(
            "- Type 'help' for available commands",
            Style::default().fg(Color::Blue),
        )),
        Line::from(Span::styled(
            "- Press 'q' to close pop-ups",
            Style::default().fg(Color::Blue),
        )),
    ];
    let text = Text::from(lines);

    let area = popup_area(frame.area(), 60, 20);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(text).block(
            Block::new()
                .bold()
                .fg(Color::Green)
                .borders(Borders::ALL)
                .title("Welcome")
                .title_position(TitlePosition::Top),
        ),
        area,
    );
}

pub fn popup_help(frame: &mut Frame) {
    let lines = vec![
        Line::from(Span::styled(
            "Available Commands:",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(Span::styled(
            " exit | quit   - Quit",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(Span::styled(
            " help          - Show this help message",
            Style::default().fg(Color::Blue),
        )),
        Line::from(Span::styled(
            " status        - Show current conversation status",
            Style::default().fg(Color::Blue),
        )),
        Line::from(Span::styled(
            " clear         - Clear chat history",
            Style::default().fg(Color::Blue),
        )),
    ];
    let text = Text::from(lines);

    let area = popup_area(frame.area(), 60, 20);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(text).block(
            Block::new()
                .bold()
                .fg(Color::Green)
                .borders(Borders::ALL)
                .title("Help")
                .title_position(TitlePosition::Top),
        ),
        area,
    );
}

pub fn popup_status(frame: &mut Frame, messages: &[Message]) {
    let user_messages = messages.iter().filter(|m| m.role == "user").count();
    let assistant_messages = messages.iter().filter(|m| m.role == "assistant").count();

    let lines = vec![
        Line::from(Span::styled(
            "Conversation Status:",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(Span::styled(
            format!("- Messages in history: {}", messages.len() - 1,),
            Style::default().fg(Color::Blue),
        )),
        Line::from(Span::styled(
            format!("- User messages: {}", user_messages),
            Style::default().fg(Color::Blue),
        )),
        Line::from(Span::styled(
            format!("- AI responses: {}", assistant_messages),
            Style::default().fg(Color::Blue),
        )),
    ];
    let text = Text::from(lines);

    let area = popup_area(frame.area(), 60, 20);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(text).block(
            Block::new()
                .bold()
                .fg(Color::Green)
                .borders(Borders::ALL)
                .title("Status")
                .title_position(TitlePosition::Top),
        ),
        area,
    );
}

pub fn popup_sending_message(frame: &mut Frame) {
    let lines = vec![Line::from(Span::styled(
        "Sending message...",
        Style::default().fg(Color::Yellow),
    ))];
    let text = Text::from(lines);

    let area = popup_area(frame.area(), 60, 20);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(text).block(
            Block::new()
                .bold()
                .fg(Color::Green)
                .borders(Borders::ALL)
                .title("Info")
                .title_position(TitlePosition::Top),
        ),
        area,
    );
}

pub fn popup_error(frame: &mut Frame, error_msg: &str) {
    let lines = vec![Line::from(Span::styled(
        error_msg,
        Style::default().fg(Color::Yellow),
    ))];
    let text = Text::from(lines);

    let area = popup_area(frame.area(), 60, 20);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(text).block(
            Block::new()
                .bold()
                .fg(Color::Green)
                .borders(Borders::ALL)
                .title("Error")
                .title_position(TitlePosition::Top),
        ),
        area,
    );
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
