# Materialization Plan — escher (setup-project checkpoint)

_Phase 0 output. Phases 1–6 read ONLY this file. First run (no CLAUDE.md) on an adopted project._

- **Project:** escher · version workspace `escher-0.1.0/` · branch `build/escher-0.1.0` · HEAD `0f60502e`
- **Development Style:** agent-driven (arch §Cross-cutting Patterns)
- **Primary language / stack fragment:** Rust (Cargo workspace, edition 2024, rust-version 1.91.0) · `rust`
- **Host:** Linux (POSIX) — `host-win32.md` skipped (noted no-op)
- **Upstreams read:** input · architecture · security-plan · design-system · layout-templates · test-plan · obs-plan · a11y-plan · master-route (no chunks promoted) · none U35-migrated (no `.andromeda/registries/`, no amendment sidecars)

## Adoption

- `Adopted at 0f60502ea724ef703b34220b3847bb3483f70b5f · push target refs/remotes/origin/build/escher-0.1.0 · secret scan not measured`
- Sections reading NOT YET MEASURED / NO RECORDED INTENT / observed absent (render nothing from a template default):
  - security-plan: Threat Model (application-level) NOT YET MEASURED; TLS/HSTS NOT YET MEASURED; key mgmt/retention NOT YET MEASURED; lockfile/CVE SLA NOT YET MEASURED; SBOM NOT YET MEASURED; dev secret storage / secret scanning NOT YET MEASURED; security-event logging NOT YET MEASURED; Compliance, Anti-Patterns, Decisions Log NO RECORDED INTENT; auth observed absent; dependency audit observed absent
  - design-system: Brand Identity, Depth, Radius, Motion, Anti-Patterns, Self-Validation, Decisions Log NO RECORDED INTENT; project colour tokens, type scale, spacing, elevation, radius NOT YET MEASURED; cli/web-spa navigation + platform notes NOT YET MEASURED
  - layout-templates: every surface's tooling/expression/signature NOT YET MEASURED; Decisions Log NO RECORDED INTENT
  - test-plan: §1 tier justification NOT YET MEASURED; **§3 5-command contract NOT YET MEASURED** (in-process `Harness` measured); §6 windowed E2E NOT YET MEASURED; §9 coverage observed absent; §10 Quality Gates NOT YET MEASURED; §11 Anti-Patterns, §12 Decisions NO RECORDED INTENT
  - obs-plan: §1 telemetry surface NOT YET MEASURED; §2 naming NOT YET MEASURED; §3 product mode / identity / log schema / sink / heartbeat NOT YET MEASURED; OTel observed absent; §4 spans observed absent; §6 log schema NOT YET MEASURED; §8 scrubbing observed absent; §10 SLO, §11 Anti-Patterns, §12 Decisions NO RECORDED INTENT
  - a11y-plan: critical paths NOT YET MEASURED; §2 strategy NOT YET MEASURED; §3 violation schema NOT YET MEASURED; keyboard/Tab tests observed absent; contrast checks observed absent; §9 CI observed absent; §10 SLO, §11 Anti-Patterns, §12 Decisions NO RECORDED INTENT
- **test-plan §3 is NOT YET MEASURED for the 5-command contract → the Foundation chunk that owns the harness:** `Stand test contract — agent-invocable boot, run, status, cleanup and JSON-line logs for stand checks and blitz-tests (per test-plan §3)` (working route, Epoch 1)
- Counts are ceilings, never floors: warnings ≤10, modules ≤15, pointers ≤25; session-learnings seed may be empty.

## Tier 1 — CLAUDE.md

