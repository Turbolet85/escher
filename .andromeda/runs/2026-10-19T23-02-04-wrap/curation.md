CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A new name that is a prefix of the name it replaced counts the old sites" · + "An evidence time is read, never estimated"
  Filters: 1 dup · 0 task-specific · 0 conflict · 1 below threshold · 0 deferred
  No-other-home: "A new name that is a prefix of the name it replaced counts the old sites"
  CLAUDE.md size: 150/200 · T1 3.5 KB, 0 over 600 B

## Applied
- Tier 3 «A new name that is a prefix of the name it replaced counts the old sites» — confidence 0.8: verified by
  measurement +0.4 (it changed P1's sweep pattern: the report's Decisions & corrections «the token `HeldNet` is a prefix
  of the removed `HeldNetProvider` … this wrap's pattern used `HeldNet\b`»), specific technical detail +0.2, reached no
  other durable home +0.2 (no master, route annotation, playbook or hook carries it; the other signals total exactly
  0.6). Tier 3: a sweep hazard whose artifact (the `.andromeda` masters) no rule file's `paths:` covers.
  Proof: escher-0.2.0/chunks/2026-10-19-shared-fixtures/report.md, Decisions & corrections, «Sweep hazard».
- Tier 3 «An evidence time is read, never estimated» — confidence 0.7: verified by a real gate failure +0.4 (the
  stamp-ahead PostToolUse hook refused an estimated end time in `evidence/counts.md`), «never» language +0.3. Tier 3:
  it spans chunk evidence of every kind, with no path-scoped rule home.
  Proof: escher-0.2.0/chunks/2026-10-19-shared-fixtures/report.md, Decisions & corrections, «Hook refusal».

## Decisions & corrections — dispositions
- Plan-step ambiguity (a `serve_mcp` boot closure named as a `fixtures::hold` site) → rejected, Filter 4: 0.3
  (verified +0.4, detail +0.2, one-off −0.3); carried by implement's friction record and Deviation 1 of the report.
- Guard refusal (`cd` into `tests/blitz-tests/tests` at implement P1; and twice in this window, into the skill's
  references and into the run dir, each re-issued once) → rejected, Filter 1: a duplicate of host-linux §Paths («`cd`
  only inside a subshell»). Not a recurrence: the Bash guard refused each act at the cost of its one re-issue, so the
  rule held, and nothing goes to the handoff.
- Hook refusal (stamp-ahead) → applied, Tier 3 (above).
- Sweep hazard (`HeldNet` a prefix of `HeldNetProvider`) → applied, Tier 3 (above).
- Operator direction (the operator pass, entries 21–24) → not a learning: a one-time direction, recorded in the report
  and `evidence/ci.md`.
- Operator direction («run P1 only and stop», the two-window form) → not a learning: this wrap's own shape.
- P4 forks («Third module fixtures/ (Recommended)» · «Existing selections (Recommended)») → not a learning: recorded in
  the plan's Provenance and carried into test-plan §3 and the sidecar this wrap.
