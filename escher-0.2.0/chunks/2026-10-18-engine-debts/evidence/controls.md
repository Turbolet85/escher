# Mutation controls — 2026-10-18-engine-debts

Step 10, one-shot, run 2026-10-18 at /implement on the dev host. Per control: the touched source files' bytes saved,
the one fix reverted by exact string replacement (each replacement asserted to match once), the check run
(`cargo test -p blitz-tests --locked --test {check}`) and read red as a FAILED test — never a compile error — the
saved bytes written back and their sha256 compared, then the check run again and read green. The seconds are the red
run's wall clock, its rebuild included; the test named is the first that read FAILED.

The reverts, by control:
- `inert_subtree` (F1): `Node::is_focussable`'s inert walk, `hit_inner`'s inert stop and the builder's inert
  exclusion each short-circuited off (`node.rs`, `accessibility.rs`).
- `stand_keyboard` (F7): `ElementData::is_disabled` back to presence on any element (`element.rs`).
- `accessibility_states` (F2): the builder's `toggled` read back to `aria-pressed` on every role (`accessibility.rs`).
  Re-run 2026-10-18 after the operator's two corrections inside F2 A (a native checkbox or radio takes neither
  `aria-checked` nor `aria-pressed`), on the read's final shape — `is_native_checkable` → none, else
  `is_checkable(role)` → `aria-checked`, else `aria-pressed` — with the checkable arm short-circuited off; the row is
  that run's. The native gate's own control — `is_native_checkable` short-circuited off alone — read the same check
  red at `aria_checked_reads_its_three_values_on_a_checkable_role_alone` (3.2 s), sha256 equal, green; it is recorded
  in `readings.md`, not as a row here.
- `scrolled_box_absolute_position` (F4): `absolute_position` subtracting the node's own scroll offset again, and
  `scroll_into_view`'s add-back restored (`node.rs`, `scrolling.rs`).
- `hit_clipped_at_scrolling_box` (F5): `hit_inner`'s padding-box stop at `contain: paint` short-circuited off
  (`node.rs`).
- `transformed_client_rect` and `stand_act_transformed` (F8): `is_transformed_chain` reading no transform
  (`document.rs`).
- `password_caret_click` (F6): the click target's `password` read false (`pointer.rs`).

| check | test | red | seconds | restore | green |
|---|---|---|---|---|---|
| inert_subtree | the_tree_holds_no_node_for_an_inert_subtree | red | 3.3 s | sha256 equal | green |
| stand_keyboard | a_disabled_attribute_skips_a_form_control_and_not_a_div | red | 7.5 s | sha256 equal | green |
| accessibility_states | aria_pressed_on_a_checkable_role_reads_nothing_and_a_long_aria_checked_is_nowhere | red | 3.4 s | sha256 equal | green |
| scrolled_box_absolute_position | a_scrolled_box_reads_its_absolute_position_where_it_stands | red | 3.0 s | sha256 equal | green |
| hit_clipped_at_scrolling_box | a_contain_paint_box_stops_the_hit_at_its_padding_box | red | 3.0 s | sha256 equal | green |
| transformed_client_rect | a_translated_box_reads_its_rect_moved_by_the_translation | red | 3.6 s | sha256 equal | green |
| stand_act_transformed | the_snapshot_bounds_of_a_translated_button_read_where_it_shows | red | 3.6 s | sha256 equal | green |
| password_caret_click | a_click_at_one_offset_places_the_caret_by_the_bullets | red | 3.4 s | sha256 equal | green |
