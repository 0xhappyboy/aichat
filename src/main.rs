mod ai;
mod ai_models;
mod app;
mod i18n;
mod ui;

use app::{App, AppState, InputMode};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    res
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    let mut last_blink_update = Instant::now();
    loop {
        if last_blink_update.elapsed() >= Duration::from_millis(500) {
            app.update_cursor_blink();
            last_blink_update = Instant::now();
        }
        terminal.draw(|f| ui::render(app, f))?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.app_state {
                        AppState::Welcome => match key.code {
                            KeyCode::Enter => app.start_chatting(),
                            KeyCode::Char('c') | KeyCode::Char('C') => app.switch_to_chinese(),
                            KeyCode::Char('e') | KeyCode::Char('E') => app.switch_to_english(),
                            KeyCode::Char('1') => app.change_theme(0),
                            KeyCode::Char('2') => app.change_theme(1),
                            KeyCode::Char('3') => app.change_theme(2),
                            KeyCode::Char('4') => app.change_theme(3),
                            KeyCode::F(1) => app.show_help = true,
                            KeyCode::Char('q') => return Ok(()),
                            _ => {}
                        },
                        AppState::Chatting => {
                            if app.show_help {
                                app.show_help = false;
                            } else {
                                match app.input_mode {
                                    InputMode::Normal => match key.code {
                                        KeyCode::Left => {
                                            if app.input_mode == InputMode::Normal {
                                                let available_width = 100;
                                                let max_visible =
                                                    app.calculate_max_visible(available_width);
                                                app.select_previous_model(max_visible);
                                            }
                                        }
                                        KeyCode::Right => {
                                            if app.input_mode == InputMode::Normal {
                                                let available_width = 100;
                                                let max_visible =
                                                    app.calculate_max_visible(available_width);
                                                app.select_next_model(max_visible);
                                            }
                                        }
                                        KeyCode::Up => {
                                            let current = app.ai_list_state.selected().unwrap_or(0);
                                            let max_scroll = app.get_max_scroll_offset();
                                            if current < max_scroll {
                                                app.ai_list_state.select(Some(current + 1));
                                                app.auto_scroll = false;
                                            }
                                        }
                                        KeyCode::Down => {
                                            let current = app.ai_list_state.selected().unwrap_or(0);
                                            if current > 0 {
                                                app.ai_list_state.select(Some(current - 1));
                                                app.auto_scroll = false;
                                            }
                                        }
                                        KeyCode::PageUp => {
                                            let current = app.ai_list_state.selected().unwrap_or(0);
                                            let max_scroll = app.get_max_scroll_offset();
                                            app.ai_list_state
                                                .select(Some((current + 10).min(max_scroll)));
                                            app.auto_scroll = false;
                                        }
                                        KeyCode::PageDown => {
                                            let current = app.ai_list_state.selected().unwrap_or(0);
                                            app.ai_list_state
                                                .select(Some(current.saturating_sub(10).max(0)));
                                            app.auto_scroll = false;
                                        }
                                        KeyCode::Home => {
                                            app.ai_list_state.select(Some(0));
                                            app.auto_scroll = false;
                                        }
                                        KeyCode::End => {
                                            app.scroll_to_end();
                                        }
                                        KeyCode::Char('i') => app.input_mode = InputMode::Editing,
                                        KeyCode::Char('c') | KeyCode::Char('C') => {
                                            app.switch_to_chinese()
                                        }
                                        KeyCode::Char('e') | KeyCode::Char('E') => {
                                            app.switch_to_english()
                                        }
                                        KeyCode::Enter => {
                                            if !app.input.is_empty() {
                                                app.send_message();
                                                app.scroll_to_end();
                                            }
                                        }
                                        KeyCode::F(1) => app.toggle_help(),
                                        KeyCode::Char('1') => app.change_theme(0),
                                        KeyCode::Char('2') => app.change_theme(1),
                                        KeyCode::Char('3') => app.change_theme(2),
                                        KeyCode::Char('4') => app.change_theme(3),
                                        KeyCode::Char('q') => return Ok(()),
                                        _ => {}
                                    },
                                    InputMode::Editing => match key.code {
                                        KeyCode::Enter => {
                                            app.send_message();
                                            app.input_mode = InputMode::Normal;
                                        }
                                        KeyCode::Esc => {
                                            app.clear_input();
                                            app.input_mode = InputMode::Normal;
                                        }
                                        KeyCode::Char(c) => {
                                            app.input.push(c);
                                        }
                                        KeyCode::Backspace => {
                                            app.input.pop();
                                        }
                                        KeyCode::Delete => {
                                            app.clear_input();
                                        }
                                        _ => {}
                                    },
                                }
                            }
                        }
                        AppState::Help => {
                            if key.kind == KeyEventKind::Press {
                                app.show_help = false;
                            }
                        }
                    }
                    app.clear_notification();
                }
            }
        }
    }
}
