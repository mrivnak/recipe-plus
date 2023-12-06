# Recipe Plus

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/mrivnak/recipe-plus/build.yml)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/mrivnak/recipe-plug/test.yml?label=tests)
[![Coverage Status](https://coveralls.io/repos/github/mrivnak/recipe-plus/badge.svg?branch=main)](https://coveralls.io/github/mrivnak/recipe-plus?branch=main)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/mrivnak/recipe-plus)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/axum-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Leptos](https://img.shields.io/badge/leptos-%23FF0135.svg?style=for-the-badge&logo=leptos&logoColor=white)
![GitHub Actions](https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white)
![Coveralls](https://img.shields.io/badge/coveralls-%23b94947.svg?style=for-the-badge&logo=coveralls&logoColor=white)
![Renovate](https://img.shields.io/badge/renovate-%230281a1?style=for-the-badge&logo=renovatebot&logoColor=white)

Self-hosted recipe manager with a focus on usability and readability.

## Features

- Resize recipes proportionally
- Convert between metric and imperial units
- Quickly reference quantities in the instructions with tooltips

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Rustup](https://rustup.rs/) // to install toolchains
- [cargo-leptos](https://crates.io/crates/cargo-leptos)

This project requires nightly Rust. To install nightly, run:

```bash
rustup toolchain install nightly
```

as well as the wasm32-unknown-unknown target:

```bash
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### Running

Use `cargo-leptos` to run the project:

```bash
cargo leptos serve
```