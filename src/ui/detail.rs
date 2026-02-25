//! Market detail view — deep dive into a single market.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::api::types::{format_volume, Market};
use super::theme;

pub fn draw_market_detail(frame: &mut Frame, market: &Market, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),  // header info
            Constraint::Min(8),    // description
            Constraint::Length(6), // price info
        ])
        .split(area);

    draw_header(frame, market, chunks[0]);
    draw_description(frame, market, chunks[1]);
    draw_price_info(frame, market, chunks[2]);
}

fn draw_header(frame: &mut Frame, market: &Market, area: Rect) {
    let q = market.question.as_deref().unwrap_or("???");
    let slug = market.slug.as_deref().unwrap_or("");

    let lines = vec![
        Line::from(Span::styled(
            format!("  {}", q),
            theme::accent_style().add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("  Slug: ", theme::dim_style()),
            Span::raw(slug),
        ]),
        Line::from(vec![
            Span::styled("  Volume: ", theme::dim_style()),
            Span::styled(
                format_volume(market.volume_f64()),
                theme::accent_style(),
            ),
            Span::styled("  │  Liquidity: ", theme::dim_style()),
            Span::styled(
                format_volume(market.liquidity_f64()),
                theme::accent_style(),
            ),
            Span::styled("  │  24h: ", theme::dim_style()),
            Span::styled(
                format_volume(market.volume_24h_f64()),
                theme::accent_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("  End: ", theme::dim_style()),
            Span::raw(market.end_date.as_deref().unwrap_or("N/A")),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" Market Detail ", theme::title_style()));

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_description(frame: &mut Frame, market: &Market, area: Rect) {
    let desc = market
        .description
        .as_deref()
        .unwrap_or("No description available.");

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" Description ", theme::title_style()));

    let paragraph = Paragraph::new(format!("  {}", desc))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn draw_price_info(frame: &mut Frame, market: &Market, area: Rect) {
    let yes = market.yes_price();
    let no = market.no_price();
    let dev = market.price_deviation();

    let yes_bar = price_bar(yes, 30);
    let no_bar = price_bar(no, 30);

    let lines = vec![
        Line::from(vec![
            Span::styled("  YES ", theme::green_style().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:.1}¢ ", yes * 100.0),
                theme::green_style(),
            ),
            Span::styled(&yes_bar, theme::green_style()),
        ]),
        Line::from(vec![
            Span::styled("  NO  ", theme::red_style().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:.1}¢ ", no * 100.0),
                theme::red_style(),
            ),
            Span::styled(&no_bar, theme::red_style()),
        ]),
        Line::from(vec![
            Span::styled("  Sum: ", theme::dim_style()),
            Span::styled(
                format!("{:.2}¢", (yes + no) * 100.0),
                if dev > 0.01 {
                    theme::yellow_style()
                } else {
                    theme::dim_style()
                },
            ),
            if dev > 0.01 {
                Span::styled(
                    format!("  ⚠ deviation: {:.2}¢", dev * 100.0),
                    theme::yellow_style(),
                )
            } else {
                Span::styled("  ✓ prices balanced", theme::dim_style())
            },
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .title(Span::styled(" Prices ", theme::title_style()));

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn price_bar(price: f64, width: usize) -> String {
    let filled = (price * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
