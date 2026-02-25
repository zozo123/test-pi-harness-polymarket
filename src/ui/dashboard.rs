//! Dashboard view — overview with top markets, stats, and quick opportunities.

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
            Constraint::Min(10),   // main content
            Constraint::Length(1), // status bar
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
            Line::from(Span::styled(
                format!(" {} ", v.label()),
                style,
            ))
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
        .select(app.view.index())
        .highlight_style(
            Style::default()
                .fg(theme::TAB_ACTIVE)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, area);
}

fn draw_stats(frame: &mut Frame, app: &App, area: Rect) {
    let total_markets = app.markets.len();
    let active_markets = app
        .markets
        .iter()
        .filter(|m| m.active == Some(true) && m.closed != Some(true))
        .count();
    let total_volume: f64 = app.markets.iter().map(|m| m.volume_f64()).sum();
    let total_opps = app.opportunities.len();
    let high_score_opps = app
        .opportunities
        .iter()
        .filter(|o| o.relevance >= 0.5)
        .count();

    let stats_text = vec![
        Line::from(vec![
            Span::styled("  Markets: ", theme::dim_style()),
            Span::styled(
                format!("{}", total_markets),
                theme::accent_style().add_modifier(Modifier::BOLD),
            ),
            Span::styled("  Active: ", theme::dim_style()),
            Span::styled(format!("{}", active_markets), theme::green_style()),
            Span::styled("  │  Total Volume: ", theme::dim_style()),
            Span::styled(
                format_volume(total_volume),
                theme::accent_style().add_modifier(Modifier::BOLD),
            ),
            Span::styled("  │  Events: ", theme::dim_style()),
            Span::styled(
                format!("{}", app.events.len()),
                theme::accent_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Opportunities: ", theme::dim_style()),
            Span::styled(
                format!("{}", total_opps),
                if total_opps > 0 {
                    theme::green_style().add_modifier(Modifier::BOLD)
                } else {
                    theme::dim_style()
                },
            ),
            Span::styled("  High-score (≥50%): ", theme::dim_style()),
            Span::styled(
                format!("{}", high_score_opps),
                if high_score_opps > 0 {
                    theme::yellow_style().add_modifier(Modifier::BOLD)
                } else {
                    theme::dim_style()
                },
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
        Cell::from("  Question").style(theme::header_style()),
        Cell::from("YES").style(theme::header_style()),
        Cell::from("NO").style(theme::header_style()),
        Cell::from("Volume").style(theme::header_style()),
        Cell::from("24h Vol").style(theme::header_style()),
        Cell::from("Dev").style(theme::header_style()),
    ])
    .height(1);

    let rows: Vec<Row> = app
        .markets
        .iter()
        .enumerate()
        .take(100)
        .map(|(i, m)| {
            let is_selected = i == app.market_cursor;
            let q = m.short_question(55);
            let yes = m.yes_price();
            let no = m.no_price();
            let vol = format_volume(m.volume_f64());
            let vol24 = format_volume(m.volume_24h_f64());
            let dev = m.price_deviation();

            let dev_str = if dev > 0.005 {
                format!("{:.1}¢", dev * 100.0)
            } else {
                "—".into()
            };

            let style = if is_selected {
                theme::selected_style()
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(format!("  {}", q)),
                Cell::from(format!("{:.1}¢", yes * 100.0)).style(theme::price_style(yes)),
                Cell::from(format!("{:.1}¢", no * 100.0)).style(theme::price_style(no)),
                Cell::from(vol),
                Cell::from(vol24),
                Cell::from(dev_str).style(if dev > 0.01 {
                    theme::yellow_style()
                } else {
                    theme::dim_style()
                }),
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
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(6),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(Span::styled(
                " Top Markets by Volume ",
                theme::title_style(),
            )),
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
            Span::styled(app.status_msg.as_str(), theme::yellow_style()),
        ])
    } else {
        Line::from(vec![
            Span::styled(
                format!(" {} ", app.status_msg),
                theme::dim_style(),
            ),
            Span::raw("  "),
            Span::styled(
                "r:refresh  Tab:switch  j/k:navigate  Enter:detail  /:search  q:quit",
                theme::dim_style(),
            ),
        ])
    };

    frame.render_widget(Paragraph::new(status), area);
}
