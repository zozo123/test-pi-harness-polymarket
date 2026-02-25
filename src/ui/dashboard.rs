//! Dashboard view — overview with stats and top markets.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs};
use ratatui::Frame;

use crate::api::types::format_volume;
use crate::app::{App, View};
use super::theme;

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // tabs
            Constraint::Length(5),  // stats bar
            Constraint::Min(10),    // main content
            Constraint::Length(1),  // status bar
        ])
        .split(size);

    draw_tabs(frame, app, chunks[0]);
    draw_stats(frame, app, chunks[1]);
    draw_top_markets(frame, app, chunks[2]);
    draw_status(frame, app, chunks[3]);
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

fn draw_stats(frame: &mut Frame, app: &App, area: Rect) {
    let total_markets = app.markets.len();
    let total_volume: f64 = app.markets.iter().map(|m| m.volume_f64()).sum();
    let total_24h: f64 = app.markets.iter().map(|m| m.volume_24h_f64()).sum();
    
    // Close races (40-60% YES)
    let close_races = app.markets.iter()
        .filter(|m| {
            let yes = m.yes_price();
            yes >= 0.40 && yes <= 0.60 && m.volume_f64() > 100_000.0
        })
        .count();

    let stats_text = vec![
        Line::from(vec![
            Span::styled("  Markets: ", theme::dim_style()),
            Span::styled(
                format!("{}", total_markets),
                theme::accent_style().add_modifier(Modifier::BOLD),
            ),
            Span::styled("  │  Total Volume: ", theme::dim_style()),
            Span::styled(
                format_volume(total_volume),
                theme::accent_style().add_modifier(Modifier::BOLD),
            ),
            Span::styled("  │  24h Volume: ", theme::dim_style()),
            Span::styled(format_volume(total_24h), theme::green_style()),
        ]),
        Line::from(vec![
            Span::styled("  Events: ", theme::dim_style()),
            Span::styled(format!("{}", app.events.len()), theme::accent_style()),
            Span::styled("  │  Close Races: ", theme::dim_style()),
            Span::styled(
                format!("{}", close_races),
                if close_races > 0 { theme::yellow_style() } else { theme::dim_style() },
            ),
            Span::styled("  │  Spreads loaded: ", theme::dim_style()),
            Span::styled(
                format!("{}", app.spread_data.len()),
                theme::accent_style(),
            ),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" Stats ", theme::title_style()));

    let paragraph = Paragraph::new(stats_text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_top_markets(frame: &mut Frame, app: &App, area: Rect) {
    let header = Row::new(vec![
        Cell::from("  Market").style(theme::header_style()),
        Cell::from("YES").style(theme::header_style()),
        Cell::from("Volume").style(theme::header_style()),
        Cell::from("24h").style(theme::header_style()),
    ])
    .height(1);

    let rows: Vec<Row> = app
        .markets
        .iter()
        .enumerate()
        .take(50)
        .map(|(i, m)| {
            let is_selected = i == app.market_cursor;
            let q = m.short_question(50);
            let yes = m.yes_price();
            let vol = format_volume(m.volume_f64());
            let vol24 = format_volume(m.volume_24h_f64());

            let style = if is_selected {
                theme::selected_style()
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(format!("  {}", q)),
                Cell::from(format!("{:.1}¢", yes * 100.0)).style(theme::price_style(yes)),
                Cell::from(vol),
                Cell::from(vol24),
            ])
            .style(style)
            .height(1)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Min(40),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(Span::styled(" Top Markets by Volume ", theme::title_style())),
    );

    frame.render_widget(table, area);
}

fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    let status = if let Some(err) = &app.error_msg {
        Line::from(vec![
            Span::styled(" ERROR: ", theme::red_style().add_modifier(Modifier::BOLD)),
            Span::styled(err.as_str(), theme::red_style()),
        ])
    } else if app.loading {
        Line::from(vec![
            Span::styled(" ⟳ ", theme::yellow_style()),
            Span::styled(&app.status_msg, theme::yellow_style()),
        ])
    } else {
        Line::from(vec![
            Span::styled(format!(" {} ", app.status_msg), theme::dim_style()),
            Span::styled("  r:refresh  Tab:view  j/k:nav  /:search  q:quit", theme::dim_style()),
        ])
    };

    frame.render_widget(Paragraph::new(status), area);
}
