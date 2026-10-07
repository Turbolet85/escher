# escher

<!-- GENERATED:setup start -->

## Overview
<!-- GENERATED:setup:overview start -->
escher is a fork of Blitz being turned into an agent-first UI framework — a headless driver (CLI + MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development Style is agent-driven: the framework's own driver is meant to become the harness.

**Stack:** Rust 2024 Cargo workspace (MSRV 1.91) · Stylo CSS · Taffy layout · Parley text · anyrender/Vello paint · winit shell · AccessKit · Dioxus 0.7 · Boa JS — a native-API engine library with no server and no database (the browser app keeps a rusqlite history).

**Key directories:**
- `packages/` — the engine and integration crates (DOM, paint, shell, traits, Dioxus bridge, test harness) and escher's own crates: the telemetry bootstrap and the driver session
- `tests/blitz-tests/` — integration tests, one file per behaviour; the stand checks share `tests/common/`
- `examples/` — example crates and root examples; `examples/seven_guis/` is the 7GUIs stand
- `apps/` — reference browser (`blitz`), markdown viewer (`rdme`), release `bump`
- `wpt/runner/` — the Web Platform Tests conformance runner
- `scripts/` — the agent-run test contract, the cold-agent run pipe and its stub, the code-graph tooling
<!-- GENERATED:setup:overview end -->

## Modules
<!-- GENERATED:setup:modules start -->
- **`blitz-dom`** — headless DOM (`BaseDocument`): tree, Stylo styling, layout, events, scrolling, accessibility tree, and the changed set (`has_changes` true while it is non-empty, `take_changed_nodes` drains it; marked by mutations of in-document nodes, focus and checked changes and typed text, not by node creation, hover or layout)
- **`blitz-traits`** — shared types and provider traits (net, navigation, shell, events, `NodeId`)
- **`blitz-html`** — html5ever/xml5ever parsing into a blitz-dom document (`HtmlDocument`)
- **`blitz-net`** — HTTP, `file:` and `data:` fetching as a `NetProvider`
- **`blitz-paint`** — paints a resolved document into an anyrender `PaintScene`
- **`blitz-shell`** — winit event loop, windowing, AccessKit adapter wiring (each poll that did work drains the document's changed set and, under `accessibility`, rebuilds the platform tree when it was non-empty)
- **`blitz`** — high-level launch API re-exporting the engine crates
- **`blitz-vibey-script`** — `<script>` execution on Boa with DOM bindings (`ScriptDocument`)
- **`stylo_taffy`** — Stylo-to-Taffy style bridge
- **`accesskit_xplat`** — cross-platform AccessKit adapter without winit
- **`debug_timer`** — opt-in phase timing
- **`dioxus-native-dom`** — headless Dioxus renderer on blitz (`DioxusDocument`); computes each element's stable element id on demand (`element_id` / `element_ids`: author key → anchored path `key//segment` under a keyed element of the same component → component path → document path) and, under `accessibility`, carries it as AccessKit `author_id` through its `Document::accessibility_tree` override, reads the screen as a snapshot tree of id · role · name · state · bounds (`DioxusDocument::snapshot`; a password or file input's value reads the fixed mask `MASKED_VALUE`), writes that tree as one compact text under a recorded size budget (`Snapshot::to_text`, `SNAPSHOT_TEXT_BUDGET`), diffs two snapshots by stable element id into the nodes added, removed and changed (`Snapshot::diff` → `SnapshotDiff` of `DiffNode`s) and lists the actionable elements that read no author key (`DioxusDocument::unkeyed_actionable`)
- **`dioxus-native`** — windowed Dioxus renderer (`launch`)
- **`blitz-test-harness`** — headless `Harness`: construction, pump, settle (`Harness::settle` runs every pass that is due and returns `Settled`, or `NotSettled` naming the busy class — render, layout or loads; a load in flight is reported at once and never waited on, and no time is advanced), input synthesis, inspection
- **`escher-telemetry`** — escher's telemetry bootstrap (`init`): stderr `tracing` subscriber with service identity, target allowlist (engine and escher targets scrubbed, every other target's record dropped), `log` bridge, chaining panic hook
- **`escher-driver`** — the driver's session: `Session` holds one headless instance its caller boots (it names no app) and `Session::act` runs one step on it and returns once it has settled (in process; `NotSettled` naming the busy class when it does not go quiet), `serve` hosts it for the life of a process, and `start` · `attach` · `stop` drive that lifecycle from another process over a local Unix-domain socket in an owner-only state directory — `hello` and `stop` only, nothing of the screen; every edge is a typed `SessionError`; unix only (`Unsupported` elsewhere), a session held in process works everywhere
- **`seven_guis`** — the 7GUIs example app, the stand every 0.1.0 capability is proven on (with a minimal fixture beside it where the tasks lack a case); its native `stand` module boots counter, flight booker, timer or CRUD headlessly in TaskShell (pinned viewport, bundled font, offline, fresh per boot), and its second binary `escher-session <task> <state-dir>` hosts one of the four as a driver session
<!-- GENERATED:setup:modules end -->

