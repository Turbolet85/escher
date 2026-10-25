# Task — arch · 2026-10-25-cold-agent-bar

## The chunk
Extends the 7GUIs stand so the driver serves every 0.2.0 check: `cells` (the one screen over the snapshot text budget) served by `escher-session start`, the flight booker's one-way / return choice made a real `select`, and a load in flight carried on the stand.
Gives the cold-agent pipe's `stand` task set goals over 0.2.0's capabilities: recording and replay, lint, `advance` on the one virtual clock, the load wait, the select and the screen read in parts.
Runs one live cold-agent run on the operator's login, counts its wrong calls from the server's own `tool_call` span lines and replays the check the agent writes; the wrong-call bar is the founder's fork at P4, not set here.
Re-pins the stand's moved coverage claims in the tests that hold them: the Tab and Enter/Space tables, contrast rows, element ids and actionable keys, snapshot, diff and act checks, the flight flow, and every stand screen under the budget.
Adds no driver verb, argument, cause, session error, carrier, log field or span and intends no engine change for every document; how the stand holds a load in flight, and whether that adds a provider or resource, is a P4 fork.

## Reached files
- examples/seven_guis/src/session_host.rs · edits the task list `start` boots
- examples/seven_guis/src/stand.rs · edits the headless boot and `stand::options`
- examples/seven_guis/src/tasks/cells.rs
- examples/seven_guis/src/tasks/flight_booker.rs · edits the one-way / return choice
- tests/blitz-tests/tests/fixtures/mod.rs
- tests/blitz-tests/tests/common/mod.rs · edits `tab_order`, `ENABLED_TAB_ORDER`
- tests/blitz-tests/tests/stand_keyboard.rs · edits the Tab order and Enter/Space tables
- tests/blitz-tests/tests/stand_contrast.rs · edits the contrast rows
- tests/blitz-tests/tests/stand_element_ids.rs
- tests/blitz-tests/tests/stand_actionable_keys.rs
- tests/blitz-tests/tests/stand_snapshot.rs
- tests/blitz-tests/tests/stand_snapshot_text.rs
- tests/blitz-tests/tests/stand_diff.rs
- tests/blitz-tests/tests/stand_act_select.rs
- tests/blitz-tests/tests/stand_load_wait.rs
- tests/blitz-tests/tests/stand_snapshot_parts.rs
- tests/blitz-tests/tests/stand_non_lean_tasks.rs
- examples/seven_guis/tests/flows/flight.sh
- scripts/cold-agent.sh · edits `TASKS`, the `stand` task set's prompt and goals
- .github/scripts/test_cold_agent.py · edits the cold-agent contract tests

