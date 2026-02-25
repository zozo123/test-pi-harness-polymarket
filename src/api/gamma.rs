//! Gamma API client — Polymarket's market/event data API.
//! Read-only, no auth needed.

use anyhow::Result;
use reqwest::Client;
use super::types::{Market, Event};

const GAMMA_BASE: &str = "https://gamma-api.polymarket.com";

pub struct GammaClient {
    client: Client,
}

impl Default for GammaClient {
    fn default() -> Self {
        Self::new()
    }
}

impl GammaClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// List markets with filters.
    pub async fn list_markets(
        &self,
        limit: u32,
        offset: u32,
        active: Option<bool>,
        closed: Option<bool>,
    ) -> Result<Vec<Market>> {
        let mut url = format!(
            "{}/markets?limit={}&offset={}",
            GAMMA_BASE, limit, offset
        );
        if let Some(a) = active {
            url.push_str(&format!("&active={}", a));
        }
        if let Some(c) = closed {
            url.push_str(&format!("&closed={}", c));
        }
        let resp = self.client.get(&url).send().await?;
        let markets: Vec<Market> = resp.json().await?;
        Ok(markets)
    }

    /// Search markets by fetching pages and filtering client-side.
    /// Gamma API text search doesn't work, so we paginate through all active markets.
    pub async fn search_markets(&self, query: &str, limit: u32) -> Result<Vec<Market>> {
        let q = query.to_lowercase();
        let mut results = Vec::new();
        let mut offset = 0u32;
        let page_size = 500u32;
        let max_pages = 4; // up to 2000 markets

        for _ in 0..max_pages {
            let url = format!(
                "{}/markets?limit={}&offset={}&active=true&closed=false&order=volumeNum&ascending=false",
                GAMMA_BASE, page_size, offset
            );
            let resp = self.client.get(&url).send().await?;
            let page: Vec<Market> = resp.json().await?;
            let page_len = page.len();

            for m in page {
                if m.question.as_deref()
                    .map(|s| s.to_lowercase().contains(&q))
                    .unwrap_or(false)
                {
                    results.push(m);
                    if results.len() >= limit as usize {
                        return Ok(results);
                    }
                }
            }

            if page_len < page_size as usize {
                break; // no more pages
            }
            offset += page_size;
        }
        Ok(results)
    }

    /// Get a single market by slug or ID.
    pub async fn get_market(&self, slug: &str) -> Result<Vec<Market>> {
        let url = format!("{}/markets?slug={}", GAMMA_BASE, urlencoding(slug));
        let resp = self.client.get(&url).send().await?;
        let markets: Vec<Market> = resp.json().await?;
        Ok(markets)
    }

    /// List events with optional filters.
    pub async fn list_events(
        &self,
        limit: u32,
        active: Option<bool>,
        closed: Option<bool>,
        tag: Option<&str>,
    ) -> Result<Vec<Event>> {
        let mut url = format!("{}/events?limit={}", GAMMA_BASE, limit);
        if let Some(a) = active {
            url.push_str(&format!("&active={}", a));
        }
        if let Some(c) = closed {
            url.push_str(&format!("&closed={}", c));
        }
        if let Some(t) = tag {
            url.push_str(&format!("&tag={}", urlencoding(t)));
        }
        let resp = self.client.get(&url).send().await?;
        let events: Vec<Event> = resp.json().await?;
        Ok(events)
    }

    /// Get a single event by ID.
    pub async fn get_event(&self, event_id: &str) -> Result<Event> {
        let url = format!("{}/events/{}", GAMMA_BASE, event_id);
        let resp = self.client.get(&url).send().await?;
        let event: Event = resp.json().await?;
        Ok(event)
    }
}

fn urlencoding(s: &str) -> String {
    s.replace(' ', "%20")
        .replace('"', "%22")
        .replace('#', "%23")
        .replace('&', "%26")
        .replace('+', "%2B")
}