## Critical Warnings (universal invariants)
<!-- GENERATED:setup:warnings start -->
- Secrets live only in GitHub Actions secrets/vars — plus, on the dev host, the operator's own Claude Code login that the cold-agent live run uses, held by the `claude` CLI — and source reads none; never put a credential in code, fixtures, logs or committed evidence.
- The workspace binds no network port; its one listener is the driver session's local Unix-domain socket (owner-only, lifecycle messages only, ratified by the founder) — a new listener, port, env var or workspace crate is an arch §Occupied Resources registration, never a silent add.
- The a11y target is WCAG SC 2.1.1 · 1.4.3 · 2.4.3 and `accessibility` is a default feature — focusability, focus order and painted colours are a11y surface.
- Engine telemetry is `tracing` behind each crate's `tracing` feature: call sites are `#[cfg(feature = "tracing")]` with a no-op fallback, never an unconditional `println!`; escher binaries install `escher_telemetry::init` (stderr only, never stdout).
- escher's sink admits two target families and drops the rest: engine targets print only safe fields, escher's own (`escher_*`) print with content-named fields redacted, and a record from any other target is dropped whole at every level — so a sink-installing binary's stderr carries no element id and no accessible name at any `RUST_LOG` level (typed text not measured), and a third-party WARN or ERROR no longer prints either — while the upstream apps' `fmt::init()` and the WPT runner's `env_logger` log URLs, attribute values and outer HTML unscrubbed — add no new user-content log fields.
- Incremental and non-incremental layout must stay identical (incremental_oracle); pipeline tests run `for incremental in [false, true]`.
- DOM mutations go through `DocumentMutator` (`doc.mutate()`), which flushes on Drop — extend the mutator rather than reaching through `DocumentMutator::doc`.
- `NodeId` is a versioned slot id: a dropped node's id stops resolving and indexing a stale id panics — use `get`/`contains_key` for ids that may be stale.
- Dependency pins are coupled (html5ever family ↔ stylo web_atoms, skrifa ↔ parley/vello, svgtypes ↔ usvg, taffy/parley git revs, winit exact beta) — never bump one side alone.
- Work is not done until `bash .github/scripts/ci-leg.sh fast` (fmt · clippy `--locked -D warnings` · workspace tests · CI scripts — the same legs CI runs) and `bash .github/scripts/ci-leg.sh doc` (rustdoc `-D warnings` over every workspace crate — `cargo doc --workspace --no-deps --locked`) pass.
<!-- GENERATED:setup:warnings end -->

