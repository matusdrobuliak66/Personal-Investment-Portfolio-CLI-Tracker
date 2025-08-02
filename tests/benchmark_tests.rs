#[cfg(test)]
mod benchmark_tests {
    use portfolio::portfolio::Portfolio;
    use portfolio::api::fetch_current_prices;
    use std::time::Instant;
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn benchmark_portfolio_processing() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("large_portfolio.json");

        // Create a larger portfolio for benchmarking
        let mut holdings = Vec::new();
        let tickers = vec!["AAPL", "TSLA", "GOOGL", "MSFT", "AMZN", "NVDA", "META", "BRK.B", "JPM", "V"];
        
        for (i, ticker) in tickers.iter().enumerate() {
            holdings.push(format!(
                r#"{{
                    "ticker": "{}",
                    "quantity": {},
                    "cost_basis": {},
                    "date_purchased": "2023-{:02}-01"
                }}"#,
                ticker,
                10.0 + i as f64,
                100.0 + i as f64 * 10.0,
                (i % 12) + 1
            ));
        }

        let test_data = format!("[{}]", holdings.join(","));
        fs::write(&file_path, test_data).unwrap();

        let start = Instant::now();

        // Load portfolio
        let portfolio = Portfolio::load_from_file(&file_path).unwrap();
        let load_time = start.elapsed();

        let start = Instant::now();
        
        // Fetch prices
        let tickers = portfolio.get_tickers();
        let prices = fetch_current_prices(&tickers).await.unwrap();
        let fetch_time = start.elapsed();

        let start = Instant::now();
        
        // Calculate holdings with prices
        let holdings_with_prices = portfolio.holdings_with_prices(&prices);
        let calc_time = start.elapsed();

        // Perform calculations on all holdings
        let start = Instant::now();
        let total_value: f64 = holdings_with_prices.iter()
            .map(|h| h.current_value())
            .sum();
        let total_cost: f64 = holdings_with_prices.iter()
            .map(|h| h.total_cost())
            .sum();
        let total_gain_loss: f64 = holdings_with_prices.iter()
            .map(|h| h.gain_loss())
            .sum();
        let analysis_time = start.elapsed();

        // Assertions to ensure the benchmark actually worked
        assert_eq!(portfolio.holdings.len(), 10);
        assert_eq!(prices.len(), 10);
        assert_eq!(holdings_with_prices.len(), 10);
        assert!(total_value > 0.0);
        assert!(total_cost > 0.0);

        // Print timing information (these will show up in test output)
        println!("Benchmark results:");
        println!("  Portfolio loading: {:?}", load_time);
        println!("  Price fetching: {:?}", fetch_time);
        println!("  Price calculations: {:?}", calc_time);
        println!("  Analysis calculations: {:?}", analysis_time);
        println!("  Total value: ${:.2}", total_value);
        println!("  Total cost: ${:.2}", total_cost);
        println!("  Total gain/loss: ${:.2}", total_gain_loss);
    }
}
