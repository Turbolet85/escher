# Seen red first, and the controls (plan step 9)

Implement's record of one-shot readings, 2026-10-10. Every line names the build it was read under: **per-package** is
`cargo test -p blitz-tests --locked --no-fail-fast --test …` (the `document.rs` body of the bounds reader),
**workspace** is `cargo test --workspace --locked --no-fail-fast --test …` (the `writing_mode.rs` body). Every check
loops both layout modes; a red check stops at its first mode, so each red below was read at `incremental=false`. The
figures are result lines and test names, written by hand; no raw cargo output is kept here.

## 1. On the engine as built (no engine line edited; the three new check files in the tree)

The same result lines under both builds:

| File | per-package | workspace |
|---|---|---|
| `scrolled_box_client_rect` | 4 passed · 2 failed | 4 passed · 2 failed |
| `hit_clipped_at_scrolling_box` | 4 passed · 5 failed | 4 passed · 5 failed |
| `scrolled_box_absolute_position` | 1 passed · 0 failed | 1 passed · 0 failed |

- **Red, the bounds regression cases (2):** `a_scrolled_box_reads_the_client_rect_it_read_unscrolled` — at "the
  scrolled box reads the client rect it read unscrolled" · `a_scrolled_box_inside_a_scrolled_box_moves_by_the_outer_offset_alone`
  — at "the outer box reads the client rect it read unscrolled".
- **Green, the four guards of the re-based callers:** `an_element_inside_a_scrolled_box_moves_by_the_box_offset` ·
  `an_inline_element_moves_with_its_scrolling_inline_root` · `an_inline_element_offset_rect_ignores_its_offset_parent_scroll`
  · `the_visible_region_inside_a_scrolled_box_is_its_padding_box_cut_by_the_viewport`.
- **Red, the hit regression cases (5):** `a_point_above_a_scrolled_box_answers_the_sibling_there` ·
  `a_point_below_an_unscrolled_box_answers_what_shows_there` · `a_hidden_overflow_box_scrolled_by_a_program_reads_the_same`
  · `an_overflow_clip_box_keeps_its_clipped_out_content_from_a_hit` ·
  `a_box_clipped_on_one_axis_is_stopped_at_its_whole_padding_box` — each at its hit assertion, after its fixture
  conditions held.
- **Green, the hit guards (4):** `a_point_inside_the_box_answers_the_row_showing_there` ·
  `an_overflow_visible_box_is_hit_through_to_its_overflowing_content` ·
  `a_scrolled_box_overlay_scrollbar_thumb_is_still_resolved` ·
  `a_contain_paint_box_with_visible_overflow_is_still_hit_through`.
- **Green, the second reader's pin:** `a_scrolled_box_reads_its_absolute_position_less_its_own_scroll_offset`.

## 2. After step 5 (the bounds reader), before step 6 and before step 7's edits

| File | per-package | workspace |
|---|---|---|
| `scrolled_box_client_rect` | 6 passed · 0 failed | 6 passed · 0 failed |
| `scrolled_box_absolute_position` | 1 passed · 0 failed | 1 passed · 0 failed |
| `stand_act_scroll` (unedited) | 4 passed · 2 failed | 4 passed · 2 failed |

Red by design, the two pinning tests of `stand_act_scroll` as they then stood:
`a_scrolled_box_reads_its_bounds_shifted_and_a_click_naming_it_lands_off_its_centre` — at "the list's bounds read
shifted up by its own scroll offset" · `a_click_naming_a_scrolled_box_is_refused_off_screen_while_the_box_is_in_view`
— at "the box's bounds read shifted up by its own scroll offset, out of the viewport".

## 3. After step 6 (the hit walk), before step 7's edits

| File | per-package | workspace |
|---|---|---|
| `hit_clipped_at_scrolling_box` | 9 passed · 0 failed | 9 passed · 0 failed |
| `scrolled_box_client_rect` | 6 passed · 0 failed | 6 passed · 0 failed |
| `scrolled_box_absolute_position` | 1 passed · 0 failed | 1 passed · 0 failed |
| `stand_act_obstructed` (unedited) | 4 passed · 1 failed | 4 passed · 1 failed |

Red by design, the pinning test of `stand_act_obstructed` as it then stood:
`a_button_where_scrolled_out_rows_extend_reads_covered_only_before_its_box` — at "the hit at the earlier button's
centre answers content of the box".

## 4. Mutation controls, on the finished tree

Each control: the file's bytes saved, the mutation written, the named tests read, the saved bytes written back, and
the file's sha256 compared with the saved bytes' before the next step. The green reading (§5) was taken after every
restore read equal.

The finished files:

| File | sha256 |
|---|---|
| `packages/blitz-dom/src/document.rs` | `e7fc110d55fc4214b84045d77400256ab35920641933c61bc80a3d74c6212026` |
| `packages/blitz-dom/src/layout/writing_mode.rs` | `15e59e675b88ead8dd12797a4ae728e824cb23289e329f48ea30bd615e3c4554` |
| `packages/blitz-dom/src/scrolling.rs` | `888130bf6aa0907ff534d97eec32a2cb2a1c758c7248fd357edf705a1783a80a` |
| `packages/blitz-dom/src/node/node.rs` | `19c81b0c8155ec85cabf51e7f8f47b9ce5793e27759f858df626a2f1b2c1551f` |

