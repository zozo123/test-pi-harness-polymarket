//! Opportunity analysis engine.
//!
//! Scans markets and events for exploitable patterns:
//! - Event arbitrage (multi-outcome probability sums ≠ 100%)
//! - Price deviation (YES + NO ≠ $1.00)
//! - Wide spreads (market-making opportunities)
//! - Volume anomalies (sudden spikes)
//! - Near-resolution easy value

use crate::api::types::{Event, Market, OrderBook};

// ── Opportunity types ───────────────────────────────────────

#[derive(Debug, Clone)]
pub enum OpportunityKind {
    /// Multi-outcome event where YES prices sum ≠ 1.0.
    /// If > 1.0: sell all YES = guaranteed profit.
    /// If < 1.0: buy all YES = guaranteed profit.
    EventArbitrage {
        event_title: String,
        event_id: String,
        total_yes: f64,
        market_count: usize,
        edge_pct: f64,
    },

    /// Binary market where YES + NO ≠ $1.00.
    PriceDeviation {
        question: String,
        market_id: String,
        yes_price: f64,
        no_price: f64,
        deviation: f64,
    },

    /// Market with wide bid-ask spread.
    WideSpread {
        question: String,
        market_id: String,
        bid: f64,
        ask: f64,
        spread: f64,
        spread_pct: f64,
    },

    /// Volume spike — 24h volume is disproportionate to total volume.
    VolumeSurge {
        question: String,
        market_id: String,
        volume_24h: f64,
        total_volume: f64,
        surge_ratio: f64,
    },

    /// Market near expiry with extreme price — easy to collect.
    NearResolution {
        question: String,
        market_id: String,
        yes_price: f64,
        end_date: String,
        days_remaining: i64,
    },
}

#[derive(Debug, Clone)]
pub struct Opportunity {
    pub kind: OpportunityKind,
    /// 0.0 to 1.0 — how good is this opportunity?
    pub score: f64,
}

impl Opportunity {
    pub fn label(&self) -> &'static str {
        match &self.kind {
            OpportunityKind::EventArbitrage { .. } => "EVENT ARB",
            OpportunityKind::PriceDeviation { .. } => "PRICE DEV",
            OpportunityKind::WideSpread { .. } => "SPREAD",
            OpportunityKind::VolumeSurge { .. } => "VOL SURGE",
            OpportunityKind::NearResolution { .. } => "NEAR RES",
        }
    }

    pub fn title(&self) -> String {
        match &self.kind {
            OpportunityKind::EventArbitrage { event_title, .. } => event_title.clone(),
            OpportunityKind::PriceDeviation { question, .. } => question.clone(),
            OpportunityKind::WideSpread { question, .. } => question.clone(),
            OpportunityKind::VolumeSurge { question, .. } => question.clone(),
            OpportunityKind::NearResolution { question, .. } => question.clone(),
        }
    }

    pub fn detail_lines(&self) -> Vec<String> {
        match &self.kind {
            OpportunityKind::EventArbitrage {
                total_yes,
                market_count,
                edge_pct,
                ..
            } => vec![
                format!("Σ YES prices: {:.1}%", total_yes * 100.0),
                format!("Markets: {}", market_count),
                format!("Edge: {:.2}%", edge_pct),
                if *total_yes > 1.0 {
                    "Strategy: SELL all YES positions".into()
                } else {
                    "Strategy: BUY all YES positions".into()
                },
            ],
            OpportunityKind::PriceDeviation {
                yes_price,
                no_price,
                deviation,
                ..
            } => vec![
                format!("YES: {:.1}¢  NO: {:.1}¢", yes_price * 100.0, no_price * 100.0),
                format!("Sum: {:.1}¢ (deviation: {:.2}¢)", (yes_price + no_price) * 100.0, deviation * 100.0),
            ],
            OpportunityKind::WideSpread {
                bid,
                ask,
                spread,
                spread_pct,
                ..
            } => vec![
                format!("Bid: {:.1}¢  Ask: {:.1}¢", bid * 100.0, ask * 100.0),
                format!("Spread: {:.2}¢ ({:.1}%)", spread * 100.0, spread_pct),
                "Strategy: Post limit orders on both sides".into(),
            ],
            OpportunityKind::VolumeSurge {
                volume_24h,
                total_volume,
                surge_ratio,
                ..
            } => vec![
                format!("24h Vol: ${:.0}  Total: ${:.0}", volume_24h, total_volume),
                format!("Surge ratio: {:.1}x daily average", surge_ratio),
            ],
            OpportunityKind::NearResolution {
                yes_price,
                days_remaining,
                end_date,
                ..
            } => vec![
                format!("YES price: {:.1}¢", yes_price * 100.0),
                format!("Days remaining: {}", days_remaining),
                format!("End date: {}", end_date),
            ],
        }
    }
}

// ── Analysis functions ──────────────────────────────────────

