# Design Summary — escher

_Distilled from `.andromeda/design-system.md` + `.andromeda/layout-templates.md` (adopted reading). wrap-session's cascade re-derives it when either plan changes._

## Brand identity
> NO RECORDED INTENT — escher has no brand or project-wide token set; the colours below are per-app values as built.

## Key design tokens (as built — no shared token set)

### Colour (selected)
| Where | Value | Use |
|---|---|---|
| seven_guis (the stand) | `#4a6cf7` (hover `#3a5ce5`, active `#2a4cd3`) | accent, on page `#f5f5f5`, text `#1a1a1a` |
| seven_guis invalid | `#e53e3e` / `#fff5f5` / `#c53030` | flight-booker invalid state |
| seven_guis success | `#ebf8ee` / `#68d391` / `#276749` | success message |
| blitz-dom UA stylesheet | links `rgb(0,0,238)`, input focus outline `#4D90FE`, button `#EFEFEF` | engine defaults |
| Engine selection highlight | rgb 180, 213, 255 | default text selection |
| Browser chrome | focus `#5E9ED6`, tabstrip `#E0E0E0` | reference browser |

- Colour model `AlphaColor<Srgb>`; viewport colour scheme `Light` (default) / `Dark` feeds `prefers-color-scheme`; a scheme change recascades everything.
- Default scrollbar thumbs follow the scheme and paint fill + thin contrast stroke.

### Typography · spacing · depth · radius · motion
> NOT YET MEASURED / NO RECORDED INTENT — no project-wide type scale, spacing unit, elevation, radius or motion tokens. Engine facts: default generic size 16px (13px monospace); WASM builds bundle DejaVu Sans for every generic family, and so does the native headless stand (system fonts off, decoded by seven_guis' `woff` feature).

## Primary surfaces
- **desktop-native** — winit windows; Dioxus apps via `dioxus_native::launch`; seven_guis Home (640px card column) and TaskShell (header + scrolling body) are the stand's screens; the headless stand skips Home and mounts one lean task in TaskShell under `main#main` at the pinned 800 × 600 Light viewport; each lean task's controls and value displays carry author `id`s (their stable element ids), adding no class, style, wrapper or order.
- **cli** — WPT runner (owo-colors status words + colour), `paint_bench`, `screenshot`, `bump`, and `scripts/agent-run.sh` (the agent-run test contract: usage on stderr, JSON lines only on stdout), `scripts/cold-agent.sh` (the cold-agent run pipe, same CLI shape); escher's driver CLI (uncoloured JSON on stdout) is a route chunk.
- **web-spa** — WASM canvas filling the body (seven_guis, todomvc), wasm_hello 640px card.
- **mobile-native** — Android browser APK (built, not tested in CI).

## Component patterns
- Each Dioxus document starts as `<html><head></head><body><main id="main"></main></body></html>`; the app mounts into `main`.
- Viewport = window surface minus safe-area insets; the test harness defaults to 800×600 @1 light.
- Devtools overlays (content/padding/border/margin) and layout outlines exist as `DevtoolSettings`.
- Browser controls are clickable `div`s, not buttons (an a11y gap noted in a11y-plan §4).

## Universal bans
> NO RECORDED INTENT.

---

**Full plans:** `.andromeda/design-system.md` + `.andromeda/layout-templates.md`.