- **Overview:** arch §Project Intent paragraph (fork of Blitz → agent-first UI framework: headless driver CLI+MCP, stable ids, compact semantic snapshot, proven on internal 7GUIs stand). Stack one-liner: Rust 2024 Cargo workspace (MSRV 1.91) · Stylo CSS · Taffy layout · Parley text · anyrender/Vello paint · winit shell · AccessKit · Dioxus 0.7 · Boa JS · no server, no DB (browser app: rusqlite history).
- **Key dirs (5):** `packages/` engine + integration crates · `tests/blitz-tests/` integration tests · `examples/` (incl. `seven_guis/` — the stand) · `apps/` browser/rdme/bump · `wpt/runner/` WPT conformance runner.
- **Modules (15, arch §Existing Scopes):** blitz-dom · blitz-traits · blitz-html · blitz-net · blitz-paint · blitz-shell · blitz · blitz-vibey-script · stylo_taffy · accesskit_xplat · debug_timer · dioxus-native-dom · dioxus-native · blitz-test-harness · seven_guis (stand). (Apps/tests/wpt named in key dirs.)
- **Universal warnings (10)** — severity security > a11y > obs > tests > arch (no plan carries an anti-pattern section; all drawn from measured facts + arch §Cross-cutting / §Established Decisions / §Conventions):
  1. [security] Secrets live only in GitHub Actions secrets/vars (signing keys, keystore, `WPT_GITHUB_TOKEN`, `GITHUB_TOKEN`); source reads none — never put a credential in code, fixtures or logs.
  2. [security] The workspace binds no network port or socket (arch §Occupied Resources) — a new listener, port, socket, env var or workspace crate is an arch §Occupied Resources registration, never a silent add.
  3. [a11y] The a11y target is WCAG SC 2.1.1 · 1.4.3 · 2.4.3 (a11y-plan §1); `accessibility` is a default feature of blitz-dom/blitz-shell — focusability, focus order and painted colours are a11y surface.
  4. [obs] Telemetry is the `tracing` crate behind each crate's `tracing` feature: call sites are `#[cfg(feature = "tracing")]` with a no-op fallback — no unconditional `println!`/tracing in engine crates.
  5. [obs] Log events carry URLs, attribute values and outer HTML unscrubbed and no scrub layer exists yet (obs-plan §8) — add no new user-content log fields.
  6. [tests] Incremental and non-incremental layout must stay identical (incremental_oracle); pipeline tests run both modes `for incremental in [false, true]`.
  7. [arch] DOM mutations go through `DocumentMutator` (`doc.mutate()`), which flushes deferred work on Drop — extend the mutator rather than reaching through `DocumentMutator::doc`.
  8. [arch] `NodeId` is a versioned slot id: a dropped node's id stops resolving and indexing a stale id panics — use `get`/`contains_key` for ids that may be stale.
  9. [arch] Dependency pins are coupled: html5ever/markup5ever/xml5ever ↔ stylo web_atoms, skrifa ↔ parley/vello, svgtypes ↔ usvg; taffy/parley git revs; winit exact beta — never bump one side alone.
  10. [arch] Gates: `cargo fmt --all --check`, `cargo clippy --workspace -- -D warnings`, rustdoc `-D warnings` (CI) — work is not done until they pass.
- **Rejected (audit):** `unsafe` needs `// SAFETY:` (path-scoped → conventions.md); deps declared once in `[workspace.dependencies]`, in-repo crates `default-features = false` (→ conventions.md); recursion caps iframe 10 / @import 16 (blitz-dom-scoped → security.md); font-dependent tests skip without fonts (→ testing.md); `StyleThreading::Parallel` panics with two concurrent documents (→ gotchas.md); `:focus-visible` never matches (→ a11y.md); JS console → `log` target `js_console` (→ observability.md).
- **Pointer table (20):** arch · directory tree / occupied resources · standard contracts · code graph · test harness §3 · obs §3 · a11y §3 · security threat model · design tokens · layout · master-route · active version workspace (derived) · vision/requirements/intent (derived) · in-process Harness API · DOM entry points · accessibility tree · the stand · WPT runner · CI workflows · Andromeda bookkeeping (drift-base / playbook / handoff).
- **Workflow commands (5):** `cargo build --workspace` · `cargo test --workspace` · `cargo test -p blitz-tests --test {name}` · `cargo fmt --all --check && cargo clippy --workspace -- -D warnings` · `just seven_guis` (stand, windowed).
- **Architecture:** 2 paragraphs from arch §Design Philosophy.
- **@imports:** `@.claude/session-handoff.md` only.
- **USER:session-learnings seed:** empty placeholder line (no plan records an anti-pattern — the seed count is a ceiling).

## Tier 2 — .claude/rules/

| File | Paths | Source (rendered from measured facts only) |
|---|---|---|
| `security.md` | none (always loaded — kept lean) | security-plan: Secret Management, Input Validation trust boundaries, API Security, Dependency Security |
| `testing.md` | `tests/**`, `**/tests/**`, `wpt/runner/src/test_runners/**`, `.github/scripts/test_*.py` | test-plan §2 §4 §5 §7 §8 + arch §Conventions Tests |
| `observability.md` | `packages/debug_timer/**`, `**/telemetry/**`, `**/tracing/**`, `**/logging/**`, `**/*telemetry*.rs`, `**/*tracing*.rs`, `**/*logging*.rs`, `**/panic*.rs` | obs-plan §2 §3 §5 §6 §7 §8 |
| `a11y.md` | `packages/blitz-dom/src/accessibility.rs`, `packages/accesskit_xplat/**`, `packages/blitz-shell/src/accessibility.rs`, `packages/blitz-dom/src/events/{keyboard,focus}.rs`, `packages/blitz-paint/src/color.rs`, `packages/blitz-paint/src/render/form_controls.rs`, `packages/blitz-dom/assets/default.css`, `tests/blitz-tests/tests/{accessibility,focus}*.rs`, `examples/seven_guis/**` | a11y-plan §1–§6 |
| `verification-harness.md` | `scripts/agent-run.*`, `packages/blitz-test-harness/**`, `tests/blitz-tests/tests/harness_*.rs` | test-plan §3 measured in-process Harness; 5-command contract NOT YET MEASURED → owned by "Stand test contract" |

