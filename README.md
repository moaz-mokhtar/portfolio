# Moaz bin Mokhtar | Software Engineer Portfolio (Rust, React, Python)

[![Built with Leptos](https://img.shields.io/badge/built%20with-Leptos-f472b6)](https://leptos.dev/)
[![Backend Rust](https://img.shields.io/badge/backend-Rust-black)](https://www.rust-lang.org/)
[![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A high-performance, SEO-optimized personal portfolio rebuilt from the ground up using **Leptos SSR** and **Axum**. This project serves as a production-grade example of a modern Rust-based web application, featuring full server-side rendering, typed data models, and automated verification.

**Live Site:** [moaz-mokhtar.netlify.app](https://moaz-mokhtar.netlify.app/)

## Key Features

- 🚀 **Leptos SSR:** Full server-side rendering for instant page loads and maximum SEO visibility.
- 🔍 **SEO & Meta Fidelity:** Integrated metadata for Open Graph, Twitter Cards, and JSON-LD structured data.
- 🦀 **Rust-Powered:** Backend built with Axum; frontend hydrated with high-speed WebAssembly.
- 🌑 **Smart Theme:** System-aware dark/light mode with zero-flash initialization.
- 📱 **Fully Responsive:** Preserved CSS contract from v7 ensuring mobile-first excellence.
- 🧪 **E2E Verified:** Automated testing suite using Playwright for interaction validation.

## Tech Stack

- **Framework:** [Leptos](https://leptos.dev/) (Full-stack Rust)
- **Server:** [Axum](https://github.com/tokio-rs/axum)
- **Rendering:** Server-Side Rendering (SSR) with Hydration
- **Data:** Typed Rust models deserialized from `portfolio.json`
- **Testing:** Playwright for End-to-End verification

## Prerequisites

- [Rust](https://www.rust-lang.org/) (stable)
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos): `cargo install cargo-leptos`
- [wasm32-unknown-unknown](https://rustwasm.github.io/): `rustup target add wasm32-unknown-unknown`

## Getting Started

### Development

Run with hot-reloading:

```bash
cargo leptos watch
```

The site will be available at `http://localhost:3000`.

### Testing

Run the automated end-to-end test suite:

```bash
cd end2end && npm install && npx playwright install chromium
cd ..
cargo leptos end-to-end
```

## Production & Deployment

### Build

Generate optimized artifacts:

```bash
cargo leptos build --release
```

### Deployment Layout

The build produces:

- **Server Binary:** `target/release/portfolio`
- **Static Assets:** `target/site/` (WASM, JS, CSS, and public assets)

### Run Production

```bash
export LEPTOS_SITE_ROOT="target/site"
./target/release/portfolio
```

## Architecture

```text
portfolio-v8/
├── src/
│   ├── app.rs          # Core application shell & routing
│   ├── components/     # Modular section components
│   ├── models/         # Type-safe data structures
│   ├── seo/            # Metadata & JSON-LD management
│   └── interactions.rs # Client-side hydration logic
├── public/             # Static assets (favicons, robots, sitemap)
└── style/              # Global CSS
```

---

## Content Notice

The source code of this repository is licensed under the MIT License.

However, personal content and branding assets are excluded from the MIT License,
including but not limited to:

- Personal photos
- Resume/CV data
- Personal biography
- Logos and branding
- Portfolio project descriptions
- Social/contact information
- Generated media and visual assets

These assets remain All Rights Reserved unless explicitly stated otherwise.

## License

MIT License — see `LICENSE`.
