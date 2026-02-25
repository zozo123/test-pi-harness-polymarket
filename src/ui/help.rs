//! Help view — keybindings and usage info.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::Frame;

use crate::app::{App, View};
use super::theme;
use ratatui::style::Style;

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // tabs
            Constraint::Min(10),  // help content
        ])
        .split(size);

    draw_tabs(frame, app, chunks[0]);
    draw_help_content(frame, chunks[1]);
}

fn draw_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = View::ALL
        .iter()
        .map(|v| {
            let style = if *v == app.view {
                Style::default()
                    .fg(theme::TAB_ACTIVE)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme::TAB_INACTIVE)
            };
            Line::from(Span::styled(format!(" {} ", v.label()), style))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme::border_style())
                .title(Span::styled(
                    " ◆ Polymarket Opportunity Explorer ",
                    theme::title_style(),
                )),
        )
        .select(app.view.index());

    frame.render_widget(tabs, area);
}

fn draw_help_content(frame: &mut Frame, area: Rect) {
    let sections = vec![
        ("", ""),
        ("  NAVIGATION", ""),
        ("  ─────────────────────────────────────────────", ""),
        ("  Tab / Shift+Tab", "Switch between views"),
        ("  1-5", "Jump to view directly"),
        ("  j / ↓", "Move cursor down"),
        ("  k / ↑", "Move cursor up"),
        ("  Ctrl+d / PgDn", "Page down (10 items)"),
        ("  Ctrl+u / PgUp", "Page up (10 items)"),
        ("  g", "Jump to top"),
        ("  Enter", "Toggle detail view"),
        ("", ""),
        ("  DATA", ""),
        ("  ─────────────────────────────────────────────", ""),
        ("  r", "Refresh all data from Polymarket APIs"),
        ("  /", "Start search (Markets view)"),
        ("  Esc", "Clear search / exit detail"),
        ("", ""),
        ("  VIEWS", ""),
        ("  ─────────────────────────────────────────────", ""),
        ("  Dashboard", "Overview with stats & top markets by volume"),
        ("  Markets", "Searchable list of all active markets"),
        ("  Events", "Multi-market events with arbitrage detection"),
        ("  Opportunities", "All detected opportunities ranked by score"),
        ("  Help", "This screen"),
        ("", ""),
        ("  OPPORTUNITY TYPES", ""),
        ("  ─────────────────────────────────────────────", ""),
        ("  🎯 EVENT ARB", "Multi-outcome events where Σ YES ≠ 100%"),
        ("  📊 PRICE DEV", "Binary markets where YES + NO ≠ $1.00"),
        ("  📈 VOL SURGE", "Abnormal 24h volume spike"),
        ("  ⏰ NEAR RES", "Near expiry with extreme prices"),
        ("  💹 SPREAD", "Wide bid-ask spread (market-making opp)"),
        ("", ""),
        ("  q / Ctrl+c", "Quit"),
    ];

    let lines: Vec<Line> = sections
        .iter()
        .map(|(key, desc)| {
            if desc.is_empty() {
                Line::from(Span::styled(
                    *key,
                    theme::accent_style().add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(vec![
                    Span::styled(
                        format!("{:<30}", key),
                        theme::accent_style(),
                    ),
                    Span::styled(*desc, Style::default().fg(theme::FG)),
                ])
            }
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(
            " Help & Keybindings ",
            theme::title_style(),
        ));

    frame.render_widget(Paragraph::new(lines).block(block), area);
}
