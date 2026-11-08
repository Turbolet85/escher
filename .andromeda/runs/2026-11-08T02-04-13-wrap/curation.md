CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-linux.md: "A Python edit script that writes a backslash-escaped quote into Rust source carries it in a raw string, or the edit goes through the Edit tool"
    Proof: the report's sweep hazard — `json::tests::each_outcome_is_written_with_its_verbs_fields_in_order` read red `row 11` after a heredoc edit turned `a\"b` into `a"b` in a Rust raw string's expected JSON (doc:2026-11-07-ui-graph/report, Decisions & corrections); a real gate failure (+0.4), specific detail (+0.2), no other durable home this wrap (+0.2) = 0.8.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 3 dup (a home in a master this wrap) · 3 task-specific · 0 conflict · 0 deferred · 1 recurrence (→ handoff)
  No-other-home: "A Python edit script that writes a backslash-escaped quote into Rust source carries it in a raw string, or the edit goes through the Edit tool"
  CLAUDE.md size: 153/200 · T1 4.3 KB, 0 over 600 B

Dispositions of the report's Decisions & corrections:
- Operator correction inputs#I7 (readers as components) — not a learning: a product decision, amended into the masters (architecture, a11y-plan, test-plan) this wrap.
- Operator correction inputs#I8 (writers confirmed or flagged; the size a stated limit) — not a learning: a product decision, in the masters and residual `res-7cc5bi4k`.
- Operator word at P5 inputs#I6 (`unconfirmed` fires only after the handler ran) — not a learning: a product rule, amended into test-plan and architecture.
- Operator word «run P1 only and stop» — not a learning: this wrap's two-window form, a process direction.
- Measured: an async handler's spawned task runs inside its dispatch (Dioxus 0.7.10) — rejected, Filter 4 at 0.6 (measured +0.4, detail +0.2): amended into architecture's cross-cutting-patterns this wrap, so no-other-home does not fire.
- Measured: a value-reading graph over Cells costs far more than a per-cell forecast — rejected, Filter 2 (task-specific figures) and a master home (test-plan test-data-fixtures, architecture standard-contracts).
- Rust gotcha: `&[writes(..)]` with a `const fn` is not promoted to `'static` (E0716) → `const { &[…] }` — rejected, Filter 4 at 0.6: amended into architecture's conventions (Apps and examples) and its leaf this wrap.
- Sweep hazard: a Python heredoc's non-raw string turned `\"` into `"` in a Rust raw string — applied, Tier 2 host-linux.md (above).
- Recurrence of the standing learning (testing.md 2026-10-10: search `tests/` for a table's readers before calling a standing check unaffected) — four checks outside research's lists read red (`stand_act_scroll`, `stand_cells_served`, `stand_snapshot_parts`, `stand_requirement_sweep`) — not a new entry: `recurrence-despite-learning`, logged to the handoff's Deferred learnings.
- Guard refusals (a `cd` into a skill dir, a `cat` heredoc, a `rec.py --help` piped to `head`) — not a learning: each guard put its call back on the documented path at one re-issue.
- Process: the companion scope-record lines were written at P2's scope read, not at each edit — rejected, Filter 4 (one-off, -0.3).
- Decision: `GRAPH_DRIVES` drives each conditional write once — not a learning: a product decision, amended into test-plan and architecture.
