# Session Handoff

**Last Updated:** 2026-11-21T21:44:00Z
**Branch:** build/escher-0.4.0 · 0 ahead of origin/build/escher-0.4.0 as read at this wrap's Setup (HEAD `9df8e29dc`)
**Status:** clean
**Last Commit:** 2026-11-21-single-select-sized-as-a-list-box — feat: single select sized as a list box (see `git log -1`)

## Position
- Done: 2026-11-21-single-select-sized-as-a-list-box.
  - A single select whose `size` parses above 1 is an in-flow list box: no popup, keys and row clicks choose, it may stand with nothing chosen, and it reads `ListBox`.
  - This is the founder's F1 A with his five HTML leans (inputs#I3).
  - CI#44940079868 is green (17/17). `res-ysz67imk` is closed by the `matrix#v040-04` ledger note.
- Next: «Dependency and CI debts» (`chk-rvaqkxvd`, `/andromeda-phase`), the epoch's last markerless entry.

## Work done
- The close landed (tx 2945–2946). 54 master operations landed in 6 masters (tx 2950–2957); layout-templates is unchanged.
- The cascade applied 83 leaf findings in 17 leaves, all by the tool.
- Curation: one Tier 3 entry, «Iterate new code's lints on the clippy leg, not on `fast`».

## Drift resolved
- Q1, one card covering 47 operations that record the engine reading `size` on a single select in every document.
  - The founder confirmed them himself, «Подтвердить все 47 (Recommended)», relayed verbatim by the overseer (inputs#I4).
  - The dialog in this window timed out after 58 min. The answer arrived by the operator's relay.
- 16 adds were rejected under check 2: one fact offered by several masters.
  - The list box itself is homed in architecture `decision/single-select-list-box`.
  - Its keys and row click: a11y-plan `contract/single-list-box/*`. The role: a11y-plan `contract/accessibility-tree/select-role`.
  - The settle: security-plan `single-select-selectedness`. The size write's close: security-plan `select-list-closes-when-select-unfit`.
- Fit read 3 `not_master`: obs-plan `contract/snapshot/single-list-box-value`, test-plan `scrolling-box-boot-lint-pinned` and design-system's narrowing set. design-system `single-select-box-shows-chosen-label` was false for every single select, so I retired it.
- architecture `hit-focus-form-engine-changes`: the proposer's set re-worded three unrelated clauses. My own set restored them, keeping «list box».

## Notes
- The epoch-growth valve surfaced: Epoch 2 «Debts of 0.1.0–0.3.0» has 10 taken-up chunks via insertions. A split is the operator's call.
- Upstreamable, not yet sent (carried):
  - the single list box (the size read, keys, row click and role) — new;
  - the post-layout focus fixup and the Tab walk's skip of unshown nodes;
  - the inline-box placeholder defect (parley 0.13);
  - the W8 fragment mask fetched as a stamped `data:` URL;
  - upstream's `docs/licensing.md` on `webpki-root-certs`.
- Measured and left as it is: a node a restyle turns `display: none` keeps its last client rect (`get_client_bounding_rect`). It is now stated in architecture and test-plan.
- Still stale, left to the whole-mode leaves pass at Epoch 2's close:
  - `.claude/rules/a11y.md`'s per-file test counts («18 tests», «25 tests», «10 tests» — the report reads 24 · 28 · 17), and the same in `tests-summary.md`;
  - testing.md's «36 readers» of `session_common`;
  - `.claude/docs/stack.md:3`'s «(citations live there)»;
  - `.claude/docs/workflow.md:8` (CARRY `cry-g3etodij`).
- The verified skia archive («Проверенный архив (Recommended)») is a directive for «Dependency and CI debts». It arrives with that phase.
- Carried: security-plan `dec-kxrtstbq`'s `verified_was` still quotes `Cargo.lock:8040-8041`; `host-linux.md` is 9 template lines behind (U04 `noted`).
- Left for the operator to remove:
  - `target/mutants-tmp`, `target/mutants-raw-2026-11-20/`, `target/tmp/upstream-547d`, `target/p5-baseline-debug`;
  - `target/cold-agent/` (`bash scripts/cold-agent.sh cleanup`);
  - `target/tmp/smoke-order-1`, `target/tmp/trial-pas`, `target/tmp/escher-session.head`;
  - the scratch clones `…/scratchpad/guardclone` and `…/guardclone-impl`.
- Last failed command: none.

## Session End Status
Completed normally at 2026-11-22 00:10:51
