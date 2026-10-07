# Curation — 2026-10-07-act-by-id

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   corrected in place — testing.md: "Never drive a stand or harness check with a deleting or other platform-bound key …" (2026-10-06)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 0 task-specific · 0 conflict · 0 deferred · 3 below threshold
  CLAUDE.md size: 138/200 · T1 0.9 KB, 0 over 600 B
```

## The correction (exempt from the cap)

- `.claude/rules/testing.md` `## Session Additions`, the 2026-10-06 entry. Its clause "(the delete
  arrives there as an Apple standard key binding the harness does not synthesize)" and its
  unqualified "the press does nothing on the macOS CI leg" were made false by this chunk. The entry
  now says the binding is not sent by `press`, is sent by `Harness::apple_keybinding`, and is added by
  the driver's `press backspace` on macOS; the directive — no deleting key through the harness's
  `press`, one deleting-key check and it is the driver's — stands. Tagged `[corrected 2026-10-07: …]`.
  Proof: the fork's CI run 37633611745 on `f8eb42c8`, job `Test (macos)` (112836079301, target
  `aarch64-apple-darwin`): `test backspace_deletes_one_character_of_what_the_driver_typed ... ok`;
  the helper at `packages/blitz-test-harness/src/input.rs:229`; the arm at
  `packages/escher-driver/src/execute.rs:155-158`. Record:
  `escher-0.1.0/chunks/2026-10-07-act-by-id/evidence/operator-pass.md`.
  It was found by reading, not by the sweep: none of the pass's twenty patterns names it
  (`cascade-dispositions.md`, Curation homes).

## Candidates that did not land

- **dup (a generated body carries it):** "with the settle removed only `advance` reads wrong on the
  stand" — `.claude/docs/services/escher-driver.md`, Crate-specific gotchas, re-derived this wrap.
- **dup (a generated body carries it):** "a click on a range input does not focus it and no key
  moves its value" — the same leaf, and the route entry this wrap adds.
- **below threshold (0.6 exactly — the lean reject):** "a gate entry's note can claim more than its
  command runs: read what a smoke boots before trusting its note". Signals: falsified a plan claim
  +0.4, a specific technical detail +0.2. Neither conditional signal applies: the fact has homes
  this wrap — test-plan §1 and `session-binary` record the gap, and a route CARRY owns it.
- **below threshold (0.5):** "`pgrep -x` matches nothing for a name over 15 characters and
  `pgrep -f` matches the shell that runs it; take a process census from `ps -eo comm=`". Signals: a
  repeated pattern, three separate events this session +0.3, a specific technical detail +0.2.
- **below threshold (0.2):** "a CI job log wraps cargo's `Running` in colour codes and writes
  Windows paths with backslashes, so a grep for `Running tests/{file}` reads 0 on a log that ran it".
  Signals: a specific technical detail +0.2, a one-off.

Not candidates: the five rulings and answers received for this wrap (they are dispositioned where
each belongs — the route, the playbook, the handoff, a master); the wrap's own procedure frictions
(the evolve stream's, never curated).

## Recurrence

None: no dedup hit matched a defect entry. The four Tier 3 entries the previous handoff carried
unreviewed stay listed in the handoff, for review at the Epoch 4 boundary (the founder,
2026-10-07, relayed verbatim by the overseer).
