use ratatui::crossterm::event::KeyEvent;
use ratatui_textarea::{Input, Key};
use std::fs;

use crate::{
    App, Popup,
    ai_logic::{ChatError, Message, manage_history, send_chat_request},
    app::FILE_PATH,
};

pub const MAX_INPUT_LENGTH: usize = 1000;

pub fn input_controller(key: KeyEvent, app: &mut App) {
    let input = Input::from(key);

    if app.popup != Popup::None {
        if app.popup == Popup::Quit {
            if input.key == Key::Esc {
                app.run = false;
            } else {
                app.popup = Popup::None
            }
        }
        if input.key == Key::Char('q') {
            app.popup = Popup::None;
        }
    } else {
        match input {
            Input {
                key: Key::Enter,
                shift: true,
                ..
            } => app.textarea.insert_newline(),
            Input { key: Key::Esc, .. } => app.popup = Popup::Quit,
            Input {
                key: Key::Enter, ..
            } => process_input(app),
            Input { key: Key::Up, .. } => {
                if app.scroll > 0 {
                    app.scroll -= 1
                }
            }
            Input { key: Key::Down, .. } => {
                app.scroll += 1;
                app.scroll = (app.scroll).min(app.max_scroll);
            }
            input => {
                app.textarea.input(input);
            }
        }
    }
}

fn process_input(app: &mut App) {
    let mut message: String = app.textarea.lines().join("\n");

    message = message.trim_end().trim_start().to_string();

    if message.is_empty() {
        return;
    }

    if message.len() > MAX_INPUT_LENGTH {
        app.popup = Popup::Error(format!(
            "Input too long (max {} characters)",
            MAX_INPUT_LENGTH
        ));
        return;
    }

    match message.to_lowercase().as_str() {
        "/exit" | "/quit" => {
            app.run = false;
            return;
        }
        "/clear" => {
            let system_msg = app.messages[0].clone();
            app.messages = vec![system_msg];
            app.textarea.clear();
            return;
        }
        "/help" => {
            app.popup = Popup::Help;
            app.textarea.clear();
            return;
        }
        "/status" => {
            app.popup = Popup::Status;
            app.textarea.clear();
            return;
        }
        _ => {}
    }

    app.should_send_message = true;
    app.popup = Popup::SendingMessage;
}

pub fn send_message(app: &mut App) {
    let message: String = app.textarea.lines().join(" ");
    app.textarea.clear();

    app.messages.push(Message::user_input(message));
    app.should_send_message = false;
    app.popup = Popup::None;

    manage_history(&mut app.messages);

    match send_chat_request(app) {
        Ok(reply) => {
            app.messages.push(Message::ai_reply(reply.clone()));

            let json_string = serde_json::to_string(&app.messages).unwrap();
            fs::write(FILE_PATH, json_string).unwrap();
        }
        Err(ChatError::EnvVar(err_msg)) => app.popup = Popup::Error(err_msg),
        Err(ChatError::Network(err_msg)) => app.popup = Popup::Error(err_msg),
        Err(ChatError::ApiResponse(err_msg)) => app.popup = Popup::Error(err_msg),
    }
}
