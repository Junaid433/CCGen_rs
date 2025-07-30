<p align="center">
  <img src="https://raw.githubusercontent.com/Junaid433/CCGen_rs/master/assets/logo.png" alt="ccgen_rs logo" width="120" height="120"/>
</p>

<h1 align="center">CCGen_rs</h1>

<p align="center">
  <b>A fast, flexible, and customizable credit card number generator library for Rust.</b><br>
  <i>Generate valid (Luhn-compliant) credit card numbers for testing, QA, and development purposes.</i>
</p>

<p align="center">
  <a href="https://github.com/Junaid433/CCGen_rs/actions/workflows/cargo-test.yml">
    <img src="https://github.com/Junaid433/CCGen_rs/actions/workflows/cargo-test.yml/badge.svg" alt="Cargo Test Status">
  </a>
  <a href="https://github.com/Junaid433/CCGen_rs/actions/workflows/cargo-build.yml">
    <img src="https://github.com/Junaid433/CCGen_rs/actions/workflows/cargo-build.yml/badge.svg" alt="Cargo Build Status">
  </a>
  <a href="https://crates.io/crates/ccgen_rs">
    <img src="https://img.shields.io/crates/v/ccgen_rs.svg" alt="Crates.io Version">
  </a>
  <a href="https://docs.rs/ccgen_rs">
    <img src="https://docs.rs/ccgen_rs/badge.svg" alt="docs.rs">
  </a>
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License">
  </a>
</p>

---

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [API Reference](#api-reference)
- [Input Validation](#input-validation)
- [Testing](#testing)
- [Continuous Integration (CI)](#continuous-integration-ci)
- [Contributing](#contributing)
- [License](#license)
- [Disclaimer](#disclaimer)

---

## Features
- Generate valid credit card numbers using the Luhn algorithm
- Customizable BIN patterns (with support for 'x' wildcards)
- Specify or randomize expiration month, year, and CVV
- Generate single or multiple cards at once
- Input validation for all fields
- Designed for easy integration and extension
- Comprehensive test suite

## Installation
Add `ccgen_rs` to your `Cargo.toml`:

```toml
[dependencies]
ccgen_rs = "0.1.0"
```

## Usage

### Basic Example
```rust
use ccgen_rs::{GeneratorOptions, Generate_Cards};

let opts = GeneratorOptions {
    bin_pattern: "411111xxxxxxxxxx", // Visa BIN with 10 random digits
    month: Some(12),
    year: Some(2030),
    cvv: Some(123),
    amount: Some(3),
};
let cards = Generate_Cards(&opts);
for card in cards {
    println!("{}|{}|{}|{}", card.number, card.month, card.year, card.cvv);
}
```

### Randomized Example
```rust
let opts = GeneratorOptions {
    bin_pattern: "5xxxxxxxxxxxxxxx", // MasterCard BIN with 15 random digits
    month: None, // random month
    year: None,  // random year
    cvv: None,   // random cvv
    amount: Some(5),
};
let cards = Generate_Cards(&opts);
```

## API Reference

### GeneratorOptions
```rust
pub struct GeneratorOptions<'a> {
    pub bin_pattern: &'a str, // 16 chars, digits or 'x'
    pub month: Option<usize>, // 1-12
    pub year: Option<usize>,  // 2025-2050
    pub cvv: Option<usize>,   // 0-999
    pub amount: Option<usize>,// number of cards to generate
}
```

### CardInfo
```rust
pub struct CardInfo {
    pub number: String, // 16-digit Luhn-valid card number
    pub month: String,  // MM
    pub year: String,   // YYYY
    pub cvv: String,    // 3 digits
}
```

### Generate_Cards
```rust
pub fn Generate_Cards(opts: &GeneratorOptions) -> Vec<CardInfo>
```
- Returns a vector of generated cards based on the provided options.
- Returns an empty vector if input is invalid (e.g., bad BIN pattern, out-of-range month/year/cvv).

## Input Validation
- `bin_pattern` must be 16 or fewer characters, only digits or 'x'.
- `month` must be 1-12.
- `year` must be 2025-2050.
- `cvv` must be 0-999.
- Invalid input returns an empty vector.

## Testing
Run the test suite with:

```sh
cargo test
```

The project includes 22+ tests covering edge cases, input validation, and Luhn compliance.

## Continuous Integration (CI)

This project uses GitHub Actions for CI. Two workflows are provided:

- **Cargo Test Workflow:** Runs `cargo test` on every push and pull request to ensure all tests pass.
- **Cargo Build Workflow:** Runs `cargo build` to verify the project builds successfully.

You can find these workflows in `.github/workflows/`.

## Contributing
Contributions are welcome! Please open issues or submit pull requests for bug fixes, new features, or improvements.

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Open a pull request

## License
This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Disclaimer
This library is for testing and development purposes only. Do not use generated card numbers for fraudulent or illegal activities.
