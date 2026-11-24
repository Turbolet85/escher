# P5 review, again — 2026-11-24-upstream-sync-ahead-of-inside-the-canvas · «Upstream sync ahead of inside the canvas»

**Plan to open in an editor:** `.andromeda/cache/docs/2026-11-24-upstream-sync-ahead-of-inside-the-canvas/plan.md`
(rev 7, 353 lines, of which 135 are the gate fence). No plan stands in the chunk's folder. Writing in that file changes
no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-upstream-sync-ahead-of-inside-the-canvas`
shows a diff of the chunk's records. Card 1 stands at
`.andromeda/runs/2026-11-24T06-26-40-phase/p5-card-1.md`; this card answers its review.

## Your review, folded
1. **Step 1 removes nothing.** The `git worktree remove` and every delete are gone. The trial
   `../escher-trial-inside-canvas` (probe `2cddaea0a`) and `target/trial-inside-canvas-build` and `target/tic` stay in
   place, and a constraint now says no step deletes them. The merge does not need them out of the way: a detached
   worktree holds no branch the merge moves and no path of this tree. The `fast` leg's sweep reads `target/debug` alone
   (`scripts/target-sweep.py:23`), so it leaves both build directories alone.
2. **The row read re-ran whole, by the letter.**
   - The scope was corrected first (rev 4): the founder's answers (inputs#I2) and the two adaptations, `layout_reason.rs`
     among them, are now named.
   - `task --repin` (7 of 7 pass), the packs (39), the pickers (117 in two parts, 75 + 42, all pass), the union
     (picked 1380 of 3592, no master at `picked 0`) and the history agents (7 of 7 pass) all ran.
   - Folded from it:
     - **CI's `counter` and `wasm` legs** become gate entries 17 and 18 (architecture · contract/ci-leg-legs/counter, wasm),
       both green at `3617bc017`.
     - **The sink-installing host under the package-alone build** becomes entry 19,
       `cargo test -p seven_guis --locked --test host_log --test mcp_log` (obs-plan · decision/sink-host-read-at-trace).
       It is green at `3617bc017` (7 + 3 passed). `fast` reads only the workspace build.
     - **The layout readings stay put through the Taffy move:** `stand_layout`, the `layout` verb's answer and a kept
       layout step in `stand_replay` (architecture · decision/layout-answer-crossing; contract/driver-session/layout),
       in step 9.
     - **No leg list or `docs/capabilities.md` count is edited** for upstream's two new test files (test-plan ·
       decision/joining-file-keeps-its-legs), in step 9.
     - **The two new test files are swept for env reads** (security sidecar `amd-xyjfnhq4`), in step 12.
     - **Three expected amendments added:**
       - architecture's sync record names `29a439e05` as the next merge base;
       - design-system records the one-axis clip as a depth row;
       - layout-templates reads at the wrap whether `dnd.rs` and `file_drop.rs` owe a cli row.
   - The operator entries moved from 17–20 to 20–23. The mechanical set re-ran whole on rev 7 and is clean: the fence
     parses (23 entries), planlint has 0 hits, 19 baselines are recorded, the inputs verify, and the lists parse.

## Goal (unmoved)
A two-parent merge of `upstream/main` `29a439e05` (6 commits) with T1 A, T2 A, D1 A, D2 A, C1 A, B1 A and P1 A.
Dioxus stays held at 0.7.3. The left-out set stays out, and L's re-check is
`evidence/left-out-recheck.md`, the founder's table to answer.

## Top acceptance criteria
- HEAD's second parent is `29a439e0563cbed6252c8f1d8ab4ed68afe03968`, and no conflict marker stands.
- The lock holds no git source.
- The lock moves only by `1 file changed, 4 insertions(+), 2 deletions(-)` against `3617bc017`.
- Dioxus stays `"0.7.3"` on 13 lines, and `AGENTS.md` and `parley_code_editor` stay absent.
- `build`, `msrv`, `fast`, `doc`, `a11y`, `audit`, `coverage`, `counter` and `wasm` each exit 0, and `stand-repeat`
  reads 3 of 3 rounds green.
- The package-alone sink host passes, and the adapter probe reads 0.
- `evidence/lock.md`, `readings.md`, `sink.md` and `limits.md` are written, and `left-out-recheck.md` stands.
- CI reads `verdict: green` on the pushed merge.

## Capability
`v040-02` is not claimed: «… drag, pan and zoom» and «… polish and ship» still carry it. After your word, a dated
`notes` line records that.

## Beyond the plan
| Proposal | Owner |
|---|---|
| L's re-check table goes back to the founder through the overseer's relay. A commit he orders put back becomes its own working-route entry, taken up on his word, never folded into this sync | the founder's answer, carried by the overseer (no route insertion now) — (standing — `.andromeda/runs/2026-11-24T06-26-40-phase/p5-card-1.md`) |

Apply? (yes / review / cancel)
