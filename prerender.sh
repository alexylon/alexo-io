#!/usr/bin/env bash
# Prerender the SSG site by driving the Dioxus fullstack server binary, then
# merge the prerendered <body> into the client build's full-<head> shell.
#
# Why this is not just `dx bundle --ssg`: the dioxus 0.7.3 SSG tooling is split-
# brained, and neither build alone produces a deployable page:
#
#   * `dx build --ssg` runs the server-side renderer. Its output has the real
#     prerendered <body> (good for SEO/first paint) but the SSR renderer rebuilds
#     <head> from the `document::*` rsx elements only — it DROPS the entire
#     template <head> (charset, <title>, OG/Twitter/canonical meta, JSON-LD) AND
#     omits the client WASM bootstrap <script>, so the page never hydrates.
#   * `dx build --web` (client only) keeps the full template <head> + bootstrap,
#     but its <body> is just an empty `<div id="main"></div>` (no prerender).
#   * `dx bundle --ssg`'s own prerender pass is stubbed out in the CLI, so it
#     just emits the empty SPA shell.
#
# So we take the best of both: prerender the <body> via the server binary, then
# splice it into the client shell that already has <head> + bootstrap. The
# result has charset + SEO meta + prerendered content + hydration, all at once.
#
# Usage: ./prerender.sh <web_dir>
#   <web_dir> is target/dx/alexo-io/<profile>/web, where `dx build --ssg` has
#   already produced the fullstack server binary `alexo-io` and a `public/`
#   bundle. This script then runs `dx build --web` itself to regenerate the
#   clean client shell (same wasm hash), so call it AFTER the --ssg build.
#
# On success, <web_dir>/public/index.html is the merged, deployable page.

set -euo pipefail

WEB_DIR_ARG="${1:?usage: prerender.sh <web_dir>}"
PORT="${SSG_PORT:-9988}"
BIN_NAME="alexo-io"
PROFILE="${SSG_PROFILE:-release}"

abs() { (cd "$1" && pwd); }

WEB_DIR="$(abs "$WEB_DIR_ARG")"
BIN="${WEB_DIR}/${BIN_NAME}"
PUBLIC="${WEB_DIR}/public"
INDEX="${PUBLIC}/index.html"

[[ -x "$BIN" ]] || { echo "[prerender] server binary not found at $BIN (run dx build --ssg first)" >&2; exit 1; }
[[ -d "$PUBLIC" ]] || { echo "[prerender] client bundle dir not found at $PUBLIC" >&2; exit 1; }

# Free the port in case a previous run left a server behind.
lsof -nP -tiTCP:"${PORT}" -sTCP:LISTEN 2>/dev/null | xargs -r kill -9 2>/dev/null || true

SERVER_PID=""
SSR_DIR=""
cleanup() {
  if [[ -n "$SERVER_PID" ]]; then kill -9 "$SERVER_PID" 2>/dev/null || true; fi
  if [[ -n "$SSR_DIR" ]]; then rm -rf "$SSR_DIR" 2>/dev/null || true; fi
}
trap cleanup EXIT

# Remove any prerendered index left by `dx build --ssg`'s own pass BEFORE
# starting the server. The incremental renderer seeds its in-memory cache from
# the on-disk file at startup and APPENDS to it when the file came from a
# different build, concatenating two copies of the page. Deleting up front gives
# each route a clean single write.
find "${PUBLIC}" -name index.html -type f -delete 2>/dev/null || true

# Start the fullstack server from its own dir so current_exe().parent()/public
# resolves to this bundle's public/.
echo "[prerender] starting server on 127.0.0.1:${PORT}"
( cd "$WEB_DIR" && PORT="$PORT" IP=127.0.0.1 DIOXUS_CLI_ENABLED=true ./"$BIN_NAME" ) \
  >/tmp/ssg_prerender.log 2>&1 &
SERVER_PID=$!

