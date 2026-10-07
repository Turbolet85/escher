## Surface: desktop-native

> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement or hero/signature section; slice s01's reading did not reach the browser app's window/page layout code (slice s04's facts below cover it).

### Primary screens

- Browser frame — a full-height flex column: TabStrip, Toolbar, one TabWebView per tab, optional FPS overlay, StatusBar (apps/browser/src/main.rs:168-198; apps/browser/assets/browser.css:5-18)
- about:newtab — a centered column of logo and 520px search input (apps/browser/src/about_pages.rs:79-109; apps/browser/assets/about-newtab.css:1-34)
- about:history — a header toolbar with clear button over a list of favicon/title/url/time rows (apps/browser/src/about_pages.rs:111-187; apps/browser/assets/about-history.css:15-85)
- seven_guis Home and TaskShell — Home is a centered 640px column of task cards; TaskShell is a header (back button, title, spacer) over a scrolling body (examples/seven_guis/src/app.rs:130-160; examples/seven_guis/src/app.rs:162-181; examples/seven_guis/src/app.rs:183-337); each of Home's seven task-card buttons carries an author `id` that is its stable element id — `task-card-counter` · `task-card-temp-converter` · `task-card-flight-booker` · `task-card-timer` · `task-card-crud` · `task-card-circle-drawer` · `task-card-cells` — adding no class, style, wrapper or order, and no Home CSS selects by them (examples/seven_guis/src/app.rs:144); the headless stand skips Home and mounts one lean task (Counter, FlightBooker, Timer, Crud) in TaskShell through `task_in_shell` — `main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body` — at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans (examples/seven_guis/src/app.rs:90; examples/seven_guis/src/stand.rs:43-69; tests/blitz-tests/tests/stand_boot.rs:48-58); the `escher-session` host is a third consumer of that same mount — it boots one lean task through `seven_guis::stand` only and holds the instance while commands arrive, adding no markup, id, class, style, wrapper or order: a held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes (examples/seven_guis/src/session_host.rs:36-43; tests/blitz-tests/tests/stand_session_fresh.rs:39-117); each lean task's controls and value displays carry author `id`s that are their stable element ids — `counter-value` · `counter-increment`; `flight-one-way` · `flight-return` · `flight-start` · `flight-return-date` · `flight-book` · `flight-booked`; `timer-progress` · `timer-elapsed` · `timer-duration` · `timer-duration-value` · `timer-reset`; `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create` · `crud-update` · `crud-delete` — and each CRUD row the author `id` `crud-person-{person.id}` beside its Dioxus key `{person.id}`, its person's model-assigned id, so a row's stable element id is its author key (fixture `crud-person-0` · `crud-person-1` · `crud-person-2`, Create from `crud-person-3`) and follows its person under filter, Create and Delete (examples/seven_guis/src/tasks/crud.rs:69); ids add no class, style, wrapper or order, and no task CSS selects by them; the six task inputs are named by attributes alone — `for` on the existing labels of `timer-duration`, `crud-filter`, `crud-name` and `crud-surname`, `aria-label` "Departure date" / "Return date" on `flight-start` / `flight-return-date` — adding no element, class, style or order (examples/seven_guis/src/tasks/timer.rs:78; examples/seven_guis/src/tasks/crud.rs:44; examples/seven_guis/src/tasks/crud.rs:86; examples/seven_guis/src/tasks/crud.rs:92; examples/seven_guis/src/tasks/flight_booker.rs:80-93) (examples/seven_guis/src/tasks/counter.rs:11-14; examples/seven_guis/src/tasks/flight_booker.rs:68-110; examples/seven_guis/src/tasks/timer.rs:74-96; examples/seven_guis/src/tasks/crud.rs:46-126)
- rdme — centers a `.markdown-body` column (apps/readme/src/markdown/comrak.rs:40-49; apps/readme/assets/blitz-markdown-overrides.css:1-5)
- todomvc — centers a 230–550px body column (examples/todomvc/src/todomvc.css:23-34)
- wgpu_texture — `main` is a grid with a 100px header row over a 1fr canvas row (examples/wgpu_texture/src/styles.css:11-16)
- custom_widget example — a grid `main` with rows `100px 1fr`, a right-hand absolute overlay at 33% width and a left-hand underlay (examples/custom_widget.rs:183-188; examples/custom_widget.rs:201-222)
- transforms example — a full-viewport absolute container with pointer and wheel handlers applying a pan/zoom `transform` (examples/transforms.rs:105-129)
- form example — centers a flex column in a 100vw x 100vh container (examples/form.rs:103-110)
- static HTML example — centers content with `display: grid; place-items: center` at full height (examples/html.rs:8-17)

### Component — Header

- Browser toolbar row: back, forward, refresh, home, urlbar with suggestion dropdown, overflow menu (apps/browser/src/toolbar.rs:318-446)
- `ShellProvider` offers window chrome controls for documents drawing their own titlebar: request_window_close, set_window_minimized, set_window_maximized, is_window_maximized, set_window_decorations, drag_window (packages/blitz-traits/src/shell.rs:45-62)

### Component — Primary navigation

- Browser tabs: tab content flexes to fill; inactive tabs are `display: none` (apps/browser/assets/browser.css:255-261; apps/browser/src/tab.rs:219-223)
- Transforms example pan/zoom: middle (Auxiliary) button drag pans; wheel zoom clamps factor 0.8-1.2 and zoom 0.05-50 (examples/transforms.rs:27; examples/transforms.rs:59-69; examples/transforms.rs:100)

### Component — Footer

- Browser status bar is fixed bottom-left at max 60% width; the menu dropdown and suggestions are absolutely positioned popovers (apps/browser/assets/browser.css:205-220; apps/browser/assets/browser.css:287-305; apps/browser/assets/browser.css:315-328)

### IA notes

- Each document starts with the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>` and the app mounts into `main` (packages/dioxus-native-dom/src/dioxus_document.rs:100-135)
- Arbitrary `index.html` templates are not supported (a TODO) (packages/dioxus-native-dom/src/dioxus_document.rs:110)
- Head elements (title, meta, script, style, link) are appended to `<head>`, with optional text contents (packages/dioxus-native-dom/src/dioxus_document.rs:157-183)
- Launch config carries `stylesheets` and `base_url` (packages/blitz/src/lib.rs:72-75; packages/blitz/src/lib.rs:113-121)
- The viewport is the window surface minus safe-area insets, while the render surface covers the whole window including the safe area (packages/blitz-shell/src/window.rs:183-190; packages/blitz-shell/src/window.rs:306-309)
- Pointer client coordinates subtract the safe-area left/top insets and page coordinates add viewport scroll (packages/blitz-shell/src/window.rs:442-463)
- The document element is the scrolling element; `window.scrollTo`/`scrollBy` scroll the root element (packages/blitz-vibey-script/src/dom/document.rs:122-126; packages/blitz-vibey-script/src/runtime.rs:2152-2187)
- The root element's `clientWidth`/`clientHeight` are viewport size minus scrollbar size (packages/blitz-vibey-script/src/dom/element.rs:1004-1049)
- Fixed-position children of the root element are not scrolled with the viewport (packages/blitz-paint/src/render.rs:1039-1054)
- The test harness defaults to an 800x600 viewport at scale 1 in light mode (packages/blitz-test-harness/src/harness.rs:23-34; packages/blitz-test-harness/src/harness.rs:60)

---

## Surface: mobile-native

> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement, primary screens beyond the browser frame, or hero, content and footer components; slice s01's reading did not reach mobile layout code.

### Wireframe — Browser frame (Android)

- On Android the frame pads 30px top and 44px bottom as a hardcoded safe area (apps/browser/src/main.rs:147-158; apps/browser/src/main.rs:171)

### Component — Header

- On mobile the toolbar omits the forward and home buttons (apps/browser/src/toolbar.rs:325-335)
- A bottom-toolbar (`column-reverse`) mobile layout is present only as commented-out CSS (apps/browser/assets/browser.css:141-155)

---

## Surface: cli

> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level or signature placement; slice s03's reading did not reach any further CLI help or argument parser beyond the `paint_bench` usage line.

### Primary screens

- `paint_bench` — usage line `paint_bench <url> [width] [height] [scale] [iters] [backend]` with `backend: vello (default) | cpu | hybrid` (examples/paint_bench.rs:6-7)
- `bump` — takes two positional arguments and emits single-line output (apps/bump/src/main.rs:67-91; apps/bump/src/main.rs:100; apps/bump/src/main.rs:109)
- `scripts/agent-run.sh` — the agent-invocable test contract, run from the repository root: usage `agent-run.sh <verb> [selection]` with verbs `boot · run · status · cleanup · logs` and run selections `stand · all · <blitz-tests file name>`, printed to stderr on a usage error (exit 2); every verb's stdout is JSON lines only, one object per event, and the raw cargo output stays in `target/agent-run/run.log`; `scripts/agent-run.ps1` is its Windows pass-through. The exit grammar and event schema live in test-plan §3 (scripts/agent-run.sh:12-17; scripts/agent-run.ps1:1-4)
- `scripts/cold-agent.sh` — the cold-agent run pipe, run from the repository root: usage `cold-agent.sh <verb> [task]` with verbs `run · status · cleanup · logs` and the one task `counter`, printed to stderr with empty stdout on a usage error (exit 2, checked before any precondition); every verb's stdout is JSON lines only, one object per event, and the run's raw session transcript stays in `target/cold-agent/transcript.jsonl`; `scripts/cold-agent.ps1` is its Windows pass-through. The exit grammar, the event and verdict schema and the stdio MCP stub it spawns live in test-plan §3 (scripts/cold-agent.sh:17-22; scripts/cold-agent.ps1:1-4)
- `escher-session` — the session host, a binary of the seven_guis package: usage `escher-session <counter|flight-booker|timer|crud> <state-dir>`, a closed argv of exactly two arguments; any other argv prints that usage line to stderr and exits 2, having booted nothing and created no state; it exits 0 after its session is stopped and 1 with an error's fixed message on stderr; it writes nothing to stdout on any path, and its stderr log lines carry `service.name=seven_guis`; unix only in 0.1.0. The session contract it serves lives in test-plan §3 → Session lifecycle (examples/seven_guis/src/session_host.rs:13-32; examples/seven_guis/src/session_host.rs:47-53; examples/seven_guis/Cargo.toml:45-47)

### Output structure — wpt runner

- Non-verbose terminal mode reserves one line per rayon thread and rewrites each thread's line in place with ANSI cursor escapes as `[done/count] thread N: STATUS name` (wpt/runner/src/main.rs:521-527; wpt/runner/src/main.rs:705-718)
- Non-terminal non-verbose mode prints `[done/count] ...` every 1000 tests and at the end (wpt/runner/src/main.rs:719-721)
- Verbose mode prints `[num/count] ` and the full result line per test (wpt/runner/src/main.rs:699-703)
- A panicking test's line is followed by the panic message, `Panicked at file:line:column` and a trimmed backtrace (wpt/runner/src/main.rs:438-453)
- After the run, an "Ordered Results" heading precedes alphabetically sorted result lines numbered `[NNNN/count]` (wpt/runner/src/main.rs:730-739)
- Summary block: duration, then FOUND/SKIPPED/RUN, subtest counts, CRASHED/PASSED/FAILED/TIMED OUT with percentages of run and found, partial-pass count, and failure buckets by feature, with counts right-aligned to width 4 (wpt/runner/src/main.rs:784-830)

---

## Surface: web-spa

> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement, navigation, header, footer or IA.

### Primary screens

- seven_guis and todomvc web builds — the WASM canvas fills the full body (examples/seven_guis/index.html:8-9; examples/todomvc/index.html:8-9)
- wasm_hello — centers its canvas with grid and renders content in a 640px card (examples/wasm_hello/index.html:9-24; examples/wasm_hello/src/lib.rs:51-59)

---

## Decisions Log

> NO RECORDED INTENT
