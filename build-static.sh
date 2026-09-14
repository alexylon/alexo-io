#!/usr/bin/env bash
# Build the deployable static site into ./site_public
#
# This is the build half of deploy.sh — `dx build --ssg`, then prerender.sh,
# then stage the result — with the native axum server build and the systemd
# restart left out. Vercel only ever receives the finished site_public/, so
# this is the single place the site is produced, whether it runs on a
# workstation or in .github/workflows/deploy-vercel.yml.
#
# Usage: ./build-static.sh

set -euo pipefail

PKG="alexo-io"
STAGE_DIR="site_public"

if ! command -v dx >/dev/null 2>&1; then
  echo "[build] dioxus CLI 'dx' not found. Install with: cargo binstall dioxus-cli@0.7.3" >&2
  exit 1
fi

# `dx build --ssg` builds the wasm client bundle and the fullstack server
# binary side by side under target/dx/<pkg>/release/web. We do NOT rely on the
# CLI's own prerender pass — it is unreliable in dioxus 0.7.3. prerender.sh
# drives the freshly built server binary directly instead; see that script.
# dx never prunes stale content-hashed assets from its bundle dir, so an
# incrementally reused target/dx accumulates every past build's .wasm/.css and
# stages all of them. Drop this package's bundle so the output is exactly one
# build; only the bundling step is redone, the cargo cache in target/release
# and target/wasm32-unknown-unknown is untouched.
rm -rf "target/dx/${PKG}"

echo "[build] dx build --release --web --ssg --package ${PKG}"
dx build --release --web --ssg --package "${PKG}"

# Scoped to the package's own dir: target/dx also holds bundles built under
# other app names, and an unscoped `head -n1` can pick the wrong one.
WEBDIR="$(find "target/dx/${PKG}" -type d -path '*/release/web' -not -path '*/public' 2>/dev/null | head -n1 || true)"
if [[ -z "${WEBDIR}" || ! -f "${WEBDIR}/index.html" && ! -d "${WEBDIR}/public" ]]; then
  echo "[build] could not find built web dir under target/dx/${PKG}/*/release/web" >&2
  exit 1
fi
PUBDIR="${WEBDIR}/public"

echo "[build] prerendering static routes (SSG)"
./prerender.sh "${WEBDIR}"

# -------------------------
# Stage static files (build aside, then swap into place)
# -------------------------
# Build the new site in a sibling dir and swap it in with two quick renames,
# same as deploy.sh: on the host that still serves site_public/ directly there
# is then no window where the live dir is empty or half-copied.
STAGE_NEW="${STAGE_DIR}.new"
STAGE_OLD="${STAGE_DIR}.old"
rm -rf "${STAGE_NEW}" "${STAGE_OLD}"
mkdir -p "${STAGE_NEW}"
cp -R "${PUBDIR}/." "${STAGE_NEW}/"

# Copy OG image to a stable path (Manganis hashes asset filenames).
cp frontend/assets/images/og-image.png "${STAGE_NEW}/og-image.png"

# Hand-written pages outside the dx bundle (the /projects redirect). Copied
# after the bundle so they win over anything at the same path.
cp -R site_extra/. "${STAGE_NEW}/"

# Sanity-check the new build before swapping it in — never replace a live site
# with a broken or empty one.
if [[ ! -f "${STAGE_NEW}/index.html" ]]; then
  echo "[build] new build is missing index.html; keeping the current site and aborting." >&2
  rm -rf "${STAGE_NEW}"
  exit 1
fi

# Guard against a non-prerendered bundle slipping through: a successful SSG
# build has real markup inside #main, not the empty "<div id=\"main\"></div>"
# shell. Refuse to ship the empty SPA shell (it would tank SEO).
if grep -q '<div id="main"></div>' "${STAGE_NEW}/index.html"; then
  echo "[build] index.html has an empty #main — prerender did not take. Aborting." >&2
  rm -rf "${STAGE_NEW}"
  exit 1
fi

# Swap: current -> .old, new -> current, then drop .old
if [[ -d "${STAGE_DIR}" ]]; then
  mv "${STAGE_DIR}" "${STAGE_OLD}"
fi
mv "${STAGE_NEW}" "${STAGE_DIR}"
rm -rf "${STAGE_OLD}"

echo "[build] OK — ${STAGE_DIR}/ ready ($(du -sh "${STAGE_DIR}" | cut -f1), $(find "${STAGE_DIR}" -type f | wc -l) files)"
