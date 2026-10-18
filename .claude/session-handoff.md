# Session Handoff

**Last Updated:** 2026-10-18T09:25:30Z
**Branch:** build/escher-0.2.0 · 0 ahead of origin/build/escher-0.2.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-10-18-one-virtual-clock — one virtual clock for every headless time read, PROVISIONAL

## Position
- Done: 2026-10-18-one-virtual-clock (v020-06).
  - Every engine time read now goes through the document's clock (`DocumentConfig.clock`): the scrollbar fade, the double click, pan, the fling and smooth scroll. That is wall time by default and the harness's `VirtualClock` headlessly.
  - `advance` moves that clock by exactly `ms` on every session; the refusal set is eight causes.
  - Two shots of one state read the same pixels. CI#39111579925 green 17/17.
- Gated: `gated 2026-10-18-engine-debts: standing — evidence/founder-word.md absent`. Epoch 2 closes at its gated wrap.
- Next: `/andromeda-phase` to promote and plan «Session recording» (v020-04), the next Epoch 3 entry.

## Work done
- A two-window wrap: P1 in the first; P2–P7 in this one, on the operator's resume directive (inputs#I4).
- Reconcile: 58 amendments over six masters and the session-lifecycle key file (47 detector proposals, 11 raised by main), in 10 sidecar entries. 0 escalations; 0 rejected.
- Citations: 378 re-pointed, 7 changed, 55 stretched. Leaves: 48 findings applied over 15 files.
- Curation: one T2 extension (`testing.md`) and one new T3 entry.

## Drift resolved
- architecture:
  - The document's clock is in Document core, and in [Scrolling and input] beside the seven engine changes.
  - The harness's time is the documents' `VirtualClock`, with `advance_time`.
  - `timer_step` returns nothing and carries its remainder.
  - `advance` runs on every session with `advanced_ms` = `ms`; the causes are eight, and `screenshot`'s help keeps one limit.
  - The `clock` module is listed; `session_common` has 23 readers.
- security-plan: the causes are eight (six returned by the executor); `advance` runs on every session; the fade's wall-clock limit and the epoch `unwrap`s are retired.
- design-system §Motion: the harness drives animation from the documents' clock. Settle's `animating` reads the fade on it.
- test-plan:
  - The two clock checks are counted. blitz-tests has 134 files / 133 targets, `session_common` 23 readers, stand-repeat 42 files, seven_guis' library tests 3.
  - The renamed tests and the eight causes are recorded.
  - §9 per-OS tallies are re-read on CI#39111579925, beside the local 194 · 1001 and stand-repeat 147 · 588.
- obs-plan: eight cause names; an accepted `advance` and its `tool_call`; the two clock checks are silent.
- a11y-plan §5: the double click on the document's clock.
- Route:
  - The clock's merge-surface CARRY is on «Upstream sync ahead of visual lint and real-app readiness».
  - A coverage CARRY is on «Session recording»: the sub-document's clock, pan and the fling are read on no check.

## Notes
- PROVISIONAL, for the founder's epoch batch (the operator's directive, inputs#I4 (a)): F1 B · F2 A · F2b A · F3 A, answered by the operator as delegate.
  - Marked in the masters wherever they state those facts.
  - Recorded in the ten sidecar entries of this marker.
  - They stand beside the engine-debts and cursor-fix batches, all unratified.
- The C10 fade residual is already `absorbed:v020-06`; `v020-06` is verified at this wrap.
- Last failed command: none outstanding.

## Deferred learnings
- recurrence-despite-learning: host-linux «Paths & the working directory». A bare `cd` was refused at implement and again at this wrap's resume.
- recurrence-despite-learning: host-linux's 2026-10-13 rule (run a pipeline tool bare). A `cascade.py window` piped into `head` was refused at this wrap.
- In both cases the guard held at one re-issue.
- Carried, not this session's to curate: read a CI job's verdict with `ci.py`, not a log pattern (confidence 0.7).
- The `host.rs:350` citation in `.claude/docs/session-learnings.md` is closed: it is an illustrative prefix example, not a claim about the file.
