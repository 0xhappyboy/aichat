use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Wrap,
    },
};

use crate::ai_models::AIModel;
use crate::app::{App, AppState, InputMode, Message, Sender};
use crate::i18n::Language;

pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub text: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
}

impl Theme {
    pub fn deep_blue() -> Self {
        Theme {
            primary: Color::Rgb(0, 119, 190),
            secondary: Color::Rgb(0, 180, 216),
            accent: Color::Rgb(144, 224, 239),
            background: Color::Rgb(12, 20, 31),
            text: Color::Rgb(230, 240, 255),
            success: Color::Rgb(46, 204, 113),
            warning: Color::Rgb(241, 196, 15),
            error: Color::Rgb(231, 76, 60),
        }
    }

    pub fn forest_green() -> Self {
        Theme {
            primary: Color::Rgb(46, 125, 50),
            secondary: Color::Rgb(76, 175, 80),
            accent: Color::Rgb(165, 214, 167),
            background: Color::Rgb(24, 30, 24),
            text: Color::Rgb(240, 255, 240),
            success: Color::Rgb(56, 142, 60),
            warning: Color::Rgb(255, 193, 7),
            error: Color::Rgb(244, 67, 54),
        }
    }

    pub fn sunset() -> Self {
        Theme {
            primary: Color::Rgb(233, 69, 96),
            secondary: Color::Rgb(255, 119, 34),
            accent: Color::Rgb(255, 190, 11),
            background: Color::Rgb(29, 23, 40),
            text: Color::Rgb(255, 240, 230),
            success: Color::Rgb(46, 204, 113),
            warning: Color::Rgb(241, 196, 15),
            error: Color::Rgb(231, 76, 60),
        }
    }

    pub fn neon() -> Self {
        Theme {
            primary: Color::Rgb(255, 0, 255),
            secondary: Color::Rgb(0, 255, 255),
            accent: Color::Rgb(255, 255, 0),
            background: Color::Rgb(0, 0, 20),
            text: Color::Rgb(255, 255, 255),
            success: Color::Rgb(0, 255, 128),
            warning: Color::Rgb(255, 128, 0),
            error: Color::Rgb(255, 0, 128),
        }
    }
}

pub fn render(app: &mut App, frame: &mut Frame) {
    match app.app_state {
        AppState::Welcome => {
            render_welcome_page(app, frame);
        }
        AppState::Chatting => {
            if app.show_help {
                render_help_modal(app, frame, frame.size());
            } else {
                render_chat_interface(app, frame);
            }
        }
        AppState::Help => {
            render_help_modal(app, frame, frame.size());
        }
    }
    if let Some(notification) = &app.notification {
        render_notification(app, frame, frame.size(), notification);
    }
}

