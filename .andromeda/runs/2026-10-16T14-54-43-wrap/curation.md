# Curation — 2026-10-16-handler-splits-and-readme

CLAUDE.md ecosystem: no new learnings this session.
  Filters: 0 dup · 2 task-specific · 0 conflict · 0 deferred · 5 below threshold or not a learning
  CLAUDE.md size: read at P7 (`health.py check`, check 1)

Scope: this window's conversation (the resumed P2–P7) and the report's *Decisions & corrections*, each entry weighed.

Dispositions, one per *Decisions & corrections* entry:
- The operator ran the operator pass through the agent, in-session (inputs#I3) → not a learning: a one-chunk directive, recorded in the report and `evidence/ci.md`.
- Entry 17 fired without `--run-dir` → rejected — Filter 4: unmeasured whether a run dir's trail would have reddened the clean-tree guard (the report says so); 0.2 (technical detail) − 0.3 (one-off) < 0.6.
- The two-window wrap (inputs#I4) → not a learning: an operator's per-wrap directive.
- Sweep hazard — `cognitive` reads two subjects (a11y-plan §8 "Cognitive Accessibility" vs the complexity metric) → rejected — Filter 4: +0.4 (measured, the P1 sweep's 2 standing hits read) +0.2 (detail) −0.3 (one-off, no emphasis) −0.2 (could be task-specific: one token, two sites) = 0.1.
- Instrument with known-verdict controls before any source edit → rejected — Filter 4: +0.4 (measured: red on the untouched tree, green on upstream copies) +0.2 (detail) −0.2 (paraphrased from the plan's own step 1) −0.3 (one-off) = 0.1; the practice already stands in testing.md's control entries (2026-10-09).
- README figures re-read at their sources → rejected — Filter 4: implicit behaviour, 0.2 − 0.2 = 0.0.
- A probe defect, the agent's own (the inputs manifest is a JSON list) → rejected — Filter 2, task-specific (one file's shape, an error message).
- Recurrence of standing learnings: none observed (P1's window) → nothing to curate.

This window:
- A `cd` into the skill's references directory chained before a grep was refused by the Bash guard and re-issued once with an absolute path → not a recurrence: the guard held host-linux.md's `cd` rule at the cost of its one re-issue (recorded as `guard-refused` on the reconcile step record); nothing to curate.
