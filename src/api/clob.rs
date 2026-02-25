//! CLOB API client — Polymarket's order book & pricing API.
//! Read-only endpoints, no auth needed.

use anyhow::Result;
use reqwest::Client;
use super::types::{OrderBook, PriceHistory};

const CLOB_BASE: &str = "https://clob.polymarket.com";

pub struct ClobClient {
    client: Client,
}

impl ClobClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Get the order book for a token.
    pub async fn get_book(&self, token_id: &str) -> Result<OrderBook> {
        let url = format!("{}/book?token_id={}", CLOB_BASE, token_id);
        let resp = self.client.get(&url).send().await?;
        let book: OrderBook = resp.json().await?;
        Ok(book)
    }

    /// Get the midpoint price.
    pub async fn get_midpoint(&self, token_id: &str) -> Result<f64> {
        let url = format!("{}/midpoint?token_id={}", CLOB_BASE, token_id);
        let resp = self.client.get(&url).send().await?;
        let data: serde_json::Value = resp.json().await?;
        let mid = data
            .get("mid")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        Ok(mid)
    }

    /// Get the spread.
    pub async fn get_spread(&self, token_id: &str) -> Result<f64> {
        let url = format!("{}/spread?token_id={}", CLOB_BASE, token_id);
        let resp = self.client.get(&url).send().await?;
        let data: serde_json::Value = resp.json().await?;
        let spread = data
            .get("spread")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        Ok(spread)
    }

    /// Get price for a side (buy/sell).
    pub async fn get_price(&self, token_id: &str, side: &str) -> Result<f64> {
        let url = format!("{}/price?token_id={}&side={}", CLOB_BASE, token_id, side);
        let resp = self.client.get(&url).send().await?;
        let data: serde_json::Value = resp.json().await?;
        let price = data
            .get("price")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        Ok(price)
    }

    /// Get price history for a token.
    pub async fn get_price_history(
        &self,
        token_id: &str,
        interval: &str,
        fidelity: u32,
    ) -> Result<PriceHistory> {
        let url = format!(
            "{}/prices-history?market={}&interval={}&fidelity={}",
            CLOB_BASE, token_id, interval, fidelity
        );
        let resp = self.client.get(&url).send().await?;
        let history: PriceHistory = resp.json().await?;
        Ok(history)
    }
}
