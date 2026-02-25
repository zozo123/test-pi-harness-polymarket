# polymarket-tui

A terminal UI for browsing [Polymarket](https://polymarket.com) prediction markets. Real data, honest math.

> ⚠️ **Not trading advice.** Read-only browser. Does NOT execute trades. You can lose money. DYOR.

## Install & Run

```bash
git clone https://github.com/zozo123/test-pi-harness-polymarket.git
cd test-pi-harness-polymarket
cargo build --release
./target/release/polymarket-tui
```

No API key or wallet needed — read-only public APIs.

---

## Demo (live data — Feb 25, 2026)

### Dashboard — $2.5B total volume

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│ [Dashboard]  Markets   Events   Spreads   Analysis   Help            │
├─ Stats ──────────────────────────────────────────────────────────────┤
│  Markets: 200   Volume: $1.3B   24h: $18.0M   Close Races: 7        │
├─ Top Markets by Volume ──────────────────────────────────────────────┤
│ ► Will Chelsea Clinton win 2028 Dem nom?    0.9¢    $39.2M   $652K  │
│   Will Indiana Pacers win 2026 NBA Finals?  0.1¢    $39.1M    $2.4M │
│   Will Oprah win 2028 Dem nomination?       0.9¢    $35.6M     $83K │
│   Will Andrew Yang win 2028 Dem nom?        0.9¢    $32.3M     $68K │
│   Will Memphis Grizzlies win 2026 NBA?      0.1¢    $31.7M     $70K │
│   Will George Clooney win 2028 Dem nom?     0.9¢    $31.4M    $108K │
│   Will Hillary Clinton win 2028 Dem nom?    0.9¢    $30.5M    $140K │
│   Will MrBeast win 2028 Dem nom?            0.9¢    $30.4M     $97K │
│   Will Zohran Mamdani win 2028 Dem nom?     1.2¢    $30.0M     $81K │
│   Will Bernie Sanders win 2028 Dem nom?     0.9¢    $29.6M     $94K │
├──────────────────────────────────────────────────────────────────────┤
│ 200 markets, 100 events   r:refresh  Tab:view  j/k:nav  q:quit      │
└──────────────────────────────────────────────────────────────────────┘
```

### Search: Iran — 31 markets, $43M+ volume

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
│   Israel strikes Iran by March 31, 2026?      52.5¢        $2.7M    │
├──────────────────────────────────────────────────────────────────────┤
│ 31 results   /:search  Esc:clear  j/k:nav                           │
└──────────────────────────────────────────────────────────────────────┘
```

### Search: Ukraine — Ceasefire timeline

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│  Dashboard  [Markets]  Events   Spreads   Analysis   Help            │
├─ 🔍 ukraine ─────────────────────────────────────────────────────────┤
│ ► Russia x Ukraine ceasefire by Mar 31?        3.0¢       $19.7M    │
│   Russia x Ukraine ceasefire by end 2026?     37.5¢       $10.3M    │
│   Russia x Ukraine ceasefire by Feb 28?        0.2¢        $5.7M    │
│   Russia x Ukraine ceasefire by Jun 30?       15.5¢        $1.9M    │
│   Zelenskyy out by end of 2026?               24.5¢        $1.7M    │
│   Russia-Ukraine Ceasefire before GTA VI?     59.0¢        $1.3M    │
│   Zelenskyy out by March 31?                   2.5¢       $331.0K   │
├──────────────────────────────────────────────────────────────────────┤
│ 8 results   /:search  Esc:clear  j/k:nav                            │
└──────────────────────────────────────────────────────────────────────┘
```

### Analysis — Iran Strike Term Structure & Close Races

The Analysis view computes **conditional probabilities** from cumulative "by date" markets.

"If there's no strike by Feb 28, what's the probability of a strike between Mar 1–7?"

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│  Dashboard   Markets   Events   Spreads  [Analysis]  Help            │
├─ 📈 US Strikes Iran — Term Structure ────────────────────────────────┤
│  Date                  Cumul.  Conditional                            │
│  february 25, 2026       1.5%     1.5%  🟢                           │
│  february 26, 2026       3.0%     1.6%  🟢                           │
│  february 27, 2026       6.5%     3.6%  🟢 ██                        │
│  february 28, 2026      12.5%     6.4%  🟡 ███                       │
│  march 1, 2026          15.5%     3.4%  🟢 ██                        │
│  march 5, 2026          27.5%     5.2%  🟡 ███                       │
│  march 7, 2026          35.5%     4.4%  🟢 ██                        │
│  march 15, 2026         47.5%    18.6%  🔴 ███████████               │
│  march 31, 2026         60.5%    24.8%  🔴 ██████████████            │
│  june 30, 2026          69.5%    22.8%  🔴 █████████████             │
│  december 31, 2026      74.5%    16.4%  🔴 █████████                 │
├─ 🎯 Close Races (35-65%, >$100K vol) ───────────────────────────────┤
│ ► Will bitcoin hit $1m before GTA VI?     48.8¢  ±1.2¢ 🟢   $177K  │
│   US strikes Iran by March 15, 2026?      47.5¢  ±2.5¢ 🟢   $441K  │
│   Israel strikes Iran by March 31?        52.5¢  ±2.5¢ 🟢    $97K  │
│   Khamenei out in 2026?                   47.5¢  ±2.5¢ 🟢    $19K  │
│   Russia-Ukraine Ceasefire before GTA VI? 59.0¢  ±9.0¢ 🟡    $703  │
├──────────────────────────────────────────────────────────────────────┤
│ ⚠️  NOT TRADING ADVICE. Close ≠ opportunity. Math ≠ signal. DYOR.    │
└──────────────────────────────────────────────────────────────────────┘
```

**Reading the term structure:**
- **Cumulative 60.5%** by Mar 31 — market says more likely than not
- **Conditional 24.8%** for Mar 8–31 — if no strike by Mar 7, ~25% chance in next 3 weeks
- **Conditional drops** after Jun 30 — if it doesn't happen by June, market thinks it probably won't

### Spreads — Real CLOB bid-ask data

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

## What the Data Says (as of Feb 25, 2026)

### 🇮🇷 Iran — Market Scenario Tree

```
                     ┌─ Khamenei out (31%)    → regime change
  US strikes (60%) ──┤
                     └─ Khamenei stays (69%)  → limited strikes
  │
  No strike (40%) ───── Neither strikes (40%) → deal / status quo

  Key: Israel essentially never strikes without the US (0% solo prob)
  Key: US already met with Iran (94.5¢) — meeting ≠ deal
  Key: Nuclear deal by Jun 30 only 41.5¢ — market skeptical
```

### 🇺🇦 Ukraine — Ceasefire Probability Curve

```
  By Mar 31:   3.1%   ← almost impossible
  By Jun 30:  15.5%   ← still unlikely
  By Dec 31:  37.5%   ← about 1 in 3

  Peace deal by end 2026:   30.5%  (< ceasefire → frozen conflict gap)
  Zelenskyy out by Dec:     24.5%
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
| `/` | Search |
| `Esc` | Clear search |
| `r` | Refresh data |
| `s` | Refresh spreads |
| `a` | Run analysis |
| `q` | Quit |

## Using with Polymarket CLI

This TUI is for **research**. To trade, use the official [Polymarket CLI](https://github.com/Polymarket/polymarket-cli):

```bash
# Install official CLI
brew tap Polymarket/polymarket-cli https://github.com/Polymarket/polymarket-cli
brew install polymarket

# Research in TUI → Trade via CLI
./target/release/polymarket-tui                # Browse, search, analyze
polymarket markets search "iran"               # Dig deeper
polymarket clob book TOKEN_ID                  # Check order book
polymarket clob market-order ...               # Execute (needs wallet)
```

## Architecture

```
src/
├── main.rs          # TUI entry point & event loop
├── app.rs           # State machine, API calls
├── analysis.rs      # Term structure, close races, cross-market checks
├── api/
│   ├── types.rs     # Market, Event, OrderBook types
│   ├── gamma.rs     # gamma-api.polymarket.com
│   └── clob.rs      # clob.polymarket.com
└── ui/
    ├── dashboard.rs    # Stats + top markets
    ├── markets.rs      # Search + browse
    ├── events.rs       # Multi-market events
    ├── spreads.rs      # Real CLOB spreads
    ├── analysis_view.rs # Term structures + close races
    ├── help.rs         # Keybindings
    └── theme.rs        # Colors
```

## License

MIT — Not financial advice. You can lose money.
