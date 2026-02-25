//! Debug API calls.

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let url = "https://gamma-api.polymarket.com/markets?limit=2&active=true&closed=false";

    println!("Fetching: {}\n", url);

    let resp = client.get(url).send().await?;
    println!("Status: {}", resp.status());

    let text = resp.text().await?;
    println!("Response length: {} bytes", text.len());
    println!("\nFirst 2000 chars:\n{}", &text[..text.len().min(2000)]);

    // Try to parse
    println!("\n\nAttempting to parse as Market vec...");
    match serde_json::from_str::<Vec<polymarket_tui::api::types::Market>>(&text) {
        Ok(markets) => {
            println!("✅ SUCCESS! Parsed {} markets", markets.len());
            for m in &markets {
                println!("  - {}", m.question.as_deref().unwrap_or("???"));
            }
        }
        Err(e) => {
            println!("❌ Parse error: {}", e);
            println!("\nError location: line {}, column {}", e.line(), e.column());

            // Show context around error
            let lines: Vec<&str> = text.lines().collect();
            if e.line() > 0 && e.line() <= lines.len() {
                println!("\nContext around error:");
                let start = e.line().saturating_sub(2);
                let end = (e.line() + 2).min(lines.len());
                for i in start..end {
                    let marker = if i + 1 == e.line() { ">>>" } else { "   " };
                    println!("{} {}: {}", marker, i + 1, &lines[i][..lines[i].len().min(100)]);
                }
            }
        }
    }

    Ok(())
}
