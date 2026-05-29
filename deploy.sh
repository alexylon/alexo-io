#!/usr/bin/env bash
# Deploy alexan.dev — build frontend + server, then run on port 7777
#
# Usage:
#   ./deploy.sh          Build and start the server
#   ./deploy.sh stop     Stop the running server

# -------------------------
# Config (edit as needed)
# -------------------------
PORT=7777
SERVICE_NAME="alexo"          # systemd unit name (see /etc/systemd/system/alexo.service)
STAGE_DIR="site_public"
SERVER_BIN="target/release/server"

# -------------------------
# Pretty output helpers
# -------------------------
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status()   { echo -e "${BLUE}[INFO]${NC} $1"; }
print_success()  { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
print_error()    { echo -e "${RED}[ERROR]${NC} $1"; }

set -euo pipefail

stop_server() {
  if systemctl is-active --quiet "${SERVICE_NAME}"; then
    sudo systemctl stop "${SERVICE_NAME}"
    print_success "Server stopped."
  else
    print_status "Server is not running."
  fi
}

# -------------------------
# Handle "stop" command
# -------------------------
if [[ "${1:-}" == "stop" ]]; then
  stop_server
  exit 0
fi

# -------------------------
# Check prerequisites
# -------------------------
if ! command -v dx >/dev/null 2>&1; then
  print_error "dioxus CLI 'dx' not found. Install with: cargo install dioxus-cli"
  exit 1
fi

# -------------------------
# Build frontend (WASM)
# -------------------------
print_status "Bundling Dioxus frontend..."
if dx bundle --release --package alexo-io; then
  print_success "Frontend bundle completed."
else
  print_error "dx bundle failed."
  exit 1
fi

PUBDIR="$(find target/dx -type d -path '*/release/web/public' 2>/dev/null | head -n1 || true)"
if [[ -z "${PUBDIR}" ]]; then
  print_error "Could not find bundled public dir under target/dx/*/release/web/public"
  exit 1
fi

# -------------------------
# Build server (native)
# -------------------------
print_status "Building server..."
if cargo build --release -p server; then
  print_success "Server build completed."
else
  print_error "Server build failed."
  exit 1
fi

if [[ ! -f "${SERVER_BIN}" ]]; then
  print_error "Server binary not found at ${SERVER_BIN}"
  exit 1
fi

# -------------------------
# Stage static files (build aside, then swap into place)
# -------------------------
# Build the new site in a sibling dir so the running server keeps serving the
# current one untouched during the slow copy, then swap it in with two quick
# renames. Avoids the window where the live dir is empty/half-copied mid-deploy.
STAGE_NEW="${STAGE_DIR}.new"
STAGE_OLD="${STAGE_DIR}.old"
rm -rf "${STAGE_NEW}" "${STAGE_OLD}"
mkdir -p "${STAGE_NEW}"
cp -R "${PUBDIR}/." "${STAGE_NEW}/"

# Copy OG image to a stable path (Manganis hashes asset filenames)
cp frontend/assets/images/og-image.png "${STAGE_NEW}/og-image.png"

# Sanity-check the new build before swapping it in — never replace a live site
# with a broken or empty one.
if [[ ! -f "${STAGE_NEW}/index.html" ]]; then
  print_error "New build missing index.html; keeping current site and aborting."
  rm -rf "${STAGE_NEW}"
  exit 1
fi

# Swap: current -> .old, new -> current, then drop .old
if [[ -d "${STAGE_DIR}" ]]; then
  mv "${STAGE_DIR}" "${STAGE_OLD}"
fi
mv "${STAGE_NEW}" "${STAGE_DIR}"
rm -rf "${STAGE_OLD}"

print_success "Staged static files to ./${STAGE_DIR}"

# -------------------------
# Restart server (managed by systemd)
# -------------------------
# `systemctl restart` stops the old instance, waits for it to fully exit and
# release the port, then starts the new one — no manual stop/start race.
print_status "Restarting ${SERVICE_NAME} service..."
sudo systemctl restart "${SERVICE_NAME}"

sleep 1
if systemctl is-active --quiet "${SERVICE_NAME}"; then
  print_success "Site is live at http://localhost:${PORT}"
  print_status "Logs:   journalctl -u ${SERVICE_NAME} -f"
  print_status "Status: systemctl status ${SERVICE_NAME}"
  print_status "Stop:   ./deploy.sh stop"
else
  print_error "Service failed to start. Check: journalctl -u ${SERVICE_NAME} -e"
  exit 1
fi
