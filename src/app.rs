use dotenvy::dotenv;
use ratatui::style::Color;
use reqwest::blocking::Client;
use std::{
    env,
    fs::{self, File},
    process,
};

use crate::ai_logic::Message;

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

#[derive(PartialEq)]
pub struct ThemeColors {
    pub chat_color: Color,
    pub message_color: Color,
    pub user_color: Color,
    pub ai_color: Color,
}

impl ThemeColors {
    fn new() -> Self {
        Self {
            chat_color: Color::Blue,
            message_color: Color::Green,
            user_color: Color::Blue,
            ai_color: Color::Red,
        }
    }
}

pub struct App {
    pub run: bool,
    pub messages: Vec<Message>,
    pub api_key: String,
    pub client: Client,
    pub input: String,
    pub popup: Popup,
    pub colors: ThemeColors,
    pub scroll: u16,
    pub max_scroll: u16,
    pub should_send_message: bool,
}

impl App {
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

        Self {
            run: true,
            messages,
            api_key,
            client: Client::new(),
            input: String::new(),
            popup: Popup::Welcome,
            colors: ThemeColors::new(),
            scroll: 0,
            max_scroll: 0,
            should_send_message: false,
        }
    }
}
