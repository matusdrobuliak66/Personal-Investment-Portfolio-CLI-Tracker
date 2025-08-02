use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

// mod declares that these modules exist and makes them part of our crate
// The compiler looks for portfolio.rs, commands.rs, api.rs (or portfolio/mod.rs, etc.)
mod portfolio;
mod commands;
mod api;

// use brings specific items from modules into scope so we don't need full paths
// Without this, we'd have to write commands::balance_command() everywhere
use commands::{balance_command, allocation_command, performance_command};

// #[derive(Parser)] - attribute that auto-generates Parser trait implementation
// Saves us from writing boilerplate code for command line parsing
#[derive(Parser)]
// #[command(...)] - clap-specific attributes that configure CLI behavior
#[command(name = "portfolio_rs")]
#[command(about = "Personal Investment Portfolio CLI Tracker")]
#[command(version = "1.0")]
struct Cli {
    // #[command(subcommand)] tells clap this field contains subcommands
    #[command(subcommand)]
    command: Commands,
}

// #[derive(Subcommand)] - auto-generates Subcommand trait for this enum
#[derive(Subcommand)]
enum Commands {
    /// Show current balances for all holdings
    Balances {
        /// Path to the portfolio JSON file
        portfolio_file: PathBuf,
    },
    /// Show asset allocation percentages
    Allocation {
        /// Path to the portfolio JSON file
        portfolio_file: PathBuf,
    },
    /// Show performance metrics for all holdings
    Performance {
        /// Path to the portfolio JSON file
        portfolio_file: PathBuf,
    },
}

// #[tokio::main] - procedural macro that transforms async main function
// Without this, we'd need to manually set up the Tokio async runtime
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Balances { portfolio_file } => {
            balance_command(&portfolio_file).await?;
        }
        Commands::Allocation { portfolio_file } => {
            allocation_command(&portfolio_file).await?;
        }
        Commands::Performance { portfolio_file } => {
            performance_command(&portfolio_file).await?;
        }
    }

    Ok(())
}
