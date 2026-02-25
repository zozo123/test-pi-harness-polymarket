//! Liquidity checker — See real spreads and depth before trading.

use polymarket_tui::api::gamma::GammaClient;
use polymarket_tui::api::clob::ClobClient;
use polymarket_tui::api::types::format_volume;

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║     LIQUIDITY CHECKER                                            ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  Check spreads before trading. Spread = your cost to enter/exit. ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let gamma = GammaClient::new();
    let clob = ClobClient::new();

    // Fetch top markets
    println!("📡 Fetching top markets by volume...");
    let markets = match gamma.list_markets(100, 0, Some(true), Some(false)).await {
        Ok(m) => m,
        Err(e) => {
            println!("❌ Failed: {}", e);
            return;
        }
    };

    let mut by_volume = markets.clone();
    by_volume.sort_by(|a, b| b.volume_f64().partial_cmp(&a.volume_f64()).unwrap());

    println!("   ✅ Loaded {} markets\n", markets.len());
    println!("═══════════════════════════════════════════════════════════════════");
    println!("📊 SPREAD & LIQUIDITY (Top 10 by volume)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    let mut checked = 0;
    for market in by_volume.iter() {
        if checked >= 10 {
            break;
        }

        let token_ids = market.parsed_token_ids();
        if token_ids.is_empty() {
            continue;
        }

        let yes_token = &token_ids[0];
        
        // Get midpoint and spread from CLOB API
        let midpoint = clob.get_midpoint(yes_token).await.unwrap_or(0.0);
        let spread = clob.get_spread(yes_token).await.unwrap_or(0.0);
        
        if midpoint == 0.0 {
            continue; // Skip markets with no pricing data
        }

        let q = market.short_question(50);
        let spread_pct = if midpoint > 0.0 { (spread / midpoint) * 100.0 } else { 0.0 };
        
        // Classify spread tightness
        let spread_rating = if spread <= 0.005 {
            "🟢 Tight"
        } else if spread <= 0.02 {
            "🟡 Okay"
        } else {
            "🔴 Wide"
        };

        println!("   {} ", q);
        println!("   │  Midpoint: {:5.1}¢  │  Spread: {:.2}¢ ({:.1}%)  │  {}", 
            midpoint * 100.0, 
            spread * 100.0,
            spread_pct,
            spread_rating);
        println!("   └─ Volume: {}  │  Liq: {}", 
            format_volume(market.volume_f64()),
            format_volume(market.liquidity_f64()));
        println!();
        
        checked += 1;
    }

    // Summary
    println!("═══════════════════════════════════════════════════════════════════");
    println!("💡 WHAT SPREAD MEANS");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("   Spread = difference between best buy and sell prices.");
    println!("   If you buy then immediately sell, you LOSE the spread.");
    println!();
    println!("   🟢 Tight (<0.5¢)  = Low cost to trade");
    println!("   🟡 Okay  (0.5-2¢) = Moderate cost");
    println!("   🔴 Wide  (>2¢)    = High cost, avoid unless confident");
    println!();
    println!("   Example: 2¢ spread on a 50¢ market = 4% round-trip cost!");
    println!("═══════════════════════════════════════════════════════════════════\n");
}
