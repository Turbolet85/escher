@history_item n=1 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — set the shot's answer line (`{"width","height","png"}`, uncoloured, no ESC, status 0, MCP as an `image` block then a text block) and the `fonts-unavailable` refusal with nothing painted; the founder's fork 1. This is the source of the picked rows `contract/escher-session-commands/screenshot`, `decision/screenshot-never-skips-text` and `contract/driver-screenshot/fonts-unavailable`. Boundary: the canvas-shot measurement must read pixels only as this line already carries them, with no new field or refusal.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand registers the bundled DejaVu Sans for every generic with system fonts off on native (`build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`), so that layout is the same on the dev host and the CI runner. This is the premise the scope questions: the no-shot cross-OS basis was read before the bundled fonts. A canvas fixture booted alone must register the same face, or the shot is refused `fonts-unavailable` (`decision/no-generic-family-lays-out-sans`).

@history_item n=3 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (Typography item) — extended the bundled DejaVu Sans to every stand boot (`boot_cells`, `boot_load`). This is the precedent for a new stand boot, such as the canvas fixture, taking the bundled font so that its text and glyph coverage do not depend on the host.

@history_item n=4 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — moved the stand's font registration out of design-system into architecture §Standard Contracts → Headless stand, beside `font_ctx()`. The WASM bundled-font item and the custom `FontContext` item (`contract/native-config/font-context`) stayed. A canvas fixture's font contract is therefore stated in architecture, not here.

@history_item n=5 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (F2's stand-row move ratified) — the founder's own word ratified the stand-row moves, the font registration among them, and stand-only values left design-system for layout-templates. A canvas stand fixture's own colours and sizes belong in layout-templates or architecture, not in design-system rows.

@history_item n=6 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography) — `+ decision/unavailable-families-use-default-font`: text in an unavailable family lays out in the engine's default font, ratified as upstream's. With `decision/no-generic-family-lays-out-sans`, this decides which face a canvas's text actually rasterizes in on each host.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a group in the open list, a header heavier than a row) — the first synthesized-bold header read red on macOS alone and was replaced, and `hint` stays `!FONT_EMBOLDEN_ENABLED`. This is a measured precedent that text raster output differs per host (an emboldening build on macOS). It bears on the scope's hypothesis that glyph coverage and antialiased edges may differ between x86_64 and aarch64 while solid fills do not.

@history_item n=8 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's colours are engine constants) — made the list's highlighted, disabled and border colours hardcoded constants of `SelectListView`, kept out of `default.css`, because the engine holds no tokens. This is the source of the picked row `decision/engine-colours-hardcoded` and of `absence/engine-color-tokens`. A canvas fixture's paint colours add no token.

@history_item n=9 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's chosen-row pair) — added the hardcoded `#c8c8c8`/`#000000` chosen-row pair as an engine default with no token, the first instance of the rule behind `decision/engine-colours-hardcoded`. The same entry's citation re-point covers `ci.yml`, whose free-disk step moved cited lines. An edit to the CI matrix moves those citations again.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the tree, the snapshot and a shot read of an open list) — a `screenshot` of a row crops to that row's `bounds` and leaves the list open. This is the standing precedent for how a shot reads an engine-painted surface outside the box tree, relevant when a canvas's painted pixels are shot by id.

@history_item n=11 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the cli surface's form: one uncoloured JSON line per command on stdout, keyed by the schema's words, no ESC byte and no terminal test, with exits 0/1/2/3. This is the source of `decision/escher-session-agent-read-surface` and `contract/escher-session-stdout/command-line-line`. Any probe that reads a shot through `escher-session` reads only this line.

@history_item n=12 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout is protocol lines only, uncoloured, with no ESC byte in any reply. The 2026-10-12 shot's image block extended this role. A shot crossing over MCP keeps this form, and the scope adds no carrier.

@history_item n=13 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — every document gets the default user-agent sheet once, through `DocumentConfig::default()` (upstream #1189); founder-confirmed. This is the source of the picked row `decision/default-ua-sheet-via-default-config`. A canvas fixture booted alone takes the sheet only through the default config.

@history_item n=14 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage answer's three stated limits) — handlers inside a canvas are not counted, owned by 0.4.0's canvas plans. This is a standing canvas register this release carries. The chunk does not discharge it but must not contradict it.

@history_item n=15 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — re-pointed design-system's `ci.yml` citations, including the ios/android matrix entries behind `decision/mobile-targets-built-not-tested`. A stand check added to the CI matrix moves these lines.

@history_item n=16 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — re-pointed the ios/android matrix citations (237-252 → 328-343) after `ci.yml` was rewritten for the gate legs. This is the register of CI-matrix line citations that a change to the matrix or the linux test leg must re-point.

@history_item n=17 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into 137 rows, each verified against the code. Every picked row's handle comes from this conversion. An amendment by this chunk names rows by handle (`+`, `~`, `−`).
