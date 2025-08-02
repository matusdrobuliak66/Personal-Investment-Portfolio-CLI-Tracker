use anyhow::Result;
use colored::*;
use std::path::Path;

use crate::api::fetch_current_prices;
use crate::portfolio::Portfolio;

pub async fn balance_command<P: AsRef<Path>>(portfolio_file: P) -> Result<()> {
    let portfolio = Portfolio::load_from_file(portfolio_file)?;
    let tickers = portfolio.get_tickers();
    let prices = fetch_current_prices(&tickers).await?;
    let holdings_with_prices = portfolio.holdings_with_prices(&prices);

    println!("{}", "--------------------------------------".cyan());
    println!("{:<8} | {:<8} | {:<13} | {:<10}", 
             "Ticker".bold(), 
             "Quantity".bold(), 
             "Current Price".bold(), 
             "Value".bold());
    println!("{}", "--------------------------------------".cyan());

    let mut total_value = 0.0;

    for holding_with_price in &holdings_with_prices {
        let holding = &holding_with_price.holding;
        let current_value = holding_with_price.current_value();
        total_value += current_value;

        println!("{:<8} | {:<8.2} | {:<13.2} | {:<10.2}",
                 holding.ticker.yellow(),
                 holding.quantity,
                 holding_with_price.current_price,
                 current_value);
    }

    println!("{}", "--------------------------------------".cyan());
    println!("{}: {:.2}", "Total Portfolio Value".bold().green(), total_value);

    Ok(())
}

pub async fn allocation_command<P: AsRef<Path>>(portfolio_file: P) -> Result<()> {
    let portfolio = Portfolio::load_from_file(portfolio_file)?;
    let tickers = portfolio.get_tickers();
    let prices = fetch_current_prices(&tickers).await?;
    let holdings_with_prices = portfolio.holdings_with_prices(&prices);

    let total_value: f64 = holdings_with_prices.iter()
        .map(|h| h.current_value())
        .sum();

    println!("{}", "--------------------------------------".cyan());
    println!("{}", "Asset Allocation".bold());
    println!("{}", "--------------------------------------".cyan());

    for holding_with_price in &holdings_with_prices {
        let holding = &holding_with_price.holding;
        let current_value = holding_with_price.current_value();
        let percentage = if total_value > 0.0 {
            (current_value / total_value) * 100.0
        } else {
            0.0
        };

        println!("{}: {:.1}%", 
                 holding.ticker.yellow(), 
                 percentage);
    }

    println!("{}", "--------------------------------------".cyan());

    Ok(())
}

pub async fn performance_command<P: AsRef<Path>>(portfolio_file: P) -> Result<()> {
    let portfolio = Portfolio::load_from_file(portfolio_file)?;
    let tickers = portfolio.get_tickers();
    let prices = fetch_current_prices(&tickers).await?;
    let holdings_with_prices = portfolio.holdings_with_prices(&prices);

    println!("{}", "--------------------------------------".cyan());
    println!("{:<8} | {:<14} | {:<13} | {:<10} | {:<12}", 
             "Ticker".bold(), 
             "Purchase Price".bold(), 
             "Current Price".bold(), 
             "Return %".bold(),
             "Gain/Loss".bold());
    println!("{}", "--------------------------------------".cyan());

    let mut total_cost = 0.0;
    let mut total_current_value = 0.0;

    for holding_with_price in &holdings_with_prices {
        let holding = &holding_with_price.holding;
        let return_percentage = holding_with_price.gain_loss_percentage();
        let gain_loss = holding_with_price.gain_loss();
        
        total_cost += holding_with_price.total_cost();
        total_current_value += holding_with_price.current_value();

        let return_str = if return_percentage >= 0.0 {
            format!("+{:.1}%", return_percentage).green()
        } else {
            format!("{:.1}%", return_percentage).red()
        };

        let gain_loss_str = if gain_loss >= 0.0 {
            format!("+{:.2}", gain_loss).green()
        } else {
            format!("{:.2}", gain_loss).red()
        };

        println!("{:<8} | {:<14.2} | {:<13.2} | {:<10} | {:<12}",
                 holding.ticker.yellow(),
                 holding.cost_basis,
                 holding_with_price.current_price,
                 return_str,
                 gain_loss_str);
    }

    println!("{}", "--------------------------------------".cyan());

    let total_return_percentage = if total_cost > 0.0 {
        ((total_current_value - total_cost) / total_cost) * 100.0
    } else {
        0.0
    };

    let total_gain_loss = total_current_value - total_cost;

    let total_return_str = if total_return_percentage >= 0.0 {
        format!("Total Return: +{:.1}%", total_return_percentage).bold().green()
    } else {
        format!("Total Return: {:.1}%", total_return_percentage).bold().red()
    };

    let total_gain_loss_str = if total_gain_loss >= 0.0 {
        format!("Total Gain/Loss: +{:.2}", total_gain_loss).bold().green()
    } else {
        format!("Total Gain/Loss: {:.2}", total_gain_loss).bold().red()
    };

    println!("{}", total_return_str);
    println!("{}", total_gain_loss_str);

    Ok(())
}
