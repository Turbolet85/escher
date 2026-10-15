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
[2026-10-06-snapshot-model] Snapshot model — screen as a tree of id, role, name, state, bounds (v010-04)
   ↓
[2026-10-06-id-stability-across-code-edits] Id stability across code edits — an element left in place keeps its id when the app's code around it changes (v010-16)
   ↓
[2026-10-06-snapshot-state-fidelity] Snapshot state fidelity — enabled, checked, value, focused per control; disabled reads disabled, typed value reads back, password values masked (v010-05)
   ↓
[2026-10-06-compact-snapshot-serialization] Compact snapshot serialization — whole stand screen readable in one tool result, size budget recorded (v010-04)
   ↓
[2026-10-07-change-tracking-and-diff] Change tracking and diff — changed-node set drained per step, truthful change flag, empty diff for a no-op (v010-06)

### Epoch 4 — Driver core
[2026-10-07-audit-corrections] Audit corrections — seven surviving dioxus-native-dom mutants killed or shown unreachable; id walk and three stand checks under the complexity ceiling; stand control tables stated once
   ↓
[2026-10-07-upstream-sync-driver-core] Upstream sync ahead of the driver core — upstream/main merged, our changes kept mostly additive; our tests and CI prove our logic survived (per intent §Principles)
   ↓
[2026-10-07-driver-session] Driver session — one headless stand instance held across commands, own lifecycle (start, attach, stop); the one process both CLI and MCP drive
   ↓
[2026-10-07-sink-target-allowlist] Sink target allowlist — escher's log sink drops every record from a target outside its allowlist; session host and windowed stand print no id or name at trace
   ↓
[2026-10-07-settle-detection] Settle detection — UI quiescence across render, layout, timers and pending loads; delayed stand update passes with no sleep (v010-10)
   ↓
[2026-10-07-command-and-refusal-schema] Command and refusal schema — one verb set, argument and result shapes, malformed arguments refused, refusal causes each with a remedy (v010-11, v010-14)
   ↓
[2026-10-07-act-by-id] Act by id — driver actions addressed by stable id, returning after settle with the diff (v010-09, v010-06)
   ↓
[2026-10-07-refusal-detection] Refusal detection — not found, stale, disabled, covered by another element, off-screen named per action (v010-11)
   ↓
[2026-10-07-driver-command-spans] Driver command spans — one span per driver command covering settle wait, diff size and refusal cause, through the scrub layer (per obs-plan §4 §8)

### Epoch 5 — Agent surfaces
[2026-10-09-audit-corrections-agent-surfaces] Audit corrections ahead of agent surfaces — the surviving snapshot mutant killed on a textarea fixture; fork CI's package install bounded and retried (the founder's ruling, 2026-10-09)
   ↓
[2026-10-10-upstream-sync-agent-surfaces] Upstream sync ahead of agent surfaces — upstream/main merged, our changes kept mostly additive; our tests and CI prove our logic survived (per intent §Principles)
   ↓
[2026-10-10-driver-cli] Driver CLI — every command with uncoloured JSON on stdout, diagnostics on stderr, accepted/refused exit codes, shell-scriptable stand flow (v010-12)
   ↓
[2026-10-10-scrolling-box-bounds-and-hit] Scrolling-box bounds and hit — a box that is scrolled reads its own bounds where it shows, and a hit never reaches content scrolled out of its box (the founder's ruling, 2026-10-10)
   ↓
[2026-10-10-snapshot-states] Snapshot states — a snapshot reads which control is selected, pressed or invalid, and the stand's tasks state all three (v010-17)
   ↓
[2026-10-10-mcp-surface] MCP surface — driver commands as MCP tools, local to the invoking user, no listener or auth surface, tool call parents its trace (v010-13)
   ↓
[2026-10-12-self-description] Self-description — verb list, help and schemas served by CLI and MCP from the one schema (v010-14)
   ↓
[2026-10-12-headless-screenshot] Headless screenshot — screen or one element by id, no display, identical pixels on repeat with bundled fonts; fails, never skips, without fonts (v010-07, v010-08)

### Epoch 6 — Polish & ship
[2026-10-13-upstream-sync-polish-and-ship] Upstream sync ahead of polish and ship — upstream/main merged, our changes kept mostly additive; our tests and CI prove our logic survived (per intent §Principles)
   ↓
[2026-10-13-stand-contrast-harness] Stand contrast harness — text/background pair of every stand control measured, per-pair result recorded (per a11y-plan §6)
   ↓
[2026-10-13-range-input-interaction] Range input interaction — arrow keys step a range input's value, a click sets it by position, an input event fires (the founder's ruling, 2026-10-07)
   ↓