fn render_welcome_page(app: &App, frame: &mut Frame) {
    let theme = match app.theme_index {
        0 => Theme::deep_blue(),
        1 => Theme::forest_green(),
        2 => Theme::sunset(),
        3 => Theme::neon(),
        _ => Theme::deep_blue(),
    };
    let area = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(4),
        ])
        .split(area);
    let title_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.primary))
        .style(Style::default().bg(theme.background));
    let title_content = Paragraph::new(vec![
        Line::from(Span::styled(
            app.t("welcome_title"),
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::SLOW_BLINK),
        )),
        Line::from(Span::styled(
            app.t("welcome_subtitle"),
            Style::default().fg(theme.secondary),
        )),
    ])
    .alignment(ratatui::layout::Alignment::Center)
    .block(title_block);
    frame.render_widget(title_content, chunks[0]);
    let features_area = centered_rect(70, 60, chunks[1]);
    let features_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.background));
    let features = vec![
        Line::from(""),
        Line::from(Span::styled(
            app.t("welcome_feature1"),
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            "  ├─ DeepSeek, ChatGPT, Claude, Gemini",
            Style::default().fg(theme.text),
        )),
        Line::from(Span::styled(
            "  ├─ Llama, Qwen, Custom Model",
            Style::default().fg(theme.text),
        )),
        Line::from(""),
        Line::from(Span::styled(
            app.t("welcome_feature2"),
            Style::default().fg(Color::Magenta),
        )),
        Line::from(Span::styled(
            "  ├─ Deep Blue Ocean",
            Style::default().fg(Color::Blue),
        )),
        Line::from(Span::styled(
            "  ├─ Forest Green",
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            "  ├─ Sunset Orange",
            Style::default().fg(Color::Rgb(255, 165, 0)),
        )),
        Line::from(Span::styled(
            "  ├─ Neon Cyber",
            Style::default().fg(Color::Rgb(255, 0, 255)),
        )),
        Line::from(""),
        Line::from(Span::styled(
            app.t("welcome_feature3"),
            Style::default().fg(Color::Yellow),
        )),
        Line::from(Span::styled(
            "  ├─ Press C for Chinese",
            Style::default().fg(theme.text),
        )),
        Line::from(Span::styled(
            "  ├─ Press E for English",
            Style::default().fg(theme.text),
        )),
        Line::from(""),
        Line::from(Span::styled(
            app.t("welcome_feature4"),
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            "  ├─ Real-time message exchange",
            Style::default().fg(theme.text),
        )),
        Line::from(Span::styled(
            "  ├─ Model-specific responses",
            Style::default().fg(theme.text),
        )),
        Line::from(""),
        Line::from(Span::styled(
            app.t("welcome_feature5"),
            Style::default().fg(Color::Red),
        )),
        Line::from(Span::styled(
            "  ├─ Arrow keys for navigation",
            Style::default().fg(theme.text),
        )),
        Line::from(Span::styled(
            "  ├─ F1 for help",
            Style::default().fg(theme.text),
        )),
        Line::from(Span::styled(
            "  ├─ 1-4 for themes",
            Style::default().fg(theme.text),
        )),
        Line::from(Span::styled(
            "  ├─ Q to quit",
            Style::default().fg(theme.text),
        )),
        Line::from(""),
    ];
    let features_paragraph = Paragraph::new(features)
        .block(features_block)
        .alignment(ratatui::layout::Alignment::Left)
        .wrap(Wrap { trim: true });
    frame.render_widget(features_paragraph, features_area);
    let hint_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.primary))
        .style(Style::default().bg(theme.background));
    let hint_content = Paragraph::new(vec![
        Line::from(Span::styled(
            app.t("welcome_start_hint"),
            Style::default()
                .fg(theme.secondary)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            app.t("welcome_press_enter"),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::SLOW_BLINK)
                .add_modifier(Modifier::BOLD),
        )),
    ])
    .alignment(ratatui::layout::Alignment::Center)
    .block(hint_block);
    frame.render_widget(hint_content, chunks[2]);
}

fn render_chat_interface(app: &mut App, frame: &mut Frame) {
    let theme = match app.theme_index {
        0 => Theme::deep_blue(),
        1 => Theme::forest_green(),
        2 => Theme::sunset(),
        3 => Theme::neon(),
        _ => Theme::deep_blue(),
    };
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(4),
        ])
        .split(frame.size());
    render_title_bar(app, frame, main_chunks[0], &theme);
    render_model_selector(app, frame, main_chunks[1], &theme);
    render_chat_area(app, frame, main_chunks[2], &theme);
    render_input_area(app, frame, main_chunks[3], &theme);
}

fn render_title_bar(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let title = app.t("app_title");
    let subtitle = app.t("app_subtitle");
    let title_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.primary))
        .style(Style::default().bg(theme.background))
        .padding(Padding::horizontal(2));
    let title_content = Paragraph::new(vec![
        Line::from(Span::styled(
            title,
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::SLOW_BLINK),
        )),
        Line::from(Span::styled(subtitle, Style::default().fg(theme.secondary))),
    ])
    .alignment(ratatui::layout::Alignment::Center)
    .block(title_block);
    frame.render_widget(title_content, area);
}

