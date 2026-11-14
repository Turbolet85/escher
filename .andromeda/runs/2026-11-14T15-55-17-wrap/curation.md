# Curation — 2026-11-14-auto-exploration

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "To plant a panicking handler in a Dioxus fixture, write a plain function that panics and pass it by name — an `onclick` closure whose whole body is `panic!` has type `!`, which fails `SpawnIfAsync` and does not compile."
    Proof: implement's deviation 2 (report Deviations from intent 2): the plan's closure form failed to compile; the planted panic is the unit fn at `tests/blitz-tests/tests/stand_explore.rs:94-97`, and `a_panicking_handler_is_caught_and_the_exploration_goes_on` reads green (CI#43608413417). Score 0.8: verified by measurement +0.4 · specific technical detail +0.2 · no other durable home +0.2 (no master, route annotation, matrix note or contract document carries it). Filter 1: no hit for `SpawnIfAsync` or a planted panic in CLAUDE.md, `.claude/rules/*`, `.claude/docs/session-learnings.md` (`git grep`).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 3 dup · 2 task-specific · 0 conflict · 0 deferred · 5 not a learning
  No-other-home: "To plant a panicking handler in a Dioxus fixture, write a plain function that panics …"
  CLAUDE.md size: 164/200 · T1 4.6 KB, 0 over 600 B

## Decisions & corrections — dispositions
- The founder's own word on three forks (inputs#I6): not a learning — ratifications, landed in the masters and the route (P2, P5).
- The driving overseer's directive for the wrap (inputs#I7; at resume inputs#I8, mid-P2 inputs#I10): not a learning — this wrap's directions.
- An unbounded app never exhausts a breadth-first frontier; exhaustion only on a finite page: rejected — Filter 1, dup: landed at P2 as test-plan `decision/exhaustion-asserted-on-finite-page` and re-derived into `tests-summary.md`.
- A Dioxus `onclick` closure of `panic!` fails `SpawnIfAsync`; a planted panic is a unit fn: applied — Tier 2, testing.md (above).
- A control that removes a bound loosens it to twice the budget: rejected — Filter 1, dup: landed at P2 in test-plan `decision/mutation-controlled-tests` and re-derived into `testing.md:23` and `tests-summary.md`.
- A refused call changes no screen, so the `Accepted` check is redundant with the dedupe: rejected — Filter 2, task-specific (one function's internal redundancy).
- The standing learning applied (the windows-gnu check after a `cfg(unix)` gate): not a learning — the testing.md 2026-10-19 entry worked as written; no recurrence.
- The operator's implement directives (a founder fork to `founder-forks.md`; a controls file's case the test name alone): rejected — Filter 1, dup: the away form in the skills' inputs contract and testing.md's 2026-11-14 entry.
- The Bash guard refused a `cd` twice at implement and a piped `rec.py get` at this wrap: not a learning — the guard's refusals, each re-issued once.
- Sweep hazard (a pre-pass with an unfired control refuses the whole call; the re-fire's record follows the refusal): rejected — Filter 1, dup: the wrap skill's site-sweep step states it (exit 3 naming unfired controls, the refusal the zero's basis).
- The operator fired CI on the full sha: rejected — Filter 2, task-specific (one run's form).
- This window: Q1's ruling (a PROVISIONAL mark held to the epoch boundary despite the founder's verbatim word): not a learning — playbook rule `rul-3ndrhilc` already states it; the held row is named in the handoff.
- This window: `cascade.py leaves --only` takes space-separated ids, and an addendum section file opens with its own heading: not a learning — tool telemetry, recorded as friction.
- Citation sweep row (`citation-dispositions.md`): `.claude/docs/session-learnings.md:326` cites `host.rs:350`, out of range — no change: the entry quotes the citation as an illustrative example of a citation form, not a claim about the tree.