[2026-10-13-stand-keyboard-harness] Stand keyboard harness — Tab, Shift+Tab and activation keys dispatched headlessly on the stand, focused node read back per step (per a11y-plan §5)
   ↓
[2026-10-14-stand-a11y-assertions] Stand a11y assertions — SC 2.1.1 keyboard reach, SC 2.4.3 focus order, SC 1.4.3 contrast on every stand control; a11y CI leg gating merges
   ↓
[2026-10-14-stand-requirement-sweep] Stand requirement sweep — every 0.1.0 capability proven headless by an agent across the stand tasks (per intent §Principles)
   ↓
[2026-10-14-quality-gates] Quality gates — coverage floor on driver crates and flakiness budget for stand checks, enforced on fork CI (per test-plan §10)
   ↓
Cold-agent test — fresh agent given only the tool completes a stand task and writes a passing check, wrong calls counted (v010-15)  CARRY: cold-agent run pipe (from 2026-10-06-cold-agent-run-pipe) — `scripts/cold-agent.sh` exists with a stdlib stdio MCP stub and the one task `counter`; this entry swaps the stub for the driver's MCP surface and `counter` for a stand task, keeping the isolation check, wrong-call count and positive-evidence verdict (test-plan §3). Measured at its live run (2026-10-06-cold-agent-run-pipe's report, Cross-project): `--allowedTools mcp__stub` (server-wide) let the MCP calls run in `-p` mode, and `--tools ""` removes only the built-in tools. Its crossings — the spawned `claude` client, the stdio stub, the outbound model path, the operator's Claude Code login — were ratified by the founder, 2026-10-06 (FOR DISCUSSION 7 closed); the founder's budget ruling was one live run, so a further live run needs the operator's word  CARRY: wrong-call bar (the founder's ruling, 2026-10-06, given as the 2026-10-06-snapshot-model wrap's directive) — 0.1.0 counts wrong calls and sets no bar on them: its run is the baseline that shows which values are reasonable; a bar starts in 0.2.0 (.andromeda/residuals.md)  CARRY: `covered` for a box with no height (from 2026-10-07-refusal-detection; the wrap's placement, on the entry that counts an agent's wrong calls — move it if it reads wrong) — a driver `click` on `task-header-spacer`, in each of the four lean tasks, is refused `covered`: its bounds are 0 high and the hit at its centre answers the header that holds it, while `timer-progress`, 0 wide at boot, is hit at its own centre and accepted, so a zero size does not predict the reading (measured at escher-0.1.0/chunks/2026-10-07-refusal-detection/evidence/stand-census.md); whether `covered` and its remedy — "another element covers this one: act on the covering element or dismiss it first" — are the right words for such a box is decided by nothing yet, and a cause's texts are reworded only with a recorded reason  CARRY: the first live trial, ahead of this test (2026-10-10, run by the operator on the founder's word — "now, small", by question dialog in the overseer session, relayed by the overseer) — a fresh agent given only the `escher-session` binary of `177d1652` behind a wrapper that logs every call, forbidden to read any file, did three tasks (the Counter to 3 · a one-way flight booked, then an earlier return and a non-date tried · three people added in the CRUD and the second deleted): 49 calls in 64 s — 39 exit `0`, 2 exit `1`, 1 exit `2`, 7 exit `3`; 3 unintended failures, all `state-dir-too-long`; 0 `unknown-verb`, 0 `malformed`, 0 unintended usage errors; no host left running; all three tasks done by the agent's own report (the figures are the wrapper's call log, which stays with the operator — the record is the overseer's relay, copied at .andromeda/runs/2026-10-10T13-01-58-wrap/relay-1.md); not exercised, so not measured: `press`, `--shift`, `advance`, `scroll`, the `timer` app and the idle expiry; the trial wrote no check and is not this test's run — v010-15's baseline is still to take
