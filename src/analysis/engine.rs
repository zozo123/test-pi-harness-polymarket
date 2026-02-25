//! Market analysis engine.
//!
//! Provides signals for research — NOT trading recommendations.
//! These are informational tools, not guaranteed opportunities.

use crate::api::types::{Event, Market, OrderBook};

// ── Signal types ────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum SignalKind {
    /// Multi-outcome event pricing summary.
    /// NOTE: Sum ≠ 100% does NOT always mean arbitrage!
    /// - "By date X" markets are cumulative, not exclusive
    /// - True arb only exists for mutually exclusive outcomes
    MultiOutcomePricing {
        event_title: String,
        event_id: String,
        total_yes: f64,
        market_count: usize,
        is_mutually_exclusive: bool, // We try to detect this
        note: String,
    },

    /// Unusual trading volume in last 24h.
    /// May indicate new information entering the market.
    VolumeAnomaly {
        question: String,
        market_id: String,
        volume_24h: f64,
        volume_7d: f64,
        daily_avg_7d: f64,
        spike_ratio: f64,
    },

    /// Market approaching resolution with high confidence pricing.
    /// High YES (>90%) or low YES (<10%) near expiry.
    HighConfidenceNearExpiry {
        question: String,
        market_id: String,
        yes_price: f64,
        days_remaining: i64,
        implied_probability: f64,
    },

    /// Large market by volume — indicates high interest/liquidity.
    HighLiquidity {
        question: String,
        market_id: String,
        volume: f64,
        liquidity: f64,
    },

    /// Market with wide bid-ask spread.
    /// Wide spreads mean higher trading costs.
    WideSpread {
        question: String,
        market_id: String,
        bid: f64,
        ask: f64,
        spread_cents: f64,
    },
}

#[derive(Debug, Clone)]
pub struct Signal {
    pub kind: SignalKind,
    /// Relevance score 0.0-1.0 (NOT a profit indicator)
    pub relevance: f64,
}

impl Signal {
    pub fn label(&self) -> &'static str {
        match &self.kind {
            SignalKind::MultiOutcomePricing { .. } => "MULTI-MKT",
            SignalKind::VolumeAnomaly { .. } => "VOLUME",
            SignalKind::HighConfidenceNearExpiry { .. } => "EXPIRING",
            SignalKind::HighLiquidity { .. } => "LIQUID",
            SignalKind::WideSpread { .. } => "SPREAD",
        }
    }

    pub fn title(&self) -> String {
        match &self.kind {
            SignalKind::MultiOutcomePricing { event_title, .. } => event_title.clone(),
            SignalKind::VolumeAnomaly { question, .. } => question.clone(),
            SignalKind::HighConfidenceNearExpiry { question, .. } => question.clone(),
            SignalKind::HighLiquidity { question, .. } => question.clone(),
            SignalKind::WideSpread { question, .. } => question.clone(),
        }
    }

    pub fn detail_lines(&self) -> Vec<String> {
        match &self.kind {
            SignalKind::MultiOutcomePricing {
                total_yes,
                market_count,
                is_mutually_exclusive,
                note,
                ..
            } => {
                let mut lines = vec![
                    format!("Σ YES prices: {:.1}%", total_yes * 100.0),
                    format!("Outcomes: {}", market_count),
                ];
                if *is_mutually_exclusive {
                    let deviation = (total_yes - 1.0).abs() * 100.0;
                    lines.push(format!("Deviation from 100%: {:.1}%", deviation));
                    if *total_yes > 1.02 {
                        lines.push("⚠️  Overpriced (fees may explain this)".into());
                    } else if *total_yes < 0.98 {
                        lines.push("⚠️  Underpriced (check liquidity)".into());
                    }
                } else {
                    lines.push(format!("Note: {}", note));
                }
                lines
            }
            SignalKind::VolumeAnomaly {
                volume_24h,
                daily_avg_7d,
                spike_ratio,
                ..
            } => vec![
                format!("24h volume: ${:.0}", volume_24h),
                format!("7d daily avg: ${:.0}", daily_avg_7d),
                format!("Spike: {:.1}x normal", spike_ratio),
                "May indicate news/information event".into(),
            ],
            SignalKind::HighConfidenceNearExpiry {
                yes_price,
                days_remaining,
                implied_probability,
                ..
            } => vec![
                format!("YES price: {:.1}¢", yes_price * 100.0),
                format!("Implied prob: {:.1}%", implied_probability * 100.0),
                format!("Days to expiry: {}", days_remaining),
                "⚠️  Low prices = market expects NO".into(),
            ],
            SignalKind::HighLiquidity {
                volume, liquidity, ..
            } => vec![
                format!("Total volume: ${:.0}", volume),
                format!("Liquidity: ${:.0}", liquidity),
                "Higher liquidity = easier to trade".into(),
            ],
            SignalKind::WideSpread {
                bid,
                ask,
                spread_cents,
                ..
            } => vec![
                format!("Best bid: {:.1}¢", bid * 100.0),
                format!("Best ask: {:.1}¢", ask * 100.0),
                format!("Spread: {:.1}¢", spread_cents),
                "Wide spread = higher trading cost".into(),
            ],
        }
    }
}

