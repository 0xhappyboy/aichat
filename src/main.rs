mod ai_models;
mod app;
mod ui;
mod i18n;

use app::{App, InputMode};
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
                    match app.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Left => app.select_previous_model(),
                            KeyCode::Right => app.select_next_model(),
                            KeyCode::Up => app.scroll_up(),
                            KeyCode::Down => app.scroll_down(),
                            KeyCode::PageUp => {
                                for _ in 0..5 {
                                    app.scroll_up();
                                }
                            }
                            KeyCode::PageDown => {
                                for _ in 0..5 {
                                    app.scroll_down();
                                }
                            }
                            KeyCode::Home => {
                                let ai_messages_count = app
                                    .messages
                                    .iter()
                                    .filter(|msg| matches!(msg.sender, app::Sender::AI(_)))
                                    .count();
                                let user_messages_count = app.messages.len() - ai_messages_count;
                                app.ai_list_state
                                    .select(Some(ai_messages_count.saturating_sub(1)));
                                app.user_list_state
                                    .select(Some(user_messages_count.saturating_sub(1)));
                            }
                            KeyCode::End => {
                                app.ai_list_state.select(Some(0));
                                app.user_list_state.select(Some(0));
                            }
                            KeyCode::Char('i') => app.input_mode = InputMode::Editing,
                            KeyCode::Char('c') => app.switch_to_chinese(),
                            KeyCode::Char('C') => app.switch_to_chinese(),
                            KeyCode::Char('e') => app.switch_to_english(),
                            KeyCode::Char('E') => app.switch_to_english(),
                            KeyCode::Enter => {
                                if !app.input.is_empty() {
                                    app.send_message();
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
                    app.clear_notification();
                    if app.show_help && key.kind == KeyEventKind::Press {
                        app.show_help = false;
                    }
                }
            }
        }
    }
}