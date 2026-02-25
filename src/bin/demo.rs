//! Polymarket Browser — Explore prediction markets.

use polymarket_tui::api::gamma::GammaClient;
use polymarket_tui::api::types::format_volume;

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║     POLYMARKET BROWSER                                           ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  Browse prediction markets. Not trading advice. DYOR.            ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let gamma = GammaClient::new();

    // Fetch data
    println!("📡 Fetching markets...");
    let markets = match gamma.list_markets(200, 0, Some(true), Some(false)).await {
        Ok(m) => m,
        Err(e) => {
            println!("❌ Failed: {}", e);
            return;
        }
    };
    println!("   ✅ {} active markets\n", markets.len());

    // Sort by volume
    let mut by_volume = markets.clone();
    by_volume.sort_by(|a, b| b.volume_f64().partial_cmp(&a.volume_f64()).unwrap());

    // ── Most Active (by volume) ─────────────────────────────────
    println!("═══════════════════════════════════════════════════════════════════");
    println!("💰 MOST ACTIVE (by total volume)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    for m in by_volume.iter().take(10) {
        let q = m.short_question(50);
        let yes = m.yes_price();
        println!("   {} ", q);
        println!("   └─ YES: {:5.1}¢  │  Vol: {}\n", yes * 100.0, format_volume(m.volume_f64()));
    }

    // ── Biggest 24h Movers ──────────────────────────────────────
    println!("═══════════════════════════════════════════════════════════════════");
    println!("📈 HIGHEST 24H VOLUME (recent activity)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    let mut by_24h: Vec<_> = markets.iter()
        .filter(|m| m.volume_24h_f64() > 1000.0)
        .collect();
    by_24h.sort_by(|a, b| b.volume_24h_f64().partial_cmp(&a.volume_24h_f64()).unwrap());

    for m in by_24h.iter().take(10) {
        let q = m.short_question(50);
        let yes = m.yes_price();
        println!("   {} ", q);
        println!("   └─ YES: {:5.1}¢  │  24h: {}\n", yes * 100.0, format_volume(m.volume_24h_f64()));
    }

    // ── Close Races (YES near 50%) ──────────────────────────────
    println!("═══════════════════════════════════════════════════════════════════");
    println!("⚖️  CLOSE RACES (YES price 40-60%)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    let close_races: Vec<_> = markets.iter()
        .filter(|m| {
            let yes = m.yes_price();
            yes >= 0.40 && yes <= 0.60 && m.volume_f64() > 100_000.0
        })
        .collect();

    if close_races.is_empty() {
        println!("   No close races with >$100K volume found.\n");
    } else {
        for m in close_races.iter().take(10) {
            let q = m.short_question(50);
            let yes = m.yes_price();
            println!("   {} ", q);
            println!("   └─ YES: {:5.1}¢  │  Vol: {}\n", yes * 100.0, format_volume(m.volume_f64()));
        }
    }

    // ── High Confidence (YES >90% or <10%) ──────────────────────
    println!("═══════════════════════════════════════════════════════════════════");
    println!("🎯 HIGH CONFIDENCE (YES >90% or <10%, Vol >$500K)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    let high_conf: Vec<_> = markets.iter()
        .filter(|m| {
            let yes = m.yes_price();
            (yes >= 0.90 || yes <= 0.10) && m.volume_f64() > 500_000.0
        })
        .collect();

    if high_conf.is_empty() {
        println!("   No high-confidence markets with >$500K volume found.\n");
    } else {
        for m in high_conf.iter().take(10) {
            let q = m.short_question(50);
            let yes = m.yes_price();
            let direction = if yes > 0.5 { "YES likely" } else { "NO likely" };
            println!("   {} ", q);
            println!("   └─ YES: {:5.1}¢  │  {}  │  Vol: {}\n", 
                yes * 100.0, direction, format_volume(m.volume_f64()));
        }
    }

    // ── Summary ─────────────────────────────────────────────────
    println!("═══════════════════════════════════════════════════════════════════");
    println!("📊 SUMMARY");
    println!("═══════════════════════════════════════════════════════════════════");
    
    let total_volume: f64 = markets.iter().map(|m| m.volume_f64()).sum();
    let total_24h: f64 = markets.iter().map(|m| m.volume_24h_f64()).sum();
    
    println!("   Active markets:     {}", markets.len());
    println!("   Total volume:       {}", format_volume(total_volume));
    println!("   24h volume:         {}", format_volume(total_24h));
    println!("   Close races:        {}", close_races.len());
    println!("   High confidence:    {}", high_conf.len());

    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("   Run './target/release/polymarket-tui' for interactive browsing.");
    println!("═══════════════════════════════════════════════════════════════════\n");
}
