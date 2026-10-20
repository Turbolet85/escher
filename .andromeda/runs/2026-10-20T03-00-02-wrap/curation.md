CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-linux.md: "A whole-tree `grep -rn` from the repository root walks `target/` and runs past the Bash tool's 120 s limit — search tracked files with `git grep`, which answers at once." (confidence 0.8)
    Proof: report.md Decisions & corrections, the search sweep hazard — the implement window's whole-tree `grep -rn` ran past the Bash tool's 120 s and was moved to the background, `git grep` answering the same question at once (a real failure +0.4, specific detail +0.2, no other durable home this wrap +0.2).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific / homed · 0 conflict · 0 deferred (cap) · 4 recurrence-despite-learning (→ handoff)
  No-other-home: "A whole-tree `grep -rn` from the repository root walks `target/` … use `git grep`"
  CLAUDE.md size: 150/200 · T1 3.5 KB, 0 over 600 B

Decisions & corrections, each entry's disposition:
- The `p5-controls.sh` hygiene red (a phase script holding the repo root, a scratchpad path and a home path) → recurrence-despite-learning: «2026-10-13 — A phase control that reads a scratchpad file lands a host temp path in the run dir» (session-learnings.md) → handoff Deferred learnings.
- The operator's standing «Stop and tell me if any entry is red» → not a learning: a directive for that operator pass, followed.
- Plan step 1's two stopping clauses disagreeing → not a learning: pipeline telemetry, recorded as evolve friction `2026-10-20T02:07:01Z-b`.
- `$TMPDIR` unset in the Bash tool's shell (a redirect to `/claude-cov.log` refused) → recurrence-despite-learning: «2026-10-07 — `$TMPDIR` is unset in the Bash tool's shell on this host» → handoff Deferred learnings.
- A whole-tree `grep -rn` running past 120 s; `git grep` at once → applied, Tier 2 host-linux.md.
- rustc reading an import used only by an intra-doc link as unused → recurrence-despite-learning: «2026-10-16 — Moving Rust items between modules strands two kinds of import» → handoff Deferred learnings.
- A `stand_*` check may sleep and read the wall clock where its subject is wall time → rejected (homed): amended into test-plan §5 by this wrap's P2.
- This wrap (P2): the report's digit-keyed site sweep missed a count in words («twenty-six checks») and two other forms → recurrence-despite-learning: «2026-10-20 — A count stated in more than one form needs every form in its sweep pattern» → handoff Deferred learnings.
- This wrap (P2): a leaf comparer returned with its own background job open → not a learning: pipeline telemetry (evolve friction `2026-10-20T03:32:15Z-d`).
