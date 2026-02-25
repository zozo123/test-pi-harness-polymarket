# polymarket-tui

A terminal UI for browsing [Polymarket](https://polymarket.com) prediction markets. Real data, honest math, no fake signals.

> ⚠️ **Not trading advice.** Read-only browser. Does NOT execute trades. You can lose money. DYOR.

## Install & Run

```bash
git clone https://github.com/zozo123/test-pi-harness-polymarket.git
cd test-pi-harness-polymarket
cargo build --release
./target/release/polymarket-tui
```

No API key or wallet needed — read-only public APIs only.

---

## Demo (live data — Feb 25, 2026)

### 1. Dashboard

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
│   Will Hillary Clinton win the 2028 Democrati…  0.9¢  $30.5M  $140K │
│   Will MrBeast win the 2028 Democratic presid…  0.9¢  $30.4M   $97K │
│   Will Zohran Mamdani win the 2028 Democratic…  1.2¢  $30.0M   $81K │
│   Will Bernie Sanders win the 2028 Democratic…  0.9¢  $29.6M   $94K │
│   Will LeBron James win the 2028 Democratic p…  0.9¢  $28.6M   $91K │
│   Will Phil Murphy win the 2028 Democratic pr…  1.1¢  $27.7M   $35K │
├──────────────────────────────────────────────────────────────────────┤
│ 200 markets, 100 events   r:refresh  Tab:view  j/k:nav  q:quit      │
└──────────────────────────────────────────────────────────────────────┘
```

### 2. Search: Iran — $43M+ volume

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│  Dashboard  [Markets]  Events   Spreads   Analysis   Help            │
├─ 🔍 iran ────────────────────────────────────────────────────────────┤
│ ► US strikes Iran by February 28, 2026?       12.5¢       $43.6M    │
│   US strikes Iran by March 31, 2026?          60.5¢       $16.7M    │
│   Khamenei out as Supreme Leader by Mar 31?   18.5¢       $16.4M    │
│   Khamenei out by February 28?                 1.2¢       $12.0M    │
│   US strikes Iran by June 30, 2026?           69.5¢        $7.4M    │
│   US strikes Iran by February 25, 2026?        1.5¢        $6.0M    │
│   Khamenei out by June 30?                    33.5¢        $5.9M    │
│   US strikes Iran by March 15, 2026?          47.5¢        $3.6M    │
│   US strikes Iran by February 26, 2026?        3.0¢        $3.5M    │
├──────────────────────────────────────────────────────────────────────┤
│ 31 results   /:search  Esc:clear  j/k:nav                           │
└──────────────────────────────────────────────────────────────────────┘
```

### 3. Search: Ukraine — Ceasefire timeline

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│  Dashboard  [Markets]  Events   Spreads   Analysis   Help            │
├─ 🔍 ukraine ─────────────────────────────────────────────────────────┤
│ ► Russia x Ukraine ceasefire by Mar 31?        3.0¢       $19.7M    │
│   Russia x Ukraine ceasefire by end 2026?     37.5¢       $10.3M    │
│   Russia x Ukraine ceasefire by Feb 28?        0.2¢        $5.7M    │
│   Russia x Ukraine ceasefire by Jun 30?       15.5¢        $1.9M    │
│   Zelenskyy out by end of 2026?               24.5¢        $1.7M    │
│   Russia-Ukraine Ceasefire before GTA VI?     58.5¢        $1.3M    │
│   Zelenskyy out by March 31?                   2.5¢       $331.0K   │
├──────────────────────────────────────────────────────────────────────┤
│ 8 results   /:search  Esc:clear  j/k:nav                            │
└──────────────────────────────────────────────────────────────────────┘
```

### 4. Analysis — Term Structure & Close Races

The Analysis view computes **conditional probabilities** from cumulative "by date" markets:
*"If there's no strike by Feb 28, what's the probability of a strike Mar 1-7?"*

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│  Dashboard   Markets   Events   Spreads  [Analysis]  Help            │
├─ 📈 US Strikes Iran — Term Structure ────────────────────────────────┤
│  Date                Cumul.  Cond.                                    │
│  february 25           1.5%   1.5%  🟢                               │
│  february 26           3.0%   1.6%  🟢                               │
│  february 27           6.5%   3.6%  🟢 ██                            │
│  february 28          12.5%   6.4%  🟡 ███                           │
│  march 1              15.5%   3.4%  🟢 ██                            │
│  march 5              27.5%   5.2%  🟡 ███                           │
│  march 7              35.5%   4.4%  🟢 ██                            │
│  march 15             47.5%  18.6%  🔴 ███████████                   │
│  march 31             60.5%  24.8%  🔴 ██████████████                │
│  june 30              69.5%  22.8%  🔴 █████████████                 │
│  december 31          74.5%  16.4%  🔴 █████████                     │
├─ 🎯 Close Races (35-65%, >$100K vol) ───────────────────────────────┤
│ ► Will bitcoin hit $1m before GTA VI?     48.9¢  ±1.2¢ 🟢    $177K  │
│   US strikes Iran by March 15, 2026?      47.5¢  ±2.5¢ 🟢    $441K  │
│   Israel strikes Iran by March 31?        52.5¢  ±2.5¢ 🟢     $97K  │
│   Khamenei out in 2026?                   47.5¢  ±2.5¢ 🟢     $19K  │
├──────────────────────────────────────────────────────────────────────┤
│ ⚠️  NOT TRADING ADVICE. Close ≠ opportunity. Math ≠ signal. DYOR.    │
└──────────────────────────────────────────────────────────────────────┘
```

**Reading the term structure:**
- **Cumulative 60.5%** by Mar 31 — market says more likely than not
- **Conditional 24.8%** for Mar 8-31 — if no strike by Mar 7, ~25% chance in next 3 weeks
- **Conditional drops** after Jun 30 — if it hasn't happened by then, market thinks it probably won't

### 5. Spreads — Real CLOB bid-ask data

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│  Dashboard   Markets   Events  [Spreads]  Analysis   Help            │
├──────────────────────────────────────────────────────────────────────┤
│  Market                                  Mid     Spread  Rating      │
│ ► Will Chelsea Clinton win 2028 Dem?    0.9¢     0.10¢  🟢 Tight    │
│   Will Indiana Pacers win 2026 NBA?     0.1¢     0.10¢  🟢 Tight    │
│   Will Oprah win 2028 Dem nomination?   0.9¢     0.10¢  🟢 Tight    │
│   Will Andrew Yang win 2028 Dem nom?    0.9¢     0.10¢  🟢 Tight    │
│   Will Memphis Grizzlies win 2026 NBA?  0.1¢     0.10¢  🟢 Tight    │
│   Will MrBeast win 2028 Dem nom?        0.9¢     0.10¢  🟢 Tight    │
├──────────────────────────────────────────────────────────────────────┤
│ 💡 Spread = your cost. Buy+sell = you lose the spread.               │
│ 🟢 <0.5¢ Tight    🟡 0.5-2¢ Okay    🔴 >2¢ Wide                    │
└──────────────────────────────────────────────────────────────────────┘
```

---

## Keybindings

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Switch views |
| `1`-`6` | Jump to view |
| `j` / `k` | Navigate |
| `Ctrl+d` / `Ctrl+u` | Page down/up |
| `g` | Jump to top |
| `/` | Search (Markets, Dashboard) |
| `Esc` | Clear search |
| `r` | Refresh data |
| `s` | Refresh spreads (Spreads view) |
| `a` | Run analysis (Analysis view) |
| `q` / `Ctrl+c` | Quit |

## Using with Polymarket CLI

This TUI is for **research only**. To trade, use the official [Polymarket CLI](https://github.com/Polymarket/polymarket-cli):

```bash
# Install
brew tap Polymarket/polymarket-cli https://github.com/Polymarket/polymarket-cli
brew install polymarket

# Research in TUI → Trade via CLI
./target/release/polymarket-tui                # Browse, search, analyze
polymarket markets search "iran"               # Dig deeper via CLI
polymarket clob market-order ...               # Execute (requires wallet)
```

## Architecture

```
polymarket-tui/
├── Cargo.toml
├── src/
│   ├── main.rs             # TUI entry point & event loop
│   ├── lib.rs              # Crate root (usable as library)
│   ├── app.rs              # State machine, API orchestration
│   ├── analysis.rs         # Term structure, close races, cross-market checks
│   ├── api/
│   │   ├── types.rs        # Market, Event, OrderBook types + serde
│   │   ├── gamma.rs        # gamma-api.polymarket.com (markets, events)
│   │   └── clob.rs         # clob.polymarket.com (midpoint, spread, book)
│   ├── ui/
│   │   ├── dashboard.rs    # Stats + top markets
│   │   ├── markets.rs      # Searchable market list
│   │   ├── events.rs       # Multi-market event groups
│   │   ├── spreads.rs      # Real CLOB bid-ask spreads
│   │   ├── analysis_view.rs # Term structures + close races
│   │   ├── help.rs         # Keybindings reference
│   │   └── theme.rs        # Color palette
│   └── bin/
│       └── show.rs         # Headless renderer (for screenshots)
└── README.md
```

**APIs used (no auth needed):**
- `gamma-api.polymarket.com` — market metadata, events, prices
- `clob.polymarket.com` — real-time midpoint, spread, order book

**~2750 lines of Rust. 3.0 MB binary. Zero dependencies beyond Cargo.**

## License

MIT — Not financial advice. You can lose money.
