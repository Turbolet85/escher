# Working Route — escher-0.1.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve edits only the markerless tail; once the chunk's master_
_record is complete, wrap P7 flip-compacts its line to `[{marker}] {title} — {scope hint}`, archiving the_
_verbatim line to route-archive.md); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation
[2026-10-05-as-built-baseline] As-built baseline — workspace build and blitz-tests green on this host, one run's wall-clock recorded
   ↓
Fork CI reached — escher build-branch pipeline green, host-reproducible legs run locally, cached builds, fast/slow split, failure artifacts uploaded, signing-secret jobs excluded
   ↓
CI gate legs — dependency audit, pinned actions, least-privilege tokens, coverage report, named a11y leg on fork CI (per security-plan, test-plan §9, a11y-plan §9)  CARRY: a real rustdoc gate — CI's docs job runs bare `cargo doc`, which documents only the lib-less root package `blitz-examples`, and `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` is red: exit 101, 3 crates not documented (blitz-dom, blitz-vibey-script, example transparent), 9 rustdoc errors, plus a `target/doc/blitz/index.html` output-filename collision (bin `blitz` of `browser` vs lib `blitz`) — measured at 2026-10-05-as-built-baseline evidence/baseline.md §Wider gates; owner named at that chunk's P5 review
   ↓
Telemetry bootstrap — tracing subscriber, opt-in OTel export, service identity, panic logging, scrub layer; logs never on stdout (per obs-plan §3 §8)
   ↓
Headless stand — seven_guis counter, flight booker, timer, CRUD in TaskShell; no display, fixed viewport, bundled fonts, no live network, fresh per check
   ↓
Stand test contract — agent-invocable boot, run, status, cleanup and JSON-line logs for stand checks and blitz-tests (per test-plan §3)
   ↓
Cold-agent run pipe — fresh agent session given only a stub tool; transcript, wrong-call count and verdict recorded green

### Epoch 2 — Element identity
Stable element ids — author key else component path, on every stand element (v010-01)
   ↓
Id persistence — same id across re-render, remount and fresh process on the stand (v010-02)
   ↓
Accessibility-tree identity — stable id on every accessibility node, stand controls carrying role and name (v010-03; per a11y-plan §2)

### Epoch 3 — Observation model
Snapshot model — screen as a tree of id, role, name, state, bounds (v010-04)
   ↓
Snapshot state fidelity — enabled, checked, value, focused per control; disabled reads disabled, typed value reads back, password values masked (v010-05)
   ↓
Compact snapshot serialization — whole stand screen readable in one tool result, size budget recorded (v010-04)
   ↓
Change tracking and diff — changed-node set drained per step, truthful change flag, empty diff for a no-op (v010-06)

### Epoch 4 — Driver core
Driver session — one headless stand instance held across commands, own lifecycle (start, attach, stop); the one process both CLI and MCP drive
   ↓
Settle detection — UI quiescence across render, layout, timers and pending loads; delayed stand update passes with no sleep (v010-10)
   ↓
Command and refusal schema — one verb set, argument and result shapes, malformed arguments refused, refusal causes each with a remedy (v010-11, v010-14)
   ↓
Act by id — driver actions addressed by stable id, returning after settle with the diff (v010-09, v010-06)
   ↓
Refusal detection — not found, stale, disabled, covered by another element, off-screen named per action (v010-11)
   ↓
Driver command spans — one span per driver command covering settle wait, diff size and refusal cause, through the scrub layer (per obs-plan §4 §8)

### Epoch 5 — Agent surfaces
Driver CLI — every command with uncoloured JSON on stdout, diagnostics on stderr, accepted/refused exit codes, shell-scriptable stand flow (v010-12)
   ↓
MCP surface — driver commands as MCP tools, local to the invoking user, no listener or auth surface, tool call parents its trace (v010-13)
   ↓
Self-description — verb list, help and schemas served by CLI and MCP from the one schema (v010-14)
   ↓
Headless screenshot — screen or one element by id, no display, identical pixels on repeat with bundled fonts; fails, never skips, without fonts (v010-07, v010-08)

### Epoch 6 — Polish & ship
Stand contrast harness — text/background pair of every stand control measured, per-pair result recorded (per a11y-plan §6)
   ↓
Stand keyboard harness — Tab, Shift+Tab and activation keys dispatched headlessly on the stand, focused node read back per step (per a11y-plan §5)
   ↓
Stand a11y assertions — SC 2.1.1 keyboard reach, SC 2.4.3 focus order, SC 1.4.3 contrast on every stand control; a11y CI leg gating merges
   ↓
Stand requirement sweep — every 0.1.0 capability proven headless by an agent across the stand tasks (per intent §Principles)
   ↓
Quality gates — coverage floor on driver crates and flakiness budget for stand checks, enforced on fork CI (per test-plan §10)
   ↓
Cold-agent test — fresh agent given only the tool completes a stand task and writes a passing check, wrong calls counted (v010-15)
