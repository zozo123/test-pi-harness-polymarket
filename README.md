# polymarket-tui

**Terminal-based opportunity explorer for [Polymarket](https://polymarket.com) prediction markets.**

Built in Rust with `ratatui` + `crossterm`. Fetches live data directly from Polymarket's Gamma & CLOB APIs.

![Rust](https://img.shields.io/badge/Rust-000?logo=rust) ![License: MIT](https://img.shields.io/badge/License-MIT-cyan)

---

## ⚠️ IMPORTANT DISCLAIMERS

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                              ⚠️  WARNING  ⚠️                                  ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  • This software is for EDUCATIONAL and RESEARCH purposes ONLY               ║
║  • This is NOT financial advice — do your own research (DYOR)                ║
║  • Prediction markets carry SIGNIFICANT RISK — you can lose money            ║
║  • Past performance does not guarantee future results                        ║
║  • "Opportunities" shown are algorithmic signals, NOT trading recommendations║
║  • The authors are NOT responsible for any financial losses                  ║
║  • Always verify data independently before making any decisions              ║
║  • This tool does NOT execute trades — it is read-only                       ║
║  • Check your local laws — prediction markets may be restricted in your area ║
║                                                                               ║
║  BY USING THIS SOFTWARE, YOU ACCEPT ALL RISKS AND RESPONSIBILITIES           ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## 🎯 What It Does

Scans Polymarket for potential trading opportunities you might miss:

| Type | What It Detects | Strategy Hint |
|------|-----------------|---------------|
| 🎯 **Event Arbitrage** | Multi-outcome events where Σ YES ≠ 100% | If <100%: buy all YES. If >100%: sell all YES |
| 📊 **Price Deviation** | Binary markets where YES + NO ≠ $1.00 | Potential mispricing to exploit |
| 📈 **Volume Surge** | Abnormal 24h volume spikes | Information event — price may move |
| ⏰ **Near Resolution** | Markets expiring soon with extreme odds | Easy value if outcome is certain |
| 💹 **Wide Spreads** | Fat bid-ask gaps in order book | Market-making opportunity |

---

## 📸 Screenshots

### Dashboard View
```
┌──────────────────────────────────────────────────────────────────────────────┐
│ ◆ Polymarket Opportunity Explorer                                            │
├──────────────────────────────────────────────────────────────────────────────┤
│  Dashboard   Markets   Events   Opportunities   Help                         │
├──────────────────────────────────────────────────────────────────────────────┤
│ Stats                                                                        │
│   Markets: 200  Active: 187  │  Total Volume: $2.4B  │  Events: 100          │
│   Opportunities: 135  High-score (≥50%): 72                                  │
├──────────────────────────────────────────────────────────────────────────────┤
│ Top Markets by Volume                                                        │
│   Question                                          YES     NO      Volume   │
│ ► Will Chelsea Clinton win 2028 Dem nomination?    0.9¢   99.2¢    $39.2M   │
│   Will Indiana Pacers win 2026 NBA Finals?         0.1¢  100.0¢    $38.2M   │
│   Will Oprah win 2028 Dem nomination?              0.9¢   99.2¢    $35.6M   │
│   Will Andrew Yang win 2028 Dem nomination?        0.9¢   99.1¢    $32.2M   │
│   Will Memphis Grizzlies win 2026 NBA Finals?      0.1¢   99.9¢    $31.7M   │
├──────────────────────────────────────────────────────────────────────────────┤
│ Ready — 200 markets, 100 events, 135 opportunities    r:refresh  Tab:switch │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Opportunities View
```
┌──────────────────────────────────────────────────────────────────────────────┐
│ ◆ Polymarket Opportunity Explorer                                            │
├──────────────────────────────────────────────────────────────────────────────┤
│  Dashboard   Markets   Events  [Opportunities]  Help                         │
├──────────────────────────────────────────────────────────────────────────────┤
│ Opportunity Summary                                                          │
│   Total: 135  │  🎯 Event Arb: 59  📈 Vol Surge: 67  ⏰ Near Res: 9          │
├──────────────────────────────────────────────────────────────────────────────┤
│ Opportunities (135)                                                          │
│   Score     Type        Market / Event                      Key Metric       │
│ ► █████     EVENT ARB   MicroStrategy sells Bitcoin by___?  Edge: 81.15%    │
│   █████     EVENT ARB   Macron out by...?                   Edge: 96.55%    │
│   █████     EVENT ARB   UK election called by...?           Edge: 90.90%    │
│   █████     VOL SURGE   James Talarico 2028 Dem nom?        Surge: 26.9x    │
│   ████░     VOL SURGE   Norway win 2026 FIFA World Cup?     Surge: 18.4x    │
├──────────────────────────────────────────────────────────────────────────────┤
│ Detail                                                                       │
│   [EVENT ARB] MicroStrategy sells any Bitcoin by ___ ?                       │
│     Σ YES prices: 18.9%                                                      │
│     Markets: 4                                                               │
│     Edge: 81.15%                                                             │
│     Strategy: BUY all YES positions                                          │
│     Score: █████ (100%)                                                      │
├──────────────────────────────────────────────────────────────────────────────┤
│ Opportunity 1/135    j/k:navigate  Tab:view  r:refresh  q:quit               │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Events View (Arbitrage Detection)
```
┌──────────────────────────────────────────────────────────────────────────────┐
│ ◆ Polymarket Opportunity Explorer                                            │
├──────────────────────────────────────────────────────────────────────────────┤
│  Dashboard   Markets  [Events]  Opportunities   Help                         │
├──────────────────────────────────────────────────────────────────────────────┤
│ Events (100)                                                                 │
│   Event                                    Markets   Σ YES    Arb Edge       │
│ ► MicroStrategy sells any Bitcoin by___?      4      18.9%    81.15%  🔥     │
│   How many people will Trump deport?          8     100.1%     0.10%         │
│   Kraken IPO by ___ ?                         3      85.5%    14.50%  🔥     │
│   2028 Democratic Nomination                 45     102.3%     2.30%         │
│   Macron out by...?                           3       3.5%    96.55%  🔥     │
├──────────────────────────────────────────────────────────────────────────────┤
│ Event Detail                                                                 │
│   MicroStrategy sells any Bitcoin by ___ ?                                   │
│     • By March 2025?                    → 3.0¢                               │
│     • By June 2025?                     → 5.4¢                               │
│     • By September 2025?                → 4.5¢                               │
│     • By December 2025?                 → 6.0¢                               │
│   Total: 18.9% — BUY ALL for guaranteed profit (minus fees)                  │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## 🚀 Quick Start

```bash
# Clone
git clone https://github.com/zozo123/test-pi-harness-polymarket.git
cd test-pi-harness-polymarket

# Build
cargo build --release

# Run the TUI
./target/release/polymarket-tui

# Or run the CLI demo
./target/release/demo
```

### Requirements
- Rust 1.70+
- Internet connection
- **No wallet or API key needed** (read-only)

---

## ⌨️ Keybindings

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Switch views |
| `1`-`5` | Jump to view directly |
| `j` / `k` / `↑` / `↓` | Navigate lists |
| `Ctrl+d` / `Ctrl+u` | Page down / up |
| `g` / `Home` | Jump to top |
| `/` | Search markets |
| `Enter` | Toggle detail view |
| `r` | Refresh data |
| `Esc` | Clear search / close detail |
| `q` / `Ctrl+c` | Quit |

---

## 🏗️ Architecture

```
polymarket-tui/
├── Cargo.toml
├── src/
│   ├── main.rs              # TUI entry point, event loop
│   ├── lib.rs               # Library root (usable as crate)
│   ├── app.rs               # Application state machine
│   ├── api/
│   │   ├── mod.rs
│   │   ├── types.rs         # Market, Event, OrderBook structs
│   │   ├── gamma.rs         # Gamma API client (markets, events)
│   │   └── clob.rs          # CLOB API client (order books)
│   ├── analysis/
│   │   ├── mod.rs
│   │   └── engine.rs        # Opportunity detection algorithms
│   └── ui/
│       ├── mod.rs           # View dispatcher
│       ├── dashboard.rs     # Dashboard view
│       ├── markets.rs       # Market browser
│       ├── events.rs        # Event browser + arb detection
│       ├── opportunities.rs # Opportunity scanner
│       ├── detail.rs        # Market detail panel
│       ├── help.rs          # Help screen
│       └── theme.rs         # Colors & styles
└── src/bin/
    ├── demo.rs              # CLI demo (non-interactive)
    └── debug_api.rs         # API debugging tool
```

---

## 📚 Using as a Library

```rust
use polymarket_tui::api::gamma::GammaClient;
use polymarket_tui::analysis::engine;

#[tokio::main]
async fn main() {
    let gamma = GammaClient::new();

    // Fetch data
    let markets = gamma.list_markets(200, 0, Some(true), Some(false)).await.unwrap();
    let events = gamma.list_events(100, Some(true), Some(false), None).await.unwrap();

    // Scan for opportunities
    let opportunities = engine::scan_all(&markets, &events);

    for opp in opportunities.iter().filter(|o| o.score >= 0.5) {
        println!("[{:.0}%] {} — {}", opp.score * 100.0, opp.label(), opp.title());
    }
}
```

---

## 🔍 How Opportunity Detection Works

### Event Arbitrage
For multi-outcome events (e.g., "Who wins 2028 nomination?" with 45 candidates), the sum of all YES prices should equal 100%. If it's less, buying all YES positions guarantees profit. If it's more, selling all guarantees profit.

```
Example: MicroStrategy sells Bitcoin by ___?
  • By March 2025:     3.0¢
  • By June 2025:      5.4¢  
  • By September 2025: 4.5¢
  • By December 2025:  6.0¢
  ─────────────────────────
  Total:              18.9¢  ← Should be ~100¢!
  
  Edge: 81.15% — BUY ALL for potential arbitrage
```

### Volume Surge
Compares 24h volume to historical daily average. A surge often indicates new information entering the market.

### Near Resolution  
Markets expiring within 14 days with >90% or <10% YES prices are likely to resolve as expected — easy value to collect.

---

## ⚠️ Limitations & Risks

1. **Fees not included** — Polymarket charges fees that may eliminate small edges
2. **Liquidity constraints** — You may not be able to execute at displayed prices
3. **Slippage** — Large orders move prices against you
4. **API delays** — Data may be seconds old
5. **Resolution risk** — Markets can resolve unexpectedly
6. **Regulatory risk** — Check your jurisdiction's laws
7. **Smart contract risk** — Blockchain-based systems have inherent risks

---

## 📜 License

MIT License — see [LICENSE](LICENSE)

---

## 🙏 Credits

- [Polymarket](https://polymarket.com) for the prediction market platform
- [ratatui](https://github.com/ratatui-org/ratatui) for the TUI framework
- [Polymarket CLI](https://github.com/Polymarket/polymarket-cli) for API inspiration

---

**Built with 🦀 Rust — 2,600+ lines of pure terminal goodness**