// Backward compatibility aliases
pub type OpportunityKind = SignalKind;
pub type Opportunity = Signal;

impl Signal {
    pub fn score(&self) -> f64 {
        self.relevance
    }
}

// ── Analysis functions ──────────────────────────────────────

/// Analyze multi-outcome event pricing.
/// Returns informational signals — NOT guaranteed arbitrage.
pub fn analyze_multi_outcome_events(events: &[Event]) -> Vec<Signal> {
    let mut signals = Vec::new();

    for event in events {
        let market_count = event.market_count();
        if market_count < 2 {
            continue;
        }

        let total_yes = event.total_yes_probability();
        if total_yes == 0.0 {
            continue;
        }

        // Try to detect if outcomes are mutually exclusive
        let title = event.title.as_deref().unwrap_or("");
        let is_cumulative = title.contains("by") 
            || title.contains("before")
            || title.contains("By")
            || title.contains("Before");
        
        let is_mutually_exclusive = !is_cumulative && market_count >= 2;

        let note = if is_cumulative {
            "Cumulative markets (by date) — sum < 100% is expected".into()
        } else if total_yes > 1.05 {
            "Sum > 100% may reflect fees or temporary mispricing".into()
        } else if total_yes < 0.95 && is_mutually_exclusive {
            "Sum < 100% on exclusive outcomes — verify market structure".into()
        } else {
            "Pricing appears reasonable".into()
        };

        // Only flag significant deviations on exclusive outcomes
        let deviation = (total_yes - 1.0).abs();
        let relevance = if is_mutually_exclusive && deviation > 0.02 {
            (deviation * 5.0).min(1.0)
        } else {
            0.2 // Low relevance for cumulative or normal pricing
        };

        signals.push(Signal {
            kind: SignalKind::MultiOutcomePricing {
                event_title: event.title.clone().unwrap_or_else(|| "Unknown".into()),
                event_id: event.id.clone().unwrap_or_default(),
                total_yes,
                market_count,
                is_mutually_exclusive,
                note,
            },
            relevance,
        });
    }

    signals.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
    signals
}

/// Find markets with unusual 24h volume relative to 7d average.
pub fn find_volume_anomalies(markets: &[Market]) -> Vec<Signal> {
    let mut signals = Vec::new();

    for market in markets {
        if market.active != Some(true) || market.closed == Some(true) {
            continue;
        }

        let vol_24h = market.volume_24h_f64();
        let vol_7d = market.volume_1wk.unwrap_or(0.0);
        
        // Need meaningful volume
        if vol_24h < 1000.0 || vol_7d < 1000.0 {
            continue;
        }

        let daily_avg_7d = vol_7d / 7.0;
        if daily_avg_7d < 100.0 {
            continue;
        }

        let spike_ratio = vol_24h / daily_avg_7d;
        
        // Only flag if 24h is significantly above 7d daily average
        if spike_ratio >= 2.0 {
            let relevance = ((spike_ratio - 2.0) / 10.0).min(1.0);
            signals.push(Signal {
                kind: SignalKind::VolumeAnomaly {
                    question: market.question.clone().unwrap_or_default(),
                    market_id: market.id.clone().unwrap_or_default(),
                    volume_24h: vol_24h,
                    volume_7d: vol_7d,
                    daily_avg_7d,
                    spike_ratio,
                },
                relevance,
            });
        }
    }

    signals.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
    signals
}

