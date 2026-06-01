# alexo.io

Full-stack Rust personal website — statically pre-rendered Dioxus/WASM frontend + axum server.

![](https://github.com/alexylon/alexo-io/actions/workflows/rust.yml/badge.svg)

Live at [alexo.io](https://alexo.io), hosted on a Raspberry Pi.

Dioxus app written entirely in Rust, **pre-rendered to static HTML** at build
time (SSG) and hydrated to WebAssembly in the browser.

- **Static site generation** — every route is pre-rendered to crawlable HTML, so
  search engines and link-preview bots see the full content (not an empty SPA
  shell), and the page paints before the WASM bundle loads. The client then
  hydrates it into the interactive app.
- **Ink & Ember dark/light theme** — detects system preference, persists user
  choice in `localStorage`. The theme class lives on `<main>`, driven by a Dioxus
  signal. A tiny pre-paint script (injected during pre-render) applies the saved
  theme before the page paints, so there's no flash on reload.
- **Scroll-aware navigation** — direction-sensitive active section highlighting
- **Accessibility**
  - Toolbar keyboard pattern: Tab into nav, `←`/`→` between buttons, Escape to leave
  - `prefers-reduced-motion` respected — disables smooth scrolling, transitions, and animations
  - Focus-visible rings, ARIA labels, semantic HTML
- **SEO** — pre-rendered content + Open Graph / Twitter Card meta tags, JSON-LD
  structured data, canonical URL
- **Print-friendly** — nav, scroll-to-top, and resume download hidden in print layout

## Server

axum static file server with production and development modes.

- **Compression** — gzip and Brotli response compression
- **SPA fallback** — serves `index.html` for unmatched routes
- **Cache-Control** — immutable long-lived caching for content-hashed assets, `no-cache` for everything else
- **Security headers** — `X-Content-Type-Options`, `X-Frame-Options`, `Referrer-Policy`
- **Live reload** — file watcher with debounced browser refresh (disabled in production with `--no-reload`)

## Project structure

```
frontend/      Dioxus app (UI, components, assets) — fullstack/SSG build
server/        axum static file server
prerender.sh   drives the SSG pre-render (see Production)
deploy.sh      build + pre-render + stage + restart the service
```

## Getting started

### Prerequisites

1. [Install Rust](https://www.rust-lang.org/tools/install)

2. Install Dioxus CLI:

```bash
cargo install cargo-binstall
cargo binstall dioxus-cli
```

### Development

```bash
dx serve --web --package alexo-io
```

Then open <http://127.0.0.1:8080>. The `--web` flag is required: the frontend is
a fullstack crate with no default platform feature, so `dx` can't infer the
target without it. The dev server renders on the fly (no separate pre-render
step) and hot-reloads on change — press `r` to rebuild, `ctrl+c` to quit.

### Production

```bash
./deploy.sh        # build + pre-render (SSG) + stage + restart the service
./deploy.sh stop   # stop the server
```

`deploy.sh` runs `dx build --release --web --ssg` (which produces the fullstack
server binary), then invokes `prerender.sh` to generate the static HTML, stages
the result into `site_public/`, and restarts the service.

> **Why a separate `prerender.sh`?** Neither of Dioxus 0.7's build commands
> produces a complete page on its own, so `prerender.sh` combines them:
>
> - `dx build --ssg` runs the server-side renderer. Its output has the real
>   pre-rendered `<body>` and the stylesheet links, but the SSR renderer rebuilds
>   `<head>` from the `document::*` elements only — it **drops the template
>   `<head>`** (charset, `<title>`, Open Graph / Twitter / canonical meta,
>   JSON-LD) and **omits the WASM bootstrap script**, so the page never hydrates.
> - `dx build --web` (client only) keeps the full `<head>` and the bootstrap
>   script, but its `<body>` is an empty `<div id="main">` and it carries no
>   stylesheet links.
> - `dx bundle --ssg`'s own pre-render pass is stubbed out in the CLI, so it just
>   emits the empty SPA shell.
>
> `prerender.sh` runs the server binary, asks `/api/static_routes` which routes
> exist, and captures each one's server-rendered HTML. It then rebuilds the
> client shell and splices in the stylesheet links and pre-rendered `<body>`,
> plus a small script after `<main>` that applies the saved theme before the
> first paint (so there's no flash). The finished page has the full `<head>`,
> stylesheets, pre-rendered content, and hydration in one file. Before exiting,
> the script checks that all of those are present and that there's exactly one
> `<body>`, so an incomplete merge never gets deployed.

The server runs as a systemd service (`alexo.service` in the repo root) so it
starts on boot and restarts automatically if it crashes. `deploy.sh` rebuilds
the site and runs `systemctl restart alexo`. First-time setup on the host:

```bash
sudo cp alexo.service /etc/systemd/system/
sudo systemctl enable --now alexo
```

Inspect with `systemctl status alexo` and `journalctl -u alexo -f`.

## Server options

```
server --port 7777 --dir ./site_public --no-reload
```

| Flag | Default | Description |
|------|---------|-------------|
| `--port`, `-p` | `3030` | Port to listen on |
| `--dir`, `-d` | `.` | Directory to serve |
| `--no-reload` | off | Disable live reload and file watching |

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)
