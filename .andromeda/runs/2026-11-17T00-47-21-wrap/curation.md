CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  extended 1
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 1 below threshold · 2 recurrence-despite-learning (→ handoff)
  No-other-home: "a widening card lists only the operations the cited word covers; one reaching past it is its own question in the same halt"
  Extended: T1/CLAUDE.md: "2026-10-10: At a wrap's Validate a boundary widening is never resolved without a halt…" + "the card lists only the operations that word itself covers; one that reaches past it is its own question in the same halt" (Extended 2026-11-17)
    Proof: the operator's directive at this wrap's invocation (2026-11-17); this wrap's card listed 9 operations under inputs#I1 and asked security-plan contract/driver-session-reply/no-attribute-text separately as Q2 (`answer-card.md`), answered «Hold for the founder». Score 0.4 (operator directive) + 0.2 (specific detail) + 0.2 (no other home: no master, route or base holds it) = 0.8.
  CLAUDE.md size: 165/200 · T1 5.0 KB, 0 over 600 B

Dispositions of the report's Decisions & corrections:
- implement · Guard refusal (a `cat` heredoc refused; re-issued through the Write tool) — rejected, Filter 1 duplicate: `.claude/rules/host-linux.md` (the Write tool for documents; setup's guard refuses a `cat`/`tee` heredoc); a guard refusal at its one re-issue.
- implement · Recurrence of `tracing::warn!` with a dotted field — recurrence-despite-learning: `.claude/rules/observability.md` Session Additions 2026-10-06 → handoff.
- implement · Sweep hazard: the engine record's message prints `[redacted]` on the sink line, so the check finds the engine line by its target and the `error=[redacted]` pair — rejected, Filter 4: 0.4 (measured, the plan's note held) + 0.2 (detail) = 0.6, exactly the threshold; the sink's engine-message redaction is a master fact (security-plan decision/telemetry-sink-target-allowlist), so no no-other-home signal.
- close · Operator directions of the pass (stop on any red; reproduce before a fix; no push while a red run is going; one fix commit for all reds) — rejected, Filter 1 duplicate: CLAUDE.md USER:session-learnings 2026-11-16 («Never push to answer a CI red while that run is still going…»).
- close · Operator direction: the two doc lines as plain code text — not a learning: a fix instruction for this chunk alone.
- close · Recurrence despite a learning: the Tier 3 «2026-11-16 — Rustdoc runs only in the doc leg» — recurrence-despite-learning → handoff; its rule now also a test-plan row (decision/doc-leg-gates-public-doc-edits, tx 1580) and four leaves (CLAUDE.md `fast` line, commands.md, tests-summary.md, testing.md).
- close · Operator directive at this wrap: the widening card lists only what the cited word covers — applied, Tier 1 extension above.
- close · Operator directive at this wrap: the doc-leg item as a test-plan row or a playbook line — dispositioned at P2 as the test-plan row (a playbook rule judges amendments, not a chunk's gate block); no curation entry.
- P2's citation row: `.claude/docs/session-learnings.md:346` `host.rs:350` out-of-range — no change: an illustrative example of a prefix clash, not a claim about the file's line 350.
