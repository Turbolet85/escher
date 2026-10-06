# escher

<!-- GENERATED:setup start -->

## Overview
<!-- GENERATED:setup:overview start -->
escher is a fork of Blitz being turned into an agent-first UI framework — a headless driver (CLI + MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development Style is agent-driven: the framework's own driver is meant to become the harness.

**Stack:** Rust 2024 Cargo workspace (MSRV 1.91) · Stylo CSS · Taffy layout · Parley text · anyrender/Vello paint · winit shell · AccessKit · Dioxus 0.7 · Boa JS — a native-API engine library with no server and no database (the browser app keeps a rusqlite history).

**Key directories:**
- `packages/` — the engine and integration crates (DOM, paint, shell, traits, Dioxus bridge, test harness) and escher's telemetry bootstrap
- `tests/blitz-tests/` — integration tests, one file per behaviour
- `examples/` — example crates and root examples; `examples/seven_guis/` is the 7GUIs stand
- `apps/` — reference browser (`blitz`), markdown viewer (`rdme`), release `bump`
- `wpt/runner/` — the Web Platform Tests conformance runner
- `scripts/` — the agent-run test contract, the cold-agent run pipe and its stub, the code-graph tooling
<!-- GENERATED:setup:overview end -->

## Modules
<!-- GENERATED:setup:modules start -->
- **`blitz-dom`** — headless DOM (`BaseDocument`): tree, Stylo styling, layout, events, scrolling, accessibility tree
- **`blitz-traits`** — shared types and provider traits (net, navigation, shell, events, `NodeId`)
- **`blitz-html`** — html5ever/xml5ever parsing into a blitz-dom document (`HtmlDocument`)
- **`blitz-net`** — HTTP, `file:` and `data:` fetching as a `NetProvider`
- **`blitz-paint`** — paints a resolved document into an anyrender `PaintScene`
- **`blitz-shell`** — winit event loop, windowing, AccessKit adapter wiring
- **`blitz`** — high-level launch API re-exporting the engine crates
- **`blitz-vibey-script`** — `<script>` execution on Boa with DOM bindings (`ScriptDocument`)
- **`stylo_taffy`** — Stylo-to-Taffy style bridge
- **`accesskit_xplat`** — cross-platform AccessKit adapter without winit
- **`debug_timer`** — opt-in phase timing
- **`dioxus-native-dom`** — headless Dioxus renderer on blitz (`DioxusDocument`)
- **`dioxus-native`** — windowed Dioxus renderer (`launch`)
- **`blitz-test-harness`** — headless `Harness`: construction, pump, input synthesis, inspection
- **`escher-telemetry`** — escher's telemetry bootstrap (`init`): stderr `tracing` subscriber with service identity, allowlist scrub, `log` bridge, chaining panic hook
- **`seven_guis`** — the 7GUIs example app, the stand every 0.1.0 capability is proven on; its native `stand` module boots counter, flight booker, timer or CRUD headlessly in TaskShell (pinned viewport, bundled font, offline, fresh per boot)
<!-- GENERATED:setup:modules end -->

## Critical Warnings (universal invariants)
<!-- GENERATED:setup:warnings start -->
- Secrets live only in GitHub Actions secrets/vars — plus, on the dev host, the operator's own Claude Code login that the cold-agent live run uses, held by the `claude` CLI — and source reads none; never put a credential in code, fixtures, logs or committed evidence.
- The workspace binds no network port or socket — a new listener, port, env var or workspace crate is an arch §Occupied Resources registration, never a silent add.
- The a11y target is WCAG SC 2.1.1 · 1.4.3 · 2.4.3 and `accessibility` is a default feature — focusability, focus order and painted colours are a11y surface.
- Engine telemetry is `tracing` behind each crate's `tracing` feature: call sites are `#[cfg(feature = "tracing")]` with a no-op fallback, never an unconditional `println!`; escher binaries install `escher_telemetry::init` (stderr only, never stdout).
- escher's sink scrubs by allowlist (engine targets print only safe fields; content-named fields redacted everywhere), but the upstream apps' `fmt::init()` and the WPT runner's `env_logger` log URLs, attribute values and outer HTML unscrubbed — add no new user-content log fields.
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
| Telemetry bootstrap (subscriber · scrub sets · panic hook) | `packages/escher-telemetry/src/{lib,format,panic}.rs` |
| Code map / impact (symbols · callers · crate deps) | `.andromeda/cache/rust/tree.db` — query via `scripts/code-graph.py query <run_dir> <marker> "<sql>"`; schema + templates in `scripts/code-graph-cookbook.md` |
| Threat model · trust boundaries | `.andromeda/security-plan.md` §Threat Model Summary / §Input Validation |
| Design tokens (as built) | `.andromeda/design-system.md` §Color Palette / §Typography |
| Screen layouts per surface | `.andromeda/layout-templates.md` |
| Test harness · agent-run contract | `.andromeda/test-plan.md` §3 · `scripts/agent-run.sh` (contract tests `.github/scripts/test_agent_run.py`) |
| Cold-agent run pipe (isolated agent session · stub MCP tool · verdict) | `.andromeda/test-plan.md` §3 · `scripts/cold-agent.sh` · `scripts/cold_agent_stub.py` (contract tests `.github/scripts/test_cold_agent.py`) |
| Observability pipeline | `.andromeda/obs-plan.md` §3 |
| WCAG criteria · a11y harness | `.andromeda/a11y-plan.md` §1 / §3 |
| Chunk history (version-agnostic) | `.andromeda/master-route.md` |
| Working route · requirements · matrix | the active `escher-X.Y.Z/` (highest version dir): `working-route.md`, `requirements.md`, `verification-matrix.json` |
| Product intent · vision | the active `escher-X.Y.Z/intent.md` / `vision.md` |
| Headless `Harness` API | `packages/blitz-test-harness/src/{harness,input,inspect}.rs` |
| DOM entry point · config | `packages/blitz-dom/src/document.rs` · `packages/blitz-dom/src/config.rs` |
| Accessibility tree | `packages/blitz-dom/src/accessibility.rs` |
| The 7GUIs stand | `examples/seven_guis/src/tasks/` · headless boot `examples/seven_guis/src/stand.rs` · checks `tests/blitz-tests/tests/stand_*.rs` |
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

The engine is already exercised headlessly — `blitz-test-harness` synthesizes input through the real event-dispatch pipeline with no window or GPU, and the WPT runner renders to CPU buffers; the seven_guis stand boots through it (`seven_guis::stand`) at a pinned viewport with its bundled font and no network. escher builds on that: stable element ids, a semantic snapshot with diffs, settle detection and a driver (CLI + MCP) that acts by id, proven on the seven_guis stand.

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
- `services/{name}.md` — per-crate notes (blitz-dom, blitz-traits, blitz-paint, blitz-shell, blitz-test-harness, dioxus-native-dom, seven_guis)
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
<!-- USER:session-learnings end -->
