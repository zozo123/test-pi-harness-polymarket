//! Help view — keybindings and usage info.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::Frame;

use crate::app::{App, View};
use super::theme;

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
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
                .title(Span::styled(" ◆ Polymarket Browser ", theme::title_style())),
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
        ("  Ctrl+d / PgDn", "Page down"),
        ("  Ctrl+u / PgUp", "Page up"),
        ("  g / Home", "Jump to top"),
        ("", ""),
        ("  ACTIONS", ""),
        ("  ─────────────────────────────────────────────", ""),
        ("  r", "Refresh market data"),
        ("  s", "Refresh spreads (on Spreads view)"),
        ("  /", "Search markets"),
        ("  Esc", "Clear search"),
        ("  q / Ctrl+c", "Quit"),
        ("", ""),
        ("  VIEWS", ""),
        ("  ─────────────────────────────────────────────", ""),
        ("  1 Dashboard", "Stats and top markets by volume"),
        ("  2 Markets", "Browse all markets, search, filter"),
        ("  3 Events", "Multi-market events"),
        ("  4 Spreads", "Real bid-ask spreads from CLOB"),
        ("  5 Help", "This screen"),
        ("", ""),
        ("  USING WITH POLYMARKET CLI", ""),
        ("  ─────────────────────────────────────────────", ""),
        ("  1. Find markets here", "Browse, search, check spreads"),
        ("  2. Trade via CLI", "polymarket clob market-order ..."),
        ("  ", ""),
        ("  Install CLI:", "brew install polymarket"),
        ("  Docs:", "github.com/Polymarket/polymarket-cli"),
        ("", ""),
        ("  ⚠️  NOT TRADING ADVICE — DYOR", ""),
    ];

    let lines: Vec<Line> = sections
        .iter()
        .map(|(key, desc)| {
            if desc.is_empty() {
                Line::from(Span::styled(
                    *key,
                    if key.contains("─") {
                        theme::dim_style()
                    } else {
                        theme::accent_style().add_modifier(Modifier::BOLD)
                    },
                ))
            } else {
                Line::from(vec![
                    Span::styled(format!("{:<25}", key), theme::accent_style()),
                    Span::styled(*desc, Style::default().fg(theme::FG)),
                ])
            }
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" Help ", theme::title_style()));

    frame.render_widget(Paragraph::new(lines).block(block), area);
}
