# Curation — 2026-10-19-recorded-check-replay

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): retired «2026-10-18 — A gated wrap with later chunks committed on top of its flip» (the operator's word)
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff); 3 below threshold
  No-other-home: "a count pattern keyed on a target's name plus a number also matches a chunk marker ending in that word"
  Extended: T3/session-learnings.md: «2026-10-12 — A sweep pattern for a bare count also matches a citation's digits» + "a chunk marker ending in the pattern's word, followed by a history count"

Writes: 2 (one retirement, one extension).

## Applied
- **Retired** the Tier-3 entry «2026-10-18 — A gated wrap with later chunks committed on top of its flip», whole.
  Proof: the operator's resume directive (c), inputs#I5 (`relay-3.md`): «ES84's shared base resolver (gate v1.21) deployed and makes it redundant»; this wrap's tools printed `gate v1.21` and resolved the base themselves (`gate.py scope`: `base 6fd1d3da (the parent of the oldest pre-CI commit d2f010a2)`; `cites.py added` and `cascade.py sweep` the same line). No other curation home, playbook or drift-base line names it (`grep` over CLAUDE.md, `.claude/`, both bases: no other site).
- **Extended** «2026-10-12 — A sweep pattern for a bare count also matches a citation's digits» with the chunk-marker facet.
  Proof: report *Decisions & corrections*, «Sweep hazard»: the pre-pass pattern `rec-tests` matched `test-plan.md:22` @c227 and `session-lifecycle.md:11` @c254, each the marker `…session-recording` followed by a history count (pre-pass trail `cascade-2026-10-19-recorded-check-replay-pre-pass.json`). Score 0.4 (measured) + 0.2 (specific detail) + 0.2 (no other durable home: no master, route annotation, playbook or hook carries it) = 0.8.

## Decisions & corrections, each disposition
- A test-only variant of a stand task must be a component of the task's own name — rejected, Filter 4 (0.6: measured + detail; its home is the masters this wrap amended — test-plan session-proof and §1 now state «a test-local `Counter` variant of the task's own name»).
- A shot's PNG bytes differ between linux x86_64 and macOS aarch64 — rejected, Filter 4 (0.6; amended into architecture, test-plan and its key file this wrap, and the testing.md leaf states the script rule).
- The operator's directive I3 (stop on a cross-OS red, never loosen the comparison) — not a learning: a one-pass directive, followed and recorded in the report.
- The correction to the operator's record line (I4's «same-host CLI-vs-MCP test») — not a learning: corrected at its source, the masters (architecture, test-plan and its key file) record `stand_session_recording` as the shot path's coverage, on linux and macOS.
- Guard refusal of a `cat` heredoc writing a scratch script (implement P1) — → handoff, `recurrence-despite-learning` of host-linux §Transports (the guard held at one re-issue).
- A tracked evidence file written after the pre-CI commit trips the push entry's clean-tree guard — rejected, Filter 4 (0.2: specific detail only; no failure measured, the pass avoided it).
- Sweep hazard `recording\W{0,3}\d+` — extended (above).
- A standing miscount (`architecture.md:333` «twenty-three readers») — not a learning: a fact amended at P2.
- Two-window wrap — not a learning: the invocation's form.
- This window: the obs-plan.md:70 provenance FAIL from an insertion before a standing parenthesis — → handoff, `recurrence-despite-learning` of the 2026-10-19 Tier-3 entry «An in-line edit beside a provenance parenthesis reads as new provenance».
- This window: the out-of-range citation row `session-learnings.md:175 host.rs:350` — no change: the line quotes `host.rs:350` as an example, not a citation of the tree.
- This window: `cites.py apply`'s stdout sent to `/dev/null` and recovered from its trail — not a learning: the skill's own rule already says it (evolve record).
- This window: directive (b)'s «each OS» against a `#![cfg(unix)]` file — not a learning: the masters say linux and macOS; reported to the operator.