/// Find event arbitrage — multi-outcome events where probabilities
/// don't sum to 100%.
pub fn find_event_arbitrage(events: &[Event], min_edge: f64) -> Vec<Opportunity> {
    let mut opps = Vec::new();

    for event in events {
        let market_count = event.market_count();
        if market_count < 2 {
            continue;
        }

        let total_yes = event.total_yes_probability();
        if total_yes == 0.0 {
            continue;
        }

        let edge = (total_yes - 1.0).abs();
        if edge >= min_edge {
            let score = (edge * 10.0).min(1.0); // scale to 0..1
            opps.push(Opportunity {
                kind: OpportunityKind::EventArbitrage {
                    event_title: event
                        .title
                        .clone()
                        .unwrap_or_else(|| "Unknown".into()),
                    event_id: event.id.clone().unwrap_or_default(),
                    total_yes,
                    market_count,
                    edge_pct: edge * 100.0,
                },
                score,
            });
        }
    }

    opps.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    opps
}

/// Find binary markets where YES + NO ≠ $1.00.
pub fn find_price_deviations(markets: &[Market], min_deviation: f64) -> Vec<Opportunity> {
    let mut opps = Vec::new();

    for market in markets {
        if market.active != Some(true) || market.closed == Some(true) {
            continue;
        }

        let prices = market.parsed_prices();
        if prices.len() != 2 {
            continue;
        }

        let deviation = market.price_deviation();
        if deviation >= min_deviation {
            let score = (deviation * 20.0).min(1.0);
            opps.push(Opportunity {
                kind: OpportunityKind::PriceDeviation {
                    question: market.question.clone().unwrap_or_default(),
                    market_id: market.id.clone().unwrap_or_default(),
                    yes_price: prices[0],
                    no_price: prices[1],
                    deviation,
                },
                score,
            });
        }
    }

    opps.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    opps
}

/// Find markets with wide bid-ask spreads (from order book data).
pub fn find_wide_spreads(
    market_books: &[(Market, OrderBook)],
    min_spread_pct: f64,
) -> Vec<Opportunity> {
    let mut opps = Vec::new();

    for (market, book) in market_books {
        let spread_pct = book.spread_pct();
        if spread_pct >= min_spread_pct {
            let score = (spread_pct / 50.0).min(1.0);
            opps.push(Opportunity {
                kind: OpportunityKind::WideSpread {
                    question: market.question.clone().unwrap_or_default(),
                    market_id: market.id.clone().unwrap_or_default(),
                    bid: book.best_bid(),
                    ask: book.best_ask(),
                    spread: book.spread(),
                    spread_pct,
                },
                score,
            });
        }
    }

    opps.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    opps
}

/// Find markets with disproportionate 24h volume.
pub fn find_volume_surges(markets: &[Market], min_ratio: f64) -> Vec<Opportunity> {
    let mut opps = Vec::new();

    for market in markets {
        if market.active != Some(true) || market.closed == Some(true) {
            continue;
        }

        let vol_24h = market.volume_24h_f64();
        let total = market.volume_f64();

        if total < 1000.0 || vol_24h < 100.0 {
            continue;
        }

        // Rough: if the market has been around, daily avg = total / ~365
        // If 24h is way above that, it's a surge
        let daily_avg = total / 180.0; // assume ~6 months average
        if daily_avg > 0.0 {
            let ratio = vol_24h / daily_avg;
            if ratio >= min_ratio {
                let score = (ratio / 20.0).min(1.0);
                opps.push(Opportunity {
                    kind: OpportunityKind::VolumeSurge {
                        question: market.question.clone().unwrap_or_default(),
                        market_id: market.id.clone().unwrap_or_default(),
                        volume_24h: vol_24h,
                        total_volume: total,
                        surge_ratio: ratio,
                    },
                    score,
                });
            }
        }
    }

    opps.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    opps
}

/// Find markets near resolution with extreme prices.
pub fn find_near_resolution(markets: &[Market], max_days: i64) -> Vec<Opportunity> {
    let mut opps = Vec::new();
    let now = chrono::Utc::now();

    for market in markets {
        if market.active != Some(true) || market.closed == Some(true) {
            continue;
        }

        let end_str = match &market.end_date {
            Some(s) => s,
            None => continue,
        };

        // Try parsing the end date
        let end_date = match chrono::DateTime::parse_from_rfc3339(end_str) {
            Ok(d) => d,
            Err(_) => continue,
        };

        let days_remaining = (end_date.signed_duration_since(now)).num_days();
        if days_remaining < 0 || days_remaining > max_days {
            continue;
        }

        let yes = market.yes_price();
        // Only interesting if price is extreme (confident resolution)
        if yes > 0.90 || yes < 0.10 {
            let extremity = if yes > 0.90 { yes } else { 1.0 - yes };
            let score = extremity * (1.0 - (days_remaining as f64 / max_days as f64));
            opps.push(Opportunity {
                kind: OpportunityKind::NearResolution {
                    question: market.question.clone().unwrap_or_default(),
                    market_id: market.id.clone().unwrap_or_default(),
                    yes_price: yes,
                    end_date: end_str.clone(),
                    days_remaining,
                },
                score: score.min(1.0),
            });
        }
    }

    opps.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    opps
}

/// Run all analyses that don't require order book data.
pub fn scan_all(markets: &[Market], events: &[Event]) -> Vec<Opportunity> {
    let mut all = Vec::new();
    all.extend(find_event_arbitrage(events, 0.02));
    all.extend(find_price_deviations(markets, 0.01));
    all.extend(find_volume_surges(markets, 3.0));
    all.extend(find_near_resolution(markets, 14));
    all.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    all
}
