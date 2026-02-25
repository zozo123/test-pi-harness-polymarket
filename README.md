# polymarket-tui

Terminal UI for researching [Polymarket](https://polymarket.com) prediction markets.

```
⚠️  FOR RESEARCH ONLY — NOT TRADING ADVICE
    You can lose money. Fees, slippage, and liquidity affect real trades.
    This tool does NOT execute trades. DYOR. Check your local laws.
```

## What It Shows

| Signal | Description |
|--------|-------------|
| 📈 **Volume Anomaly** | Unusual 24h volume vs 7-day average — may indicate news |
| 🎯 **Multi-Outcome** | Event pricing analysis (NOT arbitrage — see note below) |
| ⏰ **Expiring** | Markets near resolution with confident pricing |
| 💧 **Liquidity** | Highest volume markets for easier trading |

### ⚠️ Important: "Arbitrage" Myth

Many tools claim "arbitrage" when multi-outcome event prices don't sum to 100%. **This is often wrong:**

- **"By date" markets are cumulative**, not mutually exclusive. If something happens in March, ALL "by March", "by June", etc. resolve YES.
- **Sum < 100% is expected** for cumulative markets
- **Sum > 100%** usually reflects trading fees, not free money
- **True arbitrage is rare** and gets closed by bots in milliseconds

This tool is honest about these limitations.

## Quick Start

```bash
git clone https://github.com/zozo123/test-pi-harness-polymarket.git
cd test-pi-harness-polymarket
cargo build --release
./target/release/demo           # CLI demo
./target/release/polymarket-tui # Full TUI
```

## Demo Output

```
📡 Fetching active markets from Polymarket...
   ✅ Loaded 200 markets

📈 VOLUME ANOMALIES (Unusual 24h activity vs 7d average)
   May indicate news or information entering the market.

   📈 Will bitcoin hit $1m before GTA VI?
      24h: $176852  |  7d avg: $29498  |  6.0x spike

🎯 MULTI-OUTCOME EVENTS (Pricing Analysis)
   ⚠️  Sum ≠ 100% is NOT always arbitrage!

   🎯 Taylor Swift pregnant in 2025? [exclusive]
      Σ YES: 40.3%  |  3 outcomes
      Sum < 100% on exclusive outcomes — verify market structure

⏰ EXPIRING SOON (High confidence pricing, ≤14 days)

   ⏰ Will GTA 6 cost $100+?
      YES: 0.4¢  |  3 days left  |  NO likely

📋 SUMMARY
   Markets scanned:    200
   Events scanned:     100
   Signals generated:  116
```

## Keybindings

| Key | Action |
|-----|--------|
| `Tab` | Switch views |
| `j/k` | Navigate |
| `/` | Search |
| `r` | Refresh |
| `q` | Quit |

## Architecture

```
src/
├── main.rs           # TUI
├── app.rs            # State
├── api/              # Polymarket API clients
├── analysis/engine.rs # Signal detection (honest math)
└── ui/               # Views
```

## Using as Library

```rust
use polymarket_tui::api::gamma::GammaClient;
use polymarket_tui::analysis::engine;

#[tokio::main]
async fn main() {
    let gamma = GammaClient::new();
    let markets = gamma.list_markets(200, 0, Some(true), Some(false)).await.unwrap();
    let events = gamma.list_events(100, Some(true), Some(false), None).await.unwrap();
    
    // Returns research signals, NOT trading recommendations
    let signals = engine::scan_all(&markets, &events);
    println!("Found {} signals", signals.len());
}
```

## What This Tool Does NOT Do

- ❌ Execute trades
- ❌ Guarantee profits
- ❌ Find "free money" arbitrage
- ❌ Compete with HFT bots
- ❌ Account for fees/slippage

## What This Tool DOES Do

- ✅ Fetch real market data
- ✅ Show volume anomalies (potential news events)
- ✅ Analyze multi-outcome pricing (with honest caveats)
- ✅ Identify high-liquidity markets
- ✅ Track expiring markets

## License

MIT — See [LICENSE](LICENSE)
