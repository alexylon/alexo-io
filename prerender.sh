#!/usr/bin/env bash
# Prerender the SSG site by driving the Dioxus fullstack server binary, then
# merge the prerendered <body> into the client build's full-<head> shell.
#
# Neither dx build alone produces a deployable page:
#
#   * `dx build --ssg` gives a real prerendered <body>, but rebuilds <head> from
#     the `document::*` elements only. It drops the template <head> — charset,
#     title, meta, JSON-LD — and the WASM bootstrap, so the page never hydrates.
#   * `dx build --web` keeps the full <head> and the bootstrap, but its <body>
#     is an empty `<div id="main"></div>`.
#   * `dx bundle --ssg`'s own prerender pass is stubbed out in the CLI.
#
# So this takes the body from the first and the shell from the second.
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

# Must happen before the server starts. The incremental renderer seeds its
# cache from the file on disk and appends to it when the build differs, which
# concatenates two copies of the page.
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

# Read the response body directly rather than the file the renderer writes, so
# what gets spliced is exactly what the server produced.
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

# Same crate and profile, so the wasm asset hash matches the --ssg build and
# the shell's bootstrap stays consistent with the assets in public/.
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
  # The client build only emits index.html, so other routes need a copy.
  [[ -f "$shell" ]] || { mkdir -p "$(dirname "$shell")"; cp "${INDEX}" "$shell"; }
  SSR_FILE="$ssr" SHELL_FILE="$shell" python3 - "$route" <<'PY'
import os, re, sys
route = sys.argv[1]
ssr = open(os.environ["SSR_FILE"], encoding="utf-8").read()
shell = open(os.environ["SHELL_FILE"], encoding="utf-8").read()

# The two heads are complementary: the shell has the meta and the bootstrap but
# no stylesheet links, the SSR output has the links but no meta. The merged page
# needs both, since the prerendered body must be styled before WASM boots.

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

# 1b) The @font-face rules come from a `document::Style`, which lands in <head>
#     as a <style> rather than a <link>. Without this they never reach the
#     merged page and the site falls back to system fonts.
styles = []
for tag in re.findall(r'<style\b[^>]*>.*?</style>', ssr, re.S):
    if tag not in seen:
        seen.add(tag)
        styles.append(tag)
if not any('@font-face' in t for t in styles):
    sys.exit(f"no @font-face <style> found in SSR output for {route}")

# 2) Pull the inner HTML of the SSR's <div id="main">...</div>.
m = re.search(r'<div id="main">(.*)</div>\s*</body>', ssr, re.S) \
    or re.search(r'<div id="main">(.*)</div>', ssr, re.S)
if not m:
    sys.exit(f"could not find #main content in SSR output for {route}")
inner = m.group(1)

# 3) Inject the links and head styles just before </head> (skip any the shell
#    already has). Styles go after the links so the stylesheets load first.
if '</head>' not in shell:
    sys.exit("shell has no </head>")
to_add = [t for t in links + styles if t not in shell]
shell = shell.replace('</head>', "    " + "\n    ".join(to_add) + "\n</head>", 1)

# 4) Splice the prerendered body into the shell's empty #main.
if '<div id="main"></div>' not in shell:
    sys.exit(f"shell has no empty #main to fill for {route}")
shell = shell.replace('<div id="main"></div>', f'<div id="main">{inner}</div>', 1)

# 5) The prerendered <main> is always "theme-light", so a dark visitor would
#    see light until WASM hydrates. This script sits immediately after the
#    opening <main> tag and runs before its contents paint, setting the same
#    class the Dioxus signal will. CSS keys off body:has(main.theme-X), so the
#    swap resolves the whole theme.
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
grep -q '@font-face' "${INDEX}"    || fail "merged index.html missing @font-face rules — page would fall back to system fonts."
# All six faces must survive. A short count means one was parsed away.
face_count="$(grep -o '@font-face' "${INDEX}" | wc -l | tr -d ' ')"
[[ "${face_count}" == "6" ]] || fail "merged index.html has ${face_count} @font-face rules, expected 6 — a face was dropped."
grep -q 'type="module"' "${INDEX}" || fail "merged index.html missing wasm bootstrap — would not hydrate."
grep -q "localStorage.getItem('theme')" "${INDEX}" || fail "merged index.html missing the no-flash theme script."
grep -q 'id="main"' "${INDEX}"     || fail "merged index.html missing #main."
grep -qE '<div id="main"></div>' "${INDEX}" && fail "merged index.html still has an empty #main — body not spliced."
# Real content actually present?
grep -q 'FerroCrypt' "${INDEX}"    || fail "merged index.html has no prerendered content (expected project names)."
# The address may only appear XOR-encoded (email_link.rs). The résumé PDF is exempt.
email_hex="$(grep -o 'data-email="[0-9a-f]*"' "${INDEX}" | head -n1 | cut -d'"' -f2 || true)"
[[ -n "${email_hex}" ]] || fail "merged index.html has no encoded email link."
email="$(python3 -c 'import sys; b = bytes.fromhex(sys.argv[1]); print(bytes(x ^ b[0] for x in b[1:]).decode())' "${email_hex}")" \
  || fail "could not decode data-email=\"${email_hex}\"."
leaks="$(grep -rlaF --exclude='*.pdf' "${email}" "${PUBLIC}" || true)"
[[ -z "${leaks}" ]] || fail "the email address is in plain text in: ${leaks}"
links="$(grep -o 'data-email="' "${INDEX}" | wc -l | tr -d ' ' || true)"
decoders="$(grep -ozE '</a>(\s|<!--[^>]*-->)*<script[^>]*>[^<]*previousElementSibling' "${INDEX}" | tr -cd '\0' | wc -c | tr -d ' ' || true)"
[[ "${links}" == "${decoders}" ]] || fail "${links} encoded email links but ${decoders} decoder scripts right after them."
# Exactly one <body> — guard against duplicated/concatenated renders.
body_count="$(grep -oc '<body' "${INDEX}" || true)"
[[ "${body_count}" == "1" ]] || fail "merged index.html has ${body_count} <body> tags — duplicated render."

echo "[prerender] OK — $(wc -c < "${INDEX}" | tr -d ' ') bytes; head + body + bootstrap all present in ${INDEX}"
