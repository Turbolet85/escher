# Outcome — 2026-11-25-circle-drawer-virtual-nodes

**Outcome:** green — every non-operator gate entry green on the final tree (12 of 12), control C7 holds, smoke skipped
(no boot-path touchpoint, no smoke or self-verify entry, no headful self-verify).
**Run:** 2026-11-25T19-31-14-implement — plan revision 1 (inputs#I7, approved inputs#I8), steps 14–17; steps 1–13 stand
done by `2026-11-25T17-38-04-implement` (its outcome file, green 12 of 12, controls C1–C6 `7 holds`).
**Next:** the OPERATOR PASS — entries 13 (`gate.py hygiene`), 14 (the clean-tree push), 15 (`ci.py conclusion --sha
HEAD --wait 2700`) and 16 (the CI windows job-log read for `unused import: `snapshot``, `--id` the run id entry 15
prints) — then `/andromeda-wrap-session`. The pass writes `{work_dir}/evidence/operator-pass.md` before it is cleared.

## Files changed (this run)
- Modified (listed): `packages/escher-driver/src/json/canvas.rs` (step 14), `packages/escher-driver/src/schema.rs`
  (step 15), `packages/escher-driver/src/json_tests.rs` (step 16), `tests/blitz-tests/tests/stand_circle_nodes.rs`,
  `tests/blitz-tests/tests/stand_canvas_carriers.rs`, `tests/blitz-tests/tests/stand_canvas_commands.rs`,
  `docs/capabilities.md` (step 17).
- No new file. No out-of-list edit. `gate.py scope`: `scope: clean — changed 18 · listed 16 · recorded 2 (companion 1 ·
  mechanical 0 · in-intent 1 · widening 0) · absorbed 0 · excluded 211` (revision 2 of research listed `schema.rs` and
  `stand_canvas_carriers.rs`, so the carriers' companion line now reads `record: listed`).

## Deviations, with justifications
1. **`docs/capabilities.md` adds one clause** to the tag's sentence: «under a key no other pair of a line holds» — the
   revision's property stated where the tag is.
2. **The table lift** (step 16): the 13 rows and the no-glyph run moved into one function, `canvas_line_rows`
   (`json_tests.rs:712`), returning the rows, the no-glyph command and its whole line; both tests read it, each
   asserting the row count. The new row reads the 13 commands and the no-glyph run (14), each grouped and ungrouped.
3. **One red in the first full firing, fixed:** entry 11 (`fast`) read `red · exit 0 ✗ (exit 1)` on `cargo fmt --check`
   — `stand_circle_nodes.rs:563`, a line my scripted respell lengthened past the width (a scripted write fires no
   format hook). `rustfmt` on the script-written files; entries 6 and 11 re-run green, then the whole block green.

## Measured (step 17)
- `stand_canvas_commands`' `TABLE`: every line +3 bytes, as predicted; the 100-circle dump still 6 parts; every summary's
  size unmoved (166 · 166 · 169 · 172), every digest moved (the summary's digest is the full text's). Both layout modes
  read the same 17 rows.
- `docs/capabilities.md`'s full-text sizes: 53 → 59 at boot, 547 → 562 after one circle, 6,076 → 6,190 at 12,
  50,750 → 51,656 at 100 (`whole_text` in a temporary probe of the final tree, restored by its saved bytes, sha256
  `ca14faa8…` equal before and after).
- Predicted unmoved and read green: `stand_canvas_cross_os`, the summaries' sizes, the help's pins (none by literal).

## Gates (final firing)
1 lock probe green · 2 `ci-leg.sh build` green · 3 escher-driver lib green · 4 seven_guis lib green · 5
dioxus-native-dom lib green · 6 `stand_circle_nodes` green · 7 the three virtual-node targets green · 8 the seventeen
companions green · 9 `ci-leg.sh a11y` green · 10 `ci-leg.sh stand-repeat` green (1034.41 s) · 11 `ci-leg.sh fast`
green (658.05 s) · 12 `ci-leg.sh doc` green · 13–16 `not run — leg operator`.
`entries 16 · green 12 · red 0 · recorded 0 · timeout 0 · not-run 4` (trail
`.andromeda/runs/2026-11-25T19-31-14-implement/gate-2026-11-25-circle-drawer-virtual-nodes.json`).
First firing: `entries 16 · green 11 · red 1 (11) · recorded 0 · timeout 0 · not-run 4`.
Control C7: `control C7 · entry 3 · holds — in place green, removed red · exit 0 ✗ (exit 101) · 34.43s · 1.25s →
3.without-C7.log` · `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
(`{work_dir}/evidence/control-7.txt`). No `gate.py guard` entry in the block.
No deferral: every language touched ran its gates.
Matrix: `v040-10` ref restated with `packages/escher-driver/src/json_tests.rs::no_canvas_line_holds_a_key_twice`
(implemented → implemented).

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc (gate entries, the measuring probe) | this run | terminated |
| the gate tool's control clone `~/.cache/andromeda-control/5f63289f` | this run (`gate.py control`) | removed |
`gate.py census --comm cargo --comm rustc --comm escher-session --comm seven_guis_nati --args stand_circle_nodes`:
`census: none running`.

## Decisions & corrections
- Guard refusal, re-issued once: a `cat` heredoc appending the temporary measuring probe to `stand_canvas_commands.rs`
  (the Bash guard's file-target arm) — re-issued through the Edit tool; the file restored by its saved bytes. And a
  `cd` into the skill's references directory (the Bash guard) — re-issued as a `grep`/Read by absolute path.
- Recurrence of a standing learning: `.claude/rules/testing.md` 2026-11-25 (sweep a help text's pins by its opening
  clause) — read; the opening clause is unchanged and its one literal site its own; no pin moved.
- A scripted edit fires no format hook (the skill's P1 rule): the fast leg's fmt red is that rule met.
