# Rust Learning Notes - Key Concepts

## Module System: `mod` vs `use`

### `mod` - Module Declaration
- **Declares** that a module exists and makes it part of your crate
- Tells the compiler to include the module's code
- Creates a new namespace

### `use` - Bringing Items into Scope
- **Imports** items from modules into the current scope
- Creates shortcuts to avoid writing full paths
- Does not declare modules, only brings existing items into scope

### Example:
```rust
mod portfolio;  // Declares the portfolio module (looks for portfolio.rs or portfolio/mod.rs)
mod commands;   // Declares the commands module
mod api;        // Declares the api module

use commands::{balance_command, allocation_command, performance_command};
// Brings specific functions from the commands module into scope
```

**Without `use`, you would need full paths:**
```rust
commands::balance_command(&portfolio_file).await?;
commands::allocation_command(&portfolio_file).await?;
commands::performance_command(&portfolio_file).await?;
```

## Attributes (`#[...]`)

Attributes provide metadata and instructions to the compiler about how to process the code that follows.

### `#[derive(...)]` - Automatic Trait Implementation
```rust
#[derive(Parser)]     // Auto-generates Parser trait implementation
#[derive(Subcommand)] // Auto-generates Subcommand trait implementation
```
- Tells the compiler to automatically generate code for traits
- Saves you from writing boilerplate code manually

### `#[command(...)]` - Clap Configuration
```rust
#[command(name = "portfolio_rs")]
#[command(about = "Personal Investment Portfolio CLI Tracker")]
#[command(version = "1.0")]
```
- These are **clap-specific attributes** that configure CLI behavior
- Sets the program name, description, and version

### `#[tokio::main]` - Async Runtime Macro
```rust
#[tokio::main]
async fn main() -> Result<()> {
```
- **Procedural macro** that transforms your async main function
- Automatically sets up the Tokio async runtime

**Without `#[tokio::main]`, you'd need to write:**
```rust
fn main() -> Result<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // your async code here
    })
}
```

## Core Rust Concepts

### **Crate**
- A **compilation unit** in Rust - think of it as a package or library
- Your entire project is a crate (in your case, the portfolio CLI tool)
- Can be a binary crate (produces an executable) or library crate (produces a library)
- `Cargo.toml` defines your crate's metadata and dependencies

**Crate structure example:**
```
portfolio/ (crate root)
├── src/
│   ├── main.rs (binary crate entry point)
│   ├── portfolio.rs (module)
│   ├── commands.rs (module)
│   └── api.rs (module)
```

### **Trait**
- Defines **shared behavior** that types can implement
- Similar to interfaces in other languages
- Allows different types to have common functionality

```rust
// Example trait
trait Display {
    fn display(&self) -> String;
}

// The Parser trait (from clap) defines how to parse command line arguments
// When you use #[derive(Parser)], Rust automatically implements this trait for your struct
```

### **Procedural Macro**
- **Code that generates code** at compile time
- Takes Rust code as input and produces Rust code as output
- Runs during compilation, not runtime

**Example transformation:**
```rust
// This code:
#[tokio::main]
async fn main() -> Result<()> {
    // your async code
}

// Gets transformed into something like:
fn main() -> Result<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // your async code
    })
}
```

## Key Takeaways

- `mod` = "This module exists in my crate"
- `use` = "I want to use these items without typing the full path"
- Attributes (`#[...]`) = Metadata for the compiler to generate code or configure behavior
- **Crate**: Your entire project
- **Trait**: Shared behavior that types can implement
- **Procedural Macro**: Code that generates code at compile time