# polymarket-tui

Terminal tools for browsing [Polymarket](https://polymarket.com) prediction markets.

```
⚠️  FOR RESEARCH ONLY — NOT TRADING ADVICE
    You can lose money. DYOR. Check your local laws.
```

## Tools

| Binary | Purpose |
|--------|---------|
| `demo` | Browse markets by volume, activity, confidence |
| `liquidity` | Check real spreads before trading |
| `polymarket-tui` | Interactive TUI browser |

## Quick Start

```bash
git clone https://github.com/zozo123/test-pi-harness-polymarket.git
cd test-pi-harness-polymarket
cargo build --release

./target/release/demo       # Market browser
./target/release/liquidity  # Spread checker
./target/release/polymarket-tui  # Interactive TUI
```

## Demo Output

```
╔══════════════════════════════════════════════════════════════════╗
║     POLYMARKET BROWSER                                           ║
╚══════════════════════════════════════════════════════════════════╝

💰 MOST ACTIVE (by total volume)

   Will Chelsea Clinton win the 2028 Democratic pres… 
   └─ YES:   0.9¢  │  Vol: $39.2M

📈 HIGHEST 24H VOLUME (recent activity)

   Will the Indiana Pacers win the 2026 NBA Finals? 
   └─ YES:   0.1¢  │  24h: $2.4M

⚖️  CLOSE RACES (YES price 40-60%)

   Will bitcoin hit $1m before GTA VI? 
   └─ YES:  48.7¢  │  Vol: $3.4M

📊 SUMMARY
   Active markets:     200
   Total volume:       $1.3B
   Close races:        7
```

## Liquidity Checker

**Actually useful** — shows real spreads from the CLOB API:

```
╔══════════════════════════════════════════════════════════════════╗
║     LIQUIDITY CHECKER                                            ║
╚══════════════════════════════════════════════════════════════════╝

📊 SPREAD & LIQUIDITY (Top 10 by volume)

   Will Trump deport 250,000-500,000 people? 
   │  Midpoint:  92.5¢  │  Spread: 0.50¢ (0.5%)  │  🟢 Tight
   └─ Volume: $7.5M  │  Liq: $8.5K

   Will Jesus Christ return before GTA VI? 
   │  Midpoint:  48.5¢  │  Spread: 1.00¢ (2.1%)  │  🟡 Okay
   └─ Volume: $9.5M  │  Liq: $1.5M

💡 WHAT SPREAD MEANS
   🟢 Tight (<0.5¢)  = Low cost to trade
   🟡 Okay  (0.5-2¢) = Moderate cost  
   🔴 Wide  (>2¢)    = High cost
```

## What This Tool Does NOT Do

- ❌ Find "free money" arbitrage (it doesn't exist for retail)
- ❌ Compete with HFT bots
- ❌ Execute trades
- ❌ Guarantee profits

## What This Tool DOES Do

- ✅ Browse real market data
- ✅ Show actual spreads (your trading cost)
- ✅ Identify close races and high-confidence markets
- ✅ Track volume and activity

## Architecture

```
src/
├── main.rs              # TUI
├── bin/
│   ├── demo.rs          # Market browser
│   └── liquidity.rs     # Spread checker  
├── api/
│   ├── gamma.rs         # Market data API
│   └── clob.rs          # Order book API
└── analysis/engine.rs   # Signal detection
```

## License

MIT
