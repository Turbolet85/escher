# Working Route — escher-0.3.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve edits only the markerless tail; once the chunk's master_
_record is complete, wrap P7 flip-compacts its line to `[{marker}] {title} — {scope hint}`, archiving the_
_verbatim line to route-archive.md); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation
[2026-10-26-codebase-health] Codebase health — the lint's findings walk and two table-asserting tests under the complexity ceiling; test functions counted, engine function untouched (v030-01; Epoch 5 audit M1)
   ↓
[2026-10-27-readme-and-records] README and records — README shows 0.2.0 done, 0.3.0 planned; capabilities statement on the 2026-10-25 run; ledger refs, design stand rows, incubator ignore (v030-02, v030-16)

### Epoch 2 — Engine debts
[2026-10-27-upstream-sync-engine-debts] Upstream sync ahead of engine debts — the founder's chosen take; merge-surface owners kept; CI, a11y, audit green; per-OS tallies, sink allowlist re-read (v030-03)
   ↓
[2026-10-31-engine-debts] Engine debts — transformed scrolling box, `inert` focus fixup, native checkedness, select limits, select-option overlap, served pages' keyboard and contrast rows; each the founder's fork (v030-04)
   ↓
Select popup — a closed select opens a painted list of its options, chosen by pointer and key; the founder's fork (v030-17)  CARRY: the popup's reading of F9, `covered` and the two limits — the lint's `overlaps` exclusion is restated when options read real boxes open; `covered`'s closed-option reading ends here; a group's label and `size` are this entry's to paint or read; the focus and key contract, reduced motion and engine colours come to the founder as its P4 forks (record: .andromeda/runs/2026-10-31T03-24-40-phase/p5-card-1.md rows 1-2; `size` on his word, escher-0.3.0/chunks/2026-10-31-engine-debts/inputs/I12-engine-debts-founder-2026-10-31.md.txt)  CARRY: playbook Boundary widening — this entry's wrap halts once as the founder's answer card  CARRY: `.gitignore` gains `escher-0.4.0-incubator/` beside the two incubator lines it holds, before this entry's wrap's P1; the directory stands excluded in `.git/info/exclude` alone today  WATCH: the free-disk step in `Stand checks repeated`, not shown to be the cure of CI#41190771843's out-of-disk red (since 2026-10-31-engine-debts; retires: a recurrence, or 3 green runs — 0 so far)

### Epoch 3 — The UI graph
Upstream sync ahead of the UI graph — upstream/main merged, changes mostly additive; CI, a11y leg and audit green, per-OS tallies re-read, sink allowlist re-checked  CARRY: the upstream delta the take read at `ebacd68bd` left out — `.andromeda/residuals.md`'s left-out-delta line (git Stylo 0.23-pre, the six Taffy-main commits, Dioxus 0.8.0-alpha.3 with their ports) and six more, `a09f46792` · `527394724` · `d3ed4e2c7` · `ccfe15332` · `0980cce5f` · `ebacd68bd`, each with its measured reason — and the merge surfaces of the engine-debts fixes, the document clock and the select model (`.andromeda/residuals.md`'s merge-surfaces line), re-read row by row at that take with nothing sent upstream; both lines' owner is this entry (records: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/lock.md, evidence/readings.md, research.md §The trial takes; placed here on the operator's own word at that chunk's P5 card, escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/inputs/I5-relay-3.md.txt)  CARRY: the merge surfaces of the eight engine changes of 2026-10-31-engine-debts, each flagged upstreamable with nothing sent; `0980cce5f` has no hunk on `visible_region` and its `hit_inner` hunk is a textual conflict with the non-invertible answer (record: escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/merge-surface.md)  CARRY: the free-disk step of the `matrix_test` job (`.github/workflows/ci.yml:392-394`) is skipped on every matrix row, its `if:` naming `ubuntu-24.04` and `ubuntu-24.04-arm` while the android row runs `ubuntu-latest`, `completed · skipped` on CI#41190771843: fix its `if:` or remove it, this entry's CI read the witness (measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md)
   ↓
Dioxus runtime probe — components, handlers and state readable without a fork-level change; the reading reaches the founder before the graph (v030-05)  CARRY: the MountedData hazard — a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, and a `MountedData` call in it panics `RefCell already borrowed` at `events.rs:162`, any app meeting it (measured headlessly at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md; the windowed path read, not run); the reading that reaches the founder names it, and a fix is an engine change for every document, his fork
   ↓
