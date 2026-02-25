# polymarket-tui

**Terminal-based opportunity explorer for [Polymarket](https://polymarket.com) prediction markets.**

Built in Rust with `ratatui` + `crossterm`. Fetches live data directly from Polymarket's APIs (Gamma + CLOB) — no wallet needed for browsing.

![Rust](https://img.shields.io/badge/Rust-000?logo=rust) ![License: MIT](https://img.shields.io/badge/License-MIT-cyan)

## What It Does

Scans Polymarket for trading opportunities you might miss:

| Type | What It Finds |
|------|---------------|
| 🎯 **Event Arbitrage** | Multi-outcome events where all YES prices sum ≠ 100% — guaranteed profit if executed |
| 📊 **Price Deviation** | Binary markets where YES + NO ≠ $1.00 |
| 💹 **Wide Spreads** | Markets with fat bid-ask gaps — market-making opportunity |
| 📈 **Volume Surge** | Sudden volume spikes signaling information events |
| ⏰ **Near Resolution** | Markets near expiry with extreme odds — easy value to collect |

## Install

```bash
# Clone and build
git clone https://github.com/zozo123/test-pi-harness-polymarket.git
cd test-pi-harness-polymarket
cargo build --release

# Run
./target/release/polymarket-tui
```

### Requirements

- Rust 1.70+ (uses 2021 edition)
- Internet connection (hits Polymarket APIs)
- No wallet or API key needed for browsing

## Usage

```bash
# Just run it — auto-fetches data on startup
cargo run --release
```

### Keybindings

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Switch views |
| `1`-`5` | Jump to view |
| `j/k` or `↑/↓` | Navigate lists |
| `Ctrl+d/u` | Page down/up |
| `g` | Jump to top |
| `/` | Search markets |
| `Enter` | Toggle detail view |
| `r` | Refresh data |
| `Esc` | Clear search / close detail |
| `q` | Quit |

### Views

1. **Dashboard** — Stats overview + top markets by volume
2. **Markets** — Searchable/filterable list of all active markets
3. **Events** — Multi-market events with arbitrage detection (Σ YES column)
4. **Opportunities** — All detected opportunities ranked by score
5. **Help** — Keybindings reference

## Architecture

```
src/
  main.rs             — Entry point, terminal setup, event loop
  lib.rs              — Library root
  app.rs              — Application state machine
  api/
    types.rs          — Data types (Market, Event, OrderBook, etc.)
    gamma.rs          — Gamma API client (markets, events)
    clob.rs           — CLOB API client (order books, prices)
  analysis/
    engine.rs         — Opportunity detection algorithms
  ui/
    mod.rs            — View dispatcher
    dashboard.rs      — Dashboard view
    markets.rs        — Market browser view
    events.rs         — Event browser with arb detection
    opportunities.rs  — Opportunity scanner view
    detail.rs         — Market detail panel
    help.rs           — Help/keybindings view
    theme.rs          — Color palette & style presets
```

## Using as a Library

The core logic is a Rust library (`polymarket_tui`):

```rust
use polymarket_tui::api::gamma::GammaClient;
use polymarket_tui::analysis::engine;

#[tokio::main]
async fn main() {
    let gamma = GammaClient::new();

    // Fetch active markets
    let markets = gamma.list_markets(200, 0, Some(true), Some(false))
        .await.unwrap();

    // Fetch events
    let events = gamma.list_events(100, Some(true), Some(false), None)
        .await.unwrap();

    // Scan for opportunities
    let opportunities = engine::scan_all(&markets, &events);

    for opp in &opportunities {
        println!("[{:.0}%] {} — {}", opp.score * 100.0, opp.label(), opp.title());
    }
}
```

## Disclaimer

⚠️ **For research and educational purposes only.** Not financial advice. Prediction markets carry risk. Always do your own research. This tool does not execute trades.

## License

MIT
