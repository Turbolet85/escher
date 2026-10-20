# Curation — 2026-10-20-large-screen-in-parts

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "Read a capability's acceptance against the plan's steps before writing its test" (confidence 0.8)
  Extended: T3/session-learnings.md: "A count stated in more than one form needs every form in its sweep pattern" + spell the count as the masters do, never as a gate prints it
  Corrected (in place): T3/session-learnings.md: "An in-line edit beside a provenance parenthesis reads as new provenance" → "Provenance is counted by class and span, not by the words beside it" — exempt from the cap
  Filters: 2 dup (1 a recurrence → handoff) · 0 task-specific · 0 conflict · 0 deferred
  No-other-home: "Read a capability's acceptance against the plan's steps before writing its test"
  No-other-home: "spell the count as the masters do, never as a gate prints it" (the extension)
  CLAUDE.md size: 150/200 · T1 3.5 KB, 0 over 600 B

## Applied
- T3 new — «Read a capability's acceptance against the plan's steps before writing its test» (0.8: measured +0.4, a
  specific technical detail +0.2, no other durable home +0.2 — on no route annotation, in no master, rule or ledger note).
  Proof: report.md, Decisions & corrections, «A matrix acceptance wider than its plan step was followed, and cost one
  re-run» — plan step 12.7 named two MCP calls, `verification-matrix.json#v020-09` names «the same walk»; the MCP test
  was extended after the first green and the whole block re-ran green 17/17 (implement evolve record
  `2026-10-20T10:45:45Z-b`).
- T3 extended — the count-forms entry, the facet a gate's wording (0.8: measured +0.4, specific detail +0.2, no other
  home +0.2). Proof: `report-patterns.toml` `unitcount` = `75 passed|99 passed`, a gate's output wording, fired 0 on
  the masters while `99 unit tests` stood at test-plan.md:22, :25 and session-lifecycle.md:11; the test-plan detector
  proposed all three from the report's deltas (fanout-results.md, test-plan 3 · 7 · 8).
- T3 corrected in place — the 2026-10-19 provenance-parenthesis entry, on the operator's resume word (inputs#I4). Proof:
  every master this wrap edited in-line beside a standing provenance parenthesis — architecture.md:196 «the sixth kind,
  since 2026-10-10-driver-cli) · `Part` …», test-plan.md:22 «mcp_tests.rs 15 (12 until …», obs-plan.md:121 —
  read `provenance 0` under `classify v1.3` (classify trail `classify-2026-10-20-large-screen-in-parts.json`);
  classification-contract.md §Check 6 states the class-and-span count.

## Every Decisions & corrections entry
- The founder's rulings on the fork (inputs#I2) — not a learning: a decision, carried in the masters and sidecars this wrap.
- The operator's «yes» on the P5 card 2 (inputs#I3) — not a learning: an approval; its CARRY lands at P5.
- A matrix acceptance wider than its plan step — applied (T3 new, above).
- A plan's touchpoint row contradicted its step — rejected, Filter 4: one-off, no emphasis (0.2 + 0.2 − 0.3 = 0.1); the
  plan defect is the implement run's evolve record (`input.plan-step-ambiguous`).
- A guard refusal that cost its one re-issue (`cd` outside a subshell) — rejected, Filter 1: host-linux.md's «`cd` only
  inside a subshell» states it, and the guard refusing the act is no recurrence (evolve-system.md, `guard-refused`). The
  same refusal met once this window, the same way.
- The pre-pass sweep's unfired controls — the `unitcount` half extended (above); the `readers` half is a recurrence
  past the count-forms entry (its pattern `28 readers|twenty-eighth|23 checks|…` missed «by 28 checks», «twenty-eight
  checks» and «23 readers», which the detectors proposed) → handoff, `recurrence-despite-learning`.
- The operator's directive for the operator pass — not a learning: a one-time direction, fired in order.
- The stand-repeat job counts more than the `stand_*` files — rejected, Filter 4 at 0.6 (measured +0.4, detail +0.2):
  test-plan.md:353 already says the leg runs seven_guis' eight process checks, so no conditional signal applies.
- The operator's «run P1 only and stop» — not a learning: the two-window form, superseded by inputs#I4.
- This window — the operator's curation directive (inputs#I4) on session-learnings.md:59-66 — corrected in place (above).
