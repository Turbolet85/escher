# Outcome — 2026-11-18-a11y-re-read · implement run 2026-11-18T02-16-10-implement

**Outcome: green** (0 fix iterations; smoke skipped — no boot-path or UI-surface change).

## Files changed (4 modified · 0 new)
- `.github/scripts/ci-leg.sh` — the `a11y` arm gains `--test key_press_after_container_lost_its_box --test select_popup --test select_popup_painted --test stand_act_popup` (step 1); `stand-repeat` and `embolden` untouched.
- `.github/scripts/test_ci_workflows.py` — the `a11y` tuple of `test_gate_legs_dispatch_to_cargo_and_write_their_log` gains the four words (step 2).
- `tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs` — new test `a_focused_control_that_no_longer_shows_keeps_focus_the_tree_naming_the_window`, helpers `in_anonymous_block_then_after` and `tree_focus`, imports `accesskit::NodeId as TreeNodeId` and `blitz_dom::Document`, and the `//!` sentence naming the file the `a11y` leg's witness of the SC 2.4.3 gap (step 3).
- `tests/blitz-tests/tests/stand_keyboard.rs` — new test `a_focused_cells_editor_unmounted_by_enter_drops_focus_and_never_reads_the_limit`, imports `accesskit::NodeId as TreeNodeId` and `blitz_dom::Document` (step 4).

## Gates (trail `.andromeda/runs/2026-11-18T02-16-10-implement/gate-2026-11-18-a11y-re-read.json`)
- `cargo test -p blitz-tests --locked --test key_press_after_container_lost_its_box` — green · exit 0
- `cargo test -p blitz-tests --locked --test stand_keyboard` — green · exit 0
- `bash .github/scripts/ci-leg.sh a11y` — green · exit 0 (20.55 s)
- the count probe — green · exit 0 · last line `18`; printed `163 passed 0 failed` (per file: `.andromeda/cache/chunks/2026-11-18-a11y-re-read/evidence/counts.md`)
- `bash .github/scripts/ci-leg.sh ci-scripts` — green · exit 0
- the `gate.py guard` entry — green · last line `guard: clean`; its pass line, copied whole from `gate.py show --n 6`:
  `guard: allowed 4 — .github/scripts/ci-leg.sh, .github/scripts/test_ci_workflows.py, tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs, tests/blitz-tests/tests/stand_keyboard.rs · companions excluded 0 · changed 192 · excluded 188`
  (the 188 excluded are the pipeline's own artifacts — `.andromeda/` store logs, run dirs, master route, working route and the handoff — outside the guard's pathspec by construction; no companion excused, `scope-record.md: absent`)
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 (458.54 s)
- `gate.py hygiene` · the push · `ci.py conclusion --sha HEAD --wait 2700` — `not run — leg operator`: the operator pass's.

## Controls (step 6, `gate.py control`, each in its own clone, removed after)
- `control-1.toml` (`--timeout 600`): `control tree-focus · entry 1 · holds — in place green, removed red · exit 0 ✗ (exit 101)`; red at `key_press_after_container_lost_its_box.rs:246:13` «incremental false: Hidden: the tree's focus is the window, a node it holds» (`evidence/control-1.txt`).
- `control-2.toml` (`--timeout 900`): `control removal-drop · entry 2 · holds — in place green, removed red · exit 0 ✗ (exit 101)`; red at `stand_keyboard.rs:1165:9` «incremental=false: the engine's focus reads the root element, not the removed editor» (`evidence/control-2.txt`).

## Measured literals
- Step 3, the next Tab from a control that no longer shows: `#after` (the forecast), in both modes and all three ways.
- Step 4, after Enter unmounts the click-focused Cells editor: the harness's focused reading is the root `html` element and the tree's focus `TreeNodeId(u64::MAX)` (both forecasts).

## Deviations
- Step 3's shown-state precondition (the tree's focus is the control before the box goes) is asserted for `Hidden` and `StyleDisplayNone` only. Under `FocusWithin` the page's own `#root:focus-within { display: none; }` takes the box away as Tab focuses the control, so no shown reading exists; for that way the precondition is the anonymous block and Tab focusing the control. A one-line comment in the test says why.
- Step 5's forecast sum («115 + `select_popup` 25 + `select_popup_painted` 6 + `stand_act_popup` + the new test») left out `key_press_after_container_lost_its_box`'s three standing tests, which join the leg with the file. Measured: 116 (the fourteen, `stand_keyboard` +1) + 4 + 25 + 6 + 12 = 163.
- The step 3 test drives the button control (`BUTTON`) alone; the plan names none, and the select's key behaviour is pinned by the standing tests.
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 4 · listed 4 · recorded 0 … · excluded 189`).

## Surfaced / stuck
none.

## Ledger
No row has `chunk = 2026-11-18-a11y-re-read` (`rec.py query … coverage`: 0 rows), as the plan states — no matrix write.

## Watches (folded)
- a CI job cancelled with no runner (`cry-y6764bis`) — not exercised (no CI run in implement; the operator pass reads it).
- `Coverage report` red on a corrupt profile (`cry-nxmqzwtr`) — not exercised (as above).

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / test binaries of gate entries 1–7 | this run (gate tool) | terminated — `gate.py census --comm cargo --comm rustc --comm escher-session --comm seven_guis_nati --args blitz-tests`: `census: none running` |
| two `gate.py control` clones (`~/.cache/andromeda-control/9028b550`, `…/20344e6e`) and their builds | this run | terminated — each summary `clone removed`; census as above |

## Decisions & corrections
- Guard refusal: the bash guard refused a `cat >> file <<EOF` append to the test file («a cat/tee heredoc with a file target — … an append through the Edit tool anchored on the file's last lines»); re-issued once as an Edit anchored on the file's last lines.
- Standing learnings applied: testing.md «assert the fixture's precondition first, focus by Tab» (2026-11-01) and «a `gate.py control`'s `case` names the test function alone» (2026-11-14) — each control names one case and both read `holds`.
