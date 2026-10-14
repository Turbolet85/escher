# Curation — 2026-10-13-stand-keyboard-harness (P3, resumed 2026-10-14)

CLAUDE.md ecosystem: no new learnings this session.
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 4 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 3 below threshold

## The report's *Decisions & corrections*, each weighed
- Guard refusal, implement P1 (a `cat`/`tee` heredoc with a file target) — not a learning: the guard enforces it, and
  host-linux.md §Transports already states the rule.
- Measured engine fact (with nothing focused `Harness::focused()` / `get_focussed_node_id` returns the root, never
  `None`) — rejected, Filter 4: verified by a red run +0.4, specific detail +0.2 = 0.6, exactly at the threshold; the
  no-other-home signal does not fire because this wrap amended it into a11y-plan §5 (Test harness pattern, the
  `stand_keyboard` bullet), which now owns it.
- Sweep hazard, the driver's key names are hyphenated (`arrow-down`) — rejected, Filter 4: a refused call +0.4, detail
  +0.2, one-off −0.3 = 0.3; `help`'s `key_names` states the names.
- Recurrence of a standing learning (a pipeline tool's output read through `command grep`) — recurrence, not a new
  learning: host-linux.md Session Additions 2026-10-13 states the rule; logged by P1's evolve checkpoint
  (`recall.corpus-recurrence`), and → handoff as `recurrence-despite-learning`.
- A miscount caught by its own gate (`stand_act_ids` counts control rows) — rejected, Filter 2: task-specific; the
  amended test-plan §1 states "15 control rows over `controls(task)`".
- The derived scope row worked as designed — not a learning: a positive, recorded in the report.
- Operator directive at the operator pass — not a learning: this chunk's instruction, followed.
- Operator directive at this wrap ("run P1 only and stop") — not a learning: this wrap's two-window form.
- Phase's snapped words (inputs#I1, I4, I5, I6) — rejected, Filter 2: I4–I6 are this chunk's conditions, each landed in
  the masters; I1's two standing rules are already homed (host-linux.md 2026-10-13 for bare pipeline tools; the
  operator's auto-memory for founder-owned forks).
- For P5 / Failed commands for P6 — not learnings: P5 and P6 inputs.

## This window
- Spec-body lines of 3–46 KB edited through a scratchpad exact-once replace script — not a learning: a workaround
  recorded in P2's evolve record.
- `cd` into a run dir refused by the Bash guard (twice) — not a learning: the guard refuses the act, and host-linux.md
  §Paths states it.
- An architecture sidecar payload over 3 000 B, cut twice before its append — rejected, Filter 2: `sidecar.py check`
  catches it by construction.
