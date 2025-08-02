# Testing Documentation

This project includes comprehensive unit tests, integration tests, and benchmarks.

## Running Tests

### All Tests
```bash
cargo test
```

### Unit Tests Only
```bash
cargo test --lib
```

### Integration Tests Only
```bash
cargo test --test integration_tests
```

### Benchmark Tests
```bash
cargo test benchmark_portfolio_processing -- --nocapture
```

### Test Coverage
To see test coverage, you can use cargo-tarpaulin:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin
```

## Test Structure

### Unit Tests (`src/` modules)
- **Portfolio Tests** (`src/portfolio.rs`): Test all portfolio calculations, file loading, and data structures
- **API Tests** (`src/api.rs`): Test price fetching functionality with mock data

### Integration Tests (`tests/` directory)
- **Integration Tests** (`tests/integration_tests.rs`): End-to-end testing of portfolio loading and price calculations
- **Benchmark Tests** (`tests/benchmark_tests.rs`): Performance testing with timing metrics

## Test Coverage Areas

### Portfolio Module Tests
- `HoldingWithPrice` calculations (current value, total cost, gain/loss, percentages)
- Portfolio loading from JSON files
- Error handling for invalid files
- Empty portfolios
- Missing price data scenarios

### API Module Tests
- Mock price data retrieval
- Handling unknown tickers
- Empty ticker lists
- Mixed known/unknown ticker scenarios

### Integration Tests
- Full workflow testing (file → portfolio → prices → calculations)
- Real file I/O operations
- Async price fetching
- Performance benchmarking

## Test Data
Tests use temporary files and mock data to ensure they don't depend on external APIs or files.

## Adding New Tests
When adding new features, please include:
1. Unit tests for individual functions
2. Integration tests for complete workflows
3. Error case testing
4. Performance considerations for large portfolios

## Continuous Integration

### GitHub Actions Workflow

The project includes a simple GitHub Actions workflow (`.github/workflows/rust.yml`) for automated testing:

- **Triggers**: Push to main branch, Pull Requests
- **Workflow Steps**:
  1. **Checkout code**: Gets the latest code
  2. **Install Rust toolchain**: Sets up Rust stable version
  3. **Cache dependencies**: Speeds up builds by caching Cargo dependencies
  4. **Build project**: Compiles the code with `cargo build --verbose`
  5. **Run tests**: Executes all tests with `cargo test --verbose`

### Running Tests Locally Before Push

To ensure your changes will pass CI, run these commands locally:

```bash
# Build the project
cargo build --verbose

# Run all tests
cargo test --verbose
```

### Viewing Test Results

- **GitHub Actions**: Go to the "Actions" tab in your repository to see test results
- **Pull Requests**: Test status is automatically shown on each PR

### Test Status Badge

You can add this badge to your README.md to show the current test status:

```markdown
![Rust CI](https://github.com/matusdrobuliak66/Personal-Investment-Portfolio-CLI-Tracker/workflows/Rust%20CI/badge.svg)
```
