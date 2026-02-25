//! Headless renderer — prints what the TUI shows, plain text.

use polymarket_tui::api::gamma::GammaClient;
use polymarket_tui::api::clob::ClobClient;
use polymarket_tui::api::types::format_volume;

fn bar(width: usize) -> String { "─".repeat(width) }

#[tokio::main]
async fn main() {
    let gamma = GammaClient::new();
    let clob = ClobClient::new();
    let w = 77;

    // ── DASHBOARD ───────────────────────────────────────────
    println!("┌─ ◆ Polymarket Browser {}┐", bar(w - 27));
    println!("│ [Dashboard]  Markets   Events   Spreads   Help {:>29}│", "");
    println!("├{}┤", bar(w));

    let markets = match gamma.list_markets(200, 0, Some(true), Some(false)).await {
        Ok(m) => m,
        Err(e) => { println!("│ ❌ {:<74}│", e); return; }
    };
    let events = gamma.list_events(100, Some(true), Some(false), None).await.unwrap_or_default();

    let mut sorted = markets.clone();
    sorted.sort_by(|a, b| b.volume_f64().partial_cmp(&a.volume_f64()).unwrap());

    let total_vol: f64 = markets.iter().map(|m| m.volume_f64()).sum();
    let total_24h: f64 = markets.iter().map(|m| m.volume_24h_f64()).sum();
    let close = markets.iter().filter(|m| { let y = m.yes_price(); (0.40..=0.60).contains(&y) && m.volume_f64() > 100_000.0 }).count();

    println!("│ Stats {:>71}│", "");
    println!("│  Markets: {:>3}  │  Volume: {:>8}  │  24h: {:>8}  │  Close: {:>2} {:>7}│",
        markets.len(), format_volume(total_vol), format_volume(total_24h), close, "");
    println!("├{}┤", bar(w));
    println!("│ Top Markets by Volume {:>54}│", "");
    println!("│  {:<48} {:>6}  {:>9} {:>6}│", "Market", "YES", "Volume", "24h");
    println!("│  {:<48} {:>6}  {:>9} {:>6}│", bar(48), bar(6), bar(9), bar(6));

    for (i, m) in sorted.iter().take(12).enumerate() {
        let q = m.short_question(46);
        let yes = m.yes_price();
        let sel = if i == 0 { "►" } else { " " };
        println!("│ {} {:<47} {:>5.1}¢  {:>9} {:>6}│",
            sel, q, yes * 100.0, format_volume(m.volume_f64()), format_volume(m.volume_24h_f64()));
    }

    println!("├{}┤", bar(w));
    println!("│ {} markets, {} events       r:refresh  Tab:view  j/k:nav  /:search  q:quit │",
        markets.len(), events.len());
    println!("└{}┘", bar(w));

    // ── SEARCH ──────────────────────────────────────────────
    println!();
    println!("┌─ ◆ Polymarket Browser {}┐", bar(w - 27));
    println!("│  Dashboard  [Markets]  Events   Spreads   Help {:>29}│", "");
    println!("├─ 🔍 bitcoin {}┤", bar(w - 14));

    let btc: Vec<_> = sorted.iter()
        .filter(|m| m.question.as_deref().unwrap_or("").to_lowercase().contains("bitcoin"))
        .take(6).collect();

    println!("│  {:<48} {:>6}  {:>9} {:>6}│", "Market", "YES", "Volume", "");
    for (i, m) in btc.iter().enumerate() {
        let q = m.short_question(46);
        let sel = if i == 0 { "►" } else { " " };
        println!("│ {} {:<47} {:>5.1}¢  {:>9} {:>6}│",
            sel, q, m.yes_price() * 100.0, format_volume(m.volume_f64()), "");
    }
    println!("├{}┤", bar(w));
    println!("│ {} results   /:search  Esc:clear  j/k:nav {:>35}│", btc.len(), "");
    println!("└{}┘", bar(w));

    // ── EVENTS ──────────────────────────────────────────────
    println!();
    println!("┌─ ◆ Polymarket Browser {}┐", bar(w - 27));
    println!("│  Dashboard   Markets  [Events]  Spreads   Help {:>29}│", "");
    println!("├{}┤", bar(w));
    println!("│  {:<46} {:>4}  {:>7}  {:>9}│", "Event", "Mkts", "Σ YES", "Volume");
    println!("│  {:<46} {:>4}  {:>7}  {:>9}│", bar(46), bar(4), bar(7), bar(9));

    for (i, e) in events.iter().take(8).enumerate() {
        let t = e.title.as_deref().unwrap_or("???");
        let t = if t.len() > 44 { format!("{}…", &t[..43]) } else { t.to_string() };
        let mc = e.market_count();
        let sel = if i == 0 { "►" } else { " " };
        let yes_str = if mc >= 2 { format!("{:.1}%", e.total_yes_probability() * 100.0) } else { "—".into() };
        println!("│ {} {:<45} {:>4}  {:>7}  {:>9}│",
            sel, t, mc, yes_str, format_volume(e.volume_f64()));
    }
    println!("├{}┤", bar(w));
    println!("│ {} events   j/k:nav  Tab:view {:>45}│", events.len(), "");
    println!("└{}┘", bar(w));

    // ── SPREADS ─────────────────────────────────────────────
    println!();
    println!("┌─ ◆ Polymarket Browser {}┐", bar(w - 27));
    println!("│  Dashboard   Markets   Events  [Spreads]  Help {:>29}│", "");
    println!("├{}┤", bar(w));
    println!("│  {:<42} {:>8}  {:>7}  {:>10}│", "Market", "Midpoint", "Spread", "Rating");
    println!("│  {:<42} {:>8}  {:>7}  {:>10}│", bar(42), bar(8), bar(7), bar(10));

    let mut n = 0;
    for m in sorted.iter().take(30) {
        if n >= 8 { break; }
        let tokens = m.parsed_token_ids();
        if tokens.is_empty() { continue; }
        let mid = clob.get_midpoint(&tokens[0]).await.unwrap_or(0.0);
        let spread = clob.get_spread(&tokens[0]).await.unwrap_or(0.0);
        if mid == 0.0 { continue; }

        let q = m.short_question(40);
        let sel = if n == 0 { "►" } else { " " };
        let rating = if spread <= 0.005 { "🟢 Tight" } else if spread <= 0.02 { "🟡 Okay" } else { "🔴 Wide" };

        println!("│ {} {:<41} {:>6.1}¢  {:>5.2}¢    {:>10}│",
            sel, q, mid * 100.0, spread * 100.0, rating);
        n += 1;
    }

    println!("├{}┤", bar(w));
    println!("│ 💡 Spread = your trading cost. Buy then sell = you lose the spread.{:>9}│", "");
    println!("│ 🟢 Tight (<0.5¢)   🟡 Okay (0.5-2¢)   🔴 Wide (>2¢){:>20}│", "");
    println!("├{}┤", bar(w));
    println!("│ s:refresh spreads  j/k:nav  Tab:view  q:quit {:>30}│", "");
    println!("└{}┘", bar(w));
}
