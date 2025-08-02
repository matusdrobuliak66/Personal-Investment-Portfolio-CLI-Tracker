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
