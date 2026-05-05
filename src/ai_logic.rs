use serde::{Deserialize, Serialize};
use std::env;

use crate::App;

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn ai_character() -> Self {
        let character = match env::var("AI_PERSONALITY") {
            Ok(env) => env,
            Err(_) => "Speak clearly and shortly".into(),
        };

        Self {
            role: "system".into(),
            content: character,
        }
    }

    pub fn user_input(input: String) -> Self {
        Self {
            role: "user".into(),
            content: input,
        }
    }

    pub fn ai_reply(reply: String) -> Self {
        Self {
            role: "assistant".into(),
            content: reply,
        }
    }
}

#[derive(Serialize)]
struct ChatRequest {
    messages: Vec<Message>,
    model: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug)]
pub enum ChatError {
    EnvVar(String),
    Network(String),
    ApiResponse(String),
}

impl From<reqwest::Error> for ChatError {
    fn from(_err: reqwest::Error) -> Self {
        ChatError::Network("Please check your internet connection.".to_string())
    }
}

pub fn send_chat_request(app: &mut App) -> Result<String, ChatError> {
    let model = match env::var("AI_MODEL") {
        Ok(env) => env,
        Err(_) => {
            return Err(ChatError::EnvVar(
                "Please check your AI model in your .env file.".into(),
            ));
        }
    };

    let request = ChatRequest {
        model,
        messages: app.messages.to_vec(),
    };

    let response = app
        .client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(app.api_key.clone())
        .json(&request)
        .send()?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .unwrap_or_else(|_| "Unknown error".to_string());

        let friendly = serde_json::from_str::<serde_json::Value>(&error_text)
            .ok()
            .and_then(|v| v["error"]["message"].as_str().map(|s| s.to_string()))
            .unwrap_or(error_text);

        return Err(ChatError::Network(friendly));
    }

    let chat_response: ChatResponse = response.json()?;

    if chat_response.choices.is_empty() {
        return Err(ChatError::ApiResponse("No response from model.".into()));
    }

    Ok(chat_response.choices[0].message.content.clone())
}
