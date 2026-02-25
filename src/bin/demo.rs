//! Animated demo — outputs each view with pauses for recording.

use polymarket_tui::analysis;
use polymarket_tui::api::gamma::GammaClient;
use polymarket_tui::api::clob::ClobClient;
use polymarket_tui::api::types::format_volume;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn pause(ms: u64) {
    io::stdout().flush().unwrap_or(());
    thread::sleep(Duration::from_millis(ms));
}

fn type_line(s: &str, ms_per_char: u64) {
    for c in s.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap_or(());
        thread::sleep(Duration::from_millis(ms_per_char));
    }
    println!();
}

#[tokio::main]
async fn main() {
    let gamma = GammaClient::new();
    let clob = ClobClient::new();

    // Prefetch everything before the demo starts
    eprintln!("Prefetching data…");
    let markets = gamma.list_markets(200, 0, Some(true), Some(false)).await.unwrap_or_default();
    let events = gamma.list_events(100, Some(true), Some(false), None).await.unwrap_or_default();
    let mut all_markets = markets.clone();
    for term in &["us strikes iran", "khamenei", "israel strikes iran", "ukraine ceasefire", "russia ceasefire", "zelenskyy"] {
        if let Ok(extra) = gamma.search_markets(term, 50).await {
            for m in extra {
                if !all_markets.iter().any(|e| e.id == m.id) {
                    all_markets.push(m);
                }
            }
        }
    }
    let mut sorted = markets.clone();
    sorted.sort_by(|a, b| b.volume_f64().partial_cmp(&a.volume_f64()).unwrap_or(std::cmp::Ordering::Equal));

    let total_vol: f64 = markets.iter().map(|m| m.volume_f64()).sum();
    let total_24h: f64 = markets.iter().map(|m| m.volume_24h_f64()).sum();
    let close = markets.iter()
        .filter(|m| { let y = m.yes_price(); (0.40..=0.60).contains(&y) && m.volume_f64() > 100_000.0 })
        .count();

    // Prefetch spreads
    let mut spread_data = Vec::new();
    for m in sorted.iter().take(8) {
        let tokens = m.parsed_token_ids();
        if tokens.is_empty() { continue; }
        let mid = clob.get_midpoint(&tokens[0]).await.unwrap_or(0.0);
        let spread = clob.get_spread(&tokens[0]).await.unwrap_or(0.0);
        if mid > 0.0 { spread_data.push((m, mid, spread)); }
    }

    let iran_terms = analysis::build_term_structure(&all_markets, "us strikes iran by");
    let close_races = analysis::find_close_races(&all_markets, 100_000.0);

    eprintln!("Data ready. Starting demo…");

    // ══════════════════════════════════════════════════════
    // SCENE 1: Dashboard
    // ══════════════════════════════════════════════════════
    println!("\x1b[36;1m┌─ ◆ Polymarket Browser ────────────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[36;1m│\x1b[0m \x1b[36;1;4m Dashboard \x1b[0m  Markets   Events   Spreads   Analysis   Help               \x1b[36;1m│\x1b[0m");
    println!("\x1b[36;1m├─ Stats ───────────────────────────────────────────────────────────────────┤\x1b[0m");
    println!("\x1b[36;1m│\x1b[0m  Markets: \x1b[36;1m{}\x1b[0m   Volume: \x1b[36;1m{}\x1b[0m   24h: \x1b[32m{}\x1b[0m   Close Races: \x1b[33m{}\x1b[0m",
        markets.len(), format_volume(total_vol), format_volume(total_24h), close);
    println!("\x1b[36;1m├─ Top Markets by Volume ───────────────────────────────────────────────────┤\x1b[0m");
    pause(200);

    for (i, m) in sorted.iter().take(10).enumerate() {
        let sel = if i == 0 { "\x1b[46;30m►\x1b[0m" } else { " " };
        let q = m.short_question(48);
        let yes = m.yes_price();
        let yc = if yes >= 0.8 { "32" } else if yes <= 0.2 { "31" } else { "0" };
        println!("\x1b[36;1m│\x1b[0m {} {:<48}  \x1b[{}m{:>5.1}¢\x1b[0m  {:>9}  {:>8} \x1b[36;1m│\x1b[0m",
            sel, q, yc, yes * 100.0, format_volume(m.volume_f64()), format_volume(m.volume_24h_f64()));
        pause(80);
    }

    println!("\x1b[36;1m├──────────────────────────────────────────────────────────────────────────┤\x1b[0m");
    println!("\x1b[36;1m│\x1b[0m \x1b[90m{} markets, {} events   r:refresh  Tab:view  j/k:nav  /:search  q:quit\x1b[0m \x1b[36;1m│\x1b[0m",
        markets.len(), events.len());
    println!("\x1b[36;1m└──────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    pause(3000);

    // ══════════════════════════════════════════════════════
    // SCENE 3: Search Iran
    // ══════════════════════════════════════════════════════
    print!("\x1b[2J\x1b[H");
    println!("\x1b[36;1m┌─ ◆ Polymarket Browser ────────────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[36;1m│\x1b[0m  Dashboard  \x1b[36;1;4m Markets \x1b[0m  Events   Spreads   Analysis   Help               \x1b[36;1m│\x1b[0m");
    println!("\x1b[36;1m├─ 🔍\x1b[0m \x1b[36miran\x1b[0m \x1b[36;1m──────────────────────────────────────────────────────────────┤\x1b[0m");
    pause(500);

    let iran: Vec<_> = all_markets.iter()
        .filter(|m| {
            let q = m.question.as_deref().unwrap_or("").to_lowercase();
            q.contains("iran") && m.active == Some(true) && m.closed != Some(true)
            && !q.contains("next strike iran on") && !q.contains("week of")
            && !q.contains("meeting be in") && !q.contains("next diplomatic")
            && !q.contains("miran")
        })
        .collect();
    let mut iran_sorted = iran.clone();
    iran_sorted.sort_by(|a, b| b.volume_f64().partial_cmp(&a.volume_f64()).unwrap_or(std::cmp::Ordering::Equal));

    for (i, m) in iran_sorted.iter().take(10).enumerate() {
        let sel = if i == 0 { "\x1b[46;30m►\x1b[0m" } else { " " };
        let q = m.short_question(52);
        let yes = m.yes_price();
        let yc = if yes >= 0.5 { "33" } else { "0" };
        println!("\x1b[36;1m│\x1b[0m {} {:<52}  \x1b[{}m{:>5.1}¢\x1b[0m  {:>10} \x1b[36;1m│\x1b[0m",
            sel, q, yc, yes * 100.0, format_volume(m.volume_f64()));
        pause(80);
    }

    println!("\x1b[36;1m├──────────────────────────────────────────────────────────────────────────┤\x1b[0m");
    println!("\x1b[36;1m│\x1b[0m \x1b[90m{} results   /:search  Esc:clear  j/k:nav\x1b[0m                              \x1b[36;1m│\x1b[0m", iran_sorted.len());
    println!("\x1b[36;1m└──────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    pause(3000);

    // ══════════════════════════════════════════════════════
    // SCENE 4: Analysis — Term Structure
    // ══════════════════════════════════════════════════════
    print!("\x1b[2J\x1b[H");
    println!("\x1b[36;1m┌─ ◆ Polymarket Browser ────────────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[36;1m│\x1b[0m  Dashboard   Markets   Events   Spreads  \x1b[36;1;4m Analysis \x1b[0m  Help               \x1b[36;1m│\x1b[0m");
    println!("\x1b[36;1m├─ 📈 US Strikes Iran — Term Structure ─────────────────────────────────────┤\x1b[0m");
    println!("\x1b[36;1m│\x1b[0m  \x1b[36;1mDate                   Cumul.   Cond.                                  \x1b[0m \x1b[36;1m│\x1b[0m");
    pause(300);

    for t in &iran_terms {
        let bar_len = (t.conditional * 60.0) as usize;
        let bar = "█".repeat(bar_len.min(35));
        let (marker, color) = if t.conditional > 0.10 { ("🔴", "31") }
            else if t.conditional > 0.05 { ("🟡", "33") }
            else { ("🟢", "32") };
        println!("\x1b[36;1m│\x1b[0m  {:<22} {:>5.1}%   {:>5.1}%  {} \x1b[{}m{}\x1b[0m",
            t.label, t.cumulative * 100.0, t.conditional * 100.0, marker, color, bar);
        pause(120);
    }

    println!("\x1b[36;1m├─ 🎯 Close Races (35-65%) ─────────────────────────────────────────────────┤\x1b[0m");
    pause(200);

    for (i, r) in close_races.iter().take(5).enumerate() {
        let sel = if i == 0 { "\x1b[46;30m►\x1b[0m" } else { " " };
        let q = r.market.short_question(44);
        let dm = if r.distance_from_50 < 0.05 { "🟢" } else { "🟡" };
        println!("\x1b[36;1m│\x1b[0m {} {:<44} {:>5.1}¢  ±{:.1}¢ {}  {:>9} \x1b[36;1m│\x1b[0m",
            sel, q, r.yes_price * 100.0, r.distance_from_50 * 100.0, dm, format_volume(r.volume_24h));
        pause(80);
    }

    println!("\x1b[36;1m├──────────────────────────────────────────────────────────────────────────┤\x1b[0m");
    println!("\x1b[36;1m│\x1b[0m \x1b[31m⚠️  NOT TRADING ADVICE. Close ≠ opportunity. Math ≠ signal. DYOR.\x1b[0m       \x1b[36;1m│\x1b[0m");
    println!("\x1b[36;1m└──────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    pause(4000);

    // ══════════════════════════════════════════════════════
    // SCENE 5: Spreads
    // ══════════════════════════════════════════════════════
    print!("\x1b[2J\x1b[H");
    println!("\x1b[36;1m┌─ ◆ Polymarket Browser ────────────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[36;1m│\x1b[0m  Dashboard   Markets   Events  \x1b[36;1;4m Spreads \x1b[0m  Analysis   Help               \x1b[36;1m│\x1b[0m");
    println!("\x1b[36;1m├──────────────────────────────────────────────────────────────────────────┤\x1b[0m");
    println!("\x1b[36;1m│\x1b[0m  \x1b[36;1mMarket                                     Mid     Spread   Rating     \x1b[0m \x1b[36;1m│\x1b[0m");
    pause(200);

    for (m, mid, spread) in &spread_data {
        let q = m.short_question(42);
        let rating = if *spread <= 0.005 { "\x1b[32m🟢 Tight\x1b[0m" }
            else if *spread <= 0.02 { "\x1b[33m🟡 Okay\x1b[0m" }
            else { "\x1b[31m🔴 Wide\x1b[0m" };
        println!("\x1b[36;1m│\x1b[0m  {:<42} {:>5.1}¢   {:>5.2}¢   {}  {:>9} \x1b[36;1m│\x1b[0m",
            q, mid * 100.0, spread * 100.0, rating, format_volume(m.volume_f64()));
        pause(100);
    }

    println!("\x1b[36;1m├──────────────────────────────────────────────────────────────────────────┤\x1b[0m");
    println!("\x1b[36;1m│\x1b[0m \x1b[36;1m💡\x1b[0m \x1b[90mSpread = your cost. Buy + sell = you lose the spread.\x1b[0m                \x1b[36;1m│\x1b[0m");
    println!("\x1b[36;1m│\x1b[0m \x1b[32m🟢 <0.5¢ Tight\x1b[0m    \x1b[33m🟡 0.5-2¢ Okay\x1b[0m    \x1b[31m🔴 >2¢ Wide\x1b[0m                     \x1b[36;1m│\x1b[0m");
    println!("\x1b[36;1m└──────────────────────────────────────────────────────────────────────────┘\x1b[0m");
    println!();
    println!("  \x1b[90m⚠️  Not trading advice. DYOR. You can lose money.\x1b[0m");
    println!();
    pause(3000);
}