/// Find markets near expiry with high-confidence pricing.
pub fn find_high_confidence_expiring(markets: &[Market], max_days: i64) -> Vec<Signal> {
    let mut signals = Vec::new();
    let now = chrono::Utc::now();

    for market in markets {
        if market.active != Some(true) || market.closed == Some(true) {
            continue;
        }

        let end_str = match &market.end_date {
            Some(s) => s,
            None => continue,
        };

        let end_date = match chrono::DateTime::parse_from_rfc3339(end_str) {
            Ok(d) => d,
            Err(_) => continue,
        };

        let days_remaining = end_date.signed_duration_since(now).num_days();
        if days_remaining < 0 || days_remaining > max_days {
            continue;
        }

        let yes = market.yes_price();
        
        // High confidence = price very high or very low
        if yes > 0.90 || yes < 0.10 {
            let implied_prob = yes; // YES price ≈ implied probability
            let confidence = if yes > 0.5 { yes } else { 1.0 - yes };
            let time_factor = 1.0 - (days_remaining as f64 / max_days as f64);
            let relevance = confidence * time_factor;

            signals.push(Signal {
                kind: SignalKind::HighConfidenceNearExpiry {
                    question: market.question.clone().unwrap_or_default(),
                    market_id: market.id.clone().unwrap_or_default(),
                    yes_price: yes,
                    days_remaining,
                    implied_probability: implied_prob,
                },
                relevance: relevance.min(1.0),
            });
        }
    }

    signals.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
    signals
}

/// Find highest volume/liquidity markets.
pub fn find_high_liquidity_markets(markets: &[Market], top_n: usize) -> Vec<Signal> {
    let mut active: Vec<_> = markets
        .iter()
        .filter(|m| m.active == Some(true) && m.closed != Some(true))
        .collect();

    active.sort_by(|a, b| b.volume_f64().partial_cmp(&a.volume_f64()).unwrap());

    active
        .into_iter()
        .take(top_n)
        .enumerate()
        .map(|(i, m)| {
            let relevance = 1.0 - (i as f64 / top_n as f64);
            Signal {
                kind: SignalKind::HighLiquidity {
                    question: m.question.clone().unwrap_or_default(),
                    market_id: m.id.clone().unwrap_or_default(),
                    volume: m.volume_f64(),
                    liquidity: m.liquidity_f64(),
                },
                relevance,
            }
        })
        .collect()
}

/// Find markets with wide spreads (from order book data).
pub fn find_wide_spreads(market_books: &[(Market, OrderBook)], min_spread_cents: f64) -> Vec<Signal> {
    let mut signals = Vec::new();

    for (market, book) in market_books {
        let spread = book.spread();
        let spread_cents = spread * 100.0;
        
        if spread_cents >= min_spread_cents {
            let relevance = (spread_cents / 10.0).min(1.0);
            signals.push(Signal {
                kind: SignalKind::WideSpread {
                    question: market.question.clone().unwrap_or_default(),
                    market_id: market.id.clone().unwrap_or_default(),
                    bid: book.best_bid(),
                    ask: book.best_ask(),
                    spread_cents,
                },
                relevance,
            });
        }
    }

    signals.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
    signals
}

/// Run all analyses. Returns signals for research, not trading recommendations.
pub fn scan_all(markets: &[Market], events: &[Event]) -> Vec<Signal> {
    let mut all = Vec::new();
    
    // Multi-outcome analysis (informational, NOT arbitrage)
    all.extend(analyze_multi_outcome_events(events));
    
    // Volume anomalies
    all.extend(find_volume_anomalies(markets));
    
    // Near-expiry high-confidence markets
    all.extend(find_high_confidence_expiring(markets, 14));
    
    // Sort by relevance
    all.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
    all
}

// Keep old function name for compatibility
pub fn find_event_arbitrage(events: &[Event], _min_edge: f64) -> Vec<Signal> {
    analyze_multi_outcome_events(events)
}

pub fn find_volume_surges(markets: &[Market], _min_ratio: f64) -> Vec<Signal> {
    find_volume_anomalies(markets)
}

pub fn find_near_resolution(markets: &[Market], max_days: i64) -> Vec<Signal> {
    find_high_confidence_expiring(markets, max_days)
}

pub fn find_price_deviations(_markets: &[Market], _min_deviation: f64) -> Vec<Signal> {
    // Removed — binary YES+NO always sums to ~100% after fees
    Vec::new()
}
