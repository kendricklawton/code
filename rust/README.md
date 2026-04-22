# Rust

A collection of Rust projects organized by topic. This README serves as a quick-reference guide for working with Cargo, crates, workspaces, and common workflows.

## Current Projects

```
rust/
├── dsa/
│   └── invert_binary_tree/       # Binary tree inversion algorithm
└── the-rust-programming-language/
    ├── guessing_game/            # Interactive number guessing game (uses rand)
    ├── hello_cargo/              # Basic Cargo project
    ├── hello_world/              # Compiled without Cargo (rustc directly)
    └── variables/                # Variable shadowing and scoping
```

All Cargo projects use **edition 2024**.

---

## Getting Started

### Install Rust

Use [rustup](https://rustup.rs/) (the official installer and toolchain manager):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash
rustup update              # Update Rust toolchain
rustc --version            # Check compiler version
cargo --version            # Check Cargo version
```

### Rustup Essentials

```bash
rustup default stable          # Use the stable toolchain
rustup component add clippy    # Install the linter
rustup component add rustfmt   # Install the formatter
rustup doc                     # Open local Rust docs in browser
```

---

## Cargo — The Build System & Package Manager

Cargo handles building, dependency management, testing, and publishing. Nearly everything goes through Cargo.

### Create a New Project

```bash
cargo new my-project           # Binary (executable) project — creates src/main.rs
cargo new my-library --lib     # Library project — creates src/lib.rs
```

This generates:

```
my-project/
├── Cargo.toml
├── .gitignore
└── src/
    └── main.rs   (or lib.rs for libraries)
```

> Cargo automatically initializes a git repo. To skip: `cargo new my-project --vcs none`

### Cargo.toml

The manifest file that defines your crate:

```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"             # External crate from crates.io
serde = { version = "1.0", features = ["derive"] }  # With features
```

### Key Cargo Files

| File          | Purpose                                                   | Commit? |
|---------------|-----------------------------------------------------------|---------|
| `Cargo.toml`  | Declares package metadata, dependencies, and features     | Yes     |
| `Cargo.lock`  | Exact resolved versions of all dependencies               | Yes (binaries), Optional (libraries) |

---

## Build, Run, and Check

```bash
cargo build                # Compile (debug mode) → target/debug/
cargo build --release      # Compile (optimized) → target/release/
cargo run                  # Build and run
cargo run -- arg1 arg2     # Build and run with arguments
cargo check                # Type-check without producing a binary (fast)
```

`cargo check` is significantly faster than `cargo build` — use it for rapid feedback while developing.

### Without Cargo (rustc directly)

For standalone single-file programs:

```bash
rustc main.rs              # Compile directly
./main                     # Run the binary
```

This is how `hello_world/` in this repo is set up. For anything beyond trivial scripts, use Cargo.

---

## Dependencies

### Add a Crate

```bash
cargo add rand                         # Add latest version
cargo add serde --features derive      # Add with features
cargo add tokio@1.35                   # Add specific version
cargo remove rand                      # Remove a dependency
```

Or edit `Cargo.toml` directly under `[dependencies]` and then:

```bash
cargo build    # Downloads and compiles new dependencies automatically
```

### Find Crates

Browse [crates.io](https://crates.io/) or search from the terminal:

```bash
cargo search serde
```

### Update Dependencies

```bash
cargo update                # Update all deps to latest compatible versions
cargo update -p rand        # Update a specific crate
```

---

## Workspaces

A workspace lets you manage **multiple crates in one repo** with a shared `Cargo.lock` and `target/` directory.

### Set Up a Workspace

Create a `Cargo.toml` at the root:

```toml
[workspace]
members = [
    "dsa/invert_binary_tree",
    "the-rust-programming-language/guessing_game",
    "the-rust-programming-language/hello_cargo",
    "the-rust-programming-language/variables",
]
resolver = "2"
```

### Workspace Benefits

- **Shared `target/`** — crates don't rebuild shared dependencies separately
- **Shared `Cargo.lock`** — consistent dependency versions across all crates
- **Run commands across crates** from the workspace root

### Workspace Commands

```bash
cargo build --workspace                    # Build all crates
cargo test --workspace                     # Test all crates
cargo build -p guessing_game              # Build a specific crate
cargo run -p invert_binary_tree           # Run a specific crate
```

> **Note:** This repo currently has independent crates (each with their own git repo). To adopt a workspace, you'd create a root `Cargo.toml` with a `[workspace]` section and consolidate under one git repo.

---

## Testing

```bash
cargo test                     # Run all tests
cargo test --workspace         # Run tests across a workspace
cargo test -p my-crate         # Run tests for a specific crate
cargo test test_name           # Run tests matching a name
cargo test -- --nocapture      # Show println! output during tests
cargo test -- --test-threads=1 # Run tests sequentially
```

Test functions live in the same file as the code or in a `tests/` directory:

```rust
// Inline unit tests (same file as the code)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        divide(1, 0);
    }
}
```

Integration tests go in a top-level `tests/` directory:

```
my-project/
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

