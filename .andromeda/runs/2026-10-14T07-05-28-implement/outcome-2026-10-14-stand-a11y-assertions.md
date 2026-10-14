# Implement outcome — 2026-10-14-stand-a11y-assertions

**Outcome: green + smoke.** The gate block's 11 non-operator entries were green on the first run (1 iteration): `entries 14 · green 11 · red 0 · recorded 0 · timeout 0 · not-run 3`. The 3 `operator` entries were not run. Smoke: `recorded-not-rerun`. `run stand` ran as gate 9, and `agent-run.sh status` reads `booted true`, run `passed` 162 · 0 · 7.

Next → the OPERATOR PASS, entries 12, 13 and 14:
- 12: `gate.py hygiene`, before the pre-CI commit.
- 13: `ci-leg.sh fast`, the clean-tree guard, then the push of `build/escher-0.1.0`, after the pre-CI commit.
- 14: `ci.py conclusion --sha HEAD --wait 1800` on the pushed sha.

Then → `/andromeda-wrap-session`.

## Files changed (15, all on research's Files to modify; 0 new)
- `examples/seven_guis/src/tasks/{counter,flight_booker,timer,crud,circle_drawer,cells,temp_converter}.rs`
  - The accent is one step darker, replaced in the order `#2a4cd3`→`#1f3fc1`, `#3a5ce5`→`#2a4cd3`, `#4a6cf7`→`#3a5ce5`. That was 25 hex sites.
  - The 3 `rgba(74, 108, 247, …)` sites became `rgba(58, 92, 229, …)` with their alpha kept.
  - The diff touches colour lines only: 26 insertions and 26 deletions.
  - `crud.rs` also gains `aria_label: "People"` on `crud-list`.
- `tests/blitz-tests/tests/common/mod.rs` — `REGION_NAMES` (1 row) and `ENABLED_TAB_ORDER` (2 rows).
- `tests/blitz-tests/tests/stand_contrast.rs`
  - The five former `Fail` rows are restated as `Pass(white on #3a5ce5, 5.47)`.
  - A shared reader `read(selector)`.
  - `TEXT_ROWS` (14 rows) and `STATE_ROWS` (19 rows, `Reach` inputs).
  - Three tests: `every_measured_control_meets_its_required_ratio`, `every_list_row_and_text_outside_the_controls_meets_its_ratio` and `every_colour_state_meets_its_ratio`.
- `tests/blitz-tests/tests/stand_keyboard.rs` — three tests: `the_enabled_states_extend_the_walk_in_order`, `enter_and_space_on_the_controls_enabled_later_read_as_their_click` and `every_control_is_reached_by_tab`.
- `tests/blitz-tests/tests/stand_accessibility_ids.rs` — three tests: `every_interactive_role_element_reads_a_name`, `the_tree_the_shell_hands_the_adapter_carries_stable_ids_only` and `a_focus_move_and_a_rewritten_name_reach_the_tree_the_shell_builds`.
- `examples/seven_guis/tests/{host_log,mcp_log}.rs` — `crud-list`'s `aria-label`, read from the element, joins `names`.
- `.github/scripts/ci-leg.sh` — the `a11y` leg gains `--test stand_keyboard --test stand_contrast --test stand_accessibility_ids`.
- `.github/scripts/test_ci_workflows.py` — `LegScriptTest`'s a11y tuple gains the same three names in the same order.

Evidence: `escher-0.1.0/chunks/2026-10-14-stand-a11y-assertions/evidence/hover-active.md` (step 1).

## Measurements (step 10 and the gates)
- Step 1: `Harness::text_contrast` **does** read `:hover` (after `move_mouse_to`) and `:active` (after `mouse_down_at`), the same in both modes. Hover and active are therefore asserted in `STATE_ROWS`.
- Step 4: the control record reads pass 14 · fail 0 · exempt-inactive 3 · no-text 1, as predicted.
- `run stand` `run.end`: passed 162 · failed 0 · ignored 7, outcome `passed`, over 38 files. The baseline was 153, +9 as predicted.
- Workspace, from `target/ci-logs/test.log`: 182 result lines, 937 passed · 0 failed · 12 ignored. The baseline was 928 passed, +9.
- The `a11y` leg, from `target/ci-logs/a11y.log`, runs six files:

  | file | passed |
  |---|---|
  | accessibility_hidden | 6 |
  | accessibility_roles | 6 |
  | focusability_updates | 4 |
  | stand_accessibility_ids | 8 |
  | stand_contrast | 5 |
  | stand_keyboard | 14 |

- The census probe (gate 5) reads 0, exit 1. The silence census (gate 6) reads 0, exit 1. `git diff --quiet ccfcda48 -- packages …` (gate 7) exits 0. `host_log` and `mcp_log` are green with the list's name among the needles (gate 3).
- `ENABLED_TAB_ORDER` was measured equal to the predicted order: DOM order plus the controls disabled at boot.

