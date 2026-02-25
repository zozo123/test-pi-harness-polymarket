use serde::{Deserialize, Serialize};

// ── Market ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub id: Option<String>,
    pub question: Option<String>,
    pub condition_id: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub end_date: Option<String>,
    pub category: Option<String>,
    pub image: Option<String>,
    pub outcomes: Option<String>,
    pub outcome_prices: Option<String>,
    pub volume: Option<String>,
    pub volume_num: Option<String>,
    pub liquidity: Option<String>,
    pub liquidity_num: Option<String>,
    pub active: Option<bool>,
    pub closed: Option<bool>,
    pub market_type: Option<String>,
    pub volume24hr: Option<String>,
    #[serde(alias = "volume1wk")]
    pub volume_1wk: Option<String>,
    pub clob_token_ids: Option<String>,
    pub accepting_orders: Option<bool>,
    pub order_price_min_tick_size: Option<String>,
    pub events: Option<Vec<EventRef>>,
}

impl Market {
    /// Parse outcome prices like "[\"0.52\",\"0.48\"]" into floats.
    pub fn parsed_prices(&self) -> Vec<f64> {
        self.outcome_prices
            .as_deref()
            .and_then(|s| serde_json::from_str::<Vec<String>>(s).ok())
            .map(|v| {
                v.iter()
                    .filter_map(|p| p.parse::<f64>().ok())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Parse clob token IDs from JSON string array.
    pub fn parsed_token_ids(&self) -> Vec<String> {
        self.clob_token_ids
            .as_deref()
            .and_then(|s| serde_json::from_str::<Vec<String>>(s).ok())
            .unwrap_or_default()
    }

    /// YES price (first outcome).
    pub fn yes_price(&self) -> f64 {
        self.parsed_prices().first().copied().unwrap_or(0.0)
    }

    /// NO price (second outcome).
    pub fn no_price(&self) -> f64 {
        self.parsed_prices().get(1).copied().unwrap_or(0.0)
    }

    /// Price deviation from $1.00 — nonzero means potential arb.
    pub fn price_deviation(&self) -> f64 {
        let prices = self.parsed_prices();
        if prices.len() >= 2 {
            let sum: f64 = prices.iter().sum();
            (sum - 1.0).abs()
        } else {
            0.0
        }
    }

    /// Volume as f64.
    pub fn volume_f64(&self) -> f64 {
        self.volume_num
            .as_deref()
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0)
    }

    /// Liquidity as f64.
    pub fn liquidity_f64(&self) -> f64 {
        self.liquidity_num
            .as_deref()
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0)
    }

    /// 24h volume as f64.
    pub fn volume_24h_f64(&self) -> f64 {
        self.volume24hr
            .as_deref()
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0)
    }

    /// Short question (truncated).
    pub fn short_question(&self, max_len: usize) -> String {
        let q = self.question.as_deref().unwrap_or("???");
        if q.len() <= max_len {
            q.to_string()
        } else {
            format!("{}…", &q[..max_len - 1])
        }
    }
}

// ── Event ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: Option<String>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub end_date: Option<String>,
    pub volume: Option<String>,
    pub volume_num: Option<String>,
    pub liquidity: Option<String>,
    pub liquidity_num: Option<String>,
    pub active: Option<bool>,
    pub closed: Option<bool>,
    pub markets: Option<Vec<Market>>,
    pub tags: Option<Vec<Tag>>,
    pub created_at: Option<String>,
}

impl Event {
    pub fn volume_f64(&self) -> f64 {
        self.volume_num
            .as_deref()
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0)
    }

    /// For multi-market events: sum of all YES prices.
    /// Should be ~1.0 for well-priced events.
    pub fn total_yes_probability(&self) -> f64 {
        self.markets
            .as_ref()
            .map(|ms| ms.iter().map(|m| m.yes_price()).sum())
            .unwrap_or(0.0)
    }

    /// Arbitrage score: how far the sum deviates from 1.0.
    pub fn arb_score(&self) -> f64 {
        let total = self.total_yes_probability();
        if total > 0.0 {
            (total - 1.0).abs()
        } else {
            0.0
        }
    }

    pub fn market_count(&self) -> usize {
        self.markets.as_ref().map(|m| m.len()).unwrap_or(0)
    }
}

// ── Event reference (embedded in Market) ────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventRef {
    pub id: Option<String>,
    pub title: Option<String>,
    pub slug: Option<String>,
}

// ── Tag ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: Option<String>,
    pub label: Option<String>,
    pub slug: Option<String>,
    pub force_show: Option<bool>,
}

// ── Order Book ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub market: Option<String>,
    pub asset_id: Option<String>,
    pub bids: Option<Vec<OrderLevel>>,
    pub asks: Option<Vec<OrderLevel>>,
    pub hash: Option<String>,
    pub timestamp: Option<String>,
}

impl OrderBook {
    pub fn best_bid(&self) -> f64 {
        self.bids
            .as_ref()
            .and_then(|b| b.first())
            .map(|l| l.price_f64())
            .unwrap_or(0.0)
    }

    pub fn best_ask(&self) -> f64 {
        self.asks
            .as_ref()
            .and_then(|a| a.first())
            .map(|l| l.price_f64())
            .unwrap_or(0.0)
    }

    pub fn spread(&self) -> f64 {
        let ask = self.best_ask();
        let bid = self.best_bid();
        if ask > 0.0 && bid > 0.0 {
            ask - bid
        } else {
            0.0
        }
    }

    pub fn spread_pct(&self) -> f64 {
        let mid = self.midpoint();
        if mid > 0.0 {
            (self.spread() / mid) * 100.0
        } else {
            0.0
        }
    }

    pub fn midpoint(&self) -> f64 {
        let ask = self.best_ask();
        let bid = self.best_bid();
        if ask > 0.0 && bid > 0.0 {
            (ask + bid) / 2.0
        } else {
            0.0
        }
    }

    pub fn bid_depth(&self) -> f64 {
        self.bids
            .as_ref()
            .map(|levels| levels.iter().map(|l| l.size_f64()).sum())
            .unwrap_or(0.0)
    }

    pub fn ask_depth(&self) -> f64 {
        self.asks
            .as_ref()
            .map(|levels| levels.iter().map(|l| l.size_f64()).sum())
            .unwrap_or(0.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLevel {
    pub price: Option<String>,
    pub size: Option<String>,
}

impl OrderLevel {
    pub fn price_f64(&self) -> f64 {
        self.price
            .as_deref()
            .and_then(|p| p.parse().ok())
            .unwrap_or(0.0)
    }

    pub fn size_f64(&self) -> f64 {
        self.size
            .as_deref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0)
    }
}

// ── Price History ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistory {
    pub history: Option<Vec<PricePoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub t: Option<i64>,   // unix timestamp
    pub p: Option<f64>,   // price
}

// ── Formatting helpers ──────────────────────────────────────

pub fn format_volume(vol: f64) -> String {
    if vol >= 1_000_000.0 {
        format!("${:.1}M", vol / 1_000_000.0)
    } else if vol >= 1_000.0 {
        format!("${:.1}K", vol / 1_000.0)
    } else {
        format!("${:.0}", vol)
    }
}

pub fn format_price_cents(price: f64) -> String {
    format!("{:.1}¢", price * 100.0)
}

pub fn format_pct(value: f64) -> String {
    format!("{:.1}%", value * 100.0)
}
