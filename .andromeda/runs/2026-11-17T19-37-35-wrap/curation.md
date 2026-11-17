CLAUDE.md ecosystem: no new learnings this session.
  Filters: 1 dup (a recurrence → handoff) · 2 task-specific · 0 conflict · 0 deferred

## Decisions & corrections, each dispositioned
- implement — the `bash-guard.sh` refusal of a `cd` into the session scratchpad, re-issued once with absolute paths: rejected — Filter 1. `rules/host-linux.md` §Paths already states the guard and the remedy («anchor every cross-call path absolutely, or `cd` only inside a subshell»), and the guard itself blocks the act. This wrap hit the same refusal once (a `cd` into the skill's references) and re-issued it.
- implement — a test module reaching its parent through `use super::*` also reaches the parent's private `use` imports; moving items out leaves the tests without them: rejected — Filter 1, a dedup hit on the DEFECT record `docs/session-learnings.md` «… a `#[cfg(test)]` child module reaches its parent's private `use` imports through `use super::*` …». That is a recurrence (22 type names, found by the first compile after the move), logged in the handoff as `recurrence-despite-learning`, not a third entry.
- after implement — the operator's two directives for the pass (re-run a job no runner took; one further re-run): not a learning. They are pass-specific directives, task-specific (Filter 2). The recurrence of no-runner cancellations routes to P5's Recurrence-watch row.
- after implement — a CI job cancelled with no runner, recurring across two chunks: not a learning. It is a CI-side observation with no cause measured, owned by P5's watch.
- wrap — the operator's invocation rule for the leaves step (re-spawn a comparer whose output the apply refuses; a refused render stops; no hand edit of a leaf): not a learning. It is an instruction for this wrap alone, with no «from now on», task-specific (Filter 2); it shaped the leaves decision recorded in `cascade-dispositions.md`.
- wrap — the operator's answer to carry the two stale leaf lines to «Stand requirement sweep»: not a learning. It is a route decision, landed as a CARRY at P5.

## Citation rows
- `.claude/docs/session-learnings.md:354` `host.rs:350` (out-of-range, curation home — P3): no change. It is an illustrative prefix example inside a curated entry, not a pointer into the tree (`citation-dispositions.md`), and it stays verbatim.