---

## Formatting and Linting

```bash
cargo fmt                  # Format all code (rustfmt)
cargo fmt -- --check       # Check formatting without modifying
cargo clippy               # Lint — catches common mistakes and suggests improvements
cargo clippy -- -D warnings  # Treat warnings as errors
```

`cargo fmt` enforces a consistent style. `cargo clippy` goes further — it catches performance issues, unnecessary complexity, and idiomatic improvements. Run both before committing.

---

## Documentation

```bash
cargo doc                  # Generate docs for your crate and dependencies
cargo doc --open           # Generate and open in browser
```

Document your code with `///` (doc comments):

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Code in doc comments is run as tests with `cargo test`.

---

## Project Structure Conventions

### Binary Crate

```
my-app/
├── Cargo.toml
├── src/
│   └── main.rs
```

### Library Crate

```
my-lib/
├── Cargo.toml
├── src/
│   └── lib.rs
```

### Larger Project

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library root (optional, for both bin + lib)
│   ├── config.rs        # Module file
│   └── handlers/
│       ├── mod.rs       # Module declaration
│       └── auth.rs
├── tests/
│   └── integration.rs   # Integration tests
├── benches/
│   └── benchmark.rs     # Benchmarks
└── examples/
    └── demo.rs           # Runnable examples (cargo run --example demo)
```

---

## Cleaning Up

```bash
cargo clean                # Delete the target/ directory (all build artifacts)
```

The `target/` directory can get large. It's always safe to delete — Cargo will rebuild everything on the next `cargo build`.

> All `.gitignore` files in this repo already exclude `/target`.

---

## Common Commands Cheat Sheet

| Command                          | What It Does                                      |
|----------------------------------|---------------------------------------------------|
| `cargo new <name>`               | Create a new binary project                       |
| `cargo new <name> --lib`         | Create a new library project                      |
| `cargo build`                    | Compile (debug)                                   |
| `cargo build --release`          | Compile (optimized)                               |
| `cargo run`                      | Build and run                                     |
| `cargo check`                    | Type-check only (fast)                            |
| `cargo test`                     | Run tests                                         |
| `cargo add <crate>`              | Add a dependency                                  |
| `cargo remove <crate>`           | Remove a dependency                               |
| `cargo update`                   | Update dependencies                               |
| `cargo fmt`                      | Format code                                       |
| `cargo clippy`                   | Lint code                                         |
| `cargo doc --open`               | Generate and view docs                            |
| `cargo clean`                    | Remove build artifacts                            |
| `cargo search <query>`           | Search crates.io                                  |
| `rustup update`                  | Update Rust toolchain                             |
| `rustup doc`                     | Open local Rust documentation                     |

---

## Useful References

| Resource | Link |
|----------|------|
| The Rust Programming Language (The Book) | [doc.rust-lang.org/book](https://doc.rust-lang.org/book/) |
| Rust by Example | [doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/) |
| Standard Library Docs | [doc.rust-lang.org/std](https://doc.rust-lang.org/std/) |
| Crates.io (Package Registry) | [crates.io](https://crates.io/) |
| Cargo Book | [doc.rust-lang.org/cargo](https://doc.rust-lang.org/cargo/) |
| Rustlings (Exercises) | [github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings) |
| Rust Reference | [doc.rust-lang.org/reference](https://doc.rust-lang.org/reference/) |
| Rust Playground | [play.rust-lang.org](https://play.rust-lang.org/) |
| Rust Cheat Sheet | [cheats.rs](https://cheats.rs/) |
| This Week in Rust | [this-week-in-rust.org](https://this-week-in-rust.org/) |
