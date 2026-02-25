//! Market analysis — honest math on prediction market data.
//!
//! ⚠️ NOT TRADING ADVICE. These are analytical tools, not signals.
//! You can lose money trading prediction markets.

use crate::api::types::Market;

// ── Term Structure (cumulative "by date" markets) ───────────

/// A point on the cumulative probability curve.
#[derive(Debug, Clone)]
pub struct TermPoint {
    pub label: String,
    pub date: String,
    pub cumulative: f64,     // P(event by this date)
    pub conditional: f64,    // P(event in this period | hasn't happened yet)
    pub volume: f64,
}

/// Extract a term structure from a set of related cumulative markets.
/// Groups markets by a common stem (e.g. "US strikes Iran by ___").
pub fn build_term_structure(markets: &[Market], stem: &str) -> Vec<TermPoint> {
    let stem_lower = stem.to_lowercase();
    let mut points: Vec<(String, f64, f64)> = Vec::new();

    for m in markets {
        let q = m.question.as_deref().unwrap_or("").to_lowercase();
        if !q.contains(&stem_lower) { continue; }
        if m.active != Some(true) || m.closed == Some(true) { continue; }

        let yes = m.yes_price();
        if yes <= 0.0 { continue; }

        // Extract the date part (everything after "by")
        let date = q.split("by ").last().unwrap_or("").trim_end_matches('?').trim().to_string();
        if date.is_empty() { continue; }

        points.push((date, yes, m.volume_f64()));
    }

    // Sort by cumulative probability (proxy for date order)
    points.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut result = Vec::new();
    let mut prev_cum = 0.0;

    for (date, cum, vol) in &points {
        let cond = if prev_cum < 1.0 {
            (cum - prev_cum) / (1.0 - prev_cum)
        } else {
            0.0
        };

        result.push(TermPoint {
            label: date.clone(),
            date: date.clone(),
            cumulative: *cum,
            conditional: cond,
            volume: *vol,
        });

        prev_cum = *cum;
    }

    result
}

// ── Cross-Market Consistency ────────────────────────────────

#[derive(Debug, Clone)]
pub struct ConsistencyCheck {
    pub description: String,
    pub expected_range: (f64, f64),  // logical bounds
    pub actual: f64,
    pub status: ConsistencyStatus,
}

#[derive(Debug, Clone)]
pub enum ConsistencyStatus {
    Consistent,
    Underpriced { gap: f64 },
    Overpriced { gap: f64 },
    NeedsReview { note: String },
}

impl ConsistencyCheck {
    pub fn status_label(&self) -> &str {
        match &self.status {
            ConsistencyStatus::Consistent => "✅ OK",
            ConsistencyStatus::Underpriced { .. } => "⚠️  Under",
            ConsistencyStatus::Overpriced { .. } => "⚠️  Over",
            ConsistencyStatus::NeedsReview { .. } => "🔍 Review",
        }
    }
}

/// Check P(A or B) bounds: max(A,B) <= P(A or B) <= A + B
pub fn check_or_bounds(
    label_a: &str, prob_a: f64,
    label_b: &str, prob_b: f64,
    label_or: &str, prob_or: f64,
) -> ConsistencyCheck {
    let lower = prob_a.max(prob_b);
    let upper = (prob_a + prob_b).min(1.0);

    let status = if prob_or < lower - 0.005 {
        ConsistencyStatus::Underpriced { gap: lower - prob_or }
    } else if prob_or > upper + 0.005 {
        ConsistencyStatus::Overpriced { gap: prob_or - upper }
    } else {
        ConsistencyStatus::Consistent
    };

    ConsistencyCheck {
        description: format!("{} vs {} vs {}", label_a, label_b, label_or),
        expected_range: (lower, upper),
        actual: prob_or,
        status,
    }
}

