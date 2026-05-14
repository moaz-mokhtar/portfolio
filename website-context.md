# Portfolio v8 — Website Context

## 1. Project Overview

- **Moaz bin Mokhtar** personal portfolio website (v8)
- Showcases professional experience, skills, projects, and contact information
- Single-page scroll-based portfolio, server-side rendered with client hydration
- Live at `https://moaz-mokhtar.netlify.app/`

## 2. Tech Stack

- **Framework:** Leptos v0.8.0 (Rust WASM framework, isomorphic SSR+hydration)
- **Router:** leptos_router v0.8.0 (single route `/`)
- **Meta/SEO:** leptos_meta v0.8.0
- **HTTP Server:** Axum v0.8.0 (Tokio async)
- **Styling:** Hand-written CSS (~618 lines), SCSS compiled via dart-sass through cargo-leptos
- **Build:** cargo-leptos (cargo-leptos build --release for prod)
- **Hosting:** Netlify (manual deploy, no CI/CD config in repo)
- **E2E Tests:** Playwright (TypeScript, 4 tests)
- **Package Manager:** cargo (Rust), npm (Playwright)

## 3. Project Structure

```
portfolio-v8/
├── Cargo.toml              # Rust manifest + cargo-leptos build config
├── public/                 # Static assets (favicons, images, robots.txt, sitemap.xml)
│   └── data/portfolio.json # All portfolio content (compile-time embedded)
├── style/
│   ├── main.scss           # Minimal placeholder
│   └── main.css            # Full hand-written CSS (618 lines)
├── src/
│   ├── main.rs             # SSR server entry (Axum)
│   ├── lib.rs              # WASM hydration entry
│   ├── app.rs              # App shell + HomePage component + routing
│   ├── interactions.rs     # Client-side JS (theme toggle, mobile nav, active nav scroll)
│   ├── models/
│   │   ├── mod.rs
│   │   └── portfolio.rs   # Data models + include_str! JSON loading
│   ├── seo/
│   │   ├── mod.rs
│   │   └── meta.rs         # SEO component (title, OG, JSON-LD, Twitter Card)
│   └── components/
│       ├── mod.rs
│       ├── basmala.rs      # Islamic "Bismillah"
│       ├── navbar.rs       # Sticky nav, desktop links + mobile hamburger
│       ├── hero.rs         # Hero: name, tagline, contact links, background image
│       ├── profile.rs      # About me (list of statements)
│       ├── skills.rs       # Skills grouped by category (CSS grid)
│       ├── experience.rs   # Work history cards
│       ├── showcases.rs    # Project showcase cards
│       ├── contact.rs      # Contact links
│       └── footer.rs       # Simple footer
└── end2end/
    └── tests/interactions.spec.ts  # E2E tests
```

## 4. Rendering & Architecture

- **Isomorphic SSR + Hydration:** Axum server renders full HTML; WASM bundle hydrates on client
- **Single route:** Only `/` exists; all navigation is anchor-based (`#profile`, `#skills`, etc.) with native smooth scrolling
- **Component tree:** `Navbar` → `Basmala` → `Hero` → `Profile` → `Showcases` → `Skills` → `Experience` → `Contact` → `Footer`
- **Data flow:** Top-down props from `HomePage` to section components; no reactive signals or resources
- **Client JS** (`interactions.rs`): Theme toggling (dark/light with localStorage persistence), mobile nav hamburger, active nav link highlight on scroll

## 5. Data Sources

- **Single static JSON** (`public/data/portfolio.json`) embedded at compile time via `include_str!`
- Deserialized with `serde_json` into a `LazyLock<PortfolioData>` global
- Schema: `profile`, `skills` (grouped), `experience` (with bullets), `showcases` (with stack + bullets), `contact` (platform + URL + icon)
- No APIs, databases, CMS, or runtime data fetching

## 6. Styling System

- **Pure hand-written CSS** in `style/main.css` (618 lines)
- CSS custom properties for theming: dark mode default, `[data-theme="light"]` overrides
- Color tokens: `--bg`, `--surface`, `--text`, `--text-secondary`, `--primary`, `--border`
- Responsive breakpoints: 640px, 768px, 500px
- Staggered fade-in animation on sections (30ms delays)
- CSS Grid for skills (2 cols → 1 col at 500px)
- Card pattern for experience + showcases
- No Tailwind, Bootstrap, or CSS framework

## 7. Important Features

- Dark/light theme toggle with localStorage persistence
- Mobile responsive hamburger navigation
- Smooth scroll anchor navigation with active section tracking
- Accessibility: aria-labels on interactive elements, semantic HTML
- PWA manifest (`site.webmanifest`)

## 8. SEO & Performance

- `<title>`, canonical URL, meta description
- Open Graph tags (title, description, image, url, type)
- Twitter Card (`summary_large_image`)
- JSON-LD structured data (Schema.org `Person`)
- `robots.txt` (allow all, sitemap reference)
- `sitemap.xml` (single URL, monthly, priority 1.0)
- `og-image.png` for social sharing
- No lazy loading or image optimization implemented
- No caching strategy beyond default HTTP

## 9. Current Problems or Limitations

- **Duplicate `render_icon`** — same SVG icon function in `hero.rs` and `contact.rs`
- **No dynamic routes** — single-page limits expansion (no individual project pages)
- **Data rebuild required** — `include_str!` means any content change needs recompilation
- **No image optimization** — `hero.jpg` served as-is, no responsive images
- **No CI/CD** — deployment is manual via Netlify UI
- **No error handling** — JSON deserialization `expect()` panics on malformed data
- **Vestigial `main.scss`** — placeholder file overridden by `main.css`
- **Hardcoded canonical URL** — points to `moaz-mokhtar.netlify.app`, not environment-aware
- **end2end URL hardcoded** — `http://127.0.0.1:3000/` instead of Playwright baseURL config

## 10. Migration Goals

- Not explicitly stated; project appears stable and in active use
- Likely interests: better content maintainability (runtime JSON fetch), image optimization, CI/CD pipeline, multi-page expansion (project detail pages), improved performance/Lighthouse scores
