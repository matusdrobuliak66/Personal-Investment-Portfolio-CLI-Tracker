use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holding {
    pub ticker: String,
    pub quantity: f64,
    pub cost_basis: f64,
    pub date_purchased: String,
}

#[derive(Debug, Clone)]
pub struct HoldingWithPrice {
    pub holding: Holding,
    pub current_price: f64,
}

impl HoldingWithPrice {
    pub fn current_value(&self) -> f64 {
        self.holding.quantity * self.current_price
    }

    pub fn total_cost(&self) -> f64 {
        self.holding.quantity * self.holding.cost_basis
    }

    pub fn gain_loss(&self) -> f64 {
        self.current_value() - self.total_cost()
    }

    pub fn gain_loss_percentage(&self) -> f64 {
        if self.total_cost() == 0.0 {
            0.0
        } else {
            (self.gain_loss() / self.total_cost()) * 100.0
        }
    }
}

#[derive(Debug)]
pub struct Portfolio {
    pub holdings: Vec<Holding>,
}

impl Portfolio {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read portfolio file: {:?}", path.as_ref()))?;
        
        let holdings: Vec<Holding> = serde_json::from_str(&content)
            .with_context(|| "Failed to parse portfolio JSON")?;

        Ok(Portfolio { holdings })
    }

    pub fn get_tickers(&self) -> Vec<String> {
        self.holdings.iter().map(|h| h.ticker.clone()).collect()
    }

    pub fn holdings_with_prices(&self, prices: &std::collections::HashMap<String, f64>) -> Vec<HoldingWithPrice> {
        self.holdings
            .iter()
            .map(|holding| {
                let current_price = *prices.get(&holding.ticker).unwrap_or(&0.0);
                HoldingWithPrice {
                    holding: holding.clone(),
                    current_price,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use tempfile::tempdir;

    fn create_test_holding() -> Holding {
        Holding {
            ticker: "AAPL".to_string(),
            quantity: 10.0,
            cost_basis: 150.0,
            date_purchased: "2023-01-01".to_string(),
        }
    }

    fn create_test_portfolio() -> Portfolio {
        Portfolio {
            holdings: vec![
                Holding {
                    ticker: "AAPL".to_string(),
                    quantity: 10.0,
                    cost_basis: 150.0,
                    date_purchased: "2023-01-01".to_string(),
                },
                Holding {
                    ticker: "TSLA".to_string(),
                    quantity: 5.0,
                    cost_basis: 200.0,
                    date_purchased: "2023-02-01".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_holding_with_price_current_value() {
        let holding = create_test_holding();
        let holding_with_price = HoldingWithPrice {
            holding,
            current_price: 175.0,
        };

        assert_eq!(holding_with_price.current_value(), 1750.0); // 10 * 175
    }

    #[test]
    fn test_holding_with_price_total_cost() {
        let holding = create_test_holding();
        let holding_with_price = HoldingWithPrice {
            holding,
            current_price: 175.0,
        };

        assert_eq!(holding_with_price.total_cost(), 1500.0); // 10 * 150
    }

    #[test]
    fn test_holding_with_price_gain_loss() {
        let holding = create_test_holding();
        let holding_with_price = HoldingWithPrice {
            holding,
            current_price: 175.0,
        };

        assert_eq!(holding_with_price.gain_loss(), 250.0); // 1750 - 1500
    }

    #[test]
    fn test_holding_with_price_gain_loss_percentage() {
        let holding = create_test_holding();
        let holding_with_price = HoldingWithPrice {
            holding,
            current_price: 175.0,
        };

        let expected_percentage = (250.0 / 1500.0) * 100.0;
        assert!((holding_with_price.gain_loss_percentage() - expected_percentage).abs() < 0.001);
    }

    #[test]
    fn test_holding_with_price_gain_loss_percentage_zero_cost() {
        let mut holding = create_test_holding();
        holding.cost_basis = 0.0;
        let holding_with_price = HoldingWithPrice {
            holding,
            current_price: 175.0,
        };

        assert_eq!(holding_with_price.gain_loss_percentage(), 0.0);
    }

    #[test]
    fn test_portfolio_get_tickers() {
        let portfolio = create_test_portfolio();
        let tickers = portfolio.get_tickers();

        assert_eq!(tickers.len(), 2);
        assert!(tickers.contains(&"AAPL".to_string()));
        assert!(tickers.contains(&"TSLA".to_string()));
    }

    #[test]
    fn test_portfolio_holdings_with_prices() {
        let portfolio = create_test_portfolio();
        let mut prices = HashMap::new();
        prices.insert("AAPL".to_string(), 175.0);
        prices.insert("TSLA".to_string(), 250.0);

        let holdings_with_prices = portfolio.holdings_with_prices(&prices);

        assert_eq!(holdings_with_prices.len(), 2);
        
        let aapl_holding = holdings_with_prices.iter()
            .find(|h| h.holding.ticker == "AAPL")
            .unwrap();
        assert_eq!(aapl_holding.current_price, 175.0);
        
        let tsla_holding = holdings_with_prices.iter()
            .find(|h| h.holding.ticker == "TSLA")
            .unwrap();
        assert_eq!(tsla_holding.current_price, 250.0);
    }

    #[test]
    fn test_portfolio_holdings_with_prices_missing_ticker() {
        let portfolio = create_test_portfolio();
        let mut prices = HashMap::new();
        prices.insert("AAPL".to_string(), 175.0);
        // TSLA price is missing

        let holdings_with_prices = portfolio.holdings_with_prices(&prices);

        let tsla_holding = holdings_with_prices.iter()
            .find(|h| h.holding.ticker == "TSLA")
            .unwrap();
        assert_eq!(tsla_holding.current_price, 0.0); // Should default to 0.0
    }

    #[test]
    fn test_portfolio_load_from_file() {
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

        let portfolio = Portfolio::load_from_file(&file_path).unwrap();

        assert_eq!(portfolio.holdings.len(), 2);
        assert_eq!(portfolio.holdings[0].ticker, "AAPL");
        assert_eq!(portfolio.holdings[0].quantity, 10.0);
        assert_eq!(portfolio.holdings[1].ticker, "TSLA");
        assert_eq!(portfolio.holdings[1].quantity, 5.0);
    }

    #[test]
    fn test_portfolio_load_from_file_invalid_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalid_portfolio.json");

        fs::write(&file_path, "invalid json").unwrap();

        let result = Portfolio::load_from_file(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_portfolio_load_from_file_nonexistent() {
        let result = Portfolio::load_from_file("nonexistent_file.json");
        assert!(result.is_err());
    }
}