Skipped: `migrations` (only browser-persistence's rusqlite_migration — outside the escher surface), `api` (no served API), `events` (no event bus; DOM events are engine core), `frontend` (no TS/web frontend; WASM canvas only), `host-win32` (POSIX host).

## Tier 3 — .claude/docs/

- Core 5: `stack.md` (arch §Stack verbatim-mirror, condensed citations) · `conventions.md` (arch §Conventions) · `commands.md` (justfile + CI + arch CLIs) · `gotchas.md` (arch §Established Decisions traps) · `workflow.md` (Andromeda loop + git: build branch per version, conventional commits).
- Specialist summaries 5: security · design · tests · obs · a11y — each distilled from measured facts, NOT YET MEASURED rows named with their owning working-route chunk.
- Services (7): `blitz-dom` · `blitz-traits` · `blitz-paint` · `blitz-shell` · `blitz-test-harness` · `dioxus-native-dom` · `seven_guis` (the stand) — the crates the 0.1.0 route touches.
- `session-learnings.md`: create (missing).

## Agent harness

- Development Style agent-driven, BUT test-plan §3 5-command contract NOT YET MEASURED → **no script** rendered or validated. Card row: `none — test-plan §3 not measured → Stand test contract`. `verification-harness.md` rendered from measured facts only (above).

## Hooks (.claude/settings.json — new file)

- Formatter: `rustfmt "$f"` on `*.rs` (arch §Conventions `cargo fmt`; hooks-matrix Rust row). Linter: none at write time (clippy stays a gate). Type-checker: none (cargo check is the gate).
- PreToolUse: generated-dir write guard + Bash transport guard (matrix forms verbatim).
- env: `PYTHONUTF8=1`, `PYTHONIOENCODING=utf-8`.
- Formatter config: root `rustfmt.toml` absent → write `edition = "2024"` (Cargo.toml workspace edition). Pre-write `cargo fmt --all --check` exit 0 (tree conformant). Note: `debug_timer` and `wgpu_texture` are edition 2021 — `cargo fmt` passes their edition on the CLI; the write-time hook formats them 2024-style.

## Code-graph pipeline

- Planes: rust (root `Cargo.toml`); ts none (0 tracked `tsconfig.json`; design-system web-spa is a WASM canvas, not TS) → no forward note.
- Seed `scripts/{code-graph.py, code-graph-views.sql, scip_pb2.py, requirements.txt, code-graph-cookbook.md}` (all absent). Host: rust-analyzer on PATH · duckdb 1.5.5 + protobuf importable.
- Adopted → background `python scripts/code-graph.py refresh &` after seeding; P9 stamps `tree.db.commit` if `.refresh-done` exists.

## Code reviewer

- `.claude/agents/code-reviewer.md` from `code-reviewer-rust.md`, project name substituted.

## Gitignore / gitattributes

- `.gitignore` (existing, 12 lines): append base `.claude/backup/` · `.claude/settings.local.json` · `.andromeda/cache/` · `__pycache__/` (all unignored, measured) + rust fragment lines unignored by git's matcher: `target/` (nested — root `/target` only), `**/*.rs.bk`, `*.iml`, `.env`.
- `.gitattributes` absent; index all-LF (408 i/lf · 11 i/none · 11 i/-text, 0 i/crlf / i/mixed) → write template body; card names the operator re-checkout on an empty full porcelain.

## Seeds (Phase 6)

- `.andromeda/state.yaml` (lean, schema 3) · `.claude/session-handoff.md` skeleton · `.andromeda/drift-base.md` · `.andromeda/playbook.md` · `.claude/docs/session-learnings.md` — all absent → seed.

## Consistency check

- Every rule file named in Tier 2 is listed in Deeper Topics; agent harness row and verification-harness rule agree (no script, rule from measured facts); pointer table cites no baked version path except via "the active escher-X.Y.Z/" derivation; @imports = handoff only (seeded in P6 before P8's check 3).
