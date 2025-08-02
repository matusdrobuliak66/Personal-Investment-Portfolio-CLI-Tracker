use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;

// For this demo, we'll use Alpha Vantage API (free tier)
// In a real application, you might want to use a different service
const API_KEY: &str = "demo"; // Use "demo" for demonstration, replace with real API key

#[derive(Debug, Deserialize)]
struct AlphaVantageResponse {
    #[serde(rename = "Global Quote")]
    global_quote: Option<GlobalQuote>,
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Note")]
    note: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GlobalQuote {
    #[serde(rename = "05. price")]
    price: String,
}

// Mock price data for demonstration when API is not available
fn get_mock_prices() -> HashMap<String, f64> {
    let mut prices = HashMap::new();
    
    // Stock prices
    prices.insert("AAPL".to_string(), 170.0);
    prices.insert("TSLA".to_string(), 700.0);
    prices.insert("GOOGL".to_string(), 2800.0);
    prices.insert("MSFT".to_string(), 350.0);
    prices.insert("AMZN".to_string(), 3200.0);
    prices.insert("NVDA".to_string(), 450.0);
    prices.insert("META".to_string(), 320.0);
    prices.insert("BRK.B".to_string(), 325.0);
    prices.insert("JPM".to_string(), 145.0);
    prices.insert("V".to_string(), 240.0);
    
    // Crypto prices
    prices.insert("BTC-USD".to_string(), 95000.0);
    prices.insert("ETH-USD".to_string(), 3800.0);
    prices.insert("BNB-USD".to_string(), 680.0);
    prices.insert("ADA-USD".to_string(), 1.2);
    prices.insert("SOL-USD".to_string(), 180.0);
    
    prices
}

pub async fn fetch_current_prices(tickers: &[String]) -> Result<HashMap<String, f64>> {
    let mut prices = HashMap::new();
    let mock_prices = get_mock_prices();

    // For demonstration purposes, we'll use mock data
    // In a real application, you would make actual API calls
    for ticker in tickers {
        if let Some(&price) = mock_prices.get(ticker) {
            prices.insert(ticker.clone(), price);
        } else {
            // Try to fetch real price if available
            match fetch_real_price(ticker).await {
                Ok(price) => {
                    prices.insert(ticker.clone(), price);
                }
                Err(_) => {
                    // Fallback to a default price for unknown tickers
                    println!("Warning: Could not fetch price for {}, using default", ticker);
                    prices.insert(ticker.clone(), 100.0);
                }
            }
        }
    }

    Ok(prices)
}

async fn fetch_real_price(ticker: &str) -> Result<f64> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        ticker, API_KEY
    );

    let response: AlphaVantageResponse = client
        .get(&url)
        .send()
        .await
        .context("Failed to make API request")?
        .json()
        .await
        .context("Failed to parse API response")?;

    if let Some(error) = response.error_message {
        anyhow::bail!("API Error: {}", error);
    }

    if let Some(note) = response.note {
        anyhow::bail!("API Note: {}", note);
    }

    if let Some(quote) = response.global_quote {
        let price: f64 = quote.price.parse()
            .context("Failed to parse price as number")?;
        Ok(price)
    } else {
        anyhow::bail!("No price data found for ticker: {}", ticker);
    }
}
