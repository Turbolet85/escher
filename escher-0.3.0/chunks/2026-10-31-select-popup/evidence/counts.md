# Counts — 2026-10-31-select-popup

Each count this chunk moves, against the plan's forecast. Readings from the last full block of run
`.andromeda/runs/2026-10-31T18-16-50-implement/`: 27 green, 0 red, 4 operator entries not run.

## Against the forecasts

| Reading | Base | Forecast | Measured | Reason |
|---|---|---|---|---|
| `stand_keyboard` tests | 21 | more | 23 | the open walk and the click's walk |
| `select_interaction` tests | 18 | more | 18 | the one test restated keeps the count: the list's own rows stand in `select_popup` |
| `stand_act_select` tests | 10 | 10 | 10 | |
| `STATE_ROWS` | 18 | 20 | 20 | the open list's two plain rows, tally (19, 0, 1, 0) |
| `a11y` leg | 14 · 112 · 0 | passed above 112 | 14 · 114 · 0 | `stand_keyboard` +2 |
| `run stand`, passed · failed · ignored | 222 · 0 · 7, 50 files | above 222, 51 files | 237 · 0 · 7, 51 files | `stand_act_popup` +12, `stand_keyboard` +2, `stand_lint` +1 |
| `run all`, passed · failed · ignored | not forecast | | 727 · 0 · 12 | |
| `stand_act_ids` accepted · refused | 68 · 9 | unmoved | 68 · 9 | the two closed options stay `covered` |
| Flight booker's closed screen | 1295 B | unmoved | 1295 B | read through the binary |
| Flight booker's open screen | none | under 10,000 B | 1303 B | read through the binary |
| `Focussed node` lines | not read | unmoved | **not settled** | see below |
| `cli_commands` tests | 14 | 15 | 15 | |
| New targets | none | three | `select_popup` 25 · `select_popup_painted` 6 · `stand_act_popup` 12 | |

## `Focussed node` is not settled by this reading

Through the workspace-built `escher-session` at `RUST_LOG=info`, a flow of open, choose, open and close read **0** lines
holding that text. That says nothing about moved or unmoved, since no reading of the base was taken and 0 fits a line the
sink does not print as much as a focus that never moved. What does hold the forecast's premise is `stand_keyboard`'s
`holder`, which reads the engine, the snapshot and the tree agreeing on the select at every step with the list open.

## Controls, red before green

Hand mutations, each file's bytes restored and its sha256 compared before the green reading.

| Mutation | Red | Tests |
|---|---|---|
| `paint_scene` calls no list | 5 | all in `select_popup_painted` |
| `hit_with_scrollbar` answers no list | 10 | across all three targets |
| the first layout does not re-read the anchor | 1 | `a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened` |
| a click anywhere reads as landing on a row | 0, then 1 | see below |

The fourth survived its first firing. In the select's own box the chosen option is the only one that shows, so choosing it
and closing read alike. The point test decides only for a chosen option that is disabled, whose row leaves the list open.
`a_click_in_the_selects_box_on_a_disabled_chosen_option_closes_as_on_any_other` was added for that case, and the control
then read 1 red.

## The list seen

A whole-screen shot through `escher-session` with the flight type's list open and the highlight moved to the return flight
read as intended: rows directly under the select, the highlight on the second row, the value still the first, the page's
own controls hidden behind the list. Not committed: a shot is pixels of a session.
