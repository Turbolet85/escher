# Session Handoff

**Last Updated:** 2026-11-18T00:37:06Z
**Branch:** build/escher-0.3.0 · 0 ahead of origin/build/escher-0.3.0 as read at this wrap's Setup (HEAD `feaa41386`, the operator pre-CI commit)
**Status:** wrapped — the chunk commit and push are this wrap's P7
**Last Commit:** chore(2026-11-17-stand-requirement-sweep): operator pre-CI commit (the wrap commit follows; see `git log -1`)

## Position
- Done: 2026-11-17-stand-requirement-sweep (Epoch 6 «Polish & ship»).
  - Every 0.3.0 requirement now names a witness that the `stand-repeat` arm runs, in a table pinned by function name (`WITNESSES_030`). `STAND_REPEAT_NAMED` grows from 10 to 17 entries, and the leg check reads the arm alone.
  - The new `stand_masks_on_new_answers` check holds the masks on the graph, actions and layout answers, on kept and explored checks, and on a file input over MCP.
  - The browser's five `make_doc_config` field kills landed.
  - CI#44197885534 reads green 17/17 on `feaa41386` at attempt 2. Attempt 1's `Coverage report` was red on a corrupt `.profraw`; it is now watched.
- Wrap run dir: `.andromeda/runs/2026-11-18T00-09-04-wrap/`.
- Next: `/andromeda-phase` for «A11y re-read» (`chk-ajxm4sin`).

## Work done
- Report closed: `## After implement` and `## New text, by line` landed (report rev 4). Two inputs were snapped: I2, the wrap directives, and I3, the operator's CARRY door.
- Masters: 5 operations landed, in tx 1902–1905.
  - architecture `decision/masked-input-values`: its guard is now `stand_masks_on_new_answers`.
  - security-plan `decision/mask-holds-on-every-carrier`: its guard gains `stand_masks_on_new_answers`.
  - test-plan `decision/typed-password-never-returned`: the rule is widened to the new carriers and a file input's path, and its guard gains the new check.
  - test-plan gains `decision/requirement-witness-inside-stand-repeat`.
- Leaves: 13 comparers ran, finding 7 items. 3 were applied, to `escher-driver.md:63`, `tests-summary.md:24` and `commands.md:61`, each adding the new mask check. 4 were declined, with reasons in `cascade-dispositions.md` (## Leaves).
- CARRY `cry-b3h6o7do` is closed. Both lines were compared, but no comparer could see the counts. The operator lifted the hand-edit ban for these two lines once (inputs#I3): the per-file test counts and totals were removed from `escher-driver.md:60` and `tests-summary.md:24`, not re-measured, and landed through `rec.py harness land` (tx 1910 and 1911).
- Curation: one Tier-1 entry (a docs leaf states no per-file test count) and one extension (host-linux: `git grep` reads tracked files alone).
- Route, tx 1916, both on «A11y re-read»:
  - the runner-less-cancellation WATCH, migrated (1/3);
  - a new WATCH on the corrupt-profile `Coverage report` red (0/3).
- Ledger, at P7.3: the refs of v030-01, v030-09 and v030-10 are re-pointed to the renamed tests (directive I2 (1)), and the dated witness notes for v030-04…17 are written.

## Drift resolved
- 9 operations proposed: 5 applied, 4 rejected.
  - Rejected under check 2: the architecture add duplicated test-plan's, and the test-plan caused-item add duplicated security-plan's `except`.
  - Rejected under check 3: obs-plan's `why`.
  - layout-templates read `@none`.
- 0 escalations. Fit: 5 fits, 0 not_master, 0 stands. No `gated` record stands.

## Notes
- The two CARRY leaf lines now carry no counts. Their lists of test files are still the pre-split ones: `caused.rs`, `explore_tests.rs`, `fixture_tests.rs` and `layout_tests.rs` are unnamed. Correcting that is outside the ruling and no comparer sees it.
- For /andromeda-route: residual `res-h7acz56c` (`re-carried`) names the four zip ignores, which zip 9.6.1 dropped. Its disposition is route's to flip.
- Open (carried): security-plan `dec-kxrtstbq`'s `verified_was` evidence still quotes `Cargo.lock:8040-8041`.
- `host-linux.md` is 9 template lines behind (U04 `noted`). Regenerating it is the operator's word.
- Left for the operator to remove: implement's scratch clones (`…/scratchpad/guardclone`, `…/guardclone-impl`), `target/tmp/trial-pas` and `target/tmp/escher-session.head`.
- Last failed command: none.
