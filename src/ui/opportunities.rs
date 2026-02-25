//! Signals view — shows market analysis signals for research.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs};
use ratatui::Frame;

use crate::analysis::engine::SignalKind;
use crate::app::{App, View};
use super::theme;

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // tabs
            Constraint::Length(4),  // summary
            Constraint::Min(10),   // signal list
            Constraint::Length(8), // detail panel
            Constraint::Length(1), // status
        ])
        .split(size);

    draw_tabs(frame, app, chunks[0]);
    draw_summary(frame, app, chunks[1]);
    draw_signal_table(frame, app, chunks[2]);
    draw_signal_detail(frame, app, chunks[3]);
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
                    " ◆ Polymarket Market Scanner ",
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
            Span::styled("  Signals: ", theme::dim_style()),
            Span::styled(
                format!("{}", total),
                theme::accent_style().add_modifier(Modifier::BOLD),
            ),
            Span::styled("  │  ", theme::dim_style()),
            Span::styled("📈 Volume: ", theme::dim_style()),
            Span::styled(format!("{}", by_type("VOLUME")), theme::yellow_style()),
            Span::styled("  🎯 Multi-Mkt: ", theme::dim_style()),
            Span::styled(format!("{}", by_type("MULTI-MKT")), theme::accent_style()),
            Span::styled("  ⏰ Expiring: ", theme::dim_style()),
            Span::styled(format!("{}", by_type("EXPIRING")), Style::default().fg(theme::ACCENT2)),
        ]),
        Line::from(vec![
            Span::styled("  ⚠️  Research signals only — not trading advice", theme::dim_style()),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" Signal Summary ", theme::title_style()));

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_signal_table(frame: &mut Frame, app: &App, area: Rect) {
    let header = Row::new(vec![
        Cell::from("  Relevance").style(theme::header_style()),
        Cell::from("Type").style(theme::header_style()),
        Cell::from("Market / Event").style(theme::header_style()),
        Cell::from("Key Info").style(theme::header_style()),
    ])
    .height(1);

    let rows: Vec<Row> = app
        .opportunities
        .iter()
        .enumerate()
        .map(|(i, signal)| {
            let is_selected = i == app.opp_cursor;
            let score_bar = score_to_bar(signal.relevance);
            let title = {
                let t = signal.title();
                if t.len() > 45 {
                    format!("{}…", &t[..44])
                } else {
                    t
                }
            };

            let key_info = match &signal.kind {
                SignalKind::MultiOutcomePricing { total_yes, market_count, .. } => {
                    format!("Σ{:.0}% / {}mkts", total_yes * 100.0, market_count)
                }
                SignalKind::VolumeAnomaly { spike_ratio, .. } => {
                    format!("{:.1}x spike", spike_ratio)
                }
                SignalKind::HighConfidenceNearExpiry { days_remaining, yes_price, .. } => {
                    format!("{}d / {:.0}¢", days_remaining, yes_price * 100.0)
                }
                SignalKind::HighLiquidity { volume, .. } => {
                    format!("${:.0}", volume)
                }
                SignalKind::WideSpread { spread_cents, .. } => {
                    format!("{:.1}¢ spread", spread_cents)
                }
            };

            let style = if is_selected {
                theme::selected_style()
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(format!("  {}", score_bar)).style(theme::score_style(signal.relevance)),
                Cell::from(signal.label()).style(type_style(signal.label())),
                Cell::from(title),
                Cell::from(key_info).style(theme::dim_style()),
            ])
            .style(style)
            .height(1)
        })
        .collect();

    let title = format!(" Signals ({}) ", app.opportunities.len());

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Length(10),
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

fn draw_signal_detail(frame: &mut Frame, app: &App, area: Rect) {
    let signal = app.opportunities.get(app.opp_cursor);

    let lines = if let Some(s) = signal {
        let mut lines = vec![Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                format!("[{}] ", s.label()),
                type_style(s.label()).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                s.title(),
                theme::accent_style().add_modifier(Modifier::BOLD),
            ),
        ])];

        for detail in s.detail_lines() {
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled(detail, Style::default().fg(theme::FG)),
            ]));
        }

        lines.push(Line::from(vec![
            Span::styled("    Relevance: ", theme::dim_style()),
            Span::styled(
                format!("{} ({:.0}%)", score_to_bar(s.relevance), s.relevance * 100.0),
                theme::score_style(s.relevance),
            ),
        ]));

        lines
    } else {
        vec![Line::from(Span::styled(
            "  No signals — press 'r' to refresh data",
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
                " Signal {}/{} ",
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
        "MULTI-MKT" => theme::accent_style(),
        "VOLUME" => theme::yellow_style(),
        "EXPIRING" => Style::default().fg(theme::ACCENT2),
        "LIQUID" => theme::green_style(),
        "SPREAD" => theme::red_style(),
        _ => Style::default(),
    }
}
