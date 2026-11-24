# Session Handoff

**Last Updated:** 2026-11-24T11:52:00Z
**Branch:** build/escher-0.4.0 · 0 ahead of origin/build/escher-0.4.0 as read at this wrap's Setup (HEAD `895a134f2`, the pre-CI merge commit)
**Status:** clean
**Last Commit:** feat(2026-11-24-upstream-sync-ahead-of-inside-the-canvas) — this wrap's commit (see `git log -1`)

## Position
- Done: «Upstream sync ahead of inside the canvas» (v040-02, Epoch 4's head) is wrapped. It is the two-parent merge `895a134f2` of upstream/main `29a439e05`, and CI#45375406542 reads green 17/17. Taffy is now 0.16.0 from crates.io and the workspace takes no git dependency. It also takes upstream's drop input, one-axis `overflow: clip` paint, `<br>` in the text's flex item and `preventDefault` ignored on a non-cancelable event.
- `v040-02` stays pooled and unclaimed; its ledger note records this sync. L (the left-out set) was not ruled. `evidence/left-out-recheck.md` stays as the measurement for the founder and the next sync.
- Next: «Canvas draw commands» (v040-10) is the first markerless entry of Epoch 4. It carries WATCH `cry-em4q2mag`, now at 1 of 3.

## Work done
- Report closed (`## After implement`, `## New text, by line`). The wrap directive was snapped as `inputs#I4`.
- Masters: 25 operations in 6 TXs (3565–3570).
  - Architecture and security-plan: the git exception is retired from 6 rows (2 architecture, 4 security-plan), and Taffy 0.16.
  - Engine changes C1, B1 and P1, plus `CalcHandle`, landed on the founder's own answers (inputs#I2).
  - The drop input: architecture `decision/os-drop-input` and 7 contracts; security-plan `dropped-file-read-unrestricted` and `os-drag-and-drop-untrusted`.
  - a11y-plan `absence/keyboard-drag-and-drop`; obs-plan `dropped-data-never-logged`.
- Leaves: 15 comparer findings applied, 3 declined, and 1 written by hand (`docs/workflow.md`'s merge base). CLAUDE.md's warning now reads «The workspace takes no git dependency».
- Curation: CARRY `cry-3lvz2y2v` discharged (the four stale figures removed), and `host-linux.md`'s `git grep` entry extended.
- Route: the Dioxus-hold CARRY was re-pinned on «Upstream sync ahead of drag, pan and zoom» (`cry-72op4j6x`). Residuals `res-jczchrrm` (the dropped-file reader's 136 untested lines) and `res-2zajbqhg` (the drop input's lack of harness reach) were added, both with lines in `escher-0.5.0-incubator/deferred-from-0.4.0.md`.

## Drift resolved
- 41 proposals: 16 applied, 12 escalated as a boundary widening by upstream merge, 11 rejected as one fact held in two places, and 2 rejected at check 1. Fit read 6 `not_master`.
- Answer card Q1 covered 10 drop-input operations. It was confirmed by the escher overseer as a **delegate**, so those rows stand **PROVISIONAL** (`decision` rows `state: provisional`; contract rows marked in their sidecar entries). They wait for the founder's own word in the Epoch 4 boundary batch.
- Two security-plan adds read `not_master` and were not landed: `os-drop-input` and `root-drop-examples`.

## Notes
- PROVISIONAL, awaiting the founder: the 10 card-Q1 rows above (`rec.py marks`).
- Upstreamable, not yet sent: the click synthesis on a custom-widget target (`res-bwyu6dcw`), the paint placement (#1578), the widget pointer frame, `Widget::fonts` and `BaseDocument::font_ctx()`.
  - Also carried: the single list box; the post-layout focus fixup and the Tab walk's skip of unshown nodes; the inline-box placeholder defect (parley 0.13); the W8 fragment mask as a stamped `data:` URL; and upstream's `docs/licensing.md` on `webpki-root-certs`.
- Carried:
  - security-plan `dec-kxrtstbq`'s `verified_was` still quotes `Cargo.lock:8040-8041`.
  - `host-linux.md` is 9 template lines behind (U04 `noted`).
- Left in place on the operator's word:
  - Worktrees: `../escher-trial-inside-canvas`; `target/tmp/probe-circle-digest` and `../escher-probe-snap`, with the branch `build/probe-circle-digest` (local and origin); `target/tmp/probe-canvas-shot`, with the branch `build/probe-canvas-shot`; `target/tmp/trial-canvas`, `target/tmp/trial-dioxus8` and `target/tmp/trial-editor`.
  - Build directories: `target/trial-inside-canvas-build`, `target/tic`, `target/probe-circle-build`, `target/trial-dioxus8-build`.
  - The scratchpads and scratch clones.
- Left for the operator to remove: `target/mutants-peak`, `target/mutants-tmp`, `target/mutants-raw-2026-11-20/`, `target/tmp/upstream-547d`, `target/p5-baseline-debug`, `target/cold-agent/` (`bash scripts/cold-agent.sh cleanup`), `target/tmp/smoke-order-1`, `target/tmp/trial-pas` and `target/tmp/escher-session.head`.
- Last failed command: none.

## Session End Status
Completed normally at 2026-11-24 18:44:36
