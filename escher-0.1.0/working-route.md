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
[2026-10-05-fork-ci-reached] Fork CI reached — escher build-branch pipeline green, host-reproducible legs run locally, cached builds, fast/slow split, failure artifacts uploaded, signing-secret jobs excluded
   ↓
[2026-10-05-ci-gate-legs] CI gate legs — dependency audit, pinned actions, least-privilege tokens, coverage report, named a11y leg on fork CI (per security-plan, test-plan §9, a11y-plan §9)
   ↓
[2026-10-06-telemetry-bootstrap] Telemetry bootstrap — tracing subscriber, opt-in OTel export, service identity, panic logging, scrub layer; logs never on stdout (per obs-plan §3 §8)
   ↓
[2026-10-06-headless-stand] Headless stand — seven_guis counter, flight booker, timer, CRUD in TaskShell; no display, fixed viewport, bundled fonts, no live network, fresh per check
   ↓
[2026-10-06-stand-test-contract] Stand test contract — agent-invocable boot, run, status, cleanup and JSON-line logs for stand checks and blitz-tests (per test-plan §3)
   ↓
[2026-10-06-cold-agent-run-pipe] Cold-agent run pipe — fresh agent session given only a stub tool; transcript, wrong-call count and verdict recorded green

### Epoch 2 — Element identity
[2026-10-06-upstream-sync-element-identity] Upstream sync ahead of element identity — upstream/main merged, our changes kept additive; our tests and CI prove our logic survived (per intent §Principles)
   ↓
[2026-10-06-stable-element-ids] Stable element ids — author key else component path, on every stand element (v010-01)
   ↓
[2026-10-06-id-persistence] Id persistence — same id across re-render, remount and fresh process on the stand (v010-02)
   ↓
[2026-10-06-project-readme] Project README — the repository front page describes escher, not Blitz: what it is, why it exists, its Blitz lineage, the plans, a contact
   ↓
[2026-10-06-accessibility-tree-identity] Accessibility-tree identity — stable id on every accessibility node, stand controls carrying role and name (v010-03; per a11y-plan §2)

### Epoch 3 — Observation model
[2026-10-06-upstream-sync-observation-model] Upstream sync ahead of the observation model — upstream/main merged, our changes kept additive; our tests and CI prove our logic survived (per intent §Principles)
   ↓
[2026-10-06-snapshot-model] Snapshot model — screen as a tree of id, role, name, state, bounds (v010-04)  CARRY: upstream inline-geometry APIs (from 2026-10-06-upstream-sync-element-identity) — the `23354585` merge brought `Node::inline_fragment_boxes` (per-fragment `taffy::Rect<f32>` boxes of a non-atomic inline) and `BaseDocument::inline_fragment_rects` returning `Option<impl Iterator<Item = BoundingRect>>` (was a `Vec`); no escher code calls either yet — candidates for an inline element's snapshot bounds (that chunk's report, Changes → Symbols / APIs)
   ↓
Snapshot state fidelity — enabled, checked, value, focused per control; disabled reads disabled, typed value reads back, password values masked (v010-05)  CARRY: Dioxus boolean attributes (from 2026-10-06-headless-stand) — dioxus-native-dom now removes a falsy `disabled` / `checked`, but still writes `readonly`, `required`, `hidden`, `multiple`, `selected`, `open`, `autofocus` as the literal `"false"`, which a presence read takes as set (arch §Standard Contracts → Dioxus DOM bridge); and blitz-dom reads `disabled` two ways — presence for the DISABLED state, `:disabled` and click targeting, a parsed bool for focusability (arch §Established Decisions → DOM semantics) — so a snapshot's enabled/disabled must pick one reader; hypothesis: a Dioxus `hidden: false` node drops out of the accessibility tree (not measured)
   ↓
Compact snapshot serialization — whole stand screen readable in one tool result, size budget recorded (v010-04)
   ↓
Change tracking and diff — changed-node set drained per step, truthful change flag, empty diff for a no-op (v010-06)

### Epoch 4 — Driver core
Upstream sync ahead of the driver core — upstream/main merged, our changes kept additive; our tests and CI prove our logic survived (per intent §Principles)
   ↓
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
Upstream sync ahead of agent surfaces — upstream/main merged, our changes kept additive; our tests and CI prove our logic survived (per intent §Principles)
   ↓
Driver CLI — every command with uncoloured JSON on stdout, diagnostics on stderr, accepted/refused exit codes, shell-scriptable stand flow (v010-12)
   ↓
