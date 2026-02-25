//! Spreads view — shows real bid-ask spreads from CLOB API.

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
            Constraint::Min(10),    // table
            Constraint::Length(6),  // explanation
            Constraint::Length(1),  // status
        ])
        .split(size);

    draw_tabs(frame, app, chunks[0]);
    draw_spread_table(frame, app, chunks[1]);
    draw_explanation(frame, chunks[2]);
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
                    " ◆ Polymarket Browser ",
                    theme::title_style(),
                )),
        )
        .select(app.view.index());

    frame.render_widget(tabs, area);
}

fn draw_spread_table(frame: &mut Frame, app: &App, area: Rect) {
    let header = Row::new(vec![
        Cell::from("  Market").style(theme::header_style()),
        Cell::from("Midpoint").style(theme::header_style()),
        Cell::from("Spread").style(theme::header_style()),
        Cell::from("Rating").style(theme::header_style()),
        Cell::from("Volume").style(theme::header_style()),
    ])
    .height(1);

    let rows: Vec<Row> = app
        .spread_data
        .iter()
        .enumerate()
        .map(|(i, (market, mid, spread))| {
            let is_selected = i == app.spread_cursor;
            let q = market.short_question(45);
            
            let spread_rating = if *spread <= 0.005 {
                ("🟢 Tight", theme::green_style())
            } else if *spread <= 0.02 {
                ("🟡 Okay", theme::yellow_style())
            } else {
                ("🔴 Wide", theme::red_style())
            };

            let style = if is_selected {
                theme::selected_style()
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(format!("  {}", q)),
                Cell::from(format!("{:5.1}¢", mid * 100.0)),
                Cell::from(format!("{:5.2}¢", spread * 100.0)),
                Cell::from(spread_rating.0).style(spread_rating.1),
                Cell::from(format_volume(market.volume_f64())),
            ])
            .style(style)
            .height(1)
        })
        .collect();

    let title = if app.loading_spreads {
        " Spreads (loading...) ".to_string()
    } else {
        format!(" Spreads ({}) ", app.spread_data.len())
    };

    let table = Table::new(
        rows,
        [
            Constraint::Min(35),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(12),
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

fn draw_explanation(frame: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(vec![
            Span::styled("  💡 SPREAD = YOUR TRADING COST", theme::accent_style().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  If you buy then immediately sell, you lose the spread.", theme::dim_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  🟢 Tight (<0.5¢) ", theme::green_style()),
            Span::styled("= Low cost   ", theme::dim_style()),
            Span::styled("🟡 Okay (0.5-2¢) ", theme::yellow_style()),
            Span::styled("= Moderate   ", theme::dim_style()),
            Span::styled("🔴 Wide (>2¢) ", theme::red_style()),
            Span::styled("= Expensive", theme::dim_style()),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style());

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    let line = Line::from(vec![
        Span::styled(
            format!(" {} ", if app.loading_spreads { "Loading spreads..." } else { "Press 's' to refresh spreads" }),
            theme::dim_style(),
        ),
        Span::styled("  j/k:nav  Tab:view  q:quit", theme::dim_style()),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}
