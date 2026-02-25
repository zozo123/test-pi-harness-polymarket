# polymarket-tui

A terminal UI for browsing [Polymarket](https://polymarket.com) prediction markets. Built in Rust.

> ⚠️ **Not trading advice.** Read-only. Does NOT execute trades. You can lose money. DYOR.

## Install & Run

```bash
git clone https://github.com/zozo123/test-pi-harness-polymarket.git
cd test-pi-harness-polymarket
cargo build --release
./target/release/polymarket-tui
```

No API key or wallet needed.

---

## Demo (live data, Feb 25 2026)

### Dashboard

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│ [Dashboard]  Markets   Events   Spreads   Help                       │
├─ Stats ──────────────────────────────────────────────────────────────┤
│  Markets: 200   Volume: $1.3B   24h: $18.0M   Close Races: 7        │
├─ Top Markets by Volume ──────────────────────────────────────────────┤
│ ► Will Chelsea Clinton win 2028 Dem nom?    0.9¢    $39.2M   $652K  │
│   Will Indiana Pacers win 2026 NBA Finals?  0.1¢    $38.7M    $2.4M │
│   Will Oprah win 2028 Dem nomination?       0.9¢    $35.6M     $83K │
│   Will Andrew Yang win 2028 Dem nom?        0.9¢    $32.3M     $68K │
│   Will Memphis Grizzlies win 2026 NBA?      0.1¢    $31.7M     $70K │
│   Will George Clooney win 2028 Dem nom?     0.9¢    $31.4M    $108K │
│   Will Hillary Clinton win 2028 Dem nom?    0.9¢    $30.5M    $140K │
│   Will MrBeast win 2028 Dem nom?            0.9¢    $30.4M     $97K │
│   Will Zohran Mamdani win 2028 Dem nom?     1.2¢    $30.0M     $81K │
│   Will Bernie Sanders win 2028 Dem nom?     0.9¢    $29.6M     $94K │
│   Will LeBron James win 2028 Dem nom?       0.9¢    $28.6M     $91K │
│   Will Phil Murphy win 2028 Dem nom?        1.1¢    $27.7M     $35K │
├──────────────────────────────────────────────────────────────────────┤
│ 200 markets, 100 events   r:refresh  Tab:view  j/k:nav  q:quit      │
└──────────────────────────────────────────────────────────────────────┘
```

### Search: Iran

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│  Dashboard  [Markets]  Events   Spreads   Help                       │
├─ 🔍 iran ────────────────────────────────────────────────────────────┤
│ ► US strikes Iran by February 28, 2026?       12.5¢       $43.4M    │
│   US strikes Iran by March 31, 2026?          60.5¢       $16.7M    │
│   Khamenei out as Supreme Leader by Mar 31?   18.5¢       $16.4M    │
│   Khamenei out by February 28?                 1.3¢       $12.0M    │
│   Israel strikes Iran by March 31, 2026?      52.5¢        $2.7M    │
│   Israel strikes Iran by February 28, 2026?   10.5¢        $2.4M    │
│   Iran Strike on Israel by February 28?        9.5¢       $535.0K   │
├──────────────────────────────────────────────────────────────────────┤
│ 7 results   /:search  Esc:clear  j/k:nav                            │
└──────────────────────────────────────────────────────────────────────┘
```

### Search: Ukraine

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│  Dashboard  [Markets]  Events   Spreads   Help                       │
├─ 🔍 ukraine ─────────────────────────────────────────────────────────┤
│ ► Russia x Ukraine ceasefire by Mar 31?        3.1¢       $19.7M    │
│   Russia x Ukraine ceasefire by end of 2026?  37.5¢       $10.3M    │
│   Russia x Ukraine ceasefire by Feb 28?        0.2¢        $5.7M    │
│   Russia x Ukraine ceasefire by Jun 30?       15.5¢        $1.9M    │
│   Russia-Ukraine Ceasefire before GTA VI?     59.0¢        $1.3M    │
│   Zelenskyy out as president by Mar 31?        2.5¢       $331.0K   │
├──────────────────────────────────────────────────────────────────────┤
│ 6 results   /:search  Esc:clear  j/k:nav                            │
└──────────────────────────────────────────────────────────────────────┘
```

### Events

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│  Dashboard   Markets  [Events]  Spreads   Help                       │
├─ Events (100) ───────────────────────────────────────────────────────┤
│ ► MicroStrategy sells any Bitcoin by ___?   4 mkts   Σ 19.6%        │
│   Kraken IPO by ___ ?                       3 mkts   Σ 85.5%        │
│   Macron out by...?                         3 mkts   Σ  3.5%        │
│   How many people will Trump deport?        9 mkts   Σ101.6%        │
│   UK election called by...?                 4 mkts   Σ  9.1%        │
│   China x India military clash by...?       3 mkts   Σ 10.0%        │
│   NATO/EU troops fighting in Ukraine?       2 mkts   Σ  2.5%        │
│   Starmer out by...?                        5 mkts   Σ126.3%        │
│   Ukraine recognizes Russian sovereignty?   3 mkts   Σ 23.0%        │
├──────────────────────────────────────────────────────────────────────┤
│ 100 events   j/k:nav  Tab:view  r:refresh                           │
└──────────────────────────────────────────────────────────────────────┘
```

### Spreads (live CLOB data)

```
┌─ ◆ Polymarket Browser ───────────────────────────────────────────────┐
│  Dashboard   Markets   Events  [Spreads]  Help                       │
├──────────────────────────────────────────────────────────────────────┤
│  Market                                  Mid     Spread  Rating      │
│ ► Will Chelsea Clinton win 2028 Dem?    0.9¢     0.10¢  🟢 Tight    │
│   Will Indiana Pacers win 2026 NBA?     0.1¢     0.10¢  🟢 Tight    │
│   Will Oprah win 2028 Dem nomination?   0.9¢     0.10¢  🟢 Tight    │
│   Will Andrew Yang win 2028 Dem nom?    0.9¢     0.10¢  🟢 Tight    │
│   Will Memphis Grizzlies win 2026 NBA?  0.1¢     0.10¢  🟢 Tight    │
│   Will MrBeast win 2028 Dem nom?        0.9¢     0.10¢  🟢 Tight    │
│   Will Zohran Mamdani win 2028 Dem?     1.2¢     0.10¢  🟢 Tight    │
│   Will Bernie Sanders win 2028 Dem?     0.9¢     0.10¢  🟢 Tight    │
├──────────────────────────────────────────────────────────────────────┤
│ 💡 Spread = your cost. Buy+sell = you lose the spread.               │
│ 🟢 <0.5¢ Tight    🟡 0.5-2¢ Okay    🔴 >2¢ Wide                    │
├──────────────────────────────────────────────────────────────────────┤
│ s:refresh spreads  j/k:nav  Tab:view  q:quit                        │
└──────────────────────────────────────────────────────────────────────┘
```

---

## Record a Demo Video

```bash
# Option 1: asciinema (https://asciinema.org)
asciinema rec demo.cast -c "./target/release/polymarket-tui"
asciinema play demo.cast

# Option 2: vhs (https://github.com/charmbracelet/vhs)
vhs < demo.tape
```

## Keybindings

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Switch views |
| `1`-`5` | Jump to view |
| `j` / `k` | Navigate |
| `Ctrl+d` / `Ctrl+u` | Page down/up |
| `g` | Jump to top |
| `/` | Search |
| `Esc` | Clear search |
| `r` | Refresh data |
| `s` | Refresh spreads |
| `q` | Quit |

## Using with Polymarket CLI

This TUI is for **research**. To trade, use the official [Polymarket CLI](https://github.com/Polymarket/polymarket-cli):

```bash
# Install official CLI
brew tap Polymarket/polymarket-cli https://github.com/Polymarket/polymarket-cli
brew install polymarket

# Research in TUI → Trade via CLI
./target/release/polymarket-tui           # Find markets, check spreads
polymarket markets search "iran"          # Dig deeper
polymarket clob book TOKEN_ID             # Check order book
polymarket clob market-order ...          # Execute (requires wallet)
```

## Architecture

```
src/
├── main.rs          # TUI entry point & event loop
├── app.rs           # State machine, API calls
├── api/
│   ├── types.rs     # Market, Event, OrderBook types
│   ├── gamma.rs     # gamma-api.polymarket.com
│   └── clob.rs      # clob.polymarket.com
└── ui/
    ├── dashboard.rs # Stats + top markets
    ├── markets.rs   # Search + browse
    ├── events.rs    # Multi-market events
    ├── spreads.rs   # Real CLOB spreads
    ├── help.rs      # Keybindings
    └── theme.rs     # Colors
```

## License

MIT — Not financial advice. You can lose money.
