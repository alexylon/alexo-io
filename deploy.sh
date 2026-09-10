#!/usr/bin/env bash
# Deploy alexo-io — build frontend + server, then run on port 7777
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
SERVE_ROOT="/srv/alexo"       # what the service actually serves; see alexo.service
PACKAGE="alexo-io"            # cargo package; also names the dir dx builds into

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

# Every path here is relative, and one of them gets removed, so anchor to this
# script's own directory: the deploy must act on this project whatever
# directory it was started from.
cd "$(dirname -- "${BASH_SOURCE[0]}")" || exit 1
PROJECT="$(pwd -P)"

# The directory dx builds into, named after the package rather than searched
# for. A stale sibling from an earlier name still sits under target/dx, and
# picking the first match would deploy the wrong site.
BUILD_WEB="target/dx/${PACKAGE}/release/web"

stop_server() {
  if systemctl is-active --quiet "${SERVICE_NAME}"; then
    sudo systemctl stop "${SERVICE_NAME}"
    print_success "Server stopped."
  else
    print_status "Server is not running."
  fi
}

# -------------------------
# Clearing what the last build left
# -------------------------
# dx adds each build's hashed assets to its output directory and never removes
# the ones before it, so every deploy copied all of them across: 56 wasm
# bundles where one is live, 51 MB where one is enough. Clearing that directory
# first is the fix. Everything about the path is checked before anything goes.
remove_build_output() {
  local target="$1"

  [[ -n "${target}" ]] || {
    print_error "Refusing to remove an empty path."
    exit 1
  }

  # Relative, under target/dx, and with no way back up out of it.
  case "${target}" in
    /* | *..*)
      print_error "Refusing to remove '${target}': not a plain path inside this project."
      exit 1
      ;;
    target/dx/*/release/web) ;;
    *)
      print_error "Refusing to remove '${target}': not a dx build directory."
      exit 1
      ;;
  esac

  # A first build has nothing to clear.
  [[ -e "${target}" ]] || return 0

  # A symbolic link could lead anywhere; only a real directory is removed.
  if [[ -L "${target}" || ! -d "${target}" ]]; then
    print_error "Refusing to remove '${target}': not a plain directory."
    exit 1
  fi

  # The last word belongs to the resolved path: inside this project, and not
  # the project itself.
  local resolved
  resolved="$(cd "${target}" && pwd -P)"
  if [[ "${resolved}" == "${PROJECT}" || "${resolved}" != "${PROJECT}/"* ]]; then
    print_error "Refusing to remove '${resolved}': outside ${PROJECT}."
    exit 1
  fi

  rm -rf "${resolved}"
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

# The site is served by servio, which the systemd unit starts. The unit runs it
# from /usr/local/bin, because the service cannot see /home at all.
if [[ ! -x /usr/local/bin/servio ]]; then
  print_error "/usr/local/bin/servio not found. Install with: sudo cp \"$(command -v servio)\" /usr/local/bin/servio"
  exit 1
fi

# The published site lives outside the repo so the account serving it has no
# read access to anything of yours. This deploy writes there as you; the
# service only ever reads it.
if [[ ! -d "${SERVE_ROOT}/releases" || ! -w "${SERVE_ROOT}/releases" ]]; then
  print_error "${SERVE_ROOT}/releases is missing or not writable. First-time setup:"
  print_error "  sudo useradd --system --no-create-home --shell /usr/sbin/nologin alexo-web"
  print_error "  sudo install -d -o ${USER} -g alexo-web -m 750 ${SERVE_ROOT} ${SERVE_ROOT}/releases"
  exit 1
fi

# -------------------------
# Build frontend (WASM client + fullstack server) and prerender (SSG)
# -------------------------
# Clear first, so what is left is this build and nothing before it. If the
# build then fails, the site keeps serving: only the swap further down touches
# what is live.
print_status "Clearing the last build from ${BUILD_WEB}..."
remove_build_output "${BUILD_WEB}"

# `dx build --ssg` builds both the wasm client bundle and the fullstack server
# binary side by side under target/dx/.../release/web. We do NOT rely on the
# CLI's own prerender pass — it is unreliable in dioxus 0.7.3 (the `dx bundle`
# path has it stubbed out; `dx build --ssg` runs it only intermittently).
# Instead, prerender.sh drives the freshly built server binary directly to
# write the static HTML into public/. See prerender.sh for the why.
print_status "Building Dioxus frontend (client + server)..."
if dx build --release --web --ssg --package "${PACKAGE}"; then
  print_success "Frontend build completed."
else
  print_error "dx build failed."
  exit 1
fi

WEBDIR="${BUILD_WEB}"
if [[ ! -f "${WEBDIR}/index.html" && ! -d "${WEBDIR}/public" ]]; then
  print_error "dx did not build into ${WEBDIR}"
  exit 1
fi
PUBDIR="${WEBDIR}/public"

print_status "Prerendering static routes (SSG)..."
if ./prerender.sh "${WEBDIR}"; then
  print_success "Prerender completed."
else
  print_error "Prerender failed; not deploying a non-prerendered site."
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

# Guard against a non-prerendered bundle slipping through: a successful SSG
# build has real markup inside #main, not the empty "<div id=\"main\"></div>"
# shell. Refuse to deploy the empty SPA shell (it would tank SEO).
if grep -q '<div id="main"></div>' "${STAGE_NEW}/index.html"; then
  print_error "index.html has an empty #main — prerender did not take. Aborting."
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
# Publish into the served directory
# -------------------------
# Copy the finished build to ${SERVE_ROOT} as a new release and point `current`
# at it. Flipping a symlink is one rename, so a visitor mid-deploy gets either
# the previous release or this one, never a half-copied directory. Old releases
# stay for a while: rolling back is repointing the symlink.
publish_release() {
  local release
  release="${SERVE_ROOT}/releases/$(date -u +%Y%m%d-%H%M%S)"

  mkdir -p "${release}"
  cp -R "${STAGE_DIR}/." "${release}/"
  # The serving account is not in your group and owns none of this: it needs
  # the world bits to read the files and walk into the directories.
  chmod -R a+rX "${release}"

  ln -sfn "${release}" "${SERVE_ROOT}/current.new"
  mv -T "${SERVE_ROOT}/current.new" "${SERVE_ROOT}/current"

  # Keep the three most recent; drop the rest.
  ( cd "${SERVE_ROOT}/releases" && ls -1dt -- */ 2>/dev/null | tail -n +4 | xargs -r rm -rf )

  print_success "Published to ${release}"
}

print_status "Publishing to ${SERVE_ROOT}..."
publish_release

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
