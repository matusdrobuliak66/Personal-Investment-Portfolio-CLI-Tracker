# Personal Investment Portfolio CLI Tracker

A complete Rust CLI application for tracking investment portfolios with colored terminal output.

## Features

- ✅ **CLI Interface**: Full command-line interface with clap
- ✅ **Multiple Commands**: balances, allocation, performance
- ✅ **JSON Portfolio Storage**: Simple file-based portfolio storage
- ✅ **Portfolio Analysis**: 
   - Current balances and values
   - Asset allocation percentages
   - Performance metrics with gains/losses
- ✅ **Mock Price Data**: Built-in price data for stocks and crypto
- ✅ **Colored Output**: Enhanced terminal output with colors
- ✅ **Error Handling**: Comprehensive error handling with anyhow
- ✅ **Multi-Asset Support**: Stocks and cryptocurrency support

## Quick Start

1. **Build**: `cargo build --release` (requires Rust)
2. **Run**: `./target/release/portfolio_rs balances portfolio.json`

### Portfolio Format

```json
[
  {
    "ticker": "AAPL",
    "quantity": 10,
    "cost_basis": 150,
    "date_purchased": "2024-06-01"
  },
  {
    "ticker": "TSLA", 
    "quantity": 5,
    "cost_basis": 600,
    "date_purchased": "2024-05-10"
  }
]
```

### Commands

```bash
# View current balances
portfolio_rs balances portfolio.json

# View asset allocation
portfolio_rs allocation portfolio.json  

# View performance metrics
portfolio_rs performance portfolio.json
```

## Example Output

### Balances Command
```
--------------------------------------
Ticker   | Quantity | Current Price | Value
--------------------------------------
AAPL     | 10.00    | 170.00        | 1700.00
TSLA     | 5.00     | 700.00        | 3500.00
GOOGL    | 2.00     | 2800.00       | 5600.00
MSFT     | 15.00    | 350.00        | 5250.00
--------------------------------------
Total Portfolio Value: 16050.00
```

### Allocation Command
```
--------------------------------------
Asset Allocation
--------------------------------------
AAPL: 10.6%
TSLA: 21.8%
GOOGL: 34.9%
MSFT: 32.7%
--------------------------------------
```

### Performance Command
```
--------------------------------------
Ticker   | Purchase Price | Current Price | Return % | Gain/Loss
--------------------------------------
AAPL     | 150.00         | 170.00        | +13.3%   | +200.00
TSLA     | 600.00         | 700.00        | +16.7%   | +500.00
GOOGL    | 2500.00        | 2800.00       | +12.0%   | +600.00
MSFT     | 300.00         | 350.00        | +16.7%   | +750.00
--------------------------------------
Total Return: +14.8%
Total Gain/Loss: +2050.00
```

## Development

Run without building a release binary:
```bash
cargo run -- balances portfolio.json
cargo run -- allocation portfolio.json
cargo run -- performance portfolio.json
```

## Price Data

Uses mock price data for demonstration. For production:
1. Sign up for a stock price API (Alpha Vantage, IEX Cloud, etc.)
2. Add your API key to `src/api.rs`
3. Replace mock data with real API calls

## Technologies Used / Dependencies

- **Rust**: Core programming language
- **Clap**: Command-line argument parsing
- **Serde**: JSON serialization/deserialization
- **Tokio**: Async runtime
- **Reqwest**: HTTP client (prepared for real API integration)
- **Colored**: Terminal color output
- **Anyhow**: Error handling
- **Chrono**: Date/time handling


## Future Enhancements

- [ ] Real API integration
- [ ] Encryption support for portfolio files
- [ ] Historical performance tracking
- [ ] Portfolio rebalancing suggestions
- [ ] Support for multiple asset classes (bonds, crypto, etc.)
- [ ] Export to CSV/Excel
- [ ] Web dashboard
