//! Headless demo — renders all TUI views with live Polymarket data.

use polymarket_tui::analysis;
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

    // Get broader set for analysis
    eprintln!("Fetching Iran, Ukraine, and geopolitical markets...");
    let mut all_markets = markets.clone();
    for term in &["us strikes iran", "khamenei", "israel strikes iran", "iran strike", "iran nuclear", "iran strait", "ukraine ceasefire", "russia ceasefire", "zelenskyy"] {
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

    // ── DASHBOARD ────────────────────────────────────────
    println!("┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐");
    println!("│ [Dashboard]  Markets   Events   Spreads   Analysis   Help            │");
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
    let iran: Vec<_> = all_markets.iter()
        .filter(|m| {
            let q = m.question.as_deref().unwrap_or("").to_lowercase();
            q.contains("iran") && m.active == Some(true) && m.closed != Some(true)
            && !q.contains("next strike iran on") && !q.contains("week of")
            && !q.contains("meeting be in") && !q.contains("next diplomatic")
            && !q.contains("miran")  // exclude Stephen Miran
        })
        .collect();

    let mut iran_sorted = iran.clone();
    iran_sorted.sort_by(|a, b| b.volume_f64().partial_cmp(&a.volume_f64()).unwrap_or(std::cmp::Ordering::Equal));

    println!();
    println!("┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐");
    println!("│  Dashboard  [Markets]  Events   Spreads   Analysis   Help            │");
    println!("├─ 🔍 iran ────────────────────────────────────────────────────────────┤");

    for (i, m) in iran_sorted.iter().take(10).enumerate() {
        let sel = if i == 0 { "►" } else { " " };
        let q = m.short_question(48);
        println!("│ {} {:<48}  {:>5.1}¢  {:>10} │",
            sel, q, m.yes_price() * 100.0, format_volume(m.volume_f64()));
    }

    println!("├──────────────────────────────────────────────────────────────────────┤");
    println!("│ {} results   /:search  Esc:clear  j/k:nav                            │", iran_sorted.len());
    println!("└──────────────────────────────────────────────────────────────────────┘");

    // ── SEARCH: ukraine ──────────────────────────────────
    let ukraine: Vec<_> = all_markets.iter()
        .filter(|m| {
            let q = m.question.as_deref().unwrap_or("").to_lowercase();
            (q.contains("ukraine") || (q.contains("russia") && q.contains("ceasefire")))
            && m.active == Some(true) && m.closed != Some(true)
        })
        .collect();

    let mut ukr_sorted = ukraine.clone();
    ukr_sorted.sort_by(|a, b| b.volume_f64().partial_cmp(&a.volume_f64()).unwrap_or(std::cmp::Ordering::Equal));

    println!();
    println!("┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐");
    println!("│  Dashboard  [Markets]  Events   Spreads   Analysis   Help            │");
    println!("├─ 🔍 ukraine ─────────────────────────────────────────────────────────┤");

    for (i, m) in ukr_sorted.iter().take(10).enumerate() {
        let sel = if i == 0 { "►" } else { " " };
        let q = m.short_question(48);
        println!("│ {} {:<48}  {:>5.1}¢  {:>10} │",
            sel, q, m.yes_price() * 100.0, format_volume(m.volume_f64()));
    }

    println!("├──────────────────────────────────────────────────────────────────────┤");
    println!("│ {} results   /:search  Esc:clear  j/k:nav                            │", ukr_sorted.len());
    println!("└──────────────────────────────────────────────────────────────────────┘");

    // ── ANALYSIS: TERM STRUCTURE ─────────────────────────
    let iran_terms = analysis::build_term_structure(&all_markets, "us strikes iran by");

    println!();
    println!("┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐");
    println!("│  Dashboard   Markets   Events   Spreads  [Analysis]  Help            │");
    println!("├─ 📈 US Strikes Iran — Term Structure ────────────────────────────────┤");
    println!("│  Date                  Cumul.   Conditional                           │");

    for t in &iran_terms {
        let bar_len = (t.conditional * 60.0) as usize;
        let bar = "█".repeat(bar_len.min(35));
        let marker = if t.conditional > 0.10 { "🔴" }
            else if t.conditional > 0.05 { "🟡" }
            else { "🟢" };
        println!("│  {:<22} {:>5.1}%    {:>5.1}%  {} {} │",
            t.label, t.cumulative * 100.0, t.conditional * 100.0, marker, bar);
    }

    // Close races
    let close_races = analysis::find_close_races(&all_markets, 100_000.0);

    println!("├─ 🎯 Close Races (35-65%, >$100K volume) ────────────────────────────┤");

    for (i, r) in close_races.iter().take(8).enumerate() {
        let sel = if i == 0 { "►" } else { " " };
        let q = r.market.short_question(42);
        let dist_marker = if r.distance_from_50 < 0.05 { "🟢" }
            else if r.distance_from_50 < 0.10 { "🟡" }
            else { "⚪" };
        println!("│ {} {:<42} {:>5.1}¢  ±{:.1}¢ {}  {:>9} │",
            sel, q, r.yes_price * 100.0, r.distance_from_50 * 100.0, dist_marker, format_volume(r.volume_24h));
    }

    println!("├──────────────────────────────────────────────────────────────────────┤");
    println!("│ ⚠️  NOT TRADING ADVICE. Close ≠ opportunity. Math ≠ signal. DYOR.    │");
    println!("├──────────────────────────────────────────────────────────────────────┤");
    println!("│ a:analyze  j/k:nav  Tab:view  q:quit                                │");
    println!("└──────────────────────────────────────────────────────────────────────┘");

    // ── SPREADS ──────────────────────────────────────────
    eprintln!("Fetching spreads from CLOB...");
    println!();
    println!("┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐");
    println!("│  Dashboard   Markets   Events  [Spreads]  Analysis   Help            │");
    println!("├──────────────────────────────────────────────────────────────────────┤");

    let mut n = 0;
    for m in sorted.iter().take(40) {
        if n >= 8 { break; }
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
    println!("└──────────────────────────────────────────────────────────────────────┘");
}
