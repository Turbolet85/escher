# P5 review — 2026-11-22-epoch-2-corrective «Epoch 2 corrective»

**Plan:** `doc:2026-11-22-epoch-2-corrective/plan` rev 4. To read it in an editor, open the rendered copy at
`.andromeda/cache/docs/2026-11-22-epoch-2-corrective/plan.md`.
- No plan stands in a chunk's folder.
- Writing in that file changes no record until a `rec.py put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-22-epoch-2-corrective` shows
  a diff of the chunk's records.

## Goal
The chunk makes no behaviour change. It answers the founder's two answers of 2026-11-22.

**`command.rs` under 800.** Its inline test module moves whole into a `#[path]` child, `command_tests.rs`, the
driver crate's convention. Measured on scratch halves: 249 + 599 code lines, both under 800. No production item
moves, so `escher_driver`'s public paths and every caller stand.

**Escher's additions leave three upstream bodies** for functions of escher's own, one call per addition:

| function | HEAD | upstream at `547d7514b` | bar | predicted |
|---|---|---|---|---|
| `handle_key_or_input_event` | 58 | 46 | exactly 46 | 46 (scratch) |
| `compute_presentational_hints` | 176 | 167 | exactly 167 | 167 (scratch) |
| `build_accessibility_node` | 50 | 19 | at most 19 | 13 (scratch) |

- **Accessibility bar** — at most 19, on your answer «At most 19 (Recommended)» (inputs#I2). Escher's merge deleted
  three of upstream's reads, so reading exactly 19 would mean reading `aria-label`, `aria-selected` and
  `aria-expanded` twice.
- **Keyboard** — keeps two call sites, the Tab close and the key chain. They sit at two points of upstream's control
  flow, and the founder's own example `push_inline_box` is also called from two sites.
- **Fork-own functions** — every function of escher's own in the three files stays at or under cognitive 15.
- **Claims** — none (A1).

## Top acceptance criteria
1. `bars.py` reads `ours over 800 outside the seven left for a later entry: 0` and `bars: PASS`.
2. `tests_moved_whole.py` reads PASS, and the 7 `command::tests::…` tests are listed and pass.
3. `fn_scores.py` reads PASS: 46 · 167 · at most 19, and every fork-own function at or under 15.
4. Behaviour:
   - the guard names exactly the five touchpoints;
   - the added-lines count reads 0 (no `unwrap`, `expect`, panic, print, log or env read);
   - the four per-body behaviour entries, the a11y leg and the fast leg are green.
5. The coverage leg holds every floor. escher-driver is forecast a fraction of a point under its 97.97 % baseline;
   the floor is 97.
6. The stand duplication reading reads PASS (44 / 372 at baseline, the bar under 47 / 550).
7. CI on the pushed HEAD reads `verdict: green`.

## Existing checks proving each moved body unchanged
| body | entry | targets |
|---|---|---|
| `command.rs` | 6 | stand_session_lifecycle, stand_replay, stand_mcp_tools, stand_snapshot_parts, stand_graph, stand_actions, stand_act_spans, stand_masks_on_new_answers, stand_cross_os_readings, stand_handler_coverage, cli_commands, cli_flow, mcp_server, recording, host_log, mcp_log — plus entries 1-3 on the moved tests |
| `build_accessibility_node` | 7 | accessibility_states, accessibility_label, accessibility_names, accessibility_merged_states, accessibility_active_descendant_unread, inert_subtree, select_interaction, select_popup, stand_snapshot_state, stand_act_states, stand_accessibility_ids |
| `handle_key_or_input_event` | 8 | stand_keyboard, stand_act_keys, select_interaction, select_popup, stand_act_select, stand_act_popup, range_input_interaction, stand_act_range, key_press_after_container_lost_its_box, focus_fixup_unshown, checkable_change_event, text_range_no_change_event |
| `compute_presentational_hints` | 9 | select_popup, select_interaction, stand_act_select, select_popup_painted, stand_lint, stand_contrast |

For the command entry, `stand_cross_os_readings` and seven_guis' `recording` check the answers byte for byte. On top
of the table: the a11y leg (entry 10), the fast leg (11) and the coverage leg (12).

## Touchpoints
- new `packages/escher-driver/src/command_tests.rs`
- modify `packages/escher-driver/src/command.rs`
- modify `packages/blitz-dom/src/accessibility.rs`
- modify `packages/blitz-dom/src/events/keyboard.rs`
- modify `packages/blitz-dom/src/stylo/presentational_hints.rs`

## Test commands — baselines on the untouched tree (`40ec4749e`)
| # | entry | baseline |
|---|---|---|
| 1 | `cargo test -p escher-driver --locked --lib command::tests` | green — 7 passed |
| 2 | the `command::tests::` listing count | green — 7 |
| 3 | `tests_moved_whole.py` | red — `command_tests.rs` absent (the subject) |
| 4 | preservation guard | red — nothing changed yet (the subject) |
| 5 | added-lines count | green — 0 |
| 6-9 | per-body behaviour targets | green — 16 · 11 · 12 · 6 targets |
| 10 | a11y leg | green — 19 targets |
| 11 | fast leg | seen green in the previous chunk's wrap |
| 12 | coverage leg | green — escher-driver 97.97 %, dioxus-native-dom 90.31 %, blitz-test-harness 96.13 % |
| 13 | `fn_scores.py` | red — 50 · 58 · 176 (the subject) |
| 14 | `bars.py` | red — `command.rs` 848, `bars: FAIL` (the subject) |
| 15 | stand duplication | green — 44 pairs / 372 lines, PASS |
| 16-18 | operator pass | hygiene · push · `ci.py conclusion --wait 3000` |

**Controls** were fired as the entries on a scratch clone at HEAD, each with a must-pass and a must-fail input:
entries 3, 4, 5, 13 and 14 each read green and red as intended. The record is
`.andromeda/runs/2026-11-22T16-25-24-phase/p5-controls.md`.

## Expected amendments (wrap)
- architecture §Stack and Technologies — a decision row beside decision/cognitive-complexity-ceiling, the founder's
  standing rule (cry-6gvl75lk).
- The masters' citations into the moved lines are re-pointed, with each claim unchanged.
- Doc leaves the wrap re-points: `CLAUDE.md:92`, `.claude/docs/services/escher-driver.md:51`,
  `.claude/docs/tests-summary.md:24`.

## Notes
- **Rows read by search.** The rows files sum 303,779 B, past the 250,000 B whole-read bound, so the plan read them
  by search. Every row the plan cites (20) was checked present.
- **Founder forks:** none came up; no `founder-forks.md`.
- **Snapshots:**
  - inputs#I1 — the take-up directive;
  - inputs#I2 — your accessibility bar;
  - inputs#I3 — the cargo hold lifted.
- **Setup's CI reds** (11fcd5bf0, 1b63fd780) are noted on your word. HEAD's run `40ec4749e` was in progress at Setup.

## Beyond the plan
none
