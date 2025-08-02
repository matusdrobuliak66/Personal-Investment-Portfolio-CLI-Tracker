use portfolio::api::{fetch_current_prices};
use portfolio::portfolio::{Portfolio};
use std::fs;
use tempfile::tempdir;

// Integration tests for the commands module
// These test the full flow including file I/O and API calls

#[tokio::test]
async fn test_portfolio_loading_and_price_fetching() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_portfolio.json");

    let test_data = r#"[
        {
            "ticker": "AAPL",
            "quantity": 10.0,
            "cost_basis": 150.0,
            "date_purchased": "2023-01-01"
        },
        {
            "ticker": "TSLA",
            "quantity": 5.0,
            "cost_basis": 200.0,
            "date_purchased": "2023-02-01"
        }
    ]"#;

    fs::write(&file_path, test_data).unwrap();

    // Test portfolio loading
    let portfolio = Portfolio::load_from_file(&file_path).unwrap();
    assert_eq!(portfolio.holdings.len(), 2);

    // Test price fetching
    let tickers = portfolio.get_tickers();
    let prices = fetch_current_prices(&tickers).await.unwrap();
    
    assert!(prices.contains_key("AAPL"));
    assert!(prices.contains_key("TSLA"));

    // Test holdings with prices
    let holdings_with_prices = portfolio.holdings_with_prices(&prices);
    assert_eq!(holdings_with_prices.len(), 2);

    for holding_with_price in &holdings_with_prices {
        assert!(holding_with_price.current_price > 0.0);
        assert!(holding_with_price.current_value() > 0.0);
        assert!(holding_with_price.total_cost() > 0.0);
    }
}

#[tokio::test]
async fn test_portfolio_calculations() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("calculation_test_portfolio.json");

    let test_data = r#"[
        {
            "ticker": "AAPL",
            "quantity": 10.0,
            "cost_basis": 150.0,
            "date_purchased": "2023-01-01"
        }
    ]"#;

    fs::write(&file_path, test_data).unwrap();

    let portfolio = Portfolio::load_from_file(&file_path).unwrap();
    let tickers = portfolio.get_tickers();
    let prices = fetch_current_prices(&tickers).await.unwrap();
    let holdings_with_prices = portfolio.holdings_with_prices(&prices);

    let holding_with_price = &holdings_with_prices[0];
    
    // Test calculations
    assert_eq!(holding_with_price.total_cost(), 1500.0); // 10 * 150
    
    // Current value should be 10 * current_price (which is 170.0 for AAPL mock)
    assert_eq!(holding_with_price.current_value(), 1700.0); // 10 * 170
    
    // Gain/loss should be current_value - total_cost
    assert_eq!(holding_with_price.gain_loss(), 200.0); // 1700 - 1500
    
    // Percentage should be (gain_loss / total_cost) * 100
    let expected_percentage = (200.0 / 1500.0) * 100.0;
    assert!((holding_with_price.gain_loss_percentage() - expected_percentage).abs() < 0.001);
}

#[tokio::test]
async fn test_empty_portfolio() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("empty_portfolio.json");

    fs::write(&file_path, "[]").unwrap();

    let portfolio = Portfolio::load_from_file(&file_path).unwrap();
    assert_eq!(portfolio.holdings.len(), 0);

    let tickers = portfolio.get_tickers();
    assert!(tickers.is_empty());

    let prices = fetch_current_prices(&tickers).await.unwrap();
    assert!(prices.is_empty());

    let holdings_with_prices = portfolio.holdings_with_prices(&prices);
    assert!(holdings_with_prices.is_empty());
}