The UI graph — per screen: node, component, handlers, written state, its readers; "reacts to a click" reading; values masked, never logged (v030-05)
   ↓
What can I do here — actionable nodes' verbs and current refusal causes, from graph and snapshot; actionable-key check, an operator-asked crossing, if it fits (v030-06)
   ↓
Handler coverage — one command: the handlers no recorded check exercised, read from the UI graph (v030-07)

### Epoch 4 — Determinism and exploration
Upstream sync ahead of determinism and exploration — upstream/main merged, changes mostly additive; CI, a11y leg and audit green, per-OS tallies re-read, sink allowlist re-checked
   ↓
Seeded randomness — a run's randomness from one seed; two runs of one seed read the same state (v030-13)
   ↓
Bundled fonts — a run's fonts bundled from the stand's DejaVu Sans; `fonts-unavailable` kept; every CI OS reads one state, never compared by shot bytes (v030-14)
   ↓
Start in a state — a session starts in a given fixture state, a bounded validated argument, on 0.2.0's shared fixtures (v030-08)
   ↓
Auto-exploration — breadth-first over available actions within a step budget; panics, refusals caught; each path an owner-only replayable check, no shot; navigation a boundary fork (v030-09)

### Epoch 5 — Errors and explanation
Upstream sync ahead of errors and explanation — upstream/main merged, changes mostly additive; CI, a11y leg and audit green, per-OS tallies re-read, sink allowlist re-checked
   ↓
Errors with the action — panic, error or handler log it caused, in the answer by node and handler; scrub held; widening the founder's fork (v030-10)
   ↓
Layout explained — why an element sits where it does at its size, from the style and layout chain; new fields per the widening rule (v030-11)
   ↓
A component in isolation — one component alone with given props, a validated widening, at the stand's viewport and font, across the graph's component boundary (v030-12)

### Epoch 6 — Polish & ship
Upstream sync ahead of polish and ship — upstream/main merged, changes mostly additive; CI, a11y leg and audit green, per-OS tallies re-read, sink allowlist re-checked
   ↓
Stand requirement sweep — 0.3.0 capabilities proven on stand or fixture inside stand-repeat; floors held, agent-run re-counted; stderr content-free; masks, stand layout held (intent §Principles)  CARRY: the 0.2.0 sweep test's leg check cannot see a target dropped from `stand-repeat` alone while the `a11y` leg still names it — it reads `ci-leg.sh` whole by `contains`, so stand-repeat membership is held only jointly with `test_stand_repeat_leg_runs_three_rounds` today; close that reach gap (measured at 2026-10-26-codebase-health's row-guard control (a), a gap predating it — record: escher-0.3.0/chunks/2026-10-26-codebase-health/report.md, Spec claims disproved; placed here on the operator's answer at that wrap's P2 halt, .andromeda/runs/2026-10-27T02-06-43-wrap/escalation-answers.md)  CARRY: `stand_requirement_sweep`'s `STAND_REPEAT_NAMED` holds the first ten of the leg's fifteen named targets; widen it, beside the reach gap the line's first CARRY names (read at 2026-10-31-engine-debts' citation sweep)  CARRY: three records 2026-10-31-engine-debts did not move: `.andromeda/obs-plan.md:76`'s twenty-nine `session_common` readers against 31 (escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md); Cells' 47,949 bytes and 740 lines, read before `cells-grid` took its id and label; and whether slice s03's no-tests absence (`.andromeda/test-plan.md:104`, `:237`) still holds now blitz-shell has a unit test
   ↓
A11y re-read — SC 2.1.1, 2.4.3, 1.4.3 re-asserted on stand and served pages after engine debts and bundled fonts; checks in the `a11y` leg (a11y-plan §9)
   ↓
Capability statement — what 0.3.0 adds, its stated limits and the README's 0.3.0 done, where Viola's planning reads it (intent §Principles)
   ↓
Cold-agent bar — cold-agent pipe widened past its twelve verbs, span counts with it; fresh agent over 0.3.0's capabilities; founder's bar from 0.2.0's baseline (v030-15)
