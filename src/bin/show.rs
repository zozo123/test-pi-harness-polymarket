//! Headless demo — renders TUI views with live Polymarket data.

use polymarket_tui::api::gamma::GammaClient;
use polymarket_tui::api::clob::ClobClient;
use polymarket_tui::api::types::format_volume;

#[tokio::main]
async fn main() {
    let gamma = GammaClient::new();
    let clob = ClobClient::new();

    eprintln!("Fetching live data from Polymarket...");
    let markets = gamma.list_markets(200, 0, Some(true), Some(false)).await.unwrap_or_default();
    let events = gamma.list_events(100, Some(true), Some(false), None).await.unwrap_or_default();

    let mut sorted = markets.clone();
    sorted.sort_by(|a, b| b.volume_f64().partial_cmp(&a.volume_f64()).unwrap());

    let total_vol: f64 = markets.iter().map(|m| m.volume_f64()).sum();
    let total_24h: f64 = markets.iter().map(|m| m.volume_24h_f64()).sum();
    let close = markets.iter()
        .filter(|m| { let y = m.yes_price(); (0.40..=0.60).contains(&y) && m.volume_f64() > 100_000.0 })
        .count();

    // ── DASHBOARD ────────────────────────────────────────
    println!("┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐");
    println!("│ [Dashboard]  Markets   Events   Spreads   Help                       │");
    println!("├─ Stats ──────────────────────────────────────────────────────────────┤");
    println!("│  Markets: {}   Volume: {}   24h: {}   Close Races: {}",
        markets.len(), format_volume(total_vol), format_volume(total_24h), close);
    println!("├─ Top Markets by Volume ──────────────────────────────────────────────┤");

    for (i, m) in sorted.iter().take(12).enumerate() {
        let sel = if i == 0 { "►" } else { " " };
        let q = m.short_question(44);
        println!("│ {} {:<44}  {:>5.1}¢  {:>9}  {:>8} │",
            sel, q, m.yes_price() * 100.0, format_volume(m.volume_f64()), format_volume(m.volume_24h_f64()));
    }

    println!("├──────────────────────────────────────────────────────────────────────┤");
    println!("│ {} markets, {} events    r:refresh  Tab:view  j/k:nav  q:quit        │",
        markets.len(), events.len());
    println!("└──────────────────────────────────────────────────────────────────────┘");

    // ── SEARCH: iran ─────────────────────────────────────
    let iran: Vec<_> = sorted.iter()
        .filter(|m| {
            let q = m.question.as_deref().unwrap_or("").to_lowercase();
            q.contains("iran") && m.active == Some(true) && m.closed != Some(true)
        })
        .collect();

    println!();
    println!("┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐");
    println!("│  Dashboard  [Markets]  Events   Spreads   Help                       │");
    println!("├─ 🔍 iran ────────────────────────────────────────────────────────────┤");

    for (i, m) in iran.iter().take(8).enumerate() {
        let sel = if i == 0 { "►" } else { " " };
        let q = m.short_question(48);
        println!("│ {} {:<48}  {:>5.1}¢  {:>12} │",
            sel, q, m.yes_price() * 100.0, format_volume(m.volume_f64()));
    }

    println!("├──────────────────────────────────────────────────────────────────────┤");
    println!("│ {} results   /:search  Esc:clear  j/k:nav                            │", iran.len());
    println!("└──────────────────────────────────────────────────────────────────────┘");

    // ── SEARCH: ukraine ──────────────────────────────────
    let ukraine: Vec<_> = sorted.iter()
        .filter(|m| {
            let q = m.question.as_deref().unwrap_or("").to_lowercase();
            (q.contains("ukraine") || q.contains("ceasefire")) && m.active == Some(true) && m.closed != Some(true)
        })
        .collect();

    println!();
    println!("┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐");
    println!("│  Dashboard  [Markets]  Events   Spreads   Help                       │");
    println!("├─ 🔍 ukraine ─────────────────────────────────────────────────────────┤");

    for (i, m) in ukraine.iter().take(8).enumerate() {
        let sel = if i == 0 { "►" } else { " " };
        let q = m.short_question(48);
        println!("│ {} {:<48}  {:>5.1}¢  {:>12} │",
            sel, q, m.yes_price() * 100.0, format_volume(m.volume_f64()));
    }

    println!("├──────────────────────────────────────────────────────────────────────┤");
    println!("│ {} results   /:search  Esc:clear  j/k:nav                            │", ukraine.len());
    println!("└──────────────────────────────────────────────────────────────────────┘");

    // ── EVENTS ───────────────────────────────────────────
    println!();
    println!("┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐");
    println!("│  Dashboard   Markets  [Events]  Spreads   Help                       │");
    println!("├─ Events ({}) ────────────────────────────────────────────────────────┤", events.len());

    for (i, e) in events.iter().take(10).enumerate() {
        let t = e.title.as_deref().unwrap_or("???");
        let t = if t.len() > 42 { format!("{}…", &t[..41]) } else { t.to_string() };
        let mc = e.market_count();
        let sel = if i == 0 { "►" } else { " " };
        let yes = if mc >= 2 { format!("{:.1}%", e.total_yes_probability() * 100.0) } else { "—".into() };
        println!("│ {} {:<42}  {:>3} mkts  {:>7}  {:>9} │",
            sel, t, mc, yes, format_volume(e.volume_f64()));
    }

    println!("├──────────────────────────────────────────────────────────────────────┤");
    println!("│ {} events   j/k:nav  Tab:view  r:refresh                             │", events.len());
    println!("└──────────────────────────────────────────────────────────────────────┘");

    // ── SPREADS ──────────────────────────────────────────
    println!();
    println!("┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐");
    println!("│  Dashboard   Markets   Events  [Spreads]  Help                       │");
    println!("├──────────────────────────────────────────────────────────────────────┤");

    let mut n = 0;
    for m in sorted.iter().take(40) {
        if n >= 10 { break; }
        let tokens = m.parsed_token_ids();
        if tokens.is_empty() { continue; }
        let mid = clob.get_midpoint(&tokens[0]).await.unwrap_or(0.0);
        let spread = clob.get_spread(&tokens[0]).await.unwrap_or(0.0);
        if mid == 0.0 { continue; }

        let q = m.short_question(38);
        let sel = if n == 0 { "►" } else { " " };
        let rating = if spread <= 0.005 { "🟢 Tight" } else if spread <= 0.02 { "🟡 Okay " } else { "🔴 Wide " };

        println!("│ {} {:<38}  {:>6.1}¢  {:>5.2}¢   {}  {:>9} │",
            sel, q, mid * 100.0, spread * 100.0, rating, format_volume(m.volume_f64()));
        n += 1;
    }

    println!("├──────────────────────────────────────────────────────────────────────┤");
    println!("│ 💡 Spread = your cost. Buy+sell = you lose the spread.               │");
    println!("│ 🟢 <0.5¢ Tight    🟡 0.5-2¢ Okay    🔴 >2¢ Wide                    │");
    println!("├──────────────────────────────────────────────────────────────────────┤");
    println!("│ s:refresh spreads  j/k:nav  Tab:view  q:quit                        │");
    println!("└──────────────────────────────────────────────────────────────────────┘");
}
