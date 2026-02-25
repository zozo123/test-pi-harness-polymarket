# polymarket-tui

A terminal UI for browsing [Polymarket](https://polymarket.com) prediction markets.

```
┌────────────────────────────────────────────────────────────────────┐
│  ⚠️  FOR RESEARCH ONLY — NOT TRADING ADVICE                        │
│  This tool does NOT execute trades. You can lose money. DYOR.      │
└────────────────────────────────────────────────────────────────────┘
```

## Screenshots

### Dashboard
```
┌─ ◆ Polymarket Browser ─────────────────────────────────────────────┐
│  Dashboard   Markets   Events   Spreads   Help                     │
├─ Stats ────────────────────────────────────────────────────────────┤
│  Markets: 200  │  Total Volume: $1.3B  │  24h Volume: $18.0M       │
│  Events: 100   │  Close Races: 7       │  Spreads loaded: 20       │
├─ Top Markets by Volume ────────────────────────────────────────────┤
│  Market                                      YES      Volume   24h │
│► Will Chelsea Clinton win 2028 Dem nom?     0.9¢    $39.2M  $652K │
│  Will the Indiana Pacers win 2026 NBA?      0.1¢    $38.2M  $2.4M │
│  Will Oprah win 2028 Dem nomination?        0.9¢    $35.6M  $180K │
│  Will Andrew Yang win 2028 Dem nom?         0.9¢    $32.2M   $95K │
│  Will Jesus Christ return before GTA VI?   48.5¢     $9.5M  $120K │
├────────────────────────────────────────────────────────────────────┤
│ Ready — 200 markets, 100 events   r:refresh  Tab:view  q:quit     │
└────────────────────────────────────────────────────────────────────┘
```

### Spreads View
```
┌─ ◆ Polymarket Browser ─────────────────────────────────────────────┐
│  Dashboard   Markets   Events  [Spreads]  Help                     │
├─ Spreads (20) ─────────────────────────────────────────────────────┤
│  Market                              Midpoint  Spread   Rating     │
│► Will Indiana Pacers win 2026 NBA?     0.1¢    0.20¢   🟢 Tight   │
│  Will Trump deport 250-500k people?   92.5¢    0.50¢   🟢 Tight   │
│  Will Jesus Christ return b4 GTA VI?  48.5¢    1.00¢   🟡 Okay    │
│  Will bitcoin hit $1m before GTA VI?  48.8¢    0.80¢   🟡 Okay    │
├────────────────────────────────────────────────────────────────────┤
│  💡 SPREAD = YOUR TRADING COST                                     │
│  If you buy then immediately sell, you lose the spread.            │
│  🟢 Tight (<0.5¢) = Low   🟡 Okay (0.5-2¢)   🔴 Wide (>2¢)        │
├────────────────────────────────────────────────────────────────────┤
│ Press 's' to refresh spreads   j/k:nav  Tab:view  q:quit          │
└────────────────────────────────────────────────────────────────────┘
```

### Markets Search
```
┌─ ◆ Polymarket Browser ─────────────────────────────────────────────┐
│  Dashboard  [Markets]  Events   Spreads   Help                     │
├─ Search ───────────────────────────────────────────────────────────┤
│  🔍 bitcoin▊  (3 results)                                          │
├─ Markets (3) ──────────────────────────────────────────────────────┤
│► Will bitcoin hit $1m before GTA VI?    48.8¢   $3.4M    Active   │
│  Will bitcoin hit $150k in February?     0.1¢  $28.1M    Active   │
│  Will bitcoin dip to $50k in February?   0.9¢   $4.7M    Active   │
├────────────────────────────────────────────────────────────────────┤
│ Market 1/3   /:search  Esc:clear  j/k:nav  Tab:view  q:quit       │
└────────────────────────────────────────────────────────────────────┘
```

## Install

```bash
git clone https://github.com/zozo123/test-pi-harness-polymarket.git
cd test-pi-harness-polymarket
cargo build --release
./target/release/polymarket-tui
```

## Keybindings

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Switch views |
| `1-5` | Jump to view |
| `j` / `k` / `↑` / `↓` | Navigate |
| `Ctrl+d` / `Ctrl+u` | Page down/up |
| `g` | Jump to top |
| `/` | Search (Markets view) |
| `Esc` | Clear search |
| `r` | Refresh data |
| `s` | Refresh spreads (Spreads view) |
| `q` | Quit |

## Views

| View | Description |
|------|-------------|
| **Dashboard** | Stats overview, top markets by volume |
| **Markets** | Browse all markets, search by keyword |
| **Events** | Multi-market event groupings |
| **Spreads** | Real bid-ask spreads from CLOB API |
| **Help** | Keybindings reference |

## Using with Polymarket CLI

This TUI is for **research**. To actually trade, use the official [Polymarket CLI](https://github.com/Polymarket/polymarket-cli):

```bash
# Install the official CLI
brew tap Polymarket/polymarket-cli https://github.com/Polymarket/polymarket-cli
brew install polymarket

# Browse here, trade there
./target/release/polymarket-tui     # Find markets, check spreads
polymarket clob market-order ...     # Execute trades via CLI
```

### Workflow

1. **Find markets** — Use this TUI to browse, search, check spreads
2. **Research** — Check volume, liquidity, price history
3. **Trade** — Use `polymarket` CLI with your wallet to execute

## What This Tool Does

| ✅ Does | ❌ Does NOT |
|---------|-------------|
| Browse real market data | Execute trades |
| Show actual spreads (CLOB API) | Require a wallet |
| Search and filter markets | Guarantee profits |
| Track volume and activity | Find "free" arbitrage |

## API Endpoints

| Endpoint | Purpose |
|----------|---------|
| `gamma-api.polymarket.com/markets` | Market data |
| `gamma-api.polymarket.com/events` | Event groupings |
| `clob.polymarket.com/midpoint` | Current price |
| `clob.polymarket.com/spread` | Bid-ask spread |

No API key required — all read-only public endpoints.

## Tech Stack

- **Rust** — Fast, safe, single binary
- **ratatui** — Terminal UI framework
- **crossterm** — Cross-platform terminal handling
- **tokio** — Async runtime
- **reqwest** — HTTP client

## License

MIT — See [LICENSE](LICENSE)

---

**Not financial advice. You can lose money on prediction markets. DYOR.**