fn render_model_selector(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
        ])
        .split(area);
    let models_area = horizontal_chunks[0];
    let themes_area = horizontal_chunks[1];
    let language_area = horizontal_chunks[2];
    let mut model_spans = Vec::new();
    for (i, model) in app.ai_models.iter().enumerate() {
        let is_selected = i == app.selected_model_index;
        let (primary, _, text) = model.colors();
        if i > 0 {
            model_spans.push(Span::styled(" │ ", Style::default().fg(theme.accent)));
        }
        let content = if is_selected {
            format!(" {} ", model.name(app.language))
        } else {
            format!(" {} ", model.name(app.language))
        };
        let style = if is_selected {
            Style::default()
                .fg(text)
                .bg(primary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(primary).add_modifier(Modifier::BOLD)
        };
        model_spans.push(Span::styled(content, style));
    }
    let models_line = Line::from(model_spans);
    let models_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.accent))
        .title(app.t("model_selector_title"))
        .title_style(Style::default().fg(theme.primary))
        .style(Style::default().bg(theme.background));
    let models_paragraph = Paragraph::new(models_line)
        .alignment(ratatui::layout::Alignment::Center)
        .block(models_block);
    frame.render_widget(models_paragraph, models_area);
    let themes = ["1", "2", "3", "4"];
    let mut theme_spans = Vec::new();
    theme_spans.push(Span::styled(
        "Theme ",
        Style::default()
            .fg(theme.secondary)
            .add_modifier(Modifier::BOLD),
    ));
    for (i, theme_num) in themes.iter().enumerate() {
        let is_active = i == app.theme_index;
        let style = if is_active {
            Style::default()
                .fg(theme.text)
                .bg(theme.primary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(theme.secondary)
                .add_modifier(Modifier::BOLD)
        };
        if i > 0 {
            theme_spans.push(Span::styled(" ", Style::default()));
        }
        let content = format!("{}", theme_num);
        theme_spans.push(Span::styled(content, style));
    }
    let theme_names = app.t("theme_names");
    theme_spans.push(Span::styled(" ", Style::default()));
    theme_spans.push(Span::styled(theme_names, Style::default().fg(theme.accent)));
    let theme_line = Line::from(theme_spans);
    let theme_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.accent))
        .title(app.t("theme_selector_title"))
        .title_style(Style::default().fg(theme.primary))
        .style(Style::default().bg(theme.background));
    let theme_paragraph = Paragraph::new(theme_line)
        .alignment(ratatui::layout::Alignment::Center)
        .block(theme_block);
    frame.render_widget(theme_paragraph, themes_area);
    let language_text = match app.language {
        Language::Chinese => "🇨🇳 中文",
        Language::English => "🇬🇧 English",
    };
    let language_spans = vec![Span::styled(
        language_text,
        Style::default()
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD),
    )];
    let language_line = Line::from(language_spans);
    let language_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.accent))
        .title(app.t("language_selector"))
        .title_style(Style::default().fg(theme.primary))
        .style(Style::default().bg(theme.background));
    let language_paragraph = Paragraph::new(language_line)
        .alignment(ratatui::layout::Alignment::Center)
        .block(language_block);
    frame.render_widget(language_paragraph, language_area);
}

fn render_chat_area(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let chat_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.primary))
        .style(Style::default().bg(theme.background));
    frame.render_widget(chat_block, area);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    render_messages(app, frame, inner_area, theme);
}

fn render_messages(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let messages = app.messages.lock().unwrap();
    if messages.is_empty() {
        let empty_text = Paragraph::new(app.t("chat_empty"))
            .style(Style::default().fg(theme.secondary))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(empty_text, area);
        return;
    }
    let mut lines = Vec::new();
    for msg in messages.iter() {
        let prefix = match &msg.sender {
            Sender::User => "👤 ",
            Sender::AI(_) => "🤖 ",
            Sender::Thinking(_) => "🤔 ",
        };
        let timestamp = msg.timestamp.format("%H:%M").to_string();
        lines.push(Line::from(vec![
            Span::styled(
                format!("[{}] ", timestamp),
                Style::default().fg(theme.accent),
            ),
            Span::styled(prefix, Style::default().fg(theme.primary)),
        ]));
        for line in msg.content.lines() {
            lines.push(Line::from(Span::styled(
                format!("  {}", line),
                Style::default().fg(theme.text),
            )));
        }
        lines.push(Line::from(""));
    }
    let scroll_offset = if app.auto_scroll {
        let total_lines = lines.len();
        let viewport_height = area.height as usize;
        if total_lines > viewport_height {
            (total_lines - viewport_height) as u16
        } else {
            0
        }
    } else {
        let selected_index = app.ai_list_state.selected().unwrap_or(0);
        (selected_index * 3).min(lines.len().saturating_sub(1)) as u16
    };
    let paragraph = Paragraph::new(lines)
        .style(Style::default().bg(theme.background))
        .wrap(Wrap { trim: true })
        .scroll((scroll_offset, 0));
    frame.render_widget(paragraph, area);
}

