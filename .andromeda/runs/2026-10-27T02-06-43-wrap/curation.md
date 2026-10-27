# Curation — 2026-10-26-codebase-health · wrap 2026-10-27T02-06-43

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): extended «2026-10-06 — Grepping `agent-run` also finds the cold-agent marker and "agent-runnable"» + the file-citation facet
  Filters: 0 dup · 2 task-specific / below threshold · 0 conflict · 0 deferred · 1 recurrence (→ handoff)
  Extended: T3/session-learnings.md: "Grepping `agent-run` also finds the cold-agent marker and "agent-runnable"" + "a bare file-name citation pattern matches a longer file name ending in it; anchor on the directory"
  No-other-home: "a bare file-name citation pattern matches a longer file name ending in it"

- Extended T3 `.claude/docs/session-learnings.md` (the 2026-10-06 `agent-run` entry): «a file-citation pattern on a bare
  file name (`lint\.rs:\d+`) also matches every citation of a file whose name ends in it (`stand_lint.rs:`)».
  Proof: the P1 site sweep's pattern `lintcite` — 11 hits, 9 of them `tests/blitz-tests/tests/stand_lint.rs:` citations
  (report.md, Counts / qualifiers moved and Decisions & corrections); the plan's Expected amendments list carried five
  `stand_lint.rs` spans as `lint.rs` spans (report.md, Deviations). Score: verified by measurement +0.4 (it falsified
  the plan's expected-amendment list) · specific technical detail +0.2 · no other durable home +0.2 (not on the route,
  not in a master, no hook refuses it) = 0.8.

## Decisions & corrections, each weighed
- The operator's wrap directive (inputs#I3), the two-window form — not a learning: this chunk's process direction.
- The two implement notes needing a wrap owner — not a learning: dispositioned at P2 (three sentences amended, Q1–Q2)
  and P5 (the CARRY on «Stand requirement sweep», Q3).
- Sweep hazard `lint\.rs:\d+` × `stand_lint.rs:` — extended (above).
- Sweep hazard `over 15` × `hover 152` — recurrence-despite-learning: the 2026-10-12 count-pattern entry's
  «Extended 2026-10-25» facet (open every count alternative with `\b`) already states it → handoff, Deferred learnings.
- Sweep hazard `` `readings` `` naming two functions — rejected, Filter 2 / 4: a one-off name collision (−0.3), a
  function name outside the public API; verified +0.4, technical +0.2 → 0.3.
- A TOML literal string cannot hold a `'` (`'''…'''`) — rejected, Filter 4: the skill's own letter states it
  (amendment-flow §Tool); one-off (−0.3) + gate failure +0.4 + technical +0.2 = 0.3.
- The Bash guard's `cd` arm refused two reads — rejected, Filter 1: `.claude/rules/host-linux.md` §Paths already
  states it (guard-refused, not a learning).
- Implement's outcome: overseer1's friction-line rewrite directive — not a learning (a one-time ledger action);
  «stop and say so before a re-run on a red with no failing value» — not a learning (no red met); the standing
  learnings held — duplicates, already curated; the stale handoff — P6's refresh, not a learning.
- The rust-code-analysis `cognitive.sum` fact — rejected, Filter 4 at exactly 0.6 (verified +0.4, technical +0.2):
  amended into test-plan §10 by this wrap's P2 (`quality-gates-coverage-targets.md`, the complexity record), so no
  conditional signal applies.
- This window: the apply writer's NO_PROVENANCE refusal on main's own `as:` direction — not a project learning: the
  skill's letter already bars provenance from a body (amendment-flow §Apply step 1); recorded as evolve friction.
- This window: the listings' muted stdout and the late I4 snap — not learnings: deviations from the skill's own letter,
  recorded as evolve `problem` facts.
- The citation sweep's `curation home — P3` row (`session-learnings.md:257`, `host.rs:350`) — no change: an
  illustrative example inside a learning, not a citation of the tree's file.

CLAUDE.md size: see the P7 health row.
