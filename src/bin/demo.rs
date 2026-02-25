//! Demo — shows market analysis signals (NOT trading recommendations).

use polymarket_tui::api::gamma::GammaClient;
use polymarket_tui::api::types::format_volume;
use polymarket_tui::analysis::engine;

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║     POLYMARKET MARKET SCANNER — Research Tool                    ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  ⚠️  FOR RESEARCH ONLY — NOT TRADING ADVICE                      ║");
    println!("║  Signals shown are informational. DYOR. You can lose money.      ║");
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
    println!("📊 TOP 10 MARKETS BY VOLUME (Most Liquid)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    let mut sorted_markets = markets.clone();
    sorted_markets.sort_by(|a, b| b.volume_f64().partial_cmp(&a.volume_f64()).unwrap());

    for (i, m) in sorted_markets.iter().take(10).enumerate() {
        let q = m.short_question(55);
        let yes = m.yes_price();
        let vol = format_volume(m.volume_f64());
        println!(
            "  {:2}. {:<57} YES: {:5.1}¢  Vol: {}",
            i + 1,
            q,
            yes * 100.0,
            vol
        );
    }

    // ── Volume Anomalies ────────────────────────────────────────
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("📈 VOLUME ANOMALIES (Unusual 24h activity vs 7d average)");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("   May indicate news or information entering the market.\n");

    let vol_signals = engine::find_volume_anomalies(&markets);
    
    if vol_signals.is_empty() {
        println!("   No significant volume anomalies detected.\n");
    } else {
        for signal in vol_signals.iter().take(5) {
            if let engine::SignalKind::VolumeAnomaly {
                question,
                volume_24h,
                daily_avg_7d,
                spike_ratio,
                ..
            } = &signal.kind
            {
                let q = if question.len() > 60 {
                    format!("{}…", &question[..59])
                } else {
                    question.clone()
                };
                println!("   📈 {}", q);
                println!("      24h: ${:.0}  |  7d avg: ${:.0}  |  {:.1}x spike", 
                    volume_24h, daily_avg_7d, spike_ratio);
                println!();
            }
        }
    }

    // ── Multi-Outcome Events ────────────────────────────────────
    println!("═══════════════════════════════════════════════════════════════════");
    println!("🎯 MULTI-OUTCOME EVENTS (Pricing Analysis)");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("   ⚠️  Sum ≠ 100% is NOT always arbitrage!");
    println!("   'By date' markets are cumulative — sum < 100% is expected.\n");

    let multi_signals = engine::analyze_multi_outcome_events(&events);
    let interesting: Vec<_> = multi_signals.iter()
        .filter(|s| s.relevance > 0.3)
        .take(5)
        .collect();

    if interesting.is_empty() {
        println!("   No significant pricing anomalies in exclusive-outcome events.\n");
    } else {
        for signal in interesting {
            if let engine::SignalKind::MultiOutcomePricing {
                event_title,
                total_yes,
                market_count,
                is_mutually_exclusive,
                note,
                ..
            } = &signal.kind
            {
                let title = if event_title.len() > 55 {
                    format!("{}…", &event_title[..54])
                } else {
                    event_title.clone()
                };
                let exclusive_str = if *is_mutually_exclusive { "exclusive" } else { "cumulative" };
                println!("   🎯 {} [{}]", title, exclusive_str);
                println!("      Σ YES: {:.1}%  |  {} outcomes", total_yes * 100.0, market_count);
                println!("      {}", note);
                println!();
            }
        }
    }

    // ── Near Expiry ─────────────────────────────────────────────
    println!("═══════════════════════════════════════════════════════════════════");
    println!("⏰ EXPIRING SOON (High confidence pricing, ≤14 days)");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("   Markets near resolution with >90% or <10% YES prices.\n");

    let expiry_signals = engine::find_high_confidence_expiring(&markets, 14);
    
    if expiry_signals.is_empty() {
        println!("   No high-confidence markets expiring soon.\n");
    } else {
        for signal in expiry_signals.iter().take(5) {
            if let engine::SignalKind::HighConfidenceNearExpiry {
                question,
                yes_price,
                days_remaining,
                ..
            } = &signal.kind
            {
                let q = if question.len() > 55 {
                    format!("{}…", &question[..54])
                } else {
                    question.clone()
                };
                let direction = if *yes_price > 0.5 { "YES likely" } else { "NO likely" };
                println!("   ⏰ {}", q);
                println!("      YES: {:.1}¢  |  {} days left  |  {}", 
                    yes_price * 100.0, days_remaining, direction);
                println!();
            }
        }
    }

    // ── Summary ─────────────────────────────────────────────────
    let all_signals = engine::scan_all(&markets, &events);
    
    println!("═══════════════════════════════════════════════════════════════════");
    println!("📋 SUMMARY");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("   Markets scanned:    {}", markets.len());
    println!("   Events scanned:     {}", events.len());
    println!("   Signals generated:  {}", all_signals.len());
    
    let high_relevance = all_signals.iter().filter(|s| s.relevance > 0.5).count();
    println!("   High relevance:     {}", high_relevance);

    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("⚠️  REMINDER: This is research data, not trading advice.");
    println!("   • Fees, slippage, and liquidity affect real trades");
    println!("   • Markets can resolve unexpectedly");
    println!("   • Always verify data independently");
    println!("═══════════════════════════════════════════════════════════════════\n");
}
