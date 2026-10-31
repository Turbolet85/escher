# Counts — 2026-10-31-engine-debts

Each count this chunk moves, by its reason. Readings by `gate.py`, run `.andromeda/runs/2026-10-31T08-53-14-implement/`.

## The base (step 1, before any edit, on `2109f1c2e`)

| Reading | Recorded | Read |
|---|---|---|
| workspace tally, entry 17 | `209 · 1125 · 0 · 12` | `lines 209 passed 1125 failed 0 ignored 12` |
| `a11y` leg tally, entry 25 | `14 · 85 · 0 · 0` | `lines 14 passed 85 failed 0` |

Entries 16 and 24 (`ci-leg.sh fast`, `ci-leg.sh a11y`) read green.

## The tree at the last full block

| Reading | Base | Now | Move |
|---|---|---|---|
| workspace tally, entry 17 | `209 · 1125 · 0 · 12` | `214 · 1181 · 0 · 12` | +5 result lines, the five new targets; +56 passed, by name below; ignored unmoved |
| `a11y` leg, entry 25 | `14 · 85 · 0 · 0` | `14 · 112 · 0 · 0` | fourteen files unmoved, +27 by the rows below |
| `agent-run.sh run stand`, entry 27 | `205 · 0 · 7` | `222 · 0 · 7` over 50 files | +17 |
| `agent-run.sh run all`, entry 29 | | `681 · 0 · 12` | |
| `.rs` files in `tests/blitz-tests/tests/`, entry 19 | 148 | 153 | the five new checks |
| `stand_*.rs` files, entry 28 | 50 | 50 | none added |
| CI scripts, entry 20 | `Ran 108 tests`, `OK` | `Ran 108 tests`, `OK` | the list pin edited in place |

### Each move in the workspace tally

| Target | Base | Now | Reason |
|---|---:|---:|---|
| blitz-dom's unit tests | 106 | 112 | `node::select`'s step, span, gesture and list tables (F6, F7), `form`'s two tests |
| blitz-shell's unit tests | 0 | 1 | CARRY 2, the redraw route's first hop |
| blitz-vibey-script `dom` | 30 | 31 | F10, a radio's row |
| `visible_region_transformed` | new | 6 | F1 |
| `hit_non_invertible_transform` | new | 2 | F2 |
| `open_state` | new | 4 | CARRY 2, `:open` |
| `checkable_change_event` | new | 3 | F10 |
| `text_range_no_change_event` | new | 2 | F11 |
| `inert_subtree` | 4 | 6 | F3, F4, as forecast |
| `accessibility_states` | 5 | 6 | F5 |
| `select_interaction` | 7 | 18 | F6, F7, F8's stated limit |
| `stand_act_select` | 5 | 10 | F6, F7, a row by id |
| `stand_act_transformed` | 2 | 3 | F1 through the driver |
| `stand_lint` | 7 | 8 | F9 |
| `stand_keyboard` | 15 | 21 | F12, F13 |
| `stand_contrast` | 5 | 7 | F12, F13 |
| `stand_load_page` | 2 | 3 | F12, as forecast |
| `stand_cells_served` | 4 | 5 | F13 |

The moves sum to +56. `pseudo_element_flow_classification` holds its three tests, each now running both layout modes.

### The other readings the plan asks for

| Reading | Forecast | Read | Reason |
|---|---|---|---|
| `stand_lint`'s `at_boot` | `[0, 1, 0, 0]` → `[0, 0, 0, 0]` | `[0, 0, 0, 0]` | F9; `flight-type-one-way` × `flight-type-return` is the one stand reading moved |
| `stand_non_lean_tasks`' Cells | 676, unmoved | **677** | **The forecast is falsified.** The count is every element with the listener mark that reads `GenericContainer`. Step 15 gives `cells-grid` an `onkeydown` and no role, so it joins the 676 data cells. The pin moved, with its reason stated in the check |
| `PARTS` | 5 | 5 | unmoved: the keyed grid re-anchors the header ids and the text stays in five parts |
| `FINDINGS_AT_BOOT` | 490 | 490 | unmoved |
| `stand_act_ids` | 68 and 9 | 68 and 9, green | no stand option moves |
| `stand_contrast`'s `flight-type` | 17.40 | 17.40, green | the stand holds no `multiple` select or group |
| `stand_actionable_keys` | unmoved | green, unmoved | `cells-grid` reads its author key |
| escher-driver's unit tests | 100 | 100 | texts restated in place |
| dioxus-native-dom's unit tests | 82 | 82 | |
| `describe`'s count row | `[12, 13, 12, 3, 8, 4, 15]` | green, unmoved | no verb, node key, cause or session error is added |
| `info_span!` sites in escher-driver | 2 | 2 | |

### Step 21, the audit

`Cargo.lock` moves `pulldown-cmark` 0.13.4 → 0.13.5 and its checksum, no other line (entry 23 reads 0). Entry 22 reads
`advisories ok`, `licenses ok`, with RUSTSEC-2026-0414 gone and `deny.toml` unmoved.

### Cognitive complexity, against 15 on the dev host

`rust-code-analysis-cli`: `findings` 7 · `press_multiple_select_key` 6 · `click_select_row` 6. Beside them
`overlaps` 13 · `visible_region` 11 · `padding_box_in_view` 1 · `gestured` 1 · `apply_row_gesture` 2 · `select_holding` 5.