fn render_input_area(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(match app.input_mode {
            InputMode::Normal => Style::default().fg(theme.accent),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .title(match app.input_mode {
            InputMode::Normal => app.t("input_hint_normal"),
            InputMode::Editing => app.t("input_hint_editing"),
        })
        .title_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(theme.background));
    let display_text = if app.input.is_empty() {
        match app.input_mode {
            InputMode::Normal => "".to_string(),
            InputMode::Editing => {
                if app.cursor_blink_state {
                    format!("{}█", app.input)
                } else {
                    format!("{} ", app.input)
                }
            }
        }
    } else {
        if app.cursor_blink_state && app.input_mode == InputMode::Editing {
            format!("{}█", app.input)
        } else {
            format!("{} ", app.input)
        }
    };
    let input_style = match app.input_mode {
        InputMode::Normal => Style::default().fg(theme.text),
        InputMode::Editing => Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    };
    let input_paragraph = Paragraph::new(display_text)
        .style(input_style)
        .block(input_block)
        .wrap(Wrap { trim: true });
    frame.render_widget(input_paragraph, area);
}

fn render_help_modal(app: &App, frame: &mut Frame, area: Rect) {
    let theme = Theme::deep_blue();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(theme.primary))
        .title(app.t("help_title"))
        .title_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(Color::Black));
    let text = vec![
        Line::from(Span::styled(
            match app.language {
                Language::Chinese => "AI 聊天终端 - 用户指南",
                Language::English => "AI Chat Terminal - User Guide",
            },
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            app.t("help_nav_title"),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(app.t("help_nav_line1")),
        Line::from(app.t("help_nav_line2")),
        Line::from(app.t("help_nav_line3")),
        Line::from(app.t("help_nav_line4")),
        Line::from(app.t("help_nav_line5")),
        Line::from(app.t("help_nav_line6")),
        Line::from(app.t("help_nav_line7")),
        Line::from(app.t("help_nav_line8")),
        Line::from(app.t("help_nav_line9")),
        Line::from(app.t("help_nav_line10")),
        Line::from(""),
        Line::from(Span::styled(
            app.t("help_edit_title"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(app.t("help_edit_line1")),
        Line::from(app.t("help_edit_line2")),
        Line::from(app.t("help_edit_line3")),
        Line::from(app.t("help_edit_line4")),
        Line::from(""),
        Line::from(Span::styled(
            app.t("help_theme_title"),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(app.t("help_theme_line1")),
        Line::from(app.t("help_theme_line2")),
        Line::from(app.t("help_theme_line3")),
        Line::from(app.t("help_theme_line4")),
        Line::from(""),
        Line::from(Span::styled(
            app.t("help_tips_title"),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(app.t("help_tips_line1")),
        Line::from(app.t("help_tips_line2")),
        Line::from(app.t("help_tips_line3")),
        Line::from(app.t("help_tips_line4")),
        Line::from(""),
        Line::from(Span::styled(
            app.t("help_close_hint"),
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::ITALIC),
        )),
    ];
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    let area = centered_rect(70, 80, area);
    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

fn render_notification(app: &App, frame: &mut Frame, area: Rect, notification: &str) {
    let theme = match app.theme_index {
        0 => Theme::deep_blue(),
        1 => Theme::forest_green(),
        2 => Theme::sunset(),
        3 => Theme::neon(),
        _ => Theme::deep_blue(),
    };
    let notification_text = vec![
        Line::from(Span::styled(
            app.t("notification_title"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(notification),
        Line::from(""),
        Line::from(Span::styled(
            app.t("notification_continue"),
            Style::default().fg(Color::Gray),
        )),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.primary))
        .title("🔔")
        .style(Style::default().bg(theme.background));
    let paragraph = Paragraph::new(notification_text)
        .block(block)
        .wrap(Wrap { trim: true });
    let notification_area = centered_rect(50, 30, area);
    frame.render_widget(Clear, notification_area);
    frame.render_widget(paragraph, notification_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn darken_color(color: Color, factor: f32) -> Color {
    match color {
        Color::Rgb(r, g, b) => {
            let r = (r as f32 * factor) as u8;
            let g = (g as f32 * factor) as u8;
            let b = (b as f32 * factor) as u8;
            Color::Rgb(r, g, b)
        }
        _ => color,
    }
}
