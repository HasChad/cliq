use ratatui::style::Color;

// #[derive(PartialEq, Serialize, Deserialize, Default)]
pub struct ThemeColors {
    pub chat_color: Color,
    pub message_color: Color,
    pub user_color: Color,
    pub ai_color: Color,
}

impl ThemeColors {
    pub fn new() -> Self {
        Self {
            chat_color: Color::Blue,
            message_color: Color::Green,
            user_color: Color::Blue,
            ai_color: Color::Red,
        }
    }
}

pub struct Settings {
    pub show_welcome: bool,
    pub colors: ThemeColors,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            show_welcome: false,
            colors: ThemeColors::new(),
        }
    }
}
