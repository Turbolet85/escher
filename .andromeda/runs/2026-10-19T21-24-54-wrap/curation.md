# Curation — 2026-10-19-test-splits (wrap 2026-10-19T21-24-54, resumed)

CLAUDE.md ecosystem: no new learnings this session.
  Tier 1 (CLAUDE.md USER:session-learnings):  —
  Tier 2 (.claude/rules/*):                   —
  Tier 3 (.claude/docs/session-learnings.md): —
  Filters: 1 dup · 0 task-specific · 0 conflict · 0 deferred (cap) · 1 below threshold · 2 recurrence-despite-learning (→ handoff)

Sources: this window's conversation (the resumed P2) and the report's *Decisions & corrections* (the prior window's candidates).

## Dispositions, per *Decisions & corrections* entry
- A coverage premise a test split rests on (a `*_tests.rs` `#[path]` child is no record of the lcov, so a split moves the crate's reading) → rejected — Filter 1 (duplicate): this wrap's P2 amended it into test-plan §10 Coverage floors and the cascade wrote it into `.claude/rules/testing.md`'s generated body (its `paths:` cover the tests), the dedup-reject-only match; it reached a durable home. Proof of the fact: `escher-0.2.0/chunks/2026-10-19-test-splits/evidence/ci.md` § A premise the run falsified.
- Sweep hazard: a basename pattern (`recording\.rs:`, `tests/common/mod.rs:`) matches other files' citations → rejected — Filter 4: specific technical detail +0.2, observed on one sweep (9 of 17 and 9 of 24 rows), no claim falsified, no repeat → 0.2, below 0.6.
- Sweep hazard: the shell's `grep` refused a bounded repetition `.{200}` and printed nothing → recurrence-despite-learning (Filter 1 hit on the T1 entry of 2026-10-07 and the global CLAUDE.md Shell rule; a defect record) → handoff Deferred learnings.
- The operator's words (the P5 "yes", the operator-pass directive, "run P1 only and stop") → not a learning — chunk directives, carried by the inputs entry and the report.
- Entry 26 fired twice (the base run as comparison) → not a learning — this chunk's evidence handling.
- jscpd's residual clone resolved by sharing → not a learning — the plan's Implementation notes already state the rule.
- No guard or hook refusal in the implementing window → not a learning.

## This window
- The Bash guard refused one `cd` into the run dir while reading the fan-out files (re-issued once with absolute paths) → recurrence-despite-learning: host-linux §Paths («`cd` only inside a subshell») → handoff Deferred learnings.
- `cascade-patterns.toml` first written with a doubled single quote inside a TOML literal string (refused, exit 2) → not a project learning — `amendment-flow.md` §Tool already states the `'''…'''` form; recorded as evolve friction, not curated.
