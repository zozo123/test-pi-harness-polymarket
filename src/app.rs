//! Application state — manages data, views, and user interaction.

use crate::api::gamma::GammaClient;
use crate::api::clob::ClobClient;
use crate::api::types::{Event, Market};

// ── View enum ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Dashboard,
    Markets,
    Events,
    Spreads,
    Help,
}

impl View {
    pub const ALL: [View; 5] = [
        View::Dashboard,
        View::Markets,
        View::Events,
        View::Spreads,
        View::Help,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            View::Dashboard => "Dashboard",
            View::Markets => "Markets",
            View::Events => "Events",
            View::Spreads => "Spreads",
            View::Help => "Help",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            View::Dashboard => 0,
            View::Markets => 1,
            View::Events => 2,
            View::Spreads => 3,
            View::Help => 4,
        }
    }
}

// ── App state ───────────────────────────────────────────────

pub struct App {
    pub view: View,
    pub running: bool,

    // Data
    pub markets: Vec<Market>,
    pub events: Vec<Event>,
    pub spread_data: Vec<(Market, f64, f64)>, // (market, midpoint, spread)

    // UI state
    pub market_cursor: usize,
    pub event_cursor: usize,
    pub spread_cursor: usize,
    pub search_query: String,
    pub searching: bool,
    pub filtered_indices: Vec<usize>,

    // Loading state
    pub loading: bool,
    pub loading_spreads: bool,
    pub status_msg: String,
    pub error_msg: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            view: View::Dashboard,
            running: true,
            markets: Vec::new(),
            events: Vec::new(),
            spread_data: Vec::new(),
            market_cursor: 0,
            event_cursor: 0,
            spread_cursor: 0,
            search_query: String::new(),
            searching: false,
            filtered_indices: Vec::new(),
            loading: false,
            loading_spreads: false,
            status_msg: "Press 'r' to refresh data".into(),
            error_msg: None,
        }
    }

    pub fn set_view(&mut self, view: View) {
        self.view = view;
        self.searching = false;
    }

    pub fn next_view(&mut self) {
        let idx = self.view.index();
        let next = (idx + 1) % View::ALL.len();
        self.set_view(View::ALL[next]);
    }

    pub fn prev_view(&mut self) {
        let idx = self.view.index();
        let prev = if idx == 0 { View::ALL.len() - 1 } else { idx - 1 };
        self.set_view(View::ALL[prev]);
    }

    pub fn cursor_down(&mut self) {
        match self.view {
            View::Markets | View::Dashboard => {
                let len = self.visible_market_count();
                if len > 0 {
                    self.market_cursor = (self.market_cursor + 1).min(len - 1);
                }
            }
            View::Events => {
                let len = self.events.len();
                if len > 0 {
                    self.event_cursor = (self.event_cursor + 1).min(len - 1);
                }
            }
            View::Spreads => {
                let len = self.spread_data.len();
                if len > 0 {
                    self.spread_cursor = (self.spread_cursor + 1).min(len - 1);
                }
            }
            _ => {}
        }
    }

    pub fn cursor_up(&mut self) {
        match self.view {
            View::Markets | View::Dashboard => {
                self.market_cursor = self.market_cursor.saturating_sub(1);
            }
            View::Events => {
                self.event_cursor = self.event_cursor.saturating_sub(1);
            }
            View::Spreads => {
                self.spread_cursor = self.spread_cursor.saturating_sub(1);
            }
            _ => {}
        }
    }

    pub fn page_down(&mut self) {
        for _ in 0..10 {
            self.cursor_down();
        }
    }

    pub fn page_up(&mut self) {
        for _ in 0..10 {
            self.cursor_up();
        }
    }

    pub fn cursor_top(&mut self) {
        match self.view {
            View::Markets | View::Dashboard => self.market_cursor = 0,
            View::Events => self.event_cursor = 0,
            View::Spreads => self.spread_cursor = 0,
            _ => {}
        }
    }

    pub fn apply_search(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_indices.clear();
            return;
        }
        let q = self.search_query.to_lowercase();
        self.filtered_indices = self
            .markets
            .iter()
            .enumerate()
            .filter(|(_, m)| {
                m.question
                    .as_deref()
                    .map(|s| s.to_lowercase().contains(&q))
                    .unwrap_or(false)
            })
            .map(|(i, _)| i)
            .collect();
        self.market_cursor = 0;
    }

    pub fn visible_markets(&self) -> Vec<&Market> {
        if !self.search_query.is_empty() && !self.filtered_indices.is_empty() {
            self.filtered_indices
                .iter()
                .filter_map(|&i| self.markets.get(i))
                .collect()
        } else if self.search_query.is_empty() {
            self.markets.iter().collect()
        } else {
            Vec::new()
        }
    }

    pub fn visible_market_count(&self) -> usize {
        self.visible_markets().len()
    }

    pub fn selected_market(&self) -> Option<&Market> {
        let visible = self.visible_markets();
        visible.get(self.market_cursor).copied()
    }

    /// Fetch data from Polymarket APIs.
    pub async fn refresh_data(&mut self) {
        self.loading = true;
        self.status_msg = "Fetching markets…".into();
        self.error_msg = None;

        let gamma = GammaClient::new();

        // Fetch active markets
        match gamma.list_markets(200, 0, Some(true), Some(false)).await {
            Ok(markets) => {
                self.markets = markets;
                self.status_msg = format!("Loaded {} markets", self.markets.len());
            }
            Err(e) => {
                self.error_msg = Some(format!("Markets: {}", e));
            }
        }

        // Fetch active events
        self.status_msg = "Fetching events…".into();
        match gamma.list_events(100, Some(true), Some(false), None).await {
            Ok(events) => {
                self.events = events;
                self.status_msg = format!(
                    "Loaded {} markets, {} events",
                    self.markets.len(),
                    self.events.len()
                );
            }
            Err(e) => {
                self.error_msg = Some(format!("Events: {}", e));
            }
        }

        // Sort markets by volume descending
        self.markets
            .sort_by(|a, b| b.volume_f64().partial_cmp(&a.volume_f64()).unwrap());

        self.status_msg = format!(
            "Ready — {} markets, {} events",
            self.markets.len(),
            self.events.len(),
        );

        self.loading = false;
    }

    /// Fetch spread data for top markets.
    pub async fn refresh_spreads(&mut self) {
        self.loading_spreads = true;
        self.spread_data.clear();

        let clob = ClobClient::new();
        
        // Get top 20 markets by volume
        let top_markets: Vec<_> = self.markets.iter().take(20).cloned().collect();
        
        for market in top_markets {
            let token_ids = market.parsed_token_ids();
            if token_ids.is_empty() {
                continue;
            }

            let token = &token_ids[0];
            let midpoint = clob.get_midpoint(token).await.unwrap_or(0.0);
            let spread = clob.get_spread(token).await.unwrap_or(0.0);
            
            if midpoint > 0.0 {
                self.spread_data.push((market, midpoint, spread));
            }
        }

        self.loading_spreads = false;
    }
}
