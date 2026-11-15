@history_item n=1 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — the upstream merge at `23354585` re-pointed 4 `file:line` citations into merged upstream files by the merge's measured line map, no claim text changed. Precedent for this take: citations into files the 16 commits move are re-pointed by measurement, the rows' claims unchanged.

@history_item n=2 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the `bump` line left §Surface: cli because upstream's `23b9b26a9` deleted the bump crate, taken by the sync. Precedent: an upstream take that adds or removes a CLI surface (a root example, a binary) amends §Surface: cli in the same chunk.

@history_item n=3 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — upstream's root example `multi_doc` was listed beside `paint_bench` on §Surface: cli, taken with that sync. Precedent: a CLI or example surface an upstream take brings in is described here; this take's WPT-only commits (`ea07ee716`, `e7cca1d37`, `c30096724`, `d2ee20b98`, e.g. the skia renderer option) are read against it.

@history_item n=4 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations into the files the take moved) — 4 citations into `apps/browser/src/{main,tab}.rs` and blitz-shell's `window.rs` re-pointed, and 2 blitz-vibey-script citations rewritten to new paths (`src/runtime/window.rs`, `src/dom/element/geometry.rs`) after upstream split the modules. Precedent and trap: a take that moves code between files rewrites the path, not only the digits; the browser frame citations move if this take touches the browser app.

@history_item n=5 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines; F2 ratified) — the stand's select styling (15px, weight 400, options showing the chosen one by the engine's select rule), invalid state and type lines landed on desktop-native, and the F2 gap marker dropped. Bears on `decision/stand-accent-colour` and the flight-type select rows: an engine change from the take (e.g. `hr`/SVG presentational hints, `@font-face` `font-stretch` in `f5ea1caf1`) must leave these measured values unmoved or be re-measured, never loosened.

@history_item n=6 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand: skips Home, mounts one task in TaskShell through `task_in_shell` at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans. Origin of `decision/headless-stand-pinned-viewport`; the stand checks in both layout modes must still read this after the take.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host as a consumer of the TaskShell mount; a held instance reads as a fresh boot and the mount and pinned viewport hold after commands; stderr lines carry `service.name=seven_guis`. Origin of `decision/held-instance-reads-as-fresh-boot` and `contract/escher-session-host/log-service-name`; the sink allowlist re-check reads the take's new `tracing`/`log` sites against this log surface.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session` became the driver's command line with four endings: accepted exit 0 (one uncoloured JSON line), refused exit 1, usage exit 2, session error exit 3. Origin of `contract/escher-session-exit-codes/0` and `/1`; the take must not move them.

@history_item n=9 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the lean tasks' controls carry author ids as attributes only, no class, style, wrapper or order, no CSS selecting by them. Origin of `decision/stand-ids-as-attributes-alone` and the `contract/stand-element-ids/*` rows; a merge resolution keeps "the stable ids and the snapshot".

@history_item n=10 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — CRUD rows keyed by `{person.id}` (fixture 0–2, Create from 3) instead of index, so a row's id follows its person under filter, Create and Delete. Background of `contract/stand-element-ids/crud-person-id`.

@history_item n=11 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — CRUD rows carry author id `crud-person-{person.id}` beside the Dioxus key; Home's seven `task-card-{slug}` ids added, attributes only, on the founder's rule that every actionable element reads an author key. Origin of `contract/stand-element-ids/crud-person-id`, the `task-card-*` rows and `decision/every-stand-control-keyed`.

@history_item n=12 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — Temperature Converter, Circle Drawer and Cells controls keyed (`temp-*`, `circle-*`, `cell-{A..Z}{0..25}`, `cell-input`), `unkeyed_actionable()` 0 on all seven tasks. Origin of those id rows and part of `decision/every-stand-control-keyed`.

@history_item n=13 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — six task inputs named by attributes alone (`for`, `aria-label` "Departure date" / "Return date"); 3 citations into dioxus-native-dom `dioxus_document.rs` re-pointed. Origin of `decision/stand-a11y-by-attributes-alone` and the `flight-start`/`flight-return-date`/`crud-*`/`timer-duration` naming; `dioxus_document.rs` is a merge surface this take may move.

@history_item n=14 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — `aria-invalid`, `role="listbox"`/`role="option"` with `aria-selected` stated by attributes alone, the state classes staying. Part of `decision/stand-a11y-by-attributes-alone`; a merge keeps "the accessibility state reads".

@history_item n=15 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` takes `tabindex="0"` and ArrowDown/ArrowUp/Home/End, focus on the list never a row, no `aria-activedescendant`; CRUD Tab sequence six stops. Origin of `decision/crud-list-keyboard-region`; the take keeps "the range and keyboard models".

@history_item n=16 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` named `People` by `aria-label` alone. Origin of `contract/stand-element-ids/crud-list`'s name.

@history_item n=17 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the engine rule, ratified by the founder for every document: an into-view scroll moves every holding scroller then the viewport, and `visible_region` reads what an element is seen through; the CRUD list as the stand's nested scroller. Origin of `decision/scroll-into-view-moves-every-scroller` and `decision/visible-region-through-clipping-boxes`, engine surfaces escher changed that a conflict resolution must keep.

@history_item n=18 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the bounds reader and the hit fixed in the engine, so a scrolled `crud-list` reads true bounds and a click lands on the centred row; Home's box scrolls, the Cells card below it. Bears on the carried `hit_inner` inline-root coordinate fix and the "hit stops and scrolled-box bounds" a resolution must keep (`0980cce5f`'s `hit_inner` hunk).

