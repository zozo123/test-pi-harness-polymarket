//! Demo runner — validates APIs and shows real opportunities.

use polymarket_tui::api::gamma::GammaClient;
use polymarket_tui::api::types::format_volume;
use polymarket_tui::analysis::engine;

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║     POLYMARKET OPPORTUNITY EXPLORER — DEMO                       ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let gamma = GammaClient::new();

    // ── Fetch Markets ───────────────────────────────────────────
    println!("📡 Fetching active markets from Polymarket...");
    let markets = match gamma.list_markets(200, 0, Some(true), Some(false)).await {
        Ok(m) => {
            println!("   ✅ Loaded {} markets\n", m.len());
            m
        }
        Err(e) => {
            println!("   ❌ Failed: {}\n", e);
            return;
        }
    };

    // ── Fetch Events ────────────────────────────────────────────
    println!("📡 Fetching active events...");
    let events = match gamma.list_events(100, Some(true), Some(false), None).await {
        Ok(e) => {
            println!("   ✅ Loaded {} events\n", e.len());
            e
        }
        Err(e) => {
            println!("   ❌ Failed: {}\n", e);
            return;
        }
    };

    // ── Top Markets by Volume ───────────────────────────────────
    println!("═══════════════════════════════════════════════════════════════════");
    println!("📊 TOP 10 MARKETS BY VOLUME");
    println!("═══════════════════════════════════════════════════════════════════\n");

    let mut sorted_markets = markets.clone();
    sorted_markets.sort_by(|a, b| b.volume_f64().partial_cmp(&a.volume_f64()).unwrap());

    for (i, m) in sorted_markets.iter().take(10).enumerate() {
        let q = m.short_question(55);
        let yes = m.yes_price();
        let no = m.no_price();
        let vol = format_volume(m.volume_f64());
        println!(
            "  {:2}. {:<57} YES: {:5.1}¢  NO: {:5.1}¢  Vol: {}",
            i + 1,
            q,
            yes * 100.0,
            no * 100.0,
            vol
        );
    }

    // ── Run Opportunity Analysis ────────────────────────────────
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("🎯 RUNNING OPPORTUNITY ANALYSIS...");
    println!("═══════════════════════════════════════════════════════════════════\n");

    let opportunities = engine::scan_all(&markets, &events);
    println!("   Found {} total opportunities\n", opportunities.len());

    // ── Event Arbitrage ─────────────────────────────────────────
    let arb_opps: Vec<_> = opportunities
        .iter()
        .filter(|o| o.label() == "EVENT ARB")
        .collect();

    if !arb_opps.is_empty() {
        println!("🎯 EVENT ARBITRAGE ({} found)", arb_opps.len());
        println!("   Multi-outcome events where Σ YES ≠ 100%\n");
        for opp in arb_opps.iter().take(5) {
            let score_bar = "█".repeat((opp.score * 5.0) as usize);
            let empty = "░".repeat(5 - (opp.score * 5.0) as usize);
            println!("   [{}{} {:.0}%] {}", score_bar, empty, opp.score * 100.0, opp.title());
            for line in opp.detail_lines() {
                println!("      {}", line);
            }
            println!();
        }
    }

    // ── Price Deviations ────────────────────────────────────────
    let dev_opps: Vec<_> = opportunities
        .iter()
        .filter(|o| o.label() == "PRICE DEV")
        .collect();

    if !dev_opps.is_empty() {
        println!("📊 PRICE DEVIATIONS ({} found)", dev_opps.len());
        println!("   Binary markets where YES + NO ≠ $1.00\n");
        for opp in dev_opps.iter().take(5) {
            let score_bar = "█".repeat((opp.score * 5.0) as usize);
            let empty = "░".repeat(5 - (opp.score * 5.0) as usize);
            println!("   [{}{} {:.0}%] {}", score_bar, empty, opp.score * 100.0, opp.title());
            for line in opp.detail_lines() {
                println!("      {}", line);
            }
            println!();
        }
    }

    // ── Volume Surges ───────────────────────────────────────────
    let vol_opps: Vec<_> = opportunities
        .iter()
        .filter(|o| o.label() == "VOL SURGE")
        .collect();

    if !vol_opps.is_empty() {
        println!("📈 VOLUME SURGES ({} found)", vol_opps.len());
        println!("   Abnormal 24h trading activity\n");
        for opp in vol_opps.iter().take(5) {
            let score_bar = "█".repeat((opp.score * 5.0) as usize);
            let empty = "░".repeat(5 - (opp.score * 5.0) as usize);
            println!("   [{}{} {:.0}%] {}", score_bar, empty, opp.score * 100.0, opp.title());
            for line in opp.detail_lines() {
                println!("      {}", line);
            }
            println!();
        }
    }

    // ── Near Resolution ─────────────────────────────────────────
    let near_opps: Vec<_> = opportunities
        .iter()
        .filter(|o| o.label() == "NEAR RES")
        .collect();

    if !near_opps.is_empty() {
        println!("⏰ NEAR RESOLUTION ({} found)", near_opps.len());
        println!("   Markets expiring soon with extreme prices\n");
        for opp in near_opps.iter().take(5) {
            let score_bar = "█".repeat((opp.score * 5.0) as usize);
            let empty = "░".repeat(5 - (opp.score * 5.0) as usize);
            println!("   [{}{} {:.0}%] {}", score_bar, empty, opp.score * 100.0, opp.title());
            for line in opp.detail_lines() {
                println!("      {}", line);
            }
            println!();
        }
    }

    // ── Summary ─────────────────────────────────────────────────
    println!("═══════════════════════════════════════════════════════════════════");
    println!("📋 SUMMARY");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("   Markets scanned:     {}", markets.len());
    println!("   Events scanned:      {}", events.len());
    println!("   Total opportunities: {}", opportunities.len());
    println!("   ├─ Event Arbitrage:  {}", arb_opps.len());
    println!("   ├─ Price Deviations: {}", dev_opps.len());
    println!("   ├─ Volume Surges:    {}", vol_opps.len());
    println!("   └─ Near Resolution:  {}", near_opps.len());

    let high_score = opportunities.iter().filter(|o| o.score >= 0.5).count();
    println!("\n   🔥 High-score opportunities (≥50%): {}", high_score);

    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("✅ Demo complete! Run 'cargo run --release' for the full TUI.");
    println!("═══════════════════════════════════════════════════════════════════\n");
}
