# Curation — 2026-11-08-what-can-i-do-here (wrap 2026-11-08T15-29-03, resumed)

CLAUDE.md ecosystem: no new learnings this session.
  Filters: 2 dup (recurrence) · 0 task-specific · 0 conflict · 0 deferred (cap) · 1 below threshold
  CLAUDE.md size: see the P7 report's `check 1` row.

## The report's Decisions & corrections, each weighed
- The operator's direction for the operator pass — not a learning: a one-off direction for one pass, carried out as written.
- The operator's invocation «run P1 only and stop» — not a learning: the two-window form the skill already names.
- A standing learning recurred (testing.md 2026-10-10, extended 2026-10-25: search `tests/` for what a check pins before changing a table) — rejected, Filter 1 recurrence: the matched entry records a defect and its remedy and did not prevent the miss → handoff, `recurrence-despite-learning`.
- Count pin missed by research's sweep (`stand_act_spans`' `run.calls`) — rejected, Filter 1 recurrence of the same entry (a count a check pins, unsearched before the change) → folded into the same handoff line.
- Sweep hazard: `grep -c '#\[test\]'` over `cli_commands.rs` misreads its per-OS count (two `#[cfg]` alternates) — rejected, Filter 4: specific technical detail +0.2, repeated across two wraps' measurements +0.3 = 0.5, below the threshold; no gate failed on it.
- Guard refusals (two `cd` at implement, one filtered pipe at P1) — not a learning: each cost its one re-issue (`guard-refused`), the guard putting the call back on the documented path.
- Design decision: the in-tree discrimination witness over a clone control — not a learning: recorded in the masters this wrap (test-plan unit-test-strategy, a11y screen-reader-support).
- Design decision: one snapshot and one `element_ids` map per read — not a learning: recorded in architecture's Driver session detail this wrap.
- Measurement recorded, not assumed (`ACTIONABLE_AT_BOOT`, the Cells answer's size) — not a learning: a measurement, landed in test-plan and a11y-plan this wrap.
- The two `nodes` result fields — not a learning: landed in architecture and test-plan unit-test-strategy this wrap.
- A fetch put in flight without the driver's wait — not a learning: landed in test-plan integration-test-strategy and test-data-fixtures this wrap.

## This window's own candidates
- None from the operator: inputs#I6 is a ruling on this chunk (the replay test owed, its CARRY), landed in test-plan and security-plan and carried at P5, not a convention.

## The citation sweep's curation-home row
- `.claude/docs/session-learnings.md:301` cites `host.rs:350` as an illustrative prefix example, not a claim about the file — no change; the entry is preserved verbatim.
