//! Opportunities view — the money screen. Shows all detected opportunities
//! ranked by score.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs};
use ratatui::Frame;

use crate::app::{App, View};
use super::theme;

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // tabs
            Constraint::Length(4),  // summary
            Constraint::Min(10),   // opportunity list
            Constraint::Length(8), // detail panel
            Constraint::Length(1), // status
        ])
        .split(size);

    draw_tabs(frame, app, chunks[0]);
    draw_summary(frame, app, chunks[1]);
    draw_opp_table(frame, app, chunks[2]);
    draw_opp_detail(frame, app, chunks[3]);
    draw_status(frame, app, chunks[4]);
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

fn draw_summary(frame: &mut Frame, app: &App, area: Rect) {
    let total = app.opportunities.len();
    let by_type = |label: &str| {
        app.opportunities
            .iter()
            .filter(|o| o.label() == label)
            .count()
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("  Total: ", theme::dim_style()),
            Span::styled(
                format!("{}", total),
                theme::accent_style().add_modifier(Modifier::BOLD),
            ),
            Span::styled("  │  ", theme::dim_style()),
            Span::styled("🎯 Event Arb: ", theme::dim_style()),
            Span::styled(format!("{}", by_type("EVENT ARB")), theme::green_style()),
            Span::styled("  📊 Price Dev: ", theme::dim_style()),
            Span::styled(format!("{}", by_type("PRICE DEV")), theme::yellow_style()),
            Span::styled("  📈 Vol Surge: ", theme::dim_style()),
            Span::styled(format!("{}", by_type("VOL SURGE")), theme::accent_style()),
            Span::styled("  ⏰ Near Res: ", theme::dim_style()),
            Span::styled(format!("{}", by_type("NEAR RES")), Style::default().fg(theme::ACCENT2)),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" Opportunity Summary ", theme::title_style()));

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_opp_table(frame: &mut Frame, app: &App, area: Rect) {
    let header = Row::new(vec![
        Cell::from("  Score").style(theme::header_style()),
        Cell::from("Type").style(theme::header_style()),
        Cell::from("Market / Event").style(theme::header_style()),
        Cell::from("Key Metric").style(theme::header_style()),
    ])
    .height(1);

    let rows: Vec<Row> = app
        .opportunities
        .iter()
        .enumerate()
        .map(|(i, opp)| {
            let is_selected = i == app.opp_cursor;
            let score_bar = score_to_bar(opp.score);
            let title = {
                let t = opp.title();
                if t.len() > 50 {
                    format!("{}…", &t[..49])
                } else {
                    t
                }
            };

            let key_metric = match &opp.kind {
                crate::analysis::engine::OpportunityKind::EventArbitrage { edge_pct, .. } => {
                    format!("Edge: {:.2}%", edge_pct)
                }
                crate::analysis::engine::OpportunityKind::PriceDeviation { deviation, .. } => {
                    format!("Dev: {:.2}¢", deviation * 100.0)
                }
                crate::analysis::engine::OpportunityKind::WideSpread { spread_pct, .. } => {
                    format!("Spread: {:.1}%", spread_pct)
                }
                crate::analysis::engine::OpportunityKind::VolumeSurge { surge_ratio, .. } => {
                    format!("Surge: {:.1}x", surge_ratio)
                }
                crate::analysis::engine::OpportunityKind::NearResolution {
                    days_remaining, ..
                } => {
                    format!("{}d left", days_remaining)
                }
            };

            let style = if is_selected {
                theme::selected_style()
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(format!("  {}", score_bar)).style(theme::score_style(opp.score)),
                Cell::from(opp.label()).style(type_style(opp.label())),
                Cell::from(title),
                Cell::from(key_metric).style(theme::accent_style()),
            ])
            .style(style)
            .height(1)
        })
        .collect();

    let title = format!(" Opportunities ({}) ", app.opportunities.len());

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Min(30),
            Constraint::Length(16),
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

fn draw_opp_detail(frame: &mut Frame, app: &App, area: Rect) {
    let opp = app.opportunities.get(app.opp_cursor);

    let lines = if let Some(o) = opp {
        let mut lines = vec![Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                format!("[{}] ", o.label()),
                type_style(o.label()).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                o.title(),
                theme::accent_style().add_modifier(Modifier::BOLD),
            ),
        ])];

        for detail in o.detail_lines() {
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled(detail, Style::default().fg(theme::FG)),
            ]));
        }

        lines.push(Line::from(vec![
            Span::styled("    Score: ", theme::dim_style()),
            Span::styled(
                format!("{} ({:.0}%)", score_to_bar(o.score), o.score * 100.0),
                theme::score_style(o.score),
            ),
        ]));

        lines
    } else {
        vec![Line::from(Span::styled(
            "  No opportunities found — press 'r' to refresh data",
            theme::dim_style(),
        ))]
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" Detail ", theme::title_style()));

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    let line = Line::from(vec![
        Span::styled(
            format!(
                " Opportunity {}/{} ",
                if app.opportunities.is_empty() {
                    0
                } else {
                    app.opp_cursor + 1
                },
                app.opportunities.len(),
            ),
            theme::dim_style(),
        ),
        Span::styled(
            "  j/k:navigate  Tab:view  r:refresh  q:quit",
            theme::dim_style(),
        ),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}

// ── Helpers ─────────────────────────────────────────────────

fn score_to_bar(score: f64) -> String {
    let filled = (score * 5.0).round() as usize;
    let empty = 5 - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn type_style(label: &str) -> Style {
    match label {
        "EVENT ARB" => theme::green_style(),
        "PRICE DEV" => theme::yellow_style(),
        "SPREAD" => theme::accent_style(),
        "VOL SURGE" => Style::default().fg(theme::ACCENT2),
        "NEAR RES" => Style::default().fg(theme::ACCENT2),
        _ => Style::default(),
    }
}
