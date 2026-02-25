//! Events view — browse multi-market events, spot arbitrage.

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
            Constraint::Min(10),   // table
            Constraint::Length(8), // detail panel
            Constraint::Length(1), // status
        ])
        .split(size);

    draw_tabs(frame, app, chunks[0]);
    draw_event_table(frame, app, chunks[1]);
    draw_event_detail(frame, app, chunks[2]);
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
                .title(Span::styled(
                    " ◆ Polymarket Opportunity Explorer ",
                    theme::title_style(),
                )),
        )
        .select(app.view.index());

    frame.render_widget(tabs, area);
}

fn draw_event_table(frame: &mut Frame, app: &App, area: Rect) {
    let header = Row::new(vec![
        Cell::from("  Event").style(theme::header_style()),
        Cell::from("Markets").style(theme::header_style()),
        Cell::from("Σ YES").style(theme::header_style()),
        Cell::from("Arb Edge").style(theme::header_style()),
        Cell::from("Volume").style(theme::header_style()),
        Cell::from("Status").style(theme::header_style()),
    ])
    .height(1);

    let rows: Vec<Row> = app
        .events
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let is_selected = i == app.event_cursor;
            let title = e
                .title
                .as_deref()
                .unwrap_or("???")
                .chars()
                .take(50)
                .collect::<String>();
            let mkt_count = e.market_count();
            let total_yes = e.total_yes_probability();
            let arb = e.arb_score();
            let vol = format_volume(e.volume_f64());

            let status = if e.closed == Some(true) {
                "Closed"
            } else if e.active == Some(true) {
                "Active"
            } else {
                "—"
            };

            let arb_str = if arb > 0.01 && mkt_count >= 2 {
                format!("{:.2}%", arb * 100.0)
            } else {
                "—".into()
            };

            let arb_style = if arb > 0.05 {
                theme::green_style().add_modifier(Modifier::BOLD)
            } else if arb > 0.02 {
                theme::yellow_style()
            } else {
                theme::dim_style()
            };

            let yes_str = if mkt_count >= 2 {
                format!("{:.1}%", total_yes * 100.0)
            } else {
                "—".into()
            };

            let yes_style = if (total_yes - 1.0).abs() > 0.05 && mkt_count >= 2 {
                theme::yellow_style()
            } else {
                Style::default()
            };

            let style = if is_selected {
                theme::selected_style()
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(format!("  {}", title)),
                Cell::from(format!("{}", mkt_count)).style(theme::accent_style()),
                Cell::from(yes_str).style(yes_style),
                Cell::from(arb_str).style(arb_style),
                Cell::from(vol),
                Cell::from(status).style(if e.closed == Some(true) {
                    theme::red_style()
                } else {
                    theme::green_style()
                }),
            ])
            .style(style)
            .height(1)
        })
        .collect();

    let title = format!(" Events ({}) ", app.events.len());

    let table = Table::new(
        rows,
        [
            Constraint::Min(35),
            Constraint::Length(9),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(8),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(Span::styled(&title, theme::title_style())),
    );

    frame.render_widget(table, area);
}

fn draw_event_detail(frame: &mut Frame, app: &App, area: Rect) {
    let event = app.events.get(app.event_cursor);

    let lines = if let Some(e) = event {
        let mut lines = vec![Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                e.title.as_deref().unwrap_or("???"),
                theme::accent_style().add_modifier(Modifier::BOLD),
            ),
        ])];

        if let Some(markets) = &e.markets {
            for m in markets.iter().take(5) {
                let q = m.short_question(40);
                let yes = m.yes_price();
                lines.push(Line::from(vec![
                    Span::styled("    • ", theme::dim_style()),
                    Span::raw(q),
                    Span::styled(
                        format!("  → {:.1}¢", yes * 100.0),
                        theme::price_style(yes),
                    ),
                ]));
            }
            if markets.len() > 5 {
                lines.push(Line::from(Span::styled(
                    format!("    … and {} more", markets.len() - 5),
                    theme::dim_style(),
                )));
            }
        }
        lines
    } else {
        vec![Line::from(Span::styled(
            "  No event selected",
            theme::dim_style(),
        ))]
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" Event Detail ", theme::title_style()));

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    let line = Line::from(vec![
        Span::styled(
            format!(" Event {}/{} ", app.event_cursor + 1, app.events.len()),
            theme::dim_style(),
        ),
        Span::styled(
            "  j/k:navigate  Tab:view  r:refresh  q:quit",
            theme::dim_style(),
        ),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}
