# polymarket-tui

A terminal UI for browsing [Polymarket](https://polymarket.com) prediction markets. Built in Rust.

> ⚠️ **Not trading advice.** This tool does NOT execute trades. You can lose money. DYOR.

## Install & Run

```bash
git clone https://github.com/zozo123/test-pi-harness-polymarket.git
cd test-pi-harness-polymarket
cargo build --release
./target/release/polymarket-tui
```

No API key or wallet required — read-only public APIs.

## Screenshots

### Dashboard — Stats & Top Markets

```
┌─ ◆ Polymarket Browser ──────────────────────────────────────────────────┐
│ [Dashboard]  Markets   Events   Spreads   Help                          │
├─────────────────────────────────────────────────────────────────────────┤
│ Stats                                                                   │
│  Markets: 200  │  Volume: $1.3B  │  24h: $18.0M  │  Close Races: 7     │
├─────────────────────────────────────────────────────────────────────────┤
│ Top Markets by Volume                                                   │
│  Market                                           YES     Volume   24h  │
│ ► Will Chelsea Clinton win 2028 Dem nom?         0.9¢    $39.2M  $652K  │
│   Will Indiana Pacers win 2026 NBA Finals?       0.1¢    $38.7M  $2.4M  │
│   Will Oprah win 2028 Dem nomination?            0.9¢    $35.6M   $83K  │
│   Will Andrew Yang win 2028 Dem nomination?      0.9¢    $32.3M   $68K  │
│   Will Memphis Grizzlies win 2026 NBA Finals?    0.1¢    $31.7M   $70K  │
│   Will George Clooney win 2028 Dem nom?          0.9¢    $31.4M  $108K  │
│   Will Hillary Clinton win 2028 Dem nom?         0.9¢    $30.5M  $140K  │
│   Will MrBeast win 2028 Dem nomination?          0.9¢    $30.4M   $97K  │
│   Will Zohran Mamdani win 2028 Dem nom?          1.2¢    $30.0M   $81K  │
│   Will Bernie Sanders win 2028 Dem nom?          0.9¢    $29.6M   $94K  │
├─────────────────────────────────────────────────────────────────────────┤
│ 200 markets, 100 events    r:refresh  Tab:view  j/k:nav  /:search  q:quit │
└─────────────────────────────────────────────────────────────────────────┘
```

### Markets — Search by Keyword

```
┌─ ◆ Polymarket Browser ──────────────────────────────────────────────────┐
│  Dashboard  [Markets]  Events   Spreads   Help                          │
├─ 🔍 bitcoin ───────────────────────────────────────────────────────────┤
│  Market                                           YES     Volume        │
│ ► Will bitcoin hit $1m before GTA VI?           48.8¢      $3.4M        │
├─────────────────────────────────────────────────────────────────────────┤
│ 1 result   /:search  Esc:clear  j/k:nav                                │
└─────────────────────────────────────────────────────────────────────────┘
```

### Events — Multi-Market Groupings

```
┌─ ◆ Polymarket Browser ──────────────────────────────────────────────────┐
│  Dashboard   Markets  [Events]  Spreads   Help                          │
├─────────────────────────────────────────────────────────────────────────┤
│  Event                                       Mkts   Σ YES     Volume    │
│ ► MicroStrategy sells any Bitcoin by ___?       4   19.6%         $0    │
│   Kraken IPO by ___ ?                           3   85.0%         $0    │
│   Macron out by...?                             3    3.5%         $0    │
│   How many people will Trump deport in 2025?    9  101.7%         $0    │
│   UK election called by...?                     4    9.1%         $0    │
│   China x India military clash by...?           3   10.0%         $0    │
│   NATO/EU troops fighting in Ukraine by...?     2    2.5%         $0    │
├─────────────────────────────────────────────────────────────────────────┤
│ 100 events   j/k:nav  Tab:view                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Spreads — Real Bid-Ask Data from CLOB

```
┌─ ◆ Polymarket Browser ──────────────────────────────────────────────────┐
│  Dashboard   Markets   Events  [Spreads]  Help                          │
├─────────────────────────────────────────────────────────────────────────┤
│  Market                                  Midpoint  Spread   Rating      │
│ ► Will Chelsea Clinton win 2028 Dem?       0.9¢    0.10¢   🟢 Tight    │
│   Will Indiana Pacers win 2026 NBA?        0.1¢    0.10¢   🟢 Tight    │
│   Will Oprah win 2028 Dem nomination?      0.9¢    0.10¢   🟢 Tight    │
│   Will Andrew Yang win 2028 Dem nom?       0.9¢    0.10¢   🟢 Tight    │
│   Will Memphis Grizzlies win 2026 NBA?     0.1¢    0.10¢   🟢 Tight    │
│   Will MrBeast win 2028 Dem nomination?    0.9¢    0.10¢   🟢 Tight    │
├─────────────────────────────────────────────────────────────────────────┤
│ 💡 Spread = your trading cost. Buy then sell = you lose the spread.     │
│ 🟢 Tight (<0.5¢)   🟡 Okay (0.5-2¢)   🔴 Wide (>2¢)                  │
├─────────────────────────────────────────────────────────────────────────┤
│ s:refresh spreads  j/k:nav  Tab:view  q:quit                           │
└─────────────────────────────────────────────────────────────────────────┘
```

## Keybindings

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Switch views |
| `1`-`5` | Jump to view |
| `j` / `k` | Navigate up/down |
| `Ctrl+d` / `Ctrl+u` | Page down/up |
| `g` | Jump to top |
| `/` | Search (Markets view) |
| `Esc` | Clear search |
| `r` | Refresh market data |
| `s` | Refresh spreads (Spreads view) |
| `q` | Quit |

## Using with Polymarket CLI

This TUI is for **research**. To trade, use the official [Polymarket CLI](https://github.com/Polymarket/polymarket-cli):

```bash
# Install official CLI
brew tap Polymarket/polymarket-cli https://github.com/Polymarket/polymarket-cli
brew install polymarket

# 1. Find markets here
./target/release/polymarket-tui

# 2. Trade via official CLI
polymarket clob market-order --token TOKEN_ID --side buy --amount 10
```

## Recording a Demo

Record your terminal session with [asciinema](https://asciinema.org/) or [vhs](https://github.com/charmbracelet/vhs):

```bash
# asciinema
asciinema rec demo.cast -c "./target/release/polymarket-tui"

# vhs (Charm)
vhs < demo.tape
```

## Architecture

```
src/
├── main.rs           # TUI entry point & event loop
├── app.rs            # State machine, data fetching
├── api/
│   ├── types.rs      # Market, Event, OrderBook types
│   ├── gamma.rs      # gamma-api.polymarket.com
│   └── clob.rs       # clob.polymarket.com
└── ui/
    ├── dashboard.rs  # Stats + top markets
    ├── markets.rs    # Searchable market list
    ├── events.rs     # Multi-market events
    ├── spreads.rs    # Real bid-ask spreads
    ├── help.rs       # Keybindings
    └── theme.rs      # Colors
```

## License

MIT