| Control | Build | Reading | Restore |
|---|---|---|---|
| **M1** the own-offset term put back in the `document.rs` body | per-package | **red** — `scrolled_box_client_rect` 1 passed · 5 failed | equal |
| | workspace | green — 6 passed · 0 failed (the mutated body is not the one this build compiles) | |
| **M2** the own-offset term put back in the `writing_mode.rs` body | workspace | **red** — `scrolled_box_client_rect` 1 passed · 5 failed | equal |
| | per-package | green — 6 passed · 0 failed (the mutated body is not the one this build compiles) | |
| **M3** `visible_region`'s add-back put back | per-package | **red** — `scrolled_box_client_rect` 5 passed · 1 failed; `scroll_into_view_nested` 4 passed · 3 failed | equal |
| **M4** the own-area test put back, in scrolled coordinates | per-package | **red** — `hit_clipped_at_scrolling_box` 7 passed · 2 failed | equal |
| **M5** the padding-box stop removed | per-package | **red** — `hit_clipped_at_scrolling_box` 4 passed · 5 failed; `stand_act_obstructed` 4 passed · 1 failed | equal |
| **M6** the predicate narrowed so `overflow: clip` no longer stops | per-package | **red** — `hit_clipped_at_scrolling_box` 7 passed · 2 failed | equal |

The tests each control turned red:

- **M1 and M2** (the same five, each under its own build): `a_scrolled_box_reads_the_client_rect_it_read_unscrolled` ·
  `a_scrolled_box_inside_a_scrolled_box_moves_by_the_outer_offset_alone` ·
  `an_inline_element_moves_with_its_scrolling_inline_root` ·
  `an_inline_element_offset_rect_ignores_its_offset_parent_scroll` ·
  `the_visible_region_inside_a_scrolled_box_is_its_padding_box_cut_by_the_viewport`. The three guards turn red with
  the two regression cases because the re-based callers then count the offset twice. M1 green under the workspace
  build and M2 green under the per-package build is the measured form of "a fix in one body alone reads green under
  one runner".
- **M3:** `the_visible_region_inside_a_scrolled_box_is_its_padding_box_cut_by_the_viewport` (the named case), and of
  the standing `scroll_into_view_nested`: `a_box_in_view_is_scrolled_and_the_viewport_is_not` ·
  `a_box_below_the_viewport_and_the_viewport_both_move` · `two_nested_boxes_both_move`.
- **M4:** `a_point_above_a_scrolled_box_answers_the_sibling_there` (the named case) ·
  `a_hidden_overflow_box_scrolled_by_a_program_reads_the_same`.
- **M5:** `a_point_below_an_unscrolled_box_answers_what_shows_there` (the named case) ·
  `a_point_above_a_scrolled_box_answers_the_sibling_there` · `a_hidden_overflow_box_scrolled_by_a_program_reads_the_same`
  · `an_overflow_clip_box_keeps_its_clipped_out_content_from_a_hit` ·
  `a_box_clipped_on_one_axis_is_stopped_at_its_whole_padding_box`; and the restated `stand_act_obstructed` test
  `a_button_where_scrolled_out_rows_extend_is_hit_and_clicked_as_itself` (the named one).
- **M6:** `an_overflow_clip_box_keeps_its_clipped_out_content_from_a_hit` (the named case) ·
  `a_box_clipped_on_one_axis_is_stopped_at_its_whole_padding_box`.

After the last control every one of the four files read the sha256 in the table above.

## 5. Green on the restored tree

| File | per-package | workspace |
|---|---|---|
| `scrolled_box_client_rect` | 6 passed · 0 failed | 6 passed · 0 failed |
| `hit_clipped_at_scrolling_box` | 9 passed · 0 failed | 9 passed · 0 failed |
| `scrolled_box_absolute_position` | 1 passed · 0 failed | 1 passed · 0 failed |
| `stand_act_scroll` (restated) | 6 passed · 0 failed | 6 passed · 0 failed |
| `stand_act_obstructed` (restated) | 5 passed · 0 failed | 5 passed · 0 failed |
| `scroll_into_view_nested` (unedited) | 7 passed · 0 failed | 7 passed · 0 failed |

## 6. What the pins measured

- **The second reader, `Node::absolute_position(0.0, 0.0)`** — the hypothesis read true, in both layout modes and
  under both builds: a 300 × 200 box standing at (40, 50), unscrolled, reads (40, 50); scrolled by (70, 120), with the
  viewport unscrolled and the box not moved, it reads (−30, −70) — the reading before the scroll less the box's own
  scroll offset. Pinned by `scrolled_box_absolute_position`; green on the engine as built and through the chunk. No
  line of the reader or of a caller of it was edited.
- **The remainder under option 1B, as far as measured** — a `contain: paint` box whose `overflow` computes `visible`
  on both axes: the hit at a point below the box, where a row overflows and paint clips it away, answers the row — as
  hypothesised, before and after the fix, both layout modes, both builds. So the `covered` meaning carries its clause.
  Not measured here: an image's, a sub-document's and a text input's own box.
- **The one-axis case** — `overflow-x: clip` beside `overflow-y: visible` computes as written; content overflowing
  along the visible axis is not reached by a hit after the fix (it was before), the walk stopping at the box's whole
  padding box.
- **A plain button's focus after a pointer click** (the restated `stand_act_obstructed` test, both layout modes, both
  builds): after the accepted click on the button no element reads `focused` in the snapshot and the accessibility
  tree's focus carries no author id — no row of the box, and not the button either. This is the reading a11y-plan §5
  records as read from code; it is now measured.
- **The stand's CRUD list** (the restated `stand_act_scroll` test): at 12 Creates and at 14 the scrolled list reads
  the four figures it read before the scroll, the scroll's diff does not name it, and a click naming it selects the
  row lying at the centre of its box — the row deleted is the seventh-indexed at 12 Creates and the ninth-indexed at
  14, where before the fix the click at 14 selected a different row than the one at the centre.
