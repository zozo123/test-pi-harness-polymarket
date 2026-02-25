//! Polymarket Browser — TUI for exploring prediction markets.

use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use polymarket_tui::app::{App, View};
use polymarket_tui::ui;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Run app
    let result = run(&mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

async fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();

    // Auto-load data on start
    app.loading = true;
    terminal.draw(|frame| ui::draw(frame, &app))?;
    app.refresh_data().await;

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if !app.running {
            break;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Search mode
                if app.searching {
                    match key.code {
                        KeyCode::Esc => {
                            app.searching = false;
                            app.search_query.clear();
                            app.filtered_indices.clear();
                            app.market_cursor = 0;
                        }
                        KeyCode::Enter => {
                            app.searching = false;
                        }
                        KeyCode::Backspace => {
                            app.search_query.pop();
                            app.apply_search();
                        }
                        KeyCode::Char(c) => {
                            app.search_query.push(c);
                            app.apply_search();
                        }
                        _ => {}
                    }
                    continue;
                }

                // Global keybindings
                match key.code {
                    // Quit
                    KeyCode::Char('q') => app.running = false,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.running = false
                    }

                    // View switching
                    KeyCode::Tab => app.next_view(),
                    KeyCode::BackTab => app.prev_view(),
                    KeyCode::Char('1') => app.set_view(View::Dashboard),
                    KeyCode::Char('2') => app.set_view(View::Markets),
                    KeyCode::Char('3') => app.set_view(View::Events),
                    KeyCode::Char('4') => app.set_view(View::Spreads),
                    KeyCode::Char('5') => app.set_view(View::Analysis),
                    KeyCode::Char('6') => app.set_view(View::Help),

                    // Navigation
                    KeyCode::Char('j') | KeyCode::Down => app.cursor_down(),
                    KeyCode::Char('k') | KeyCode::Up => app.cursor_up(),
                    KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.page_down()
                    }
                    KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.page_up()
                    }
                    KeyCode::PageDown => app.page_down(),
                    KeyCode::PageUp => app.page_up(),
                    KeyCode::Char('g') => app.cursor_top(),
                    KeyCode::Home => app.cursor_top(),

                    // Search
                    KeyCode::Char('/') => {
                        if app.view == View::Markets || app.view == View::Dashboard {
                            app.searching = true;
                            app.search_query.clear();
                        }
                    }
                    KeyCode::Esc => {
                        if !app.search_query.is_empty() {
                            app.search_query.clear();
                            app.filtered_indices.clear();
                            app.market_cursor = 0;
                        }
                    }

                    // Refresh
                    KeyCode::Char('r') => {
                        app.loading = true;
                        terminal.draw(|frame| ui::draw(frame, &app))?;
                        app.refresh_data().await;
                    }

                    // Refresh spreads (on Spreads view)
                    KeyCode::Char('s') => {
                        if app.view == View::Spreads && !app.loading_spreads {
                            app.loading_spreads = true;
                            terminal.draw(|frame| ui::draw(frame, &app))?;
                            app.refresh_spreads().await;
                        }
                    }

                    // Run analysis
                    KeyCode::Char('a') => {
                        if app.view == View::Analysis {
                            app.status_msg = "Loading analysis…".into();
                            terminal.draw(|frame| ui::draw(frame, &app))?;
                            app.refresh_analysis().await;
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    Ok(())
}