## Deviations from plan, with justifications
1. **Step 6, `enter_and_space_on_the_controls_enabled_later_read_as_their_click`.** The plan says "the way `BUTTONS` rows are" but states no change flag. A click on Update with unchanged fields changes nothing, so a key that did nothing would also pass that row. The reach is keyboard-only: Tab to the list, ArrowDown, Tab to `crud-name`, type `x`, then Tab to the button. Both rows then assert that their click changes the screen beyond focus, and that each key reads as the click. Rows stated: 2 buttons × 2 keys = 4.
2. **`TEXT_ROWS`, `label[for=crud-filter]`.** It reads `#000000` on `#f5f5f5` at 19.26. Research's declared-hex figure for the CRUD labels was `#333` at 11.59; the filter label carries no class. The table holds the measurement, and the plan's premise holds: every row passes, and the lowest is 9.74.
3. **`STATE_ROWS`, active rows.** CRUD's buttons and `#back-btn` declare no `:active` rule, so their active rows read the hover pair: `crud-create` white on `#2a4cd3` at 6.83, `back-btn` `#333` on `#e4e4e4` at 9.94. A comment in the table says so.
   - `back-btn`'s hover and active rows are read on the Counter task.
   - The booking message is addressed as `#flight-booked`, and the plan's condition holds: it renders with a selector-addressable element.
   - Row counts: return-chosen 3, unparseable 2 (`flight-book` exempt), booked 1, row-selected 3, hover 5, active 5. Total 19: pass 18 · exempt 1.
4. **`stand_contrast` reader.** The classification moved from `measure` into a shared `read(harness, selector, at)`, so the three new tables use one predicate. `measure` keeps its "the snapshot lists the control" assertion. The exempt class is still exactly `enabled == Some(false)`, read from the snapshot by id; a `label[for=…]` row has no id and is never exempt.
5. **`every_interactive_role_element_reads_a_name`.** The plan states no node count. The test asserts that each task's interactive nodes number at least its `controls` rows, that every one is named, and that each `REGION_NAMES` row is among them. The `ListBox` role is asserted, and so is its name in both the tree and the snapshot.
6. **`a_focus_move_and_a_rewritten_name_reach_the_tree_the_shell_builds`.** It walks Counter's whole `tab_order`, two Tabs, draining before each. Each Tab marks the changed set, and the `&dyn Document` tree's focus reads the new stop. It then drains and presses Enter on the focused `counter-increment`. The set is marked and `counter-value`'s name reads `0` → `1` in that tree.
7. **A second scratch probe.** `tests/blitz-tests/tests/scratch_rows_probe.rs` measured the `TEXT_ROWS` and `STATE_ROWS` values that step 5 says are "measured by `Harness::text_contrast` at implement". It was deleted before the gates ran. Step 1's probe was deleted the same way, and neither is in the tree.

The scope record is absent: no out-of-list file was written. `gate.py scope` reads `clean — changed 15 · listed 15 · recorded 0`.

## Surfaced / stuck
None surfaced; nothing stuck. For the wrap's amendment flow, alongside the plan's expected amendments: three standing texts now read false, and all three are already on the plan's list.
- `.claude/rules/a11y.md` §Contrast says "SC 1.4.3 is not met on the five accent buttons … the check records and gates nothing and runs in the workspace `test` leg, not the `a11y` leg".
- `.claude/rules/a11y.md` §Tests names three files in the a11y leg.
- CLAUDE.md's Critical Warnings bullet says "SC 1.4.3 is not met on the five accent buttons (white on `#4a6cf7`, 4.39)".

No matrix capability is owned by this chunk: `matrix.py show` reads 0 claimed, and the plan directs no claim, so the ledger got no write.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `cargo test` runs (two scratch probes, three single-file runs) | this run | terminated (exited; `ps -eo pid,comm,args` census at P4 reads none) |
| `gate.py run` block, with its cargo, agent-run and the `escher-session` hosts and `mcp` children spawned by `host_log` and `mcp_log` | this run | terminated (the census reads no `escher-session`, `seven_guis`, `cargo`, `rustc` or `gate.py` process) |
| agent-run | this run | starts no daemon; state in `target/agent-run/` only |

## Decisions & corrections
- Guard refusal: the bash guard's `cd` arm refused two reads that `cd`'d into the chunk dir at Setup. Each was re-issued once with absolute paths and ran.
- An Edit whose anchor named the test `…read_as_their_click` (the file reads `…read_as_its_click`) failed string-not-found. It was re-issued on another anchor.
- No operator correction or ruling arrived in this run, and no standing learning recurred.