# Wait for the static_routes endpoint (up to ~10s).
ROUTES_JSON=""
for _ in $(seq 1 20); do
  if ROUTES_JSON="$(curl -fs -m2 -X POST "http://127.0.0.1:${PORT}/api/static_routes" -d '{}' 2>/dev/null)"; then
    [[ -n "$ROUTES_JSON" ]] && break
  fi
  sleep 0.5
done
if [[ -z "$ROUTES_JSON" ]]; then
  echo "[prerender] server never answered /api/static_routes; log:" >&2
  cat /tmp/ssg_prerender.log >&2 || true
  exit 1
fi
echo "[prerender] static routes: ${ROUTES_JSON}"
ROUTES="$(printf '%s' "$ROUTES_JSON" | tr -d '[]" ' | tr ',' '\n')"

# Capture each route's prerendered HTML from the server into a temp dir. We read
# the response body directly (rather than the file the renderer writes) so we
# control exactly what gets spliced.
SSR_DIR="$(mktemp -d)"

while IFS= read -r route; do
  [[ -z "$route" ]] && continue
  echo "[prerender] rendering ${route}"
  rel="${route#/}"
  out="${SSR_DIR}/${rel:+$rel/}index.html"
  mkdir -p "$(dirname "$out")"
  curl -fs -m30 -H 'Accept: text/html' "http://127.0.0.1:${PORT}${route}" -o "$out" \
    || { echo "[prerender] failed to render ${route}" >&2; exit 1; }
done <<< "$ROUTES"

# Stop the server before rebuilding the shell.
kill -9 "$SERVER_PID" 2>/dev/null || true
SERVER_PID=""

# Regenerate the client shell: full template <head> + bootstrap + empty #main.
# Same crate/profile → same wasm asset hash as the --ssg build, so the shell's
# bootstrap and the assets in public/ stay consistent.
echo "[prerender] rebuilding client shell (dx build --web)"
dx build --"${PROFILE}" --web --package alexo-io >/tmp/ssg_shell_build.log 2>&1 \
  || { echo "[prerender] client shell build failed; log:" >&2; tail -20 /tmp/ssg_shell_build.log >&2; exit 1; }

# Splice each route's prerendered <body> content into the shell's empty #main.
while IFS= read -r route; do
  [[ -z "$route" ]] && continue
  rel="${route#/}"
  ssr="${SSR_DIR}/${rel:+$rel/}index.html"
  shell="${PUBLIC}/${rel:+$rel/}index.html"
  [[ -f "$ssr" ]] || { echo "[prerender] missing SSR capture for ${route}" >&2; exit 1; }
  # For non-root routes the shell file may not exist yet (client build only
  # emits index.html); copy the shell as a template for them.
  [[ -f "$shell" ]] || { mkdir -p "$(dirname "$shell")"; cp "${INDEX}" "$shell"; }
  SSR_FILE="$ssr" SHELL_FILE="$shell" python3 - "$route" <<'PY'
import os, re, sys
route = sys.argv[1]
ssr = open(os.environ["SSR_FILE"], encoding="utf-8").read()
shell = open(os.environ["SHELL_FILE"], encoding="utf-8").read()

# The client shell and the SSR output have COMPLEMENTARY heads:
#   * shell <head>: charset + all SEO meta + the WASM bootstrap <script>, but NO
#     stylesheet <link>s (in the SPA, WASM injects them at runtime).
#   * SSR <head>: the stylesheet/icon <link>s (manganis-hashed), but no meta/SEO.
# A merged static page needs BOTH the SEO head AND the stylesheet links present
# up front (the prerendered body must be styled before WASM boots). So we splice
# the SSR's <link> tags into the shell head, and the SSR body into the shell.

# 1) Collect the SSR's stylesheet + icon links (deduped, preserving order — the
#    SSR output lists them twice).
seen, links = set(), []
for tag in re.findall(r'<link\b[^>]*>', ssr):
    if ('stylesheet' in tag) or ('rel="icon"' in tag) or ('apple-touch-icon' in tag):
        if tag not in seen:
            seen.add(tag)
            links.append(tag)
