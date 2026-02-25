<p align="center">
  <h1 align="center">◆ polymarket-tui</h1>
  <p align="center">
    A terminal UI for browsing <a href="https://polymarket.com">Polymarket</a> prediction markets.<br>
    Real data. Honest math. No fake signals.
  </p>
</p>

<p align="center">
  <img src="demo.gif" alt="polymarket-tui demo" width="700">
</p>

<p align="center">
  <em>⚠️ Not trading advice. Read-only browser. Does NOT execute trades. You can lose money. DYOR.</em>
</p>

---

## Quick Start

```bash
git clone https://github.com/zozo123/test-pi-harness-polymarket.git
cd test-pi-harness-polymarket
cargo build --release
./target/release/polymarket-tui
```

No API key, no wallet, no signup. Reads public Polymarket APIs only.

---

## Views

### 📊 Dashboard — Top markets at a glance

Stats bar with total volume, 24h activity, and count of close races (40-60% markets).
Top markets sorted by volume with YES price and 24h change.

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│ [Dashboard]  Markets   Events   Spreads   Analysis   Help            │
├─ Stats ──────────────────────────────────────────────────────────────┤
│  Markets: 200   Volume: $1323.0M   24h: $18.0M   Close Races: 7     │
├─ Top Markets by Volume ──────────────────────────────────────────────┤
│ ► Will Chelsea Clinton win the 2028 Democrati…  0.9¢  $39.2M  $652K │
│   Will the Indiana Pacers win the 2026 NBA Fi…  0.1¢  $39.1M  $2.4M │
│   Will Oprah Winfrey win the 2028 Democratic …  0.9¢  $35.6M   $83K │
│   Will Andrew Yang win the 2028 Democratic pr…  0.9¢  $32.3M   $68K │
│   Will the Memphis Grizzlies win the 2026 NBA…  0.1¢  $31.7M   $70K │
│   Will George Clooney win the 2028 Democratic…  0.9¢  $31.4M  $108K │
│   Will MrBeast win the 2028 Democratic presid…  0.9¢  $30.4M   $97K │
├──────────────────────────────────────────────────────────────────────┤
│ r:refresh  Tab:view  j/k:nav  /:search  q:quit                      │
└──────────────────────────────────────────────────────────────────────┘
```

### 🔍 Markets — Search any topic

Type `/` to search, then browse results. Searches across 2000+ active markets.

```
┌─ 🔍 iran ────────────────────────────────────────────────────────────┐
│ ► US strikes Iran by February 28, 2026?            12.5¢     $43.6M │
│   US strikes Iran by March 31, 2026?               60.5¢     $16.7M │
│   Khamenei out as Supreme Leader by Mar 31?        18.5¢     $16.4M │
│   US strikes Iran by June 30, 2026?                69.5¢      $7.4M │
│   Khamenei out by June 30?                         34.5¢      $5.9M │
│   US strikes Iran by March 15, 2026?               46.5¢      $3.6M │
│   Israel strikes Iran by March 31, 2026?           52.5¢      $2.7M │
├──────────────────────────────────────────────────────────────────────┤
│ 31 results                                                           │
└──────────────────────────────────────────────────────────────────────┘
```

```
┌─ 🔍 ukraine ─────────────────────────────────────────────────────────┐
│ ► Russia x Ukraine ceasefire by March 31?           3.0¢     $19.7M │
│   Russia x Ukraine ceasefire by end of 2026?       37.5¢     $10.3M │
│   Russia x Ukraine ceasefire by February 28?        0.2¢      $5.7M │
│   Russia x Ukraine ceasefire by June 30?           15.5¢      $1.9M │
│   Zelenskyy out by end of 2026?                    24.5¢      $1.7M │
│   Russia-Ukraine Ceasefire before GTA VI?          58.5¢      $1.3M │
├──────────────────────────────────────────────────────────────────────┤
│ 8 results                                                            │
└──────────────────────────────────────────────────────────────────────┘
```

### 📈 Analysis — Conditional probabilities & close races

The killer feature. Computes **conditional probabilities** from cumulative "by date" markets.

*"Given no strike by March 7, what's the chance of a strike before March 31?"*

```
├─ 📈 US Strikes Iran — Term Structure ────────────────────────────────┤
│  Date                Cumul.  Cond.                                    │
│  february 25           1.5%   1.5%  🟢                               │
│  february 26           3.0%   1.6%  🟢                               │
│  february 27           6.5%   3.6%  🟢 ██                            │
│  february 28          12.5%   6.4%  🟡 ███                           │
│  march 1              15.5%   3.4%  🟢 ██                            │
│  march 5              27.5%   5.2%  🟡 ███                           │
│  march 7              35.5%   4.4%  🟢 ██                            │
│  march 15             46.5%  17.1%  🔴 ██████████                    │
│  march 31             60.5%  26.2%  🔴 ███████████████               │
│  june 30              69.5%  22.8%  🔴 █████████████                 │
│  december 31          74.5%  16.4%  🔴 █████████                     │
├──────────────────────────────────────────────────────────────────────┤
```

**How to read this:**
- **Cumulative** — Market probability that the event happens by this date
- **Conditional** — Probability it happens *in this period*, given it hasn't happened yet
- The market sees **late March as peak risk** (26.2% conditional)
- If nothing happens by June, the market thinks it probably won't (conditional drops to 16%)

**Close races** — markets nearest to 50/50, where uncertainty is highest:

```
├─ 🎯 Close Races (35-65%, >$100K vol) ───────────────────────────────┤
│ ► Will bitcoin hit $1m before GTA VI?     48.9¢  ±1.2¢ 🟢    $177K │
│   Israel strikes Iran by March 31?        52.5¢  ±2.5¢ 🟢     $97K │
│   US strikes Iran by March 15?            46.5¢  ±3.5¢ 🟢    $441K │
│   US-Iran nuclear deal before 2027?       53.0¢  ±3.0¢ 🟢     $6K  │
├──────────────────────────────────────────────────────────────────────┤
│ ⚠️  NOT TRADING ADVICE. Close ≠ opportunity. Math ≠ signal. DYOR.    │
└──────────────────────────────────────────────────────────────────────┘
```

### 💰 Spreads — Real bid-ask from the CLOB

Live midpoint and spread from Polymarket's Central Limit Order Book.
The spread is what you lose if you buy then immediately sell.

```
├──────────────────────────────────────────────────────────────────────┤
│  Market                                  Mid     Spread  Rating      │
│ ► Will Chelsea Clinton win 2028 Dem?    0.9¢     0.10¢  🟢 Tight    │
│   Will Indiana Pacers win 2026 NBA?     0.1¢     0.10¢  🟢 Tight    │
│   Will Oprah win 2028 Dem nomination?   0.9¢     0.10¢  🟢 Tight    │
│   Will Andrew Yang win 2028 Dem nom?    0.9¢     0.10¢  🟢 Tight    │
├──────────────────────────────────────────────────────────────────────┤
│ 💡 Spread = your cost. Buy + sell = you lose the spread.             │
│ 🟢 <0.5¢ Tight    🟡 0.5-2¢ Okay    🔴 >2¢ Wide                    │
└──────────────────────────────────────────────────────────────────────┘
```

---

## Keybindings

| Key | Action |
|-----|--------|
| `Tab` `Shift+Tab` | Switch views |
| `1`–`6` | Jump to view |
| `j` `k` `↑` `↓` | Navigate |
| `Ctrl+d` `Ctrl+u` | Page down / up |
| `g` | Jump to top |
| `/` | Search (Markets, Dashboard) |
| `Esc` | Clear search |
| `r` | Refresh market data |
| `s` | Refresh spreads |
| `a` | Run analysis |
| `q` `Ctrl+c` | Quit |

## Using with Polymarket CLI

This TUI is **research only**. To actually trade, use the official [Polymarket CLI](https://github.com/Polymarket/polymarket-cli):

```bash
brew tap Polymarket/polymarket-cli https://github.com/Polymarket/polymarket-cli
brew install polymarket

# Research here → Trade there
./target/release/polymarket-tui           # Find markets, check spreads
polymarket markets search "iran"          # Get token IDs
polymarket clob market-order ...          # Execute trade (requires wallet)
```

## Architecture

```
src/
├── main.rs              TUI entry point & event loop
├── app.rs               State machine, API orchestration
├── analysis.rs          Term structure, close races, cross-market math
├── api/
│   ├── types.rs         Market, Event, OrderBook + serde
│   ├── gamma.rs         gamma-api.polymarket.com
│   └── clob.rs          clob.polymarket.com
└── ui/
    ├── dashboard.rs     Stats + top markets
    ├── markets.rs       Searchable market list
    ├── events.rs        Multi-market event groups
    ├── spreads.rs       Real CLOB bid-ask spreads
    ├── analysis_view.rs Term structures + close races
    ├── help.rs          Keybindings reference
    └── theme.rs         Color palette
```

~2700 lines of Rust · 3.0 MB binary · ratatui + crossterm + reqwest + tokio

## License

MIT

---

<sub>⚠️ Not financial advice. Prediction markets are risky. You can lose your entire position. Do your own research.</sub>