## Needs
- The headless stand contract: `seven_guis::stand` boots the lean tasks offline in a pinned 800x600 Light viewport with one bundled font, installs no telemetry, reads no env var and names no `blitz_net` — the chunk widens the set it boots and must keep or amend each of those terms.
- The driver command line contract: `escher-session` over the four lean stand tasks with its `serve` host and `mcp` roles, one JSON line out and exits 0 · 1 · 2 · 3 — the task set `start` and `mcp` boot grows.
- The driver session contract: twelve verbs and eight causes stated once as static data, every call through `Session::run`, no `NodeId` crossing, nothing logging an id, name or value — the chunk exercises it and must add no verb, argument, cause or session error.
- The snapshot in parts: `SNAPSHOT_TEXT_BUDGET`, `Snapshot::text_parts`, `snapshot`'s `part` up to `MAX_PART` — reached on the stand by serving `cells`.
- The load wait: `LOAD_WAIT` bounding an acting step's wait for a load, moving no time, and the harness's `settle` reporting a load in flight without waiting — the stand must hold a load for it.
- The select model decision: the choice held as options' `CHECKED` state, every change through `apply_select_choice`, a controlled app writing `selected` from its `input` event's value or the choice reverts, no popup painted, no `multiple` or `optgroup` — binds the flight booker's new select and `type` on it.
- The Dioxus DOM bridge: stable element ids in four tiers, every actionable element reading an author key (`unkeyed_actionable`), a select's options as its children in the snapshot, the snapshot, its text and diff read-only — binds the `cells` grid and the new select.
- Embedder services injected with no-op defaults and the provider traits (`NetProvider`, a no-op one registering no pending-critical resource) — indirectly binds how the stand could carry a load in flight.
- Fixture conventions: `tests/blitz-tests/tests/fixtures/mod.rs` states the fixture boot, hold, shared pages and `HeldNet` once, a page with one reader stays in its check's file.
- Test conventions: one behaviour per integration file opening with a `//!` doc, scenarios in both layout modes, stand helpers shared through non-target modules, a test first asserting its fixture produces the condition, blitz-tests one target per file.
- Incremental and non-incremental layout producing identical layouts — every re-pinned stand check runs both modes.
- Apps and examples conventions for the stand's Dioxus tasks: `#[component]` components, `#[derive(Store)]` state, CSS in `const` raw strings.
- The recording and replay contract: a launcher-named `--record` directory, one owner-only kept check, `Check::replay` through `Session::run` alone — one of the readings of the acceptance's recorded run that passes on replay.
- The recording directory and committed checks resource: the two committed Counter checks under `tests/blitz-tests/checks/` are the one exception to owner-only — any other kept check, the live run's included, stays owner-only and uncommitted.
- A shot's PNG bytes differing between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no `screenshot` step — binds a replayed agent check.
- Occupied Resources for the cold-agent pipe: the model provider as an outbound host reached only by the spawned `claude` client on the operator-driven live run, the cold-agent state area under `target/`, the names `scripts/cold-agent.sh` and `mcp__escher` — and a new provider, listener, env var or resource is a registration, never a silent add.
- Network ports and listeners: no port, one Unix-domain socket, the `mcp` role over stdio — a stand-held load must open no listener or port.
- Process-wide state: the `escher-session` host one process with one session, its `mcp` role at most one session, telemetry installed in both seven_guis binaries.
- The driver session decision: what crosses (ids, names, masked values, snapshot text, diffs, shots, lint findings) and that a new carrier or exit widens the boundary — binds the new screens (Cells' contents, the select) the live run carries.
- Logging and the telemetry bootstrap: escher's sink drops records outside the engine and escher sets and redacts content fields; the pipe counts wrong calls from the `tool_call` span lines — the chunk adds no log field or span.
- The CI contracts: `scripts/cold-agent.sh` the cold-agent pipe contract in full in test-plan §3, `ci-leg.sh` the one leg runner — its `stand-repeat` leg globbing `stand_*.rs` and the `a11y` leg naming `stand_keyboard`, `stand_contrast` and `stand_act_select`, so a new or renamed stand check moves those sets.
- Default features: `accessibility` default-on and gating the dioxus-native-dom snapshot modules the stand checks read.
- The test harness contract: one `VirtualClock` its documents share, `tick` and `advance_time` moving it, `screenshot`, `text_contrast` and `settle` never moving it — the stand's `advance` and contrast checks read through it.
- The DOM semantics decision: `disabled` read by presence on a form control alone and `inert` honoured — binds the flight booker's select under its keyboard and act checks.
- Existing scopes for seven_guis, blitz-tests and escher-driver — the stand and its checks are the chunk's work area.

## Not needed
- The WPT runner, its outputs, execution model and bucketing: the chunk runs no web-platform test.
- The browser app and rdme: history database, URL suggester, about pages, browser runtime polling.
- JavaScript on Boa (blitz-vibey-script, `ScriptFetcher`, JS timers): the stand is a Dioxus app with no script.
- blitz-net's HTTP internals (User-Agent, per-host semaphore, HTTP cache): the stand boots offline and names no `blitz_net`.
- Dependency pinning, build profiles, MSRV, release versioning, Nix packaging and cross-compilation: no manifest or toolchain change.
- CSS approximations, unsupported CSS features, inline SVG, `@font-face` handling and layout internals (`LayoutPassState`, tables, replaced elements): no engine layout change.
- Painting internals and the password mask, the shell loop, windowing and the accesskit_xplat platform adapter: no paint, shell or windowed change.
- Scrolling, hit-testing and transformed-bounds engine decisions (`scroll_into_view`, `absolute_position`, `get_client_bounding_rect`): no engine change, only stand checks that read them.
- The fixture forms of the google and graphite pages and the example HTML fixture shapes.
- CI infrastructure environments, the publish-browser deployment model and the WPT and post-results workflows: upstream-only, untouched.
- Licensing exceptions, unsafe aliasing and the presentational-hints rendering details.
- Project intent's upstream sync state and merge base: no upstream merge in this chunk.
- The front page README and the capability statement `docs/capabilities.md`: out of scope, the statement's live-run line the wrap's to re-read.
