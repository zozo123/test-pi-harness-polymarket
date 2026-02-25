//! Markets browser view — searchable, sortable list of all markets.

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

    let mut constraints = vec![
        Constraint::Length(3), // tabs
    ];
    if app.searching || !app.search_query.is_empty() {
        constraints.push(Constraint::Length(3)); // search bar
    }
    constraints.push(Constraint::Min(10)); // table
    constraints.push(Constraint::Length(1)); // status

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(size);

    let mut idx = 0;
    draw_tabs(frame, app, chunks[idx]);
    idx += 1;

    if app.searching || !app.search_query.is_empty() {
        draw_search(frame, app, chunks[idx]);
        idx += 1;
    }

    draw_market_table(frame, app, chunks[idx]);
    idx += 1;
    draw_status(frame, app, chunks[idx]);
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

fn draw_search(frame: &mut Frame, app: &App, area: Rect) {
    let cursor = if app.searching { "▊" } else { "" };
    let line = Line::from(vec![
        Span::styled(" 🔍 ", theme::accent_style()),
        Span::styled(&app.search_query, Style::default().fg(theme::FG)),
        Span::styled(cursor, theme::accent_style()),
        Span::styled(
            format!("  ({} results)", app.visible_market_count()),
            theme::dim_style(),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" Search ", theme::title_style()));

    frame.render_widget(Paragraph::new(line).block(block), area);
}

fn draw_market_table(frame: &mut Frame, app: &App, area: Rect) {
    let header = Row::new(vec![
        Cell::from("  #").style(theme::header_style()),
        Cell::from("Question").style(theme::header_style()),
        Cell::from("YES").style(theme::header_style()),
        Cell::from("NO").style(theme::header_style()),
        Cell::from("Volume").style(theme::header_style()),
        Cell::from("Liq").style(theme::header_style()),
        Cell::from("24h").style(theme::header_style()),
        Cell::from("Status").style(theme::header_style()),
    ])
    .height(1);

    let visible = app.visible_markets();

    let rows: Vec<Row> = visible
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let is_selected = i == app.market_cursor;
            let q = m.short_question(50);
            let yes = m.yes_price();
            let no = m.no_price();
            let vol = format_volume(m.volume_f64());
            let liq = format_volume(m.liquidity_f64());
            let vol24 = format_volume(m.volume_24h_f64());

            let status = if m.closed == Some(true) {
                "Closed"
            } else if m.active == Some(true) {
                "Active"
            } else {
                "—"
            };

            let style = if is_selected {
                theme::selected_style()
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(format!("  {}", i + 1)).style(theme::dim_style()),
                Cell::from(q),
                Cell::from(format!("{:.1}¢", yes * 100.0)).style(theme::price_style(yes)),
                Cell::from(format!("{:.1}¢", no * 100.0)).style(theme::price_style(no)),
                Cell::from(vol),
                Cell::from(liq).style(theme::dim_style()),
                Cell::from(vol24),
                Cell::from(status).style(if m.closed == Some(true) {
                    theme::red_style()
                } else {
                    theme::green_style()
                }),
            ])
            .style(style)
            .height(1)
        })
        .collect();

    let title = format!(" Markets ({}) ", visible.len());

    let table = Table::new(
        rows,
        [
            Constraint::Length(5),
            Constraint::Min(35),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(9),
            Constraint::Length(9),
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

fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    let line = Line::from(vec![
        Span::styled(
            format!(
                " Market {}/{} ",
                app.market_cursor + 1,
                app.visible_market_count()
            ),
            theme::dim_style(),
        ),
        Span::styled(
            "  /:search  Esc:clear  j/k:nav  Tab:view  r:refresh  q:quit",
            theme::dim_style(),
        ),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}
