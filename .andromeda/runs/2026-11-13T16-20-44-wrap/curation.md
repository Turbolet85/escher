CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   extended host-linux.md: "`escher-session` refuses a state directory whose socket path passes 94 bytes…" + "`gate.py control`'s clone root passes it too; control a `.github/scripts` change by a narrower entry or a hand control" (confidence 0.8)
    Proof: report Decisions & corrections, "The control tool cannot measure a ci-scripts change through `fast`" — `fast` red in place in the clone, `stand_session_lifecycle` `StateDirTooLong` (`evidence/control-1.txt`); the test-plan row proposing it was fit-rejected `not_master` (no other home). Signals: measured +0.4 · technical +0.2 · no-other-home +0.2.
  Tier 3 (.claude/docs/session-learnings.md): + "A CI job cancelled for runner capacity reads red until re-run" (confidence 0.8)
    Proof: report Cross-project claims and Decisions & corrections — CI#43451279644 attempts 1 and 2 `Test (macos)` cancelled «The job was not acquired by Runner of type hosted», no step run; `ci.py conclusion` read red, green at attempt 3 (`inputs#I5`); the test-plan row proposing it was fit-rejected `not_master`. Signals: measured (a real gate failure) +0.4 · technical +0.2 · no-other-home +0.2.
  Filters: 2 dup · 0 task-specific · 0 conflict · 0 deferred · 1 below threshold
  No-other-home: "the control clone's socket bound" · "a runner-capacity cancellation reads red"
  Extended: T2/host-linux.md: "`escher-session` refuses a state directory whose socket path passes 94 bytes" + "the control tool's clone"

## Decisions & corrections — dispositions
1. The operator's Q1 and the founder's Q2 — not a learning: a ruling, carried by `inputs#I2` and the masters (P2 Q1).
2. The operator directs this wrap (`inputs#I6`) — not a learning: this wrap's direction, acted on at P2.
3. The control tool cannot measure a ci-scripts change through `fast` — extended (T2 host-linux.md).
4. A permission-classifier refusal of a hand control — not a learning: a one-off, not worked around (-0.3, below threshold).
5. The bash guard refused a pipe into `tail` and a `cd` — rejected, duplicate: the hook refuses the act itself and host-linux.md 2026-10-13 holds the pipe rule.
6. A CI job cancelled for runner capacity reads red — applied (T3).
7. Sweep hazard: `sk-[A-Za-z0-9]` matches the stand's `task-…` ids — rejected, Filter 4: 0.2 (a technical detail; the false positive changed no design and failed no gate).
8. Sweep hazard: `gh api …/logs` needs `--allow-escape-sequences` — rejected, duplicate: test-plan `decision/ci-log-ansi-strip` holds it.
9. Finding: the hit arm's returned coordinates — not a learning: an open code finding, routed to P5 as a `WATCH:`.
Citation row (`citation-dispositions.md`): `.claude/docs/session-learnings.md:301` names `host.rs:350` as an illustrative prefix-collision example, not a claim about the file — no change.
This pass: the Q1 card's miscount, the leaf comparers' blind class, the not_master re-disposition — tool telemetry (evolve records), not curated.
