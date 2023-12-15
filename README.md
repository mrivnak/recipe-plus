# Recipe Plus

![Rust](https://img.shields.io/badge/rust-%23f74c00.svg?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/axum-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Leptos](https://img.shields.io/badge/leptos-%23FF0135.svg?style=for-the-badge&logo=leptos&logoColor=white)
![Diesel](https://img.shields.io/badge/diesel-%23BB0000.svg?style=for-the-badge&logo=rust&logoColor=white)
![SQLite](https://img.shields.io/badge/sqlite-%2307405e.svg?style=for-the-badge&logo=sqlite&logoColor=white)

![GitHub Actions](https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![Nginx](https://img.shields.io/badge/nginx-%23009639.svg?style=for-the-badge&logo=nginx&logoColor=white)

Self-hosted recipe manager with a focus on usability and readability.

## Features

- Resize recipes proportionally
- Convert between metric and imperial units
- Quickly reference quantities in the instructions with tooltips

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Rustup](https://rustup.rs/)
- [Trunk](https://crates.io/crates/trunk)
- [Diesel CLI](https://crates.io/crates/diesel_cli)
- [Python 3](https://www.python.org)
- [NPM](https://www.npmjs.com)

This project requires nightly Rust. To install nightly, run:

```bash
rustup toolchain install nightly
```

as well as the wasm32-unknown-unknown target:

```bash
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### Running

Use `rustup` and `trunk` to run the project:

```bash
# Backend
cd recipe-plus-server && cargo run

# Frontend
cd recipe-plus && trunk serve
```

> The backend and frontend will run on ports 8000 and 3000, respectively.
