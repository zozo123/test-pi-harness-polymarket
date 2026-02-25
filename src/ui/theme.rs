use ratatui::style::{Color, Modifier, Style};

// ── Color palette ───────────────────────────────────────────

pub const BG: Color = Color::Reset;
pub const FG: Color = Color::White;
pub const ACCENT: Color = Color::Cyan;
pub const ACCENT2: Color = Color::Magenta;
pub const GREEN: Color = Color::Green;
pub const RED: Color = Color::Red;
pub const YELLOW: Color = Color::Yellow;
pub const DIM: Color = Color::DarkGray;
pub const HIGHLIGHT_BG: Color = Color::Rgb(30, 40, 60);
pub const TAB_ACTIVE: Color = Color::Cyan;
pub const TAB_INACTIVE: Color = Color::DarkGray;
pub const BORDER: Color = Color::Rgb(60, 65, 80);

// ── Style presets ───────────────────────────────────────────

pub fn title_style() -> Style {
    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
}

pub fn header_style() -> Style {
    Style::default()
        .fg(ACCENT)
        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
}

pub fn selected_style() -> Style {
    Style::default().bg(HIGHLIGHT_BG).fg(FG)
}

pub fn dim_style() -> Style {
    Style::default().fg(DIM)
}

pub fn green_style() -> Style {
    Style::default().fg(GREEN)
}

pub fn red_style() -> Style {
    Style::default().fg(RED)
}

pub fn yellow_style() -> Style {
    Style::default().fg(YELLOW)
}

pub fn accent_style() -> Style {
    Style::default().fg(ACCENT)
}

pub fn score_style(score: f64) -> Style {
    if score >= 0.7 {
        Style::default().fg(GREEN).add_modifier(Modifier::BOLD)
    } else if score >= 0.4 {
        Style::default().fg(YELLOW)
    } else {
        Style::default().fg(DIM)
    }
}

pub fn price_style(price: f64) -> Style {
    if price >= 0.8 {
        green_style()
    } else if price <= 0.2 {
        red_style()
    } else {
        Style::default().fg(FG)
    }
}

pub fn border_style() -> Style {
    Style::default().fg(BORDER)
}
