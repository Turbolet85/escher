# P5 review — 2026-11-24-upstream-sync-ahead-of-inside-the-canvas · «Upstream sync ahead of inside the canvas»

**Plan to open in an editor:** `.andromeda/cache/docs/2026-11-24-upstream-sync-ahead-of-inside-the-canvas/plan.md`
(rev 5, 323 lines, of which 115 are the gate fence). No plan stands in the chunk's folder. Writing in that file changes
no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-upstream-sync-ahead-of-inside-the-canvas`
shows a diff of the chunk's records.

## Goal
A two-parent merge of `upstream/main` `29a439e05` (6 commits past `165d698c3`) into `build/escher-0.4.0`, with every
fork taken as the founder ruled it (inputs#I2): T1 A, T2 A, D1 A, D2 A, C1 A, B1 A, P1 A. Dioxus stays held at
0.7.3. The left-out set stays out. L is unanswered, and its re-check is written for him.

## The re-check he asked for (L) — `.andromeda/cache/chunks/2026-11-24-upstream-sync-ahead-of-inside-the-canvas/evidence/left-out-recheck.md`
Every left-out commit was cherry-picked onto the trial merge `2cddaea0a`, and the clean ones were built and tested:
- **L4, the browser's devtools default:** the one line re-applies and builds green. The browser builds `--locked`,
  the CDP server's tests pass (19, 0 failed) and the lock does not move. Its reason was never merge pain: it ships a
  TCP listener in a default build, a boundary widening that is his call. Lean: keep out.
- **L2, upstream's `AGENTS.md`:** `22f0cd000` picks clean, but `ca0b97b05` still conflicts. No build reads the file;
  the reason is its content. Lean: keep out.
- **L5, the stand's title:** picks clean, but the build is red with the same 3 errors as the last sync. Lean: keep
  out (a stand re-write is a chunk of its own).
- **L1, the code editor:** conflicts. Even resolved, `--locked` refuses its lock, and it needs a git dependency,
  which T2 A now makes a new ruling. Lean: keep out.
- **L3, L6, L7, L8, L9:** each conflicts (1 to 8 files), and each reason re-reads as holding. Dioxus 0.8 is still
  `0.8.0-alpha.3`.
- No reason the last sync ruled on reads as failing today. The table is his; nothing is taken.

## Top acceptance criteria
- HEAD's second parent is `29a439e0563cbed6252c8f1d8ab4ed68afe03968`, and no conflict marker stands.
- `Cargo.lock` holds no git source (T1 A / T2 A).
- The lock moves only by the take: `1 file changed, 4 insertions(+), 2 deletions(-)` against `3617bc017`.
- Dioxus stays `"0.7.3"` on 13 lines, and `AGENTS.md` and `parley_code_editor` stay absent.
- `build`, `msrv`, `fast`, `doc`, `a11y`, `audit` and `coverage` each exit 0, and `stand-repeat` reads 3 of 3 rounds
  green.
- The adapter probe reads 0 on the moved lock.
- `evidence/lock.md`, `readings.md`, `sink.md` and `limits.md` are written, and `left-out-recheck.md` stands.
- CI reads `verdict: green` on the pushed merge.

## Touchpoints
- 2 edits of ours: `layout_reason.rs` (`CalcHandle`) and `dioxus-native-dom/src/events.rs` (the conflict and
  `convert_modifiers`).
- 20 files merged clean, plus 6 new files from upstream (`dnd.rs`, `datatransfer.rs`, two root examples, `flex_br`,
  `drag_and_drop`).

## Test commands (20 entries)
- 1–6: the merge-shape probes, each with a scratch-clone control (`.andromeda/runs/2026-11-24T06-26-40-phase/controls.md`).
- 7–14: CI's own legs: build, msrv, fast, doc, a11y, audit, coverage and stand-repeat, all green at `3617bc017`.
  `msrv` and `coverage` are new beside the last sync's plan.
- 15: the adapter probe.
- 16: the re-check present.
- 17–20: the operator pass: hygiene, push, CI read (`--wait 2700`) and parent read.

## Measured at P4, worth your eye
- **Coverage after the take:** on the trial, dioxus-native-dom reads 88.59 % against its 83 % floor. That is down
  from 91.14 % at HEAD: the dropped-file reader's 136 new lines are uncovered. All 3 floors hold.
- **The coverage leg's path bound:** a coverage run under a long build directory reads
  `the_same_caused_crosses_the_session_socket` red. The socket path runs to 109 bytes, past the 94-byte bound. Under
  the repository's own `target/` it runs to 83 bytes, and the baseline there was green. This is the bound, not the
  take.

## Leans for your word
- **No second row read:** T1 A adds `layout_reason.rs`, which the pinned scope did not name. Its rows are already
  picked (`contract/base-document/layout-reason` and five more). The full re-run (131 agents, about 22 min) is yours
  to order; say «re-run the row read» and P4 re-plans after it.
- **Step 1 removes this chunk's own trial:** `../escher-trial-inside-canvas` (probe `2cddaea0a`, on no branch), and
  `target/trial-inside-canvas-build` and `target/tic`. Say if you want them kept until the founder answers L.

## Capability
`v040-02` is not claimed: «… drag, pan and zoom» and «… polish and ship» still carry it. After your word, a dated
`notes` line records that.

## Validation-1
Aligned with the scope and the working entry. The scope's «stop at the forks» was the take-up's directive, and the
founder's answers (inputs#I2) close it.

## Beyond the plan
| Proposal | Owner |
|---|---|
| L's re-check table goes back to the founder through the overseer's relay. A commit he orders put back becomes its own working-route entry, taken up on his word, never folded into this sync | the founder's answer, carried by the overseer (no route insertion now) |

Apply? (yes / review / cancel)
