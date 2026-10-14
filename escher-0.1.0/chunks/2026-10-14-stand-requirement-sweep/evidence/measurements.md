# Measurements — 2026-10-14-stand-requirement-sweep

The readings this chunk's checks pin, each read green in the implement gate pass of 2026-10-14 (trail
`.andromeda/runs/2026-10-14T21-20-05-implement/`, entries 1 and 2, both layout modes, per-package build; entry 3
re-read the hit and client-rect files and the sweep under the workspace build). Each check asserts its fixture's
condition first. No mutation was run (inputs#I1).

## C3 — elements that react to a click and state no role

`stand_non_lean_tasks::elements_that_react_to_a_click_and_state_no_role_are_counted`: per task, the elements whose
accessibility node reads `GenericContainer` and that carry the listener mark (`data-dioxus-id`).

| Task | Predicted (research) | Measured |
|---|---|---|
| Counter | 0 | 0 |
| Flight Booker | 0 | 0 |
| Timer | 0 | 0 |
| CRUD | 0 | 0 |
| Temperature Converter | 0 | 0 |
| Circle Drawer | 1 (the canvas) | 1 |
| Cells | 676 (the data cells) | 676 |

Same in both layout modes. No snapshot field was added (the founder's C3 answer).

## C2 — the non-lean tasks keyed

`stand_actionable_keys::the_non_lean_tasks_are_measured`: `unkeyed_actionable()` reads Temperature Converter 0 ·
Circle Drawer 0 · Cells 0 (was 2 · 3 · 676), both modes. With Circle Drawer's dialog open, it reads 0 too
(`stand_non_lean_tasks::circle_drawers_open_dialog_reads_every_control_keyed`).

## C12 — Circle Drawer's dialog rule

`stand_non_lean_tasks::circle_drawers_dialog_rule_applies`: after a canvas click and a second click at the same
point (read as a double click), `#circle-diameter`'s border box is **240** wide in both modes — the
`.cd-dialog input[type="range"]` rule applies (predicted 240).

## C5 — the one-axis clip on the driver

`stand_act_obstructed::a_target_beside_a_box_clipped_on_one_axis_is_refused_covered`: a button held by an
`overflow-x: clip; overflow-y: visible` box, lying below the box's bottom edge with its centre inside its visible
region, is refused **`covered`** with the instance unchanged, in both modes. The CARRY's hypothesis holds.

## C4 — three boxes of their own

`hit_clipped_at_scrolling_box`, both modes, both builds:

| Hit | Answers |
|---|---|
| the centre of an `img` holding injected raster data | the `img` |
| inside a block of an `iframe`'s `srcdoc` sub-document | the `iframe` (the walk does not enter the sub-document) |
| the centre of a filled text `input` | the `input` |

## C6 — the two paths the 2026-10-10 chunk edited

- (1) A box `translate(50px, 30px)` and `overflow: auto`, scrolled 280: the hit where its ninth row shows answers
  the row, and a point inside the box where its first row lies unscrolled, beside the narrower row showing there,
  answers the box. Measured beside it: the client rect reads the transformed box at its layout position,
  `(0, 40, 300, 140)`, not where the transform moves it, so the test's points are those rects' points moved by the
  translate.
- (2) A box under `writing-mode: vertical-lr`, scrolled by (70, 120), reads the client rect it read unscrolled,
  `(40, 50, 300, 200)`, in both modes, under the per-package build (gate entry 2) and the workspace build (entry 3,
  where `writing-mode` compiles the second bounds body).

## C13 — the two range paths

`range_input_interaction`, both modes, `min="1" max="9" step="1.5" value="4"`: two arrows move it to 7; removing
`value` re-seeds **5.5** (the midpoint 5, on the grid); removing `step` re-sanitizes to **6**; after removing `min`,
Home reads **0**; after removing `max`, End reads **100**. A `type` change to `text` clears the range data, and a
change back re-seeds **4** from the `value` attribute (the key had moved it to 6).

## C11 — the click-placed password caret

`password_caret_click`, both modes, the stand's bundled font, a password field at `font-size: 20px`, filled by
typing eight characters, clicked 30 px into its content box:

| Filling | Caret index placed |
|---|---|
| eight narrow characters | **5** |
| eight wide characters | **2** |

The two indexes differ, so a click-placed caret depends on the real characters' widths. The CARRY's hypothesis holds.
It is measured only; its fix is owned by a residuals line (the founder's C11 answer).

## The gate counts (step 12)

- `bash scripts/agent-run.sh run stand` (entry 11), `run.end` at 21:34:37Z: **168** passed · **0** failed · **7**
  ignored · cargo exit 0 · outcome passed, over **40** files. Base 162 · 0 · 7 over 38: +6 = `stand_requirement_sweep`
  2 · `stand_non_lean_tasks` 3 · `stand_act_obstructed` 1.
- Workspace (`target/ci-logs/test.log`, written by entry 13 at 21:36:53Z): **185** result lines · **951** passed ·
  **0** failed · **12** ignored. Base 182 · 937 · 0 · 12: +3 targets and +14 tests = the sweep 2 · non-lean 3 ·
  obstructed 1 · hit 4 · client rect 1 · range 2 · password caret 1.
- `ci-leg.sh a11y` (entry 12): accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 4 ·
  stand_accessibility_ids 8 · stand_contrast 5 · stand_keyboard 14, unchanged.
