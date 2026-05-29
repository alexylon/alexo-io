# alexo.io

Full-stack Rust personal website — Dioxus/WASM frontend + axum server.

![](https://github.com/alexylon/alexo.io/actions/workflows/rust.yml/badge.svg)

Live at [alexo.io](https://alexo.io), hosted on a Raspberry Pi.

## Frontend

Dioxus/WASM single-page app — all UI logic in Rust, compiled to WebAssembly.

- **Gruvbox dark/light theme** — detects system preference, persists user choice in IndexedDB
- **Scroll-aware navigation** — direction-sensitive active section highlighting
- **Accessibility**
  - Toolbar keyboard pattern: Tab into nav, `←`/`→` between buttons, Escape to leave
  - `prefers-reduced-motion` respected — disables smooth scrolling, transitions, and animations
  - Focus-visible rings, ARIA labels, semantic HTML
- **SEO** — Open Graph + Twitter Card meta tags, canonical URL
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
frontend/   Dioxus WASM app (UI, components, assets)
server/     axum static file server
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
dx serve --package alexo-io
```

### Production

```bash
./deploy.sh        # build frontend + server, restart the service
./deploy.sh stop   # stop the server
```

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
