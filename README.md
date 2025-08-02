# Personal Investment Portfolio CLI Tracker

A command-line application for tracking your investment portfolio built in Rust.

## Features

- **Balance Tracking**: View current holdings, quantities, prices, and total values
- **Asset Allocation**: See percentage breakdown of your portfolio
- **Performance Analysis**: Track gains/losses and returns since purchase
- **JSON Portfolio Storage**: Simple file-based storage for your holdings

## Installation

1. Make sure you have Rust installed: https://rustup.rs/
2. Clone this repository
3. Build the application:
   ```bash
   cargo build --release
   ```

The binary will be available at `target/release/portfolio_rs`

## Usage

### Portfolio File Format

Create a JSON file (e.g., `portfolio.json`) with your holdings:

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

#### View Balances
```bash
portfolio_rs balances portfolio.json
```
Shows current holdings with live prices and total portfolio value.

#### View Asset Allocation
```bash
portfolio_rs allocation portfolio.json
```
Displays percentage breakdown of your portfolio by ticker.

#### View Performance
```bash
portfolio_rs performance portfolio.json
```
Shows gain/loss percentages and dollar amounts for each holding.

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

## Price Data

The application uses mock price data for demonstration purposes. In a production environment, you would:

1. Sign up for a stock price API (Alpha Vantage, IEX Cloud, etc.)
2. Add your API key to the `src/api.rs` file
3. Replace the mock data with real API calls

## Development

To run the application during development (without building a release binary):

```bash
cargo run -- balances portfolio.json
cargo run -- allocation portfolio.json
cargo run -- performance portfolio.json
```

## Dependencies

- `clap`: Command-line argument parsing
- `serde`: JSON serialization/deserialization
- `reqwest`: HTTP client for API calls
- `tokio`: Async runtime
- `anyhow`: Error handling
- `colored`: Terminal colors
- `chrono`: Date/time handling

## Future Enhancements

- [ ] Encryption support for portfolio files
- [ ] Historical performance tracking
- [ ] Portfolio rebalancing suggestions
- [ ] Support for multiple asset classes (bonds, crypto, etc.)
- [ ] Export to CSV/Excel
- [ ] Web dashboard
- [ ] Real-time price updates
