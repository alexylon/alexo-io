# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

Three audiences arrive to evaluate, and all three matter:

- **Recruiters and hiring managers** — scanning for stack, seniority, and trajectory, usually fast and often on a phone. They need the shape of the career legible in seconds and the resume PDF reachable without hunting.
- **Prospective clients** — considering Alexander for contract or freelance work. They need proof he ships finished things, not just proof he was employed.
- **Fellow developers and OSS users** — arriving from GitHub, crates.io, Mastodon, or a project link (ferrocrypt.app, clavir.io). They came because of a specific artifact and want to see what else exists.

No audience is assumed to already know him. Every visitor is forming a first judgement.

## Product Purpose

alexo.io is Alexander Alexandrov's personal site: the canonical, findable home for who he is and what he has built.

Success is **credibility, not conversion**. There is no single funnel action to optimise — no lead capture, no signup, no metric. The site succeeds when a visitor who lands here comes away certain they are looking at a serious engineer, and when anyone searching his name finds this page rather than a scattering of third-party profiles. Contact links, the resume PDF, and outbound project links are all present and should stay easy to act on, but none of them is *the* goal; the impression is.

This means quality of execution is not decoration on top of the message — on a credibility surface it **is** the message.

## Positioning

Two things carry the weight, together:

1. **Unusual range and depth.** A career spanning Java, TypeScript/React, and Rust, across enterprise product-modelling systems (DXC's VP/MS and Next Gen Product Accelerator), automotive security tooling (BMW Group), and digital publishing (Proxiad) — plus an academic background most engineers do not have: a PhD in Theology and an MEng in Engineering, on top of a mathematics secondary education. The claim is an uncommon mind, not merely an uncommon stack.
2. **Ships real, published tools.** FerroCrypt, Sofos Code, and clavirio are finished, published artifacts — libraries, CLIs, TUIs, and a desktop app on crates.io with their own homepages — not side experiments or employer work he can only describe secondhand.

Deliberately **not** the lead: "Rust specialist." Rust is a tool he reaches for when it is the right one (his own hero copy says exactly that), and the site's own Rust/WASM implementation is a satisfying detail rather than the argument. Future work must not turn the site into a Rust showcase or a piece of meta-commentary about itself.

## Operating Context

- Single route (`/`) — one continuous page, navigated by scroll with a direction-aware section indicator. Nav sections: Experience, Projects, Skills, Education, Contact.
- Content sections in order: Hero, Experience, Projects, Skills, Education, Certifications, Languages, Contact, Footer (colophon), plus a scroll-to-top affordance.
- Dark/light theme with system-preference detection, persisted to `localStorage`, applied pre-paint so there is no flash.
- Visitors arrive cold from search, from a link in a message or application, or from a project's GitHub/crates.io page.
- The resume PDF (`frontend/assets/docs/Resume_Alexander_Alexandrov.pdf`) travels independently of the site — it gets attached to applications and forwarded.
- Print is a real use case: nav, scroll-to-top, and the resume download are hidden in the print layout.

## Capabilities and Constraints

**Implementation (established, not up for casual revision):**

- Full-stack Rust: Dioxus/WASM frontend, statically pre-rendered to crawlable HTML at build time (SSG) and hydrated in the browser; axum static file server.
- The SSG pipeline is fragile and non-obvious — `prerender.sh` merges the output of two separate `dx` builds because neither produces a complete page alone. See README.md "Why a separate `prerender.sh`?" and the project memory before touching build, `<head>`, or theme-bootstrap behaviour.
- Theme state lives as a class on `<main>` driven by a Dioxus signal, with a pre-paint script injected after the opening `<main>` tag. Theme must not depend on `<head>` scripts (SSR strips them) and must not cause a hydration mismatch.
- Anything using `web-sys`/`js-sys` must be `#[cfg(target_arch = "wasm32")]`-gated with a native fallback, or the pre-render build panics.
- Values that differ between server render and client hydration (year counters, "N+ years") must be computed identically on both targets.
- Styling is hand-written CSS (~34 KB across `index.css` plus the two theme files). No CSS framework.
- Self-hosted on a Raspberry Pi behind a systemd service; deployed via `deploy.sh`.

**Hard constraints — future work must never break these:**

- **Stay light and fast.** Hosted on a Raspberry Pi over a home connection. Payload weight and time-to-paint are real budgets, not preferences. Every added asset, font, or dependency spends from a small account.
- **No tracking, no third parties.** No analytics, no CDN-hosted fonts, no external scripts or embeds. Everything is self-hosted.
- **The resume PDF stays canonical.** It must remain present, downloadable, and consistent with what the page claims.
- **Accessibility is non-negotiable.** See below.

**Undecided / open:**

- Whether the site grows additional routes (writing, per-project pages). Not decided; do not assume either.
- **Decided 2026-09-14: Projects stays a section of the single page, not a separate tab or route.** Five one-sentence entries are too thin to reward a click; the section is the proof for the hero's "open-source Rust tools" claim and belongs on the same page as it; a route would break the printed resume and the scroll-spy nav. `/projects` redirects to `/#projects` so READMEs and crates.io metadata have a clean address. Revisit only when there are more than about eight projects, per-project content that no longer fits one row (screenshots, release notes, write-ups), or a Writing section — then a multi-route site is natural.

## Brand Commitments

- Name and identity: **Alexander Alexandrov** / **Александър Александров** — the Cyrillic form appears in the hero and is part of the identity, not ornament. Domain and handle: alexo.io; GitHub `alexylon`; Mastodon `@lexer@fosstodon.org`; email `hi@alexo.io`.
- Stated title: **Senior Full-Stack Developer**. Based in Sofia, Bulgaria.
- Voice in existing copy: first person, plain, understated, and specific — claims are attached to concrete artifacts and measured outcomes ("reducing query times by 90%"), never to adjectives. No superlatives about himself.
- **Existing copy uses British spelling** consistently ("modernised", "optimised", "practise", "visualisation"). New copy must match.
- The portrait photograph is a deliberate part of the page (expandable to a lightbox), not a placeholder.
- Existing typefaces in the repo: Literata (variable, Latin + Cyrillic, roman and italic), IBM Plex Sans, Atkinson Hyperlegible Mono. All self-hosted and subset. Cyrillic coverage is required, not optional.

## Evidence on Hand

All of the following is real and verifiable — nothing here needs to be invented, and nothing beyond it may be:

- **Employment:** Proxiad (Senior Full-Stack Web Developer, Aug 2023–present); DXC Technology (Full-Stack Developer, Apr 2020–Aug 2023); Camplight (Web Developer Intern, Sep 2019–Apr 2020). Career start Sep 2019 — the "N+ years" figure is computed at render time, never hardcoded.
- **Published projects with live URLs:** FerroCrypt (github, crates.io, ferrocrypt.app), Sofos Code (github, crates.io/crates/sofos), Sofos Web (github), clavirio (github, crates.io, clavir.io), servio (github, crates.io), alexo.io itself (github).
- **Named production work:** AI MCP server in Rust; Java→Rust rewrite of the Next Gen Product Accelerator; an LSP server for the VP/MS modelling language; BMW Group Vulnerability Tool; a 90% query-time reduction (30s → 3s) via PostgreSQL/iBATIS optimisation.
- **Education:** PhD in Theology (Sofia University "St. Kliment Ohridski"); MEng in Engineering (University of Forestry); High School of Mathematics; Camplight Academy (React & JavaScript); MaxPlus (Java Fundamentals).
- **Certification:** Oracle Certified Associate, Java SE 8 Programmer (Jan 2019) — verifiable on Credly.
- **Languages:** Bulgarian (native), English (C1), Italian (B2), Russian (B1), Greek (A2).
- **Assets:** resume PDF, portrait photo, OG image, favicon set, self-hosted font subsets — all under `frontend/assets/`.

**Absent — must not be fabricated:** no testimonials, no client logos, no named references, no case studies, no metrics beyond the one measured figure above, no awards, no speaking history, no availability or rate statements.

## Product Principles

1. **Credibility is earned by specificity.** Every claim on this page points at a shipped thing, a named system, or a measured outcome. Adjectives about himself are not the currency; artifacts are.
2. **Range is the story, not the stack.** The unusual arc — theology and engineering, Java through Rust, insurance through publishing — is the differentiator. Do not flatten it into "Rust developer."
3. **Scannable in ten seconds, rewarding for ten minutes.** A recruiter must extract the shape of the career almost instantly; a developer who keeps reading must find real substance underneath.
4. **The budget is real.** Weight, requests, and dependencies are spent from a Raspberry Pi's account. Restraint here is engineering, not aesthetics.
5. **Independence by default.** Self-hosted, untracked, no third-party runtime dependencies. The site works because of what is in this repository and nothing else.

## Accessibility & Inclusion

Held to **WCAG AA** as a hard requirement, not an aspiration. Already implemented and to be preserved:

- Nav sections are real `#fragment` anchors in the ordinary Tab order, each carrying `aria-current` when active, plus a skip link ahead of them. (An earlier roving-tabindex toolbar with `←`/`→` was replaced: it made the theme toggle unreachable by Tab.)
- `prefers-reduced-motion: reduce` disables smooth scrolling, transitions, and animations.
- Visible focus rings (`:focus-visible`), ARIA labels, semantic landmarks and HTML.
- The portrait opens in a native `<dialog>` via `showModal()`, so the focus trap, Escape handling and focus restoration are the browser's. The element stays mounted and its `open` attribute is the single source of truth — a conditionally-rendered dialog can be closed natively while the app still believes it is open, which strands an invisible full-viewport blocker over the page.
- Atkinson Hyperlegible Mono is in the font stack — a legibility-motivated choice.
- Cyrillic must render correctly wherever it appears; `lang` attributes mark non-English content.
