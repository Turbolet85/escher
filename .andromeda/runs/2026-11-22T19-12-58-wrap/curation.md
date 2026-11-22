# Curation — 2026-11-22-epoch-2-corrective (wrap 2026-11-22T19-12-58-wrap)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A lock move is proven by a locked build, not by the audit leg"
    Proof: the plan's lock move (`cargo update -p url --precise 2.5.9`) was measured at P3 / P5 by the audit leg alone (`evidence/audit-url-2.5.9-scratch.log`); implement's first firing read entries 8–14 red on E0277 in `bindgen` 0.73.2 under `syn` 2.0.119 (report, Spec claims disproved; Decisions & corrections, fct-nejzrmgf). Confidence 0.8: verified by measurement +0.4 · specific technical detail +0.2 · load-bearing +0.2.
  Filters: 0 dup · 2 task-specific · 0 conflict · 0 deferred
  Load-bearing: "A lock move is proven by a locked build, not by the audit leg" → Upstream sync ahead of the canvas
  CLAUDE.md size: 166/200 · T1 5.7 KB, 0 over 600 B

## Every Decisions & corrections entry, with its disposition
Implement's:
- The operator's ruling inputs#I5 «Keep syn edges (Recommended)» on the url lock — not a learning: a ruling on this chunk's lock; its hazard routed to a residual row at P5 (inputs#I6 point 3).
- A phase control that fires only the audit leg on a lock move proves no build — applied, Tier 3 (above).
- This run's own claim corrected: only a `url` update re-resolves the two `syn` edges — rejected, Filter 2 (task-specific: dependency versions and edges the code states); it is the residual row's own text at P5.
- A plan's call site "after" an arm that takes its input by value cannot hold a later use of that input — rejected, Filter 4: 0.4 (measured +0.4, specific detail +0.2, could be task-specific −0.2 — general Rust semantics met once, at one call site).
- Guard refusal: a `cd` into the skill's references directory — not a learning: a guard refusal at the cost of its one re-issue (host-linux.md's paths rule already states the form).
- The evolve tool refused one record typed ahead of the clock — not a learning: the tool's own telemetry (the friction stream is not curated).
After implement's (the close):
- The operator's wrap directive inputs#I6 — not a learning: this wrap's directions, each carried out at its phase.
- The operator's `--wait 2700` on entry 20 — not a learning: a one-time direction.
This wrap's conversation:
- Proposers wrote standing master rows from a chunk-scoped ruling and from a pipeline lesson (four adds rejected at Validate) — not a learning: pipeline telemetry, recorded as a friction record at reconcile.
- Citation dispositions row: `.claude/docs/session-learnings.md:424 host.rs:350` out-of-range — no change: an illustrative number in a learning's prose about prefix-matching citations, no claim about `host.rs`; preserved verbatim.
