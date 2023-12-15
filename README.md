# Recipe Plus

![Vue.js](https://img.shields.io/badge/vuejs-%2335495e.svg?style=for-the-badge&logo=vuedotjs&logoColor=%234FC08D)
![Nuxtjs](https://img.shields.io/badge/Nuxt-002E3B?style=for-the-badge&logo=nuxtdotjs&logoColor=#00DC82)
![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)
![TailwindCSS](https://img.shields.io/badge/tailwindcss-%2338B2AC.svg?style=for-the-badge&logo=tailwind-css&logoColor=white)

![Rust](https://img.shields.io/badge/rust-%23f74c00.svg?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/axum-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Diesel](https://img.shields.io/badge/diesel-%23BB0000.svg?style=for-the-badge&logo=rust&logoColor=white)
![SQLite](https://img.shields.io/badge/sqlite-%2307405e.svg?style=for-the-badge&logo=sqlite&logoColor=white)

![GitHub Actions](https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

Self-hosted recipe manager with a focus on usability and readability.

## Features

- Resize recipes proportionally
- Convert between metric and imperial units
- Quickly reference quantities in the instructions with tooltips

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Diesel CLI](https://crates.io/crates/diesel_cli)
- [Python 3](https://www.python.org)
- [PNPM](https://pnpm.io)

### Running

Use `cargo` and `pnpm` to run the project:

```bash
# Backend
cd recipe-plus-api && cargo run

# Frontend
cd recipe-plus-web && pnpm install && pnpm dev
```

> The backend and frontend will run on ports 8000 and 3000, respectively.
