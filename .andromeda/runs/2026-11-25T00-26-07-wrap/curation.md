# Curation — 2026-11-24-canvas-lint

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A delegate's answer-card confirm names the founder's answer only where its resolved line does" (confidence 0.8)
    Proof: the architecture `rows-land` dry run (this run dir, P2) printed each Q1 row's sidecar **Why:** as the proposer's `because:` and the resolved line's text alone. The founder's F1 A label appeared in no entry until the seven resolved lines in `fanout-results.md` carried it. The re-run dry run then printed it in each entry. Signals: verified by measurement +0.4, specific technical detail +0.2, no other home +0.2 (no master, route or rule holds it).
  Filters: 3 dup · 4 task-specific · 0 conflict · 0 deferred
  No-other-home: "A delegate's answer-card confirm names the founder's answer only where its resolved line does"
  CLAUDE.md size: 169/200 · T1 6.8 KB, 0 over 600 B

## Decisions & corrections, each weighed
Implement's:
- A guard refusal (bash-guard), a heredoc with a file target → rejected — duplicate (Filter 1): `host-linux.md` §Transports «Documents: the Write tool … refuses a `cat`/`tee` heredoc with a file target», and the guard itself refuses the act.
- A guard refusal (wake-guard), a `ScheduleWakeup` fallback → rejected — duplicate (Filter 1): the guard refuses the act, and SKILL.md P7.1 forbids it already.
- A guard refusal (bash-guard), a `cd` into the log dir → rejected — duplicate (Filter 1): `host-linux.md` §Paths «`cd` only inside a subshell». This window hit the same guard twice, each time re-issued once in the guard's form (`guard-refused`, the evolve record's signal, not a recurrence).
- A measured premise, «Wide label» 102.02 px wide in DejaVu Sans → rejected — task-specific (Filter 2): a fixture's value, which the test's own case comment holds.

The close's:
- The push refused twice by the remote with an HTTP 500, then accepted on the operator's re-fire → rejected — task-specific (Filter 2): a one-off remote fault with no cause on this side, already recorded in `evidence/operator-pass.md`.
- The wrap directive's whose-word split (F1 A the founder's own; F2 A–F5 A the delegate's) → not a learning: this chunk's authorities, applied in P2's dispositions and sidecar entries.
- The wrap directive's no-claim and deferral lines → not a learning: this chunk's route and ledger acts (P5, P7.3).

This window's:
- The operator's «stop before the light gate, another project's gate is on the host» → rejected — task-specific (Filter 2): a pacing word for this wrap alone.
- The citation-sweep row `.claude/docs/session-learnings.md:433` `host.rs:350`, out of range → no change: an illustrative prefix inside a preserve-verbatim learning, not a live pointer (`citation-dispositions.md`).
- The delegate's answer-card confirm whose resolved lines named only the confirm → applied, Tier 3 (above).
