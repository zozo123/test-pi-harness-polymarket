//! Analysis view — term structures, close races, cross-market insights.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs, Wrap},
};
use crate::app::{App, View};
use crate::api::types::format_volume;
use super::theme;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // tabs
            Constraint::Percentage(50), // term structure
            Constraint::Percentage(35), // close races
            Constraint::Length(4),   // disclaimer
            Constraint::Length(1),   // status
        ])
        .split(frame.area());

    draw_tabs(frame, app, chunks[0]);
    draw_term_structure(frame, app, chunks[1]);
    draw_close_races(frame, app, chunks[2]);
    draw_disclaimer(frame, chunks[3]);
    draw_status(frame, app, chunks[4]);
}

fn draw_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = View::ALL
        .iter()
        .map(|v| {
            let style = if *v == app.view {
                Style::default().fg(theme::TAB_ACTIVE).add_modifier(Modifier::BOLD)
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

fn draw_term_structure(frame: &mut Frame, app: &App, area: Rect) {
    if app.analysis_terms.is_empty() {
        let block = Block::default()
            .title(Span::styled(" 📈 Term Structure ", theme::title_style()))
            .borders(Borders::ALL)
            .border_style(theme::border_style());
        let text = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "  Press 'a' to load analysis (fetches Iran & Ukraine data)",
                theme::dim_style(),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  Shows conditional probabilities from cumulative \"by date\" markets",
                theme::dim_style(),
            )),
        ])
        .block(block);
        frame.render_widget(text, area);
        return;
    }

    let block = Block::default()
        .title(Span::styled(format!(" 📈 {} ", app.analysis_label), theme::title_style()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let header = Row::new(vec![
        Cell::from("Date").style(theme::header_style()),
        Cell::from("Cumulative").style(theme::header_style()),
        Cell::from("Conditional").style(theme::header_style()),
        Cell::from("").style(theme::header_style()),
    ]).height(1);

    let max_rows = (area.height as usize).saturating_sub(3);
    let rows: Vec<Row> = app.analysis_terms.iter().take(max_rows).map(|t| {
        let bar_len = (t.conditional * 50.0) as usize;
        let bar = "█".repeat(bar_len.min(30));
        let cond_color = if t.conditional > 0.10 { Color::Red }
            else if t.conditional > 0.05 { Color::Yellow }
            else { Color::Green };

        Row::new(vec![
            Cell::from(t.label.clone()).style(Style::default().fg(Color::White)),
            Cell::from(format!("{:>6.1}%", t.cumulative * 100.0)).style(Style::default().fg(Color::Cyan)),
            Cell::from(format!("{:>6.1}%", t.conditional * 100.0)).style(Style::default().fg(cond_color)),
            Cell::from(bar).style(Style::default().fg(cond_color)),
        ])
    }).collect();

    let table = Table::new(rows, [
        Constraint::Length(22),
        Constraint::Length(12),
        Constraint::Length(12),
        Constraint::Min(20),
    ])
    .header(header)
    .block(block);

    frame.render_widget(table, area);
}

fn draw_close_races(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(Span::styled(" 🎯 Close Races (35-65%, >$100K vol) ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    if app.close_races.is_empty() {
        let text = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "  Press 'a' to analyze. Shows markets closest to 50/50.",
                theme::dim_style(),
            )),
        ])
        .block(block);
        frame.render_widget(text, area);
        return;
    }

    let header = Row::new(vec![
        Cell::from("  Market").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
        Cell::from("YES").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
        Cell::from("Dist").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
        Cell::from("24h Vol").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
        Cell::from("Total Vol").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
    ]).height(1);

    let max_rows = (area.height as usize).saturating_sub(3);
    let rows: Vec<Row> = app.close_races.iter().take(max_rows).enumerate().map(|(i, r)| {
        let is_selected = i == app.analysis_cursor;
        let sel = if is_selected { "► " } else { "  " };
        let q = r.market.short_question(40);
        let dist_color = if r.distance_from_50 < 0.05 { Color::Green }
            else if r.distance_from_50 < 0.10 { Color::Yellow }
            else { Color::White };

        let style = if is_selected { theme::selected_style() } else { Style::default() };

        Row::new(vec![
            Cell::from(format!("{}{}", sel, q)),
            Cell::from(format!("{:.1}¢", r.yes_price * 100.0)).style(Style::default().fg(Color::Cyan)),
            Cell::from(format!("±{:.1}¢", r.distance_from_50 * 100.0)).style(Style::default().fg(dist_color)),
            Cell::from(format_volume(r.volume_24h)).style(theme::dim_style()),
            Cell::from(format_volume(r.market.volume_f64())).style(theme::dim_style()),
        ]).style(style)
    }).collect();

    let table = Table::new(rows, [
        Constraint::Min(42),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(10),
        Constraint::Length(10),
    ])
    .header(header)
    .block(block);

    frame.render_widget(table, area);
}

fn draw_disclaimer(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style());

    let text = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("  ⚠️  ", Style::default().fg(Color::Red)),
            Span::styled("NOT TRADING ADVICE. ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled("Close races ≠ opportunities. Conditional probs = math, not signals. DYOR.", Style::default().fg(Color::DarkGray)),
        ]),
    ])
    .block(block)
    .wrap(Wrap { trim: true });

    frame.render_widget(text, area);
}

fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    let status = if let Some(err) = &app.error_msg {
        Line::from(vec![
            Span::styled(" ERROR: ", theme::red_style().add_modifier(Modifier::BOLD)),
            Span::styled(err.as_str(), theme::red_style()),
        ])
    } else {
        Line::from(vec![
            Span::styled(format!(" {} ", app.status_msg), theme::dim_style()),
            Span::styled("  a:analyze  j/k:nav  Tab:view  q:quit", theme::dim_style()),
        ])
    };
    frame.render_widget(Paragraph::new(status), area);
}