if not any('stylesheet' in t for t in links):
    sys.exit(f"no stylesheet <link> found in SSR output for {route}")

# 2) Pull the inner HTML of the SSR's <div id="main">...</div>.
m = re.search(r'<div id="main">(.*)</div>\s*</body>', ssr, re.S) \
    or re.search(r'<div id="main">(.*)</div>', ssr, re.S)
if not m:
    sys.exit(f"could not find #main content in SSR output for {route}")
inner = m.group(1)

# 3) Inject the links just before </head> (skip any the shell already has).
if '</head>' not in shell:
    sys.exit("shell has no </head>")
to_add = [t for t in links if t not in shell]
shell = shell.replace('</head>', "    " + "\n    ".join(to_add) + "\n</head>", 1)

# 4) Splice the prerendered body into the shell's empty #main.
if '<div id="main"></div>' not in shell:
    sys.exit(f"shell has no empty #main to fill for {route}")
shell = shell.replace('<div id="main"></div>', f'<div id="main">{inner}</div>', 1)

# 5) No-flash: the prerendered <main> is hardcoded to "theme-light", so a dark
#    visitor would see light until WASM hydrates and the signal corrects it. Set
#    the right class up front with a tiny script placed IMMEDIATELY after the
#    opening <main> tag — it runs the instant <main> is parsed, before its
#    contents paint, and sets the exact same class the Dioxus signal will (so no
#    hydration mismatch). CSS keys off body:has(main.theme-X), so swapping the
#    class resolves the whole theme (bg + all custom props), no !important.
PRE_PAINT = (
    "<script>(function(){try{var s=localStorage.getItem('theme');"
    "var d=s?s==='dark':(window.matchMedia&&"
    "window.matchMedia('(prefers-color-scheme:dark)').matches);"
    "var m=document.currentScript.parentElement;"
    "m.classList.remove('theme-light','theme-dark');"
    "m.classList.add(d?'theme-dark':'theme-light');}catch(e){}})();</script>"
)
shell, n = re.subn(r'(<main\b[^>]*>)', r'\1' + PRE_PAINT, shell, count=1)
if n != 1:
    sys.exit(f"could not find <main> to insert the pre-paint theme script for {route}")

open(os.environ["SHELL_FILE"], "w", encoding="utf-8").write(shell)
PY
  echo "[prerender] merged ${route} (SEO head + stylesheets + prerendered body + no-flash theme)"
done <<< "$ROUTES"

# ---- Validate the merged index.html ----
fail() { echo "[prerender] $1" >&2; exit 1; }

grep -q '<meta charset' "${INDEX}" || fail "merged index.html missing <meta charset> — encoding would break."
grep -q '<title>' "${INDEX}"       || fail "merged index.html missing <title> — head not preserved."
grep -q 'og:title' "${INDEX}"      || fail "merged index.html missing OG meta — SEO head not preserved."
grep -q 'rel="stylesheet"' "${INDEX}" || fail "merged index.html missing stylesheet links — page would render unstyled."
grep -q 'type="module"' "${INDEX}" || fail "merged index.html missing wasm bootstrap — would not hydrate."
grep -q "localStorage.getItem('theme')" "${INDEX}" || fail "merged index.html missing the no-flash theme script."
grep -q 'id="main"' "${INDEX}"     || fail "merged index.html missing #main."
grep -qE '<div id="main"></div>' "${INDEX}" && fail "merged index.html still has an empty #main — body not spliced."
# Real content actually present?
grep -q 'FerroCrypt' "${INDEX}"    || fail "merged index.html has no prerendered content (expected project names)."
# Exactly one <body> — guard against duplicated/concatenated renders.
body_count="$(grep -oc '<body' "${INDEX}" || true)"
[[ "${body_count}" == "1" ]] || fail "merged index.html has ${body_count} <body> tags — duplicated render."

echo "[prerender] OK — $(wc -c < "${INDEX}" | tr -d ' ') bytes; head + body + bootstrap all present in ${INDEX}"
