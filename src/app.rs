use chrono::Local;
use ratatui::widgets::{ListState, ScrollbarState};
use std::time::{Duration, Instant};

use crate::ai_models::AIModel;
use crate::i18n::{Language, Translations};

#[derive(Debug, Clone)]
pub struct Message {
    pub content: String,
    pub sender: Sender,
    pub timestamp: chrono::DateTime<Local>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sender {
    User,
    AI(AIModel),
}

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub ai_models: Vec<AIModel>,
    pub selected_model_index: usize,
    pub messages: Vec<Message>,
    pub input: String,
    pub input_mode: InputMode,
    pub ai_list_state: ListState,
    pub user_list_state: ListState,
    pub ai_scrollbar_state: ScrollbarState,
    pub user_scrollbar_state: ScrollbarState,
    pub theme_index: usize,
    pub cursor_blink_state: bool,
    pub last_blink_time: Instant,
    pub show_help: bool,
    pub notification: Option<String>,
    pub last_update: Instant,
    pub language: Language,
    pub translations: Translations,
}

impl App {
    pub fn new() -> Self {
        let ai_models = AIModel::all();
        let language = Language::Chinese;
        let translations = Translations::new(language);
        let mut messages = Vec::new();
        let now = Local::now();
        messages.push(Message {
            content: translations.get("welcome_message"),
            sender: Sender::AI(ai_models[0]),
            timestamp: now,
        });
        App {
            ai_models,
            selected_model_index: 0,
            messages,
            input: String::new(),
            input_mode: InputMode::Normal,
            ai_list_state: ListState::default(),
            user_list_state: ListState::default(),
            ai_scrollbar_state: ScrollbarState::new(0),
            user_scrollbar_state: ScrollbarState::new(0),
            theme_index: 0,
            cursor_blink_state: true,
            last_blink_time: Instant::now(),
            show_help: false,
            notification: None,
            last_update: Instant::now(),
            language,
            translations,
        }
    }

    pub fn current_model(&self) -> AIModel {
        self.ai_models[self.selected_model_index]
    }

    pub fn select_previous_model(&mut self) {
        if self.selected_model_index > 0 {
            self.selected_model_index -= 1;
        } else {
            self.selected_model_index = self.ai_models.len() - 1;
        }
    }

    pub fn select_next_model(&mut self) {
        if self.selected_model_index < self.ai_models.len() - 1 {
            self.selected_model_index += 1;
        } else {
            self.selected_model_index = 0;
        }
    }

    pub fn send_message(&mut self) {
        if self.input.trim().is_empty() {
            return;
        }
        let user_message = Message {
            content: self.input.clone(),
            sender: Sender::User,
            timestamp: Local::now(),
        };
        self.messages.push(user_message);
        self.user_list_state.select(Some(self.messages.len() - 1));
        let current_model = self.current_model();
        let ai_response = current_model.simulate_response(&self.input, self.language);
        let ai_message = Message {
            content: ai_response,
            sender: Sender::AI(current_model),
            timestamp: Local::now(),
        };
        self.messages.push(ai_message);
        self.ai_list_state.select(Some(self.messages.len() - 1));
        let ai_messages_count = self
            .messages
            .iter()
            .filter(|msg| matches!(msg.sender, Sender::AI(_)))
            .count();
        let user_messages_count = self.messages.len() - ai_messages_count;
        self.ai_scrollbar_state = ScrollbarState::new(ai_messages_count);
        self.user_scrollbar_state = ScrollbarState::new(user_messages_count);
        self.input.clear();
    }

    pub fn scroll_up(&mut self) {
        let ai_messages_count = self
            .messages
            .iter()
            .filter(|msg| matches!(msg.sender, Sender::AI(_)))
            .count();
        let current_ai_selection = self.ai_list_state.selected().unwrap_or(0);
        if current_ai_selection < ai_messages_count.saturating_sub(1) {
            self.ai_list_state.select(Some(current_ai_selection + 1));
        }
        let user_messages_count = self.messages.len() - ai_messages_count;
        let current_user_selection = self.user_list_state.selected().unwrap_or(0);
        if current_user_selection < user_messages_count.saturating_sub(1) {
            self.user_list_state
                .select(Some(current_user_selection + 1));
        }
    }

    pub fn scroll_down(&mut self) {
        let current_ai_selection = self.ai_list_state.selected().unwrap_or(0);
        if current_ai_selection > 0 {
            self.ai_list_state.select(Some(current_ai_selection - 1));
        }
        let current_user_selection = self.user_list_state.selected().unwrap_or(0);
        if current_user_selection > 0 {
            self.user_list_state
                .select(Some(current_user_selection - 1));
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn change_theme(&mut self, index: usize) {
        if index < 4 {
            self.theme_index = index;
            self.set_notification(self.translations.get("notification_theme_changed"));
        }
    }

    pub fn next_theme(&mut self) {
        self.theme_index = (self.theme_index + 1) % 4;
        self.set_notification(self.translations.get("notification_theme_changed"));
    }

    pub fn previous_theme(&mut self) {
        if self.theme_index == 0 {
            self.theme_index = 3;
        } else {
            self.theme_index -= 1;
        }
        self.set_notification(self.translations.get("notification_theme_changed"));
    }

    pub fn update_cursor_blink(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_blink_time) >= Duration::from_millis(500) {
            self.cursor_blink_state = !self.cursor_blink_state;
            self.last_blink_time = now;
        }
    }

    pub fn set_notification(&mut self, message: String) {
        self.notification = Some(message);
    }

    pub fn clear_notification(&mut self) {
        self.notification = None;
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    pub fn switch_language(&mut self, lang: Language) {
        self.language = lang;
        self.translations = Translations::new(lang);
        if !self.messages.is_empty() {
            self.messages[0].content = self.translations.get("welcome_message");
        }
        self.set_notification(self.translations.get("notification_language_changed"));
    }

    pub fn switch_to_chinese(&mut self) {
        self.switch_language(Language::Chinese);
    }

    pub fn switch_to_english(&mut self) {
        self.switch_language(Language::English);
    }

    pub fn t(&self, key: &str) -> String {
        self.translations.get(key)
    }
}