@history_item n=19 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the `screenshot` verb: one line `{"width","height","png"}`, `--id` crops to visible bounds; `fonts-unavailable` and `off-screen` refusals exit 1; the MCP image block then text block. Origin of `contract/escher-session-screenshot/answer`, `/refusals/off-screen`, `/refusals/fonts-unavailable` and `/mcp/screenshot-result`; `f5ea1caf1`'s `@font-face` change and the paint commits (`d176ac4e6`, `beaa796f4`, AnyRender bumps) are read against them.

@history_item n=20 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the in-process visual lint at boot in both layout modes, and the gated part made it the `lint` verb answering `{"findings"}`. Origin of `decision/stand-lints-clean-at-boot` and `contract/escher-session-lint/answer`; a moved layout or text number from the take must keep these 0s or be re-measured.

@history_item n=21 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` (1 to 1000), answering `{"text"}` or `{"text","part","parts"}`, out of bound refused `malformed`. Origin of `contract/escher-session-snapshot/answer` and `/part`.

@history_item n=22 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the flight booker's mode became a select `flight-type` with `flight-type-one-way`/`flight-type-return` (was toggle buttons); six served tasks; the load page's `load-fetch`/`load-done` with the stand accent shades; lint at boot measured. Origin of `decision/flight-type-select-boot-reading`, the `flight-type*` and `load-*` id rows and `contract/escher-session-served-tasks/flight-booker`.

@history_item n=23 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — the stand's colour rows landed on desktop-native: accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) on `#f5f5f5`/`#1a1a1a`, white text reading 5.47 vs 4.5, `#4a6cf7` a 4.39 fail. Origin of `decision/stand-accent-colour` and its `stand_contrast` guard; a paint change in the take must leave the measured ratios.

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed) — thirteen citations re-pointed into `cells.rs`, `stand_load_page.rs`, `stand_contrast.rs`, `dioxus_document.rs`, `scrolling.rs` and `stand_lint.rs`, some ranges changing length. Precedent: re-point by measurement, ranges may grow or shrink; `scrolling.rs` and `dioxus_document.rs` are engine files this take may touch.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (load page Tab sequence; Cells grid; `stand_contrast` over load and Cells) — `load-fetch` takes Enter and Space under `LOAD_TAB_ORDER`; `cells-grid` one Tab stop named "Cells" with no role, synchronous `MountedData::scroll_to` (the async form panicked `RefCell already borrowed`); `stand_contrast` reads the load page and Cells. Origin of `contract/stand-element-ids/load-fetch` and `/cells-grid`; trap: the sync handle choice must survive the take.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading) — `overlaps` skips a closed select's options, so `at_boot` reads `[0, 0, 0, 0]` in both layout modes (founder's F9); the snapshot gives each option its select's client rect. Bears on `decision/stand-lints-clean-at-boot` and `decision/flight-type-select-boot-reading`; part of "the select model and its list" a resolution keeps.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list) — the engine paints an open select list in every document, remote pages included; while open each row reads its own bounds in the snapshot and diff, after close back at the select's client rect (founder's F1–F4, F7). Origin of `decision/open-list-rows-own-bounds`; an engine surface escher changed that the take's paint changes (`paint_scene`, SVG clipping/isolated layer) must not break.

@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's rows, the scrolling exception; the lint's `in_select_box`) — a row of an open list is seen through the list's scrollport and scrolled by writing `scroll_y`, no viewport moving; `Entry.in_select_box` replaced `in_closed_select` (founder's F10). Origin of the `except` clauses of `decision/scroll-into-view-moves-every-scroller` and `decision/visible-region-through-clipping-boxes`; conflicts in `scrolling.rs`/lint keep both.

@history_item n=29 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — source citations re-pointed after `use_graph_signal` lines in the task files, `dioxus_document.rs` and `schema.rs`; `escher-session` gains the `graph` verb answering `{"nodes","states"}`, never a value. Origin of `contract/escher-session-graph/answer`; `dioxus_document.rs` (ids, snapshot, graph) is a merge surface whose behaviour the take keeps.

@history_item n=30 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the tenth instance verb `actions`, one unpaged line `{"nodes"}` of id, owner, keyed and verbs, never a value or `NodeId`; `common/mod.rs` citations moved by `ACTIONABLE_AT_BOOT`. Origin of `contract/escher-session-actions/answer`.

@history_item n=31 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (coverage role; skeleton and head notes re-cited) — `escher-session coverage <app> <check>...` replays each check on a fresh boot, answering `{"unexercised","checks","held"}`, a failed boot exit 3; a further TaskShell mount consumer; the skeleton notes' `dioxus_document.rs` citations moved by the reached-record line. Origin of `contract/escher-session-coverage/answer` and grounds of `decision/fixed-document-skeleton-main-mount`, whose citations move if the take edits that file.

@history_item n=32 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (121 rows, each verified against the code), the form every picked row now has; citations formerly in prose now back rows, so a take that moves lines reconciles the rows' grounds.

@history_item n=33 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — graph verb; coverage role, both entries) — the `graph` verb and the coverage role ratified by the founder's own word, PROVISIONAL marks gone. `contract/escher-session-graph/answer` and `contract/escher-session-coverage/answer` stand ratified; the take must not move them without a new word.
