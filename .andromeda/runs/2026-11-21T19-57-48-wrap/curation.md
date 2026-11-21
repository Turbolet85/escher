CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "Iterate new code's lints on the clippy leg, not on `fast`"
    Proof: the report's Decisions & corrections and Outcome — `ci-leg.sh fast` read red on its first two firings (clippy `some_filter` in `presentational_hints.rs`, then `type_complexity` in `select.rs`'s unit test), and `ci-leg.sh clippy` alone read clean before the third (gate trail `.andromeda/runs/2026-11-21T17-41-12-implement/gate-2026-11-21-single-select-sized-as-a-list-box.json`). Score: measured gate failure +0.4 · specific detail +0.2 · no other home +0.2 = 0.8. Tier: a session-wide directive with no rule file whose `paths:` cover where `fast` runs (verification-harness.md scopes agent-run, cold-agent and the harness crate), so it goes to Tier 3.
  Filters: 4 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: "Iterate new code's lints on the clippy leg, not on `fast`"
  CLAUDE.md size: 166/200 · T1 5.4 KB, 0 over 600 B

Dispositions of the report's Decisions & corrections:
- implement · Bash guard refusal (cat heredoc to a test file) — not a learning: the guard refuses the act itself.
- implement · Bash guard refusal (cd into the references dir) — not a learning: the guard refuses the act itself.
- implement · recurrence of the 2026-11-19 «a check written over a path just measured pins the measured value» — rejected, duplicate (Filter 1); a convention it followed, not a defect record.
- implement · `fast` stops at its first clippy error — applied, Tier 3 (above).
- implement · a Dioxus `selected: true` literal re-written on a re-render — rejected, duplicate: test-plan `decision/moved-fixture-select-controlled` (tx 2955), cascaded into testing.md.
- implement · a node turned `display: none` keeps its last client rect — rejected, duplicate: architecture `contract/geometry/get-client-bounding-rect` and test-plan `decision/box-presence-read-with-display`, cascaded.
- implement · a control's named case insensitive to its guard, widened until the guard showed — rejected, duplicate: test-plan `decision/mutation-controlled-tests` (tx 2955), cascaded into testing.md.
- after implement · the operator's pass rules (a CI red reproduced before any fix, no push over a running red run) — rejected, duplicate: CLAUDE.md 2026-11-16 learning.
- after implement · the wrap directive's four clauses (one card citing the founder's words; `res-ysz67imk` by the ledger note; a guard refusal re-issued once; a refused leaf comparer re-spawned) — not a learning: each restates this skill's own letter for this wrap.
- citation sweep · `.claude/docs/session-learnings.md:405` `host.rs:350` out-of-range — no change: an illustrative citation prefix inside a preserved entry, not a claim about `host.rs`.