MCP surface — driver commands as MCP tools, local to the invoking user, no listener or auth surface, tool call parents its trace (v010-13)
   ↓
Self-description — verb list, help and schemas served by CLI and MCP from the one schema (v010-14)
   ↓
Headless screenshot — screen or one element by id, no display, identical pixels on repeat with bundled fonts; fails, never skips, without fonts (v010-07, v010-08)

### Epoch 6 — Polish & ship
Upstream sync ahead of polish and ship — upstream/main merged, our changes kept additive; our tests and CI prove our logic survived (per intent §Principles)
   ↓
Stand contrast harness — text/background pair of every stand control measured, per-pair result recorded (per a11y-plan §6)
   ↓
Stand keyboard harness — Tab, Shift+Tab and activation keys dispatched headlessly on the stand, focused node read back per step (per a11y-plan §5)
   ↓
Stand a11y assertions — SC 2.1.1 keyboard reach, SC 2.4.3 focus order, SC 1.4.3 contrast on every stand control; a11y CI leg gating merges  CARRY: author_id on the platform tree (from 2026-10-06-accessibility-tree-identity) — the operator ratified, at that wrap (2026-10-06, P2 escalation, "Ratify + track"), the stable element id leaving the process as AccessKit `author_id` through the platform accessibility adapter; assert here that no `author_id` on the tree the shell hands the adapter carries a `NodeId`, `ElementId`, `ScopeId` or pointer form (today's headless proof: `stand_accessibility_ids`, which reads `DioxusDocument::accessibility_tree`, not the platform adapter) (security-plan §Input Validation `id` row)  CARRY: the stand binary builds no AccessKit (from 2026-10-06-accessibility-tree-identity) — seven_guis takes dioxus-native with the workspace's `default-features = false` and names no `accessibility` feature, so `just seven_guis` compiles no platform adapter and no AT can read the windowed stand (measured at that chunk: `cargo tree -p seven_guis -e features -i dioxus-native` lists only prelude · system-fonts · vello-hybrid · woff); whether the stand enables it is decided at this entry's promotion, on the operator's placement 2026-10-06 (a11y-plan §1 Dioxus crates)
   ↓
Stand requirement sweep — every 0.1.0 capability proven headless by an agent across the stand tasks (per intent §Principles)
   ↓
Quality gates — coverage floor on driver crates and flakiness budget for stand checks, enforced on fork CI (per test-plan §10)  CARRY: audit reach — cargo-deny 0.20.2's resolved graph prunes `http-cache` (blitz-net's `cache` feature) and `ravif`, so RUSTSEC-2024-0436 (paste 1.0.15, unmaintained) and RUSTSEC-2026-0186 (memmap2 0.5.10, unsound) never reach the `audit` leg though both are in the build graph; close it (a lockfile-wide scan beside it, or a graph that reaches them) — measured at 2026-10-05-ci-gate-legs evidence/audit.md, security-plan §Dependency Security; pinned on the overseer's word at that chunk's wrap (delegate overseer, under the founder's standing delegation of technical decisions)  CARRY: release metadata (from 2026-10-06-project-readme) — the root `Cargo.toml` `homepage` and `repository` still read `https://github.com/dioxuslabs/blitz` and become escher's (the fork, `https://github.com/Turbolet85/escher`); the root README already is escher's page (architecture §Project Intent → Front page) — directed by the operator at the project-readme P5 review, 2026-10-06, and pinned to this entry on the operator's answer at that chunk's wrap, 2026-10-06
   ↓
Cold-agent test — fresh agent given only the tool completes a stand task and writes a passing check, wrong calls counted (v010-15)  CARRY: cold-agent run pipe (from 2026-10-06-cold-agent-run-pipe) — `scripts/cold-agent.sh` exists with a stdlib stdio MCP stub and the one task `counter`; this entry swaps the stub for the driver's MCP surface and `counter` for a stand task, keeping the isolation check, wrong-call count and positive-evidence verdict (test-plan §3). Measured at its live run (2026-10-06-cold-agent-run-pipe's report, Cross-project): `--allowedTools mcp__stub` (server-wide) let the MCP calls run in `-p` mode, and `--tools ""` removes only the built-in tools. Its crossings — the spawned `claude` client, the stdio stub, the outbound model path, the operator's Claude Code login — were ratified by the founder, 2026-10-06 (FOR DISCUSSION 7 closed); the founder's budget ruling was one live run, so a further live run needs the operator's word
