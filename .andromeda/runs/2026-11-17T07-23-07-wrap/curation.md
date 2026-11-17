# Curation — 2026-11-17-a-component-in-isolation

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A plan's CI wait is set from the last measured CI wall"
    Proof: the operator pass's entry 12 (`evidence/operator-pass.md`): the first `ci.py conclusion --sha HEAD --wait 2100` firing returned `verdict: in progress · … running 1: oldest Stand checks repeated 1419 s` past its bound; the second firing read `verdict: green · checks 17/17 · wall 2231 s` — the run's wall passed the plan's wait. Confidence 0.8: measured (+0.4), specific detail with context (+0.2), no other durable home (+0.2: no route annotation, no master row, no rule; the pass record names it for the next plan alone).
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 4 below threshold
  No-other-home: "A plan's CI wait is set from the last measured CI wall"
  CLAUDE.md size: 165/200 · T1 5.0 KB, 0 over 600 B

## Dispositions — every Decisions & corrections entry
Implement's:
- Sweep hazard: an `EventHandler` prop cannot be built outside a Dioxus runtime — rejected, Filter 4 (0.6: measured +0.4, detail +0.2; no conditional signal — this wrap amended it into test-plan `decision/component-alone-props-as-maker`).
- Sweep hazard: `Timer` reads `TimerTicks` through `try_use_context` with a wall-clock fallback — rejected, Filter 4 (0.6; amended into test-plan `decision/timer-context-read-by-delivered-ticks`).
- A component alone has no shell CSS, so read the viewport, never the root's bounds — rejected, Filter 4 (0.6; amended into test-plan `decision/pinned-viewport-read-from-document`).
- A per-package harness test of the UI graph needs the dev-dependency's `accessibility` feature — rejected, Filter 4 (0.6; amended into test-plan `decision/harness-graph-tests-enable-accessibility`).
- Standing learning followed (testing.md 2026-11-14, a control's case) — not a learning: no recurrence, a rule held.
- Guard refusal (a `cd` into the skill's references directory) — not a learning: the guard's own rule, re-issued once.
`## After implement`'s:
- The CI wall passed the plan's 2100 s wait, so the read fired twice — applied, Tier 3 (above).
- The wrap directive inputs#I4 (a card lists only what a cited word covers; no boundary skill) — not a learning: this wrap's own direction, carried out.
This window's own:
- The citation row `.claude/docs/session-learnings.md:346 host.rs:350` (out-of-range, curation home — P3) — no change: the number is a worked example of a prefix match inside a preserved entry, not a claim about host.rs.
- The fit reader's `stands` on a retire's heir, and the comparers' three missed stale counts — not learnings: telemetry of the tool, recorded as friction.