/// Check monotonicity: "by March" <= "by June" <= "by December"
pub fn check_monotonicity(term: &[TermPoint]) -> Vec<ConsistencyCheck> {
    let mut checks = Vec::new();
    for w in term.windows(2) {
        if w[0].cumulative > w[1].cumulative + 0.005 {
            checks.push(ConsistencyCheck {
                description: format!("{} ({:.1}%) > {} ({:.1}%)",
                    w[0].label, w[0].cumulative * 100.0,
                    w[1].label, w[1].cumulative * 100.0),
                expected_range: (0.0, w[1].cumulative),
                actual: w[0].cumulative,
                status: ConsistencyStatus::NeedsReview {
                    note: "Earlier date priced higher than later date".into()
                },
            });
        }
    }
    checks
}

// ── Close Races ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CloseRace {
    pub market: Market,
    pub yes_price: f64,
    pub distance_from_50: f64,  // 0 = exactly 50/50
    pub volume_24h: f64,
}

/// Find markets closest to 50/50 with meaningful volume.
pub fn find_close_races(markets: &[Market], min_volume: f64) -> Vec<CloseRace> {
    let mut races: Vec<CloseRace> = markets.iter()
        .filter(|m| {
            m.active == Some(true) && m.closed != Some(true) && m.volume_f64() >= min_volume
        })
        .map(|m| {
            let yes = m.yes_price();
            CloseRace {
                market: m.clone(),
                yes_price: yes,
                distance_from_50: (yes - 0.5).abs(),
                volume_24h: m.volume_24h_f64(),
            }
        })
        .filter(|r| r.distance_from_50 <= 0.15) // 35-65%
        .collect();

    races.sort_by(|a, b| a.distance_from_50.partial_cmp(&b.distance_from_50).unwrap_or(std::cmp::Ordering::Equal));
    races
}

// ── Implied Joint Probabilities ─────────────────────────────

#[derive(Debug, Clone)]
pub struct JointProbs {
    pub label_a: String,
    pub label_b: String,
    pub prob_a: f64,
    pub prob_b: f64,
    pub prob_or: f64,
    pub both: f64,
    pub only_a: f64,
    pub only_b: f64,
    pub neither: f64,
    pub correlation: f64,  // implied correlation
}

/// Given P(A), P(B), and P(A or B), compute joint distribution.
pub fn compute_joint(
    label_a: &str, prob_a: f64,
    label_b: &str, prob_b: f64,
    prob_or: f64,
) -> JointProbs {
    let both = prob_a + prob_b - prob_or;
    let only_a = prob_a - both;
    let only_b = prob_b - both;
    let neither = 1.0 - prob_or;

    // Correlation: compare to independent case
    let independent_or = prob_a + prob_b - prob_a * prob_b;
    let correlation = if independent_or > 0.0 {
        (prob_or - independent_or) / independent_or
    } else {
        0.0
    };

    JointProbs {
        label_a: label_a.into(),
        label_b: label_b.into(),
        prob_a, prob_b, prob_or,
        both: both.max(0.0),
        only_a: only_a.max(0.0),
        only_b: only_b.max(0.0),
        neither: neither.max(0.0),
        correlation,
    }
}

// ── Scenario Builder ────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Scenario {
    pub label: String,
    pub probability: f64,
    pub sub_scenarios: Vec<Scenario>,
}

impl Scenario {
    pub fn leaf(label: &str, prob: f64) -> Self {
        Scenario { label: label.into(), probability: prob, sub_scenarios: Vec::new() }
    }

    pub fn branch(label: &str, prob: f64, children: Vec<Scenario>) -> Self {
        Scenario { label: label.into(), probability: prob, sub_scenarios: children }
    }

    /// Pretty print as a tree.
    pub fn format_tree(&self, indent: usize) -> String {
        let mut out = String::new();
        let pad = " ".repeat(indent);
        let bar_len = (self.probability * 40.0) as usize;
        let bar = "█".repeat(bar_len);
        out.push_str(&format!("{}{:<40} {:>5.1}%  {}\n",
            pad, self.label, self.probability * 100.0, bar));
        for child in &self.sub_scenarios {
            out.push_str(&child.format_tree(indent + 2));
        }
        out
    }
}