## Where to Look
<!-- GENERATED:setup:pointer-table start -->
| Topic | Source |
|---|---|
| Architecture decisions | `.andromeda/architecture.md` |
| Directory tree · resource registry | `.andromeda/architecture.md` §Infrastructure Patterns / §Occupied Resources |
| Public API contracts (Document, mutator, events, harness, providers, telemetry) | `.andromeda/architecture.md` §Standard Contracts |
| Telemetry bootstrap (subscriber · target allowlist and scrub sets · panic hook) | `packages/escher-telemetry/src/{lib,format,panic}.rs` |
| Code map / impact (symbols · callers · crate deps) | `.andromeda/cache/rust/tree.db` — query via `scripts/code-graph.py query <run_dir> <marker> "<sql>"`; schema + templates in `scripts/code-graph-cookbook.md` |
| Threat model · trust boundaries | `.andromeda/security-plan.md` §Threat Model Summary / §Input Validation |
| Design tokens (as built) | `.andromeda/design-system.md` §Color Palette / §Typography |
| Screen layouts per surface | `.andromeda/layout-templates.md` |
| Test harness · agent-run contract | `.andromeda/test-plan.md` §3 (keyed contracts: `.andromeda/registries/test-plan-contracts.toml`) · `scripts/agent-run.sh` (contract tests `.github/scripts/test_agent_run.py`) |
| Cold-agent run pipe (isolated agent session · stub MCP tool · verdict) | `.andromeda/test-plan.md` §3 · `scripts/cold-agent.sh` · `scripts/cold_agent_stub.py` (contract tests `.github/scripts/test_cold_agent.py`) |
| Observability pipeline | `.andromeda/obs-plan.md` §3 (keyed contracts: `.andromeda/registries/obs-plan-contracts.toml`) |
| WCAG criteria · a11y harness | `.andromeda/a11y-plan.md` §1 / §3 (keyed contracts: `.andromeda/registries/a11y-plan-contracts.toml`) |
| Chunk history (version-agnostic) | `.andromeda/master-route.md` |
| Working route · requirements · matrix | the active `escher-X.Y.Z/` (highest version dir): `working-route.md`, `requirements.md`, `verification-matrix.json` |
| Product intent · vision | the active `escher-X.Y.Z/intent.md` / `vision.md` |
| Headless `Harness` API · settle | `packages/blitz-test-harness/src/{harness,input,inspect,settle}.rs` · settle's check `tests/blitz-tests/tests/stand_settle.rs` |
| DOM entry point · config | `packages/blitz-dom/src/document.rs` · `packages/blitz-dom/src/config.rs` |
| Accessibility tree | `packages/blitz-dom/src/accessibility.rs` |
| Stable element ids · actionable-key check | `packages/dioxus-native-dom/src/{element_id,actionable}.rs` · checks `tests/blitz-tests/tests/stand_{element_ids,id_persistence,id_edits,actionable_keys}.rs` |
| Snapshot model (id · role · name · state · bounds) · its text form and size budget · its diff | `packages/dioxus-native-dom/src/{snapshot,snapshot_text,snapshot_diff}.rs` · checks `tests/blitz-tests/tests/stand_{snapshot,snapshot_state,snapshot_text,diff}.rs` |
| Change tracking (the changed set · its drain · the shell's refresh) | `packages/blitz-dom/src/document.rs` (`has_changes`, `take_changed_nodes`) · `.andromeda/architecture.md` §Cross-cutting Patterns (Invalidation and state integrity) · `packages/blitz-shell/src/window.rs` (`View::poll`) |
| Driver session (held instance · settled step · lifecycle socket · host binary) | `packages/escher-driver/src/{session,host,client,wire,error}.rs` · host `examples/seven_guis/src/session_host.rs` · checks `tests/blitz-tests/tests/stand_session_*.rs` and `stand_settle.rs` (shared `tests/session_common/mod.rs`) and `examples/seven_guis/tests/host_{binary,log}.rs` (shared `tests/common/mod.rs`) · contract `.andromeda/registries/contracts/test-plan/session-lifecycle.md` |
| The 7GUIs stand | `examples/seven_guis/src/tasks/` · headless boot `examples/seven_guis/src/stand.rs` · checks `tests/blitz-tests/tests/stand_*.rs` · their shared tables and helpers `tests/blitz-tests/tests/common/mod.rs` |
| WPT runner | `wpt/runner/src/main.rs` · `wpt/runner/src/test_runners/` |
| CI pipeline | `.github/workflows/ci.yml` · `wpt.yml` · `publish-browser.yml` (the last two upstream-only) · legs `.github/scripts/ci-leg.sh` · invariants `.github/scripts/test_ci_workflows.py` |
| Drift detectors · amendment playbook | `.andromeda/drift-base.md` · `.andromeda/playbook.md` |
<!-- GENERATED:setup:pointer-table end -->

## Workflow
<!-- GENERATED:setup:workflow start -->
**Key commands:**
- `cargo build --workspace` — build every crate (Linux needs `libfontconfig1-dev`; Arch: `fontconfig`)
- `bash .github/scripts/ci-leg.sh fast` — the local pre-push gate: fmt → clippy → workspace tests → CI scripts, as CI runs them
- `cargo test -p blitz-tests --test {name}` — one integration-test file; agent-driven: `bash scripts/agent-run.sh boot`, then `run stand|all|{name}` (JSON-line results; `status` · `logs` · `cleanup`)
- `bash .github/scripts/ci-leg.sh {leg}` — one CI leg exactly (`fmt` · `clippy` · `test` · `build` · `doc` · `audit` · `a11y` · `coverage` · …); log in `target/ci-logs/{leg}.log`
- `just seven_guis` — run the 7GUIs stand natively (windowed)

See `.claude/docs/commands.md` for the full reference.
<!-- GENERATED:setup:workflow end -->

## Architecture
<!-- GENERATED:setup:architecture start -->
Blitz is a radically modular, embeddable web engine: a headless DOM (`BaseDocument` in blitz-dom) that external code drives and any renderer paints, with parsing, networking, painting and windowing in separate crates. Embedder services — net, navigation, shell, HTML parsing — reach the DOM only through provider traits in `DocumentConfig`, defaulting to `Dummy*` no-ops; web behaviour is written against named specs and named engines. Style, damage, box construction, layout and paint topology run one incremental pipeline over versioned node ids.

The engine is already exercised headlessly — `blitz-test-harness` synthesizes input through the real event-dispatch pipeline with no window or GPU, and the WPT runner renders to CPU buffers; the seven_guis stand boots through it (`seven_guis::stand`) at a pinned viewport with its bundled font and no network. escher builds on that: stable element ids (built — `DioxusDocument::element_id(s)`, the author's HTML `id`, else a path anchored at the nearest keyed element of the same component, else the element's component path; proven the same across a re-render, a remount, a fresh process and edits of the code around an element; every element an agent can act on is keyed, checked by `DioxusDocument::unkeyed_actionable`; and the id is carried on every element's accessibility node as AccessKit `author_id`), a semantic snapshot (its data model built — `DioxusDocument::snapshot`, every element the accessibility tree keeps as a node of id · role · name · state · bounds, each field from a reader the engine already had; its state proven per control through real input — enabled, checked, value and focused — with a password's or a file input's value read as the fixed mask `MASKED_VALUE`; its text form built — `Snapshot::to_text`, one line per node nested by indent, every stand screen under the 10,000-byte `SNAPSHOT_TEXT_BUDGET`, returned to its caller only; its diff built — `Snapshot::diff`, a comparison of the snapshots taken before and after a step that names exactly the nodes added, removed or changed, by stable element id, and is empty for a step that changes nothing, proven on the stand in both layout modes — beside an engine change flag that now tells the truth, `has_changes` reading true from a tracked write until `take_changed_nodes` drains it; the driver that returns that diff after an action is still to come), a driver session (built — `escher_driver::Session` holds one headless instance across commands, one process hosts it, and `start` · `attach` · `stop` drive its lifecycle over a local socket that carries nothing of the screen; the stand hosts its own through the `escher-session` binary), settle detection (built — `Harness::settle` runs every pass that is due and returns `Settled`, or `NotSettled` naming the busy class: render and layout driven to quiet, a load in flight reported at once and never waited on, a timer or an animation neither waited on nor advanced; `Session::act` runs one step on the held instance and returns once it has settled; proven on the stand's Timer and on minimal fixtures in both layout modes — in process only, no driver action returns settled yet), and — still to come — a driver (CLI + MCP) that acts by id, proven on the seven_guis stand.

**Primary source:** `.andromeda/architecture.md` (the pointer table's row — not imported; read explicitly where a step needs it).
<!-- GENERATED:setup:architecture end -->

<!-- GENERATED:setup:imports start -->
@.claude/session-handoff.md
<!-- GENERATED:setup:imports end -->

<!-- Maintainer note: The @ imports above MUST each be on their own line — Claude Code only recognizes standalone @path lines as import directives. Keep @ imports minimal — an import rides every turn of every session. architecture.md and master-route.md are deliberately NOT imported; the pointer table names both. See section-markers.md. -->

## Deeper Topics
<!-- GENERATED:setup:deeper-topics start -->
On-demand references in `.claude/docs/` (Claude reads when relevant):
- Specialist summaries: `security-summary.md` / `design-summary.md` / `tests-summary.md` / `obs-summary.md` / `a11y-summary.md`
- Core: `stack.md` / `conventions.md` / `commands.md` / `gotchas.md` / `workflow.md`
- `services/{name}.md` — per-crate notes (blitz-dom, blitz-traits, blitz-paint, blitz-shell, blitz-test-harness, dioxus-native-dom, escher-telemetry, escher-driver, seven_guis)
- `session-learnings.md` — curated by /andromeda-wrap-session

Path-scoped rules in `.claude/rules/` (auto-load when matching files touched):
- `security.md` (always loaded) · `testing.md` · `observability.md` · `a11y.md` · `verification-harness.md`

For complete Andromeda documentation: `/andromeda-help`
<!-- GENERATED:setup:deeper-topics end -->

<!-- GENERATED:setup end -->

<!-- USER:session-learnings start -->
## Session Learnings
_This section is curated by `/andromeda-wrap-session`. It accumulates universal (Tier 1) rules captured from work sessions — one sentence each, ≤600 B._

- 2026-10-05: This checkout carries an `upstream` remote (DioxusLabs/blitz) and no `gh repo set-default`, so a bare `gh run` / `gh cache` / `gh api` call reads upstream — pass `-R Turbolet85/escher` on every read of the fork. (confidence 0.8)
- 2026-10-07: Inside a Bash tool call `grep` is a shell function running Claude Code's embedded ugrep (`/usr/bin/grep` is GNU grep), and a pattern with a long bounded repetition (a `.{0,200}` context window) exceeds ugrep's complexity limit and leaves stdout empty — use `command grep` or Python's `re`, never read an empty grep as an absence, and read stderr first (detail in `.claude/docs/session-learnings.md`). (confidence 0.7) [corrected 2026-10-07: the host's grep is GNU; the function is the tool's]
<!-- USER:session-learnings end -->
