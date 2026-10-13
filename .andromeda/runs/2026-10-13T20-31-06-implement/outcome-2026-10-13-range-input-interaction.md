# Implement outcome — 2026-10-13-range-input-interaction

**Outcome: green.** Phase 3's smoke was skipped: the chunk changes no binary or boot path, the plan
lists no smoke or self-verify entry, and the project has no headful self-verify. The evidence drive
still booted the real `escher-session` host: `start timer`, nine calls and `stop`, every call exit 0.

## Gates
- `gate.py run`, first firing: 13 green, 2 red (entries 9 and 14). Both reds were this chunk's own:
  - Entry 14, the `ci-leg.sh fast` clippy leg: `manual_clamp` at `packages/blitz-dom/src/node/range.rs`.
    The fraction is already checked finite there, so `clamp(0.0, 1.0)` cannot panic.
  - Entry 9, the census of added engine lines: it counted a `.expect(…)` in the new snapshot unit
    test, which now uses a `let … else { panic!(…) }`. The plan says inline test modules use
    assertions, not `unwrap` or `expect`.
- One fix iteration. `--only 9` read green, and the full block re-run (`gate-run-2.txt` in this run
  dir) read **entries 18 · green 15 · red 0 · not-run 3**. The three not run are the operator entries
  16, 17 and 18.
- Nothing was deferred: the Rust delta is non-zero, and every Rust gate ran.
- Verification matrix: the chunk claims no capability (`matrix.py show`: claimed 0), so no ref was
  written.

## Files changed (12; scope read: clean, changed 12 · listed 12 · recorded 0)
New:
- `packages/blitz-dom/src/node/range.rs`
- `tests/blitz-tests/tests/range_input_interaction.rs`

Modified:
- `packages/blitz-dom/src/node/mod.rs`
- `packages/blitz-dom/src/node/element.rs`
- `packages/blitz-dom/src/layout/construct.rs`
- `packages/blitz-dom/src/mutator.rs`
- `packages/blitz-dom/src/events/keyboard.rs`
- `packages/blitz-dom/src/events/pointer.rs`
- `packages/blitz-dom/src/form.rs`
- `packages/dioxus-native-dom/src/snapshot.rs`
- `tests/blitz-tests/tests/stand_act_range.rs`
- `tests/blitz-tests/tests/stand_act_ids.rs`

Evidence: `escher-0.1.0/chunks/2026-10-13-range-input-interaction/evidence/range-readings.md`
(step 12).

## Deviations from the plan, with justifications
1. **`RangeRules` and `RangeMove` are crate-internal, and `node/mod.rs` exports them
   `pub(crate)`.** The plan exports `RangeInputData` alone. The rules hold the attribute reads and
   the moves; the public surface is still `RangeInputData` (`value()`) and the `ElementData`
   accessors. Keeping the rules internal keeps the public API at what the plan names.
2. **Extra crate-internal helpers on `ElementData` and `BaseDocument`.** `ElementData` gains
   `range_rules`, `move_range_input`, `reseed_range_input` and `resanitize_range_input`.
   `BaseDocument::apply_range_move` lives in `keyboard.rs` beside `apply_generated_text_input_event`.
   One write, mark, `Input` and redraw path serves both the keys and the pointerdown, as steps 5
   and 6 require.
3. **`construct.rs` seeds and clears the data in its own function.** `flush_range_input` is called
   before the input match, not inside it. Inside the match the borrow of the tag name stays alive
   on the path that does not return. Construction continues exactly as before, with no early
   `return` (a constraint the plan binds).
4. **The step base follows the spec where `min` is absent.** It is then the `value` attribute
   (step 1's rule). So `range.rs`'s seeded table reads `value="3.4" step="0"` as `3.4`, and the
   degenerate row `value="1e308"` with no `min` has 1e308 as its base. The row still reads finite,
   inside `[0, 100]` and on its grid, as asserted.
5. **The grid index is computed as `value/step − base/step`.** That form, rather than
   `(value − base)/step`, keeps `min=-1e308 max=1e308` from overflowing. The degenerate unit table
   holds that row.
6. **`stand_act_ids` holds one more count than step 11 names.** Its pinned "eight controls change
   the screen at boot" now reads nine, because the Timer slider's click focuses it. The edit is in
   the same listed file, beside the `changes_at_boot` row the step names.
7. **`stand_act_range` asserts part of each diff's `differs`.** After a value change it asserts the
   slider's `differs` contains `value` and the display's contains `name`. It does not assert the
   whole list, because at steps A7 and A9 the display's text width changes and both nodes also read
   `bounds` (measured; see the evidence). After step A1 it asserts `differs == [focused]` exactly,
   as the plan says.
8. **The plan's hypothesis basis does not hold, though its conclusion does.** The plan says a centre
   click reads 15 because of the default sheet's 2px padding. Measured: padding 0, a 1px border and
   a 107 × 0 content box, so the symmetry comes from the border alone. A centre click reads 15, as
   predicted. The width was read by a throwaway probe test, deleted after the reading.
9. **`range_input_interaction`'s boot condition asserts the range is not focused.** It does not
   assert that nothing is focused: a fresh `HtmlDocument` already focuses a node at boot.

## Surfaced
None. No spec↔reality gap.

## Measured totals (for the wrap)
- `run stand`: passed 141 · failed 0 · ignored 7 over 37 files (`stand_act_range` keeps one test).
- Workspace (`target/ci-logs/test.log`): 181 result lines · 915 passed · 0 failed · 12 ignored. The
  base was 180 · 900 · 0 · 12.
- blitz-tests: 120 `.rs` files, 119 targets. The base was 119 files, 118 targets.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / test binaries of the two gate firings and the ad-hoc test runs | this run | terminated (none in `ps -eo pid,comm,args` at report time) |
| `agent-run.sh boot` · `run stand` (entries 12 and 13) | this run, through the gate tool | terminated |
| `escher-session` host, first evidence drive (state dir `target/tmp/range-sess`) | this run | terminated by `stop`; the pid is absent from `ps -p` and the state dir is removed |
| `escher-session` host, second evidence drive (`target/tmp/range-sess2`) | this run | terminated by `stop`; the pid is absent from `ps -p` and the state dir is removed |

## Decisions & corrections
- Guard refusal (bash-guard): a `cd` into a subdirectory that would move the session cwd. It was
  re-issued once with absolute paths and the Read tool.
- Guard refusal (bash-guard): a `cat` heredoc with a file target (a Python script). It was re-issued
  as Edit-tool edits.
- Recurrence of a standing learning (host-linux.md 2026-10-13, "pipeline tools bare or
  redirected"): none. Every `gate.py`, `matrix.py` and `evolve.py` call ran bare or redirected to a
  file that was then read whole.
- `escher-session` refused a scratchpad state directory as `state-dir-too-long` (exit 3: the socket
  path is over 94 bytes). The drive used a relative `target/tmp/…` directory, as the session checks
  do.
- No operator correction or ruling was given in this run.

## Next
The operator pass, before the wrap:
- entry 16 — `gate.py hygiene`, before the pre-CI commit;
- the operator pre-CI commit `chore(2026-10-13-range-input-interaction): operator pre-CI commit`;
- entry 17 — fast gate, clean-tree guard and push;
- entry 18 — `ci.py conclusion --sha HEAD --wait 1800`.

Then `/andromeda-wrap-session`.
