# polymarket-tui

Terminal tools for browsing [Polymarket](https://polymarket.com) prediction markets.

```
┌─────────────────────────────────────────────────────────────────┐
│  ⚠️  FOR RESEARCH ONLY — NOT TRADING ADVICE                     │
│  You can lose money. This tool does NOT execute trades. DYOR.   │
└─────────────────────────────────────────────────────────────────┘
```

## Install

```bash
git clone https://github.com/zozo123/test-pi-harness-polymarket.git
cd test-pi-harness-polymarket
cargo build --release
```

## Tools

### 1. Market Browser (`demo`)

Browse markets by volume, activity, and confidence level:

```bash
./target/release/demo
```

```
╔══════════════════════════════════════════════════════════════════╗
║     POLYMARKET BROWSER                                           ║
╠══════════════════════════════════════════════════════════════════╣
║  Browse prediction markets. Not trading advice. DYOR.            ║
╚══════════════════════════════════════════════════════════════════╝

📡 Fetching markets...
   ✅ 200 active markets

═══════════════════════════════════════════════════════════════════
💰 MOST ACTIVE (by total volume)
═══════════════════════════════════════════════════════════════════

   Will Chelsea Clinton win the 2028 Democratic pres… 
   └─ YES:   0.9¢  │  Vol: $39.2M

   Will the Indiana Pacers win the 2026 NBA Finals? 
   └─ YES:   0.1¢  │  Vol: $38.2M

   Will Oprah Winfrey win the 2028 Democratic presid… 
   └─ YES:   0.9¢  │  Vol: $35.6M

═══════════════════════════════════════════════════════════════════
📈 HIGHEST 24H VOLUME (recent activity)
═══════════════════════════════════════════════════════════════════

   Will the Indiana Pacers win the 2026 NBA Finals? 
   └─ YES:   0.1¢  │  24h: $2.4M

   Will Barack Obama win the 2028 Democratic preside… 
   └─ YES:   1.2¢  │  24h: $775.2K

═══════════════════════════════════════════════════════════════════
⚖️  CLOSE RACES (YES price 40-60%)
═══════════════════════════════════════════════════════════════════

   Russia-Ukraine Ceasefire before GTA VI? 
   └─ YES:  58.0¢  │  Vol: $1.3M

   Will Jesus Christ return before GTA VI? 
   └─ YES:  48.5¢  │  Vol: $9.5M

   Will bitcoin hit $1m before GTA VI? 
   └─ YES:  48.8¢  │  Vol: $3.4M

═══════════════════════════════════════════════════════════════════
🎯 HIGH CONFIDENCE (YES >90% or <10%, Vol >$500K)
═══════════════════════════════════════════════════════════════════

   Will Trump deport 250,000-500,000 people? 
   └─ YES:  92.5¢  │  YES likely  │  Vol: $7.5M

   Will GTA 6 cost $100+? 
   └─ YES:   0.4¢  │  NO likely  │  Vol: $6.0M

═══════════════════════════════════════════════════════════════════
📊 SUMMARY
═══════════════════════════════════════════════════════════════════
   Active markets:     200
   Total volume:       $1.3B
   24h volume:         $18.0M
   Close races:        7
   High confidence:    143
```

### 2. Liquidity Checker (`liquidity`)

Check real spreads before trading — **this is your cost to enter/exit**:

```bash
./target/release/liquidity
```

```
╔══════════════════════════════════════════════════════════════════╗
║     LIQUIDITY CHECKER                                            ║
╠══════════════════════════════════════════════════════════════════╣
║  Check spreads before trading. Spread = your cost to enter/exit. ║
╚══════════════════════════════════════════════════════════════════╝

═══════════════════════════════════════════════════════════════════
📊 SPREAD & LIQUIDITY (Top 10 by volume)
═══════════════════════════════════════════════════════════════════

   Will the Indiana Pacers win the 2026 NBA Finals? 
   │  Midpoint:   0.1¢  │  Spread: 0.20¢ (200.0%)  │  🟢 Tight
   └─ Volume: $38.2M  │  Liq: $597.2K

   Will Jesus Christ return before GTA VI? 
   │  Midpoint:  48.5¢  │  Spread: 1.00¢ (2.1%)  │  🟡 Okay
   └─ Volume: $9.5M  │  Liq: $1.5M

   Will Trump deport 250,000-500,000 people? 
   │  Midpoint:  92.5¢  │  Spread: 0.50¢ (0.5%)  │  🟢 Tight
   └─ Volume: $7.5M  │  Liq: $10.3K

═══════════════════════════════════════════════════════════════════
💡 WHAT SPREAD MEANS
═══════════════════════════════════════════════════════════════════
   Spread = difference between best buy and sell prices.
   If you buy then immediately sell, you LOSE the spread.

   🟢 Tight (<0.5¢)  = Low cost to trade
   🟡 Okay  (0.5-2¢) = Moderate cost
   🔴 Wide  (>2¢)    = High cost, avoid unless confident

   Example: 2¢ spread on a 50¢ market = 4% round-trip cost!
```

### 3. Interactive TUI (`polymarket-tui`)

Full terminal UI with keyboard navigation:

```bash
./target/release/polymarket-tui
```

| Key | Action |
|-----|--------|
| `Tab` | Switch views |
| `j/k` | Navigate up/down |
| `/` | Search |
| `r` | Refresh data |
| `q` | Quit |

## What This Does NOT Do

| ❌ | Why |
|----|-----|
| Find arbitrage | Real arb is closed by bots in milliseconds |
| Guarantee profits | Markets are efficient, you can lose money |
| Execute trades | Read-only, no wallet integration |
| Beat HFT | We're fetching data over HTTP, not colocated |

## What This DOES Do

| ✅ | How |
|----|-----|
| Browse markets | Filter by volume, activity, price range |
| Check spreads | Real CLOB data shows your trading cost |
| Find close races | Markets near 50/50 are interesting |
| Track activity | See where volume is flowing |

## Architecture

```
src/
├── bin/
│   ├── demo.rs        # Market browser CLI
│   └── liquidity.rs   # Spread checker CLI
├── api/
│   ├── gamma.rs       # Market data (gamma-api.polymarket.com)
│   └── clob.rs        # Order book (clob.polymarket.com)
├── main.rs            # Interactive TUI
└── ui/                # TUI views
```

## API Endpoints Used

| Endpoint | Purpose |
|----------|---------|
| `gamma-api.polymarket.com/markets` | Market list, prices, volume |
| `gamma-api.polymarket.com/events` | Multi-outcome event groupings |
| `clob.polymarket.com/midpoint` | Current mid price |
| `clob.polymarket.com/spread` | Bid-ask spread |

No authentication required — all public read-only APIs.

## License

MIT — Do whatever you want, but don't blame us if you lose money.
