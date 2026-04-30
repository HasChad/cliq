use dotenvy::dotenv;
use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::Paragraph,
};
use ratatui_textarea::TextArea;
use reqwest::blocking::Client;
use std::{
    env,
    fs::{self, File},
    process,
};

use crate::{ai_logic::Message, settings::Settings};

pub const FILE_PATH: &str = "messages.json";

#[derive(PartialEq)]
pub enum Popup {
    None,
    Welcome,
    Help,
    Status,
    SendingMessage,
    Quit,
    Error(String),
}

pub struct App<'a> {
    pub run: bool,
    pub textarea: TextArea<'a>,
    pub messages: Vec<Message>,
    pub wrapped_msg: Paragraph<'a>,
    pub api_key: String,
    pub client: Client,
    pub popup: Popup,
    pub settings: Settings,
    pub scroll: u16,
    pub max_scroll: u16,
    pub should_send_message: bool,
    pub w_size: usize,
    pub top_h_size: usize,
    pub bottom_h_size: usize,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        dotenv().ok();
        let system_message = Message::ai_character();
        let api_key = match env::var("GROQ_API_KEY") {
            Ok(env) => env,
            Err(_) => {
                println!(
                    "\nGROQ_API_KEY environment variable not found. Please set it in your .env file!"
                );
                process::exit(1);
            }
        };

        let messages: Vec<Message> = match File::open(FILE_PATH) {
            Ok(_) => {
                let mut json_data = fs::read_to_string(FILE_PATH).unwrap();

                if json_data.is_empty() {
                    json_data = serde_json::to_string(&vec![system_message]).unwrap();
                    fs::write(FILE_PATH, json_data.clone()).unwrap();
                }

                serde_json::from_str(&json_data).unwrap()
            }
            Err(_) => {
                File::create(FILE_PATH).unwrap();

                let json_data = serde_json::to_string(&vec![system_message]).unwrap();
                fs::write(FILE_PATH, json_data.clone()).unwrap();

                serde_json::from_str(&json_data).unwrap()
            }
        };

        let settings = Settings::new();
        let popup = if settings.show_welcome {
            Popup::Welcome
        } else {
            Popup::None
        };

        Self {
            run: true,
            textarea: TextArea::default(),
            messages,
            wrapped_msg: Paragraph::new(""),
            api_key,
            client: Client::new(),
            popup,
            settings,
            scroll: 0,
            max_scroll: 0,
            should_send_message: false,
            w_size: 0,
            top_h_size: 0,
            bottom_h_size: 0,
        }
    }

    pub fn text_wrapper(&mut self) {
        let mut lines = vec![];
        let mut wrapped_message: Vec<String> = vec![];

        for message in self.messages.iter() {
            if message.role == "system" {
                continue;
            }

            wrapped_message = textwrap::wrap(&message.content, self.w_size)
                .into_iter()
                .map(|s| s.into_owned())
                .collect();

            if message.role == "user" {
                lines.push(Line::default().spans(["━ You ━".fg(self.settings.colors.user_color)]));
            } else if message.role == "assistant" {
                lines.push(Line::default().spans(["━ AI ━".fg(self.settings.colors.ai_color)]));
            }

            for text in wrapped_message.iter() {
                lines.push(Line::default().spans([text.clone()]));
            }

            lines.push(Line::default());
        }

        self.max_scroll = lines.len().saturating_sub(self.top_h_size) as u16;
        let text = Text::from(lines);
        self.wrapped_msg = Paragraph::new(text);
    }

    pub fn scroll_bottom(&mut self) {
        self.scroll = self.max_scroll;
    }
}
