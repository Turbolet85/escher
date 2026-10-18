# Readings — 2026-10-18-engine-debts

Step 12: the measured `visible_region` reading (step 8) and every count the plan forecasts, read at /implement on the
dev host on the gate run filed under `.andromeda/runs/2026-10-18T03-17-10-implement/gate-2026-10-18-engine-debts.json`
(entries 1–25, 27, 28 green; 26 not run — its gated defer; 29–31 the operator's).

## The `visible_region` reading (step 8's forecast)

Measured as forecast: `transformed_client_rect::a_target_inside_a_translated_scrolling_box_reads_outside_its_visible_region`
— a 100 × 40 target at the top of a 300 × 100 `overflow:auto` box under `transform:translate(300px, 0)` reads its
client rect moved by (300, 0), inside the viewport, while `visible_region` reads the same region as on the
untransformed page, so the target's centre lies outside its own visible region. The test pins that reading, and
`off-screen`'s meaning gains the limit sentence in `refusal.rs` (the doc comment, `meaning()`, the unit table) and in
the crate docs of `lib.rs` («Three readings are known to be off»).

## Counts the plan forecasts

| count | forecast | measured |
|---|---|---|
| blitz-tests files (`tests/blitz-tests/tests/*.rs`, `all.rs` included) | 127 → 132 | 132 |
| blitz-tests targets (files less the unbuilt `all.rs`) | 126 → 131 | 131 |
| `run stand` files (`stand_*.rs`) | 40 → 41 | 41 |
| `session_common` readers (`mod session_common;`) | 21 → 22 | 22 |
| `stand_act_*` files | 13 → 14 | 14 |
| `accessibility_merged_states` tests | 3 | 3 passed |
| `inert_subtree` tests | 4 | 4 passed |
| `stand_keyboard` tests | 14 | 14 passed |
| `accessibility_states` tests | 4 → 5 | 5 passed |
| dioxus-native-dom lib tests | 69 → 72 | 72 passed |
| escher-driver lib tests | 74 | 74 passed |
| `accessibility_active_descendant_unread` tests | 1 | 1 passed |
| `scrolled_box_absolute_position` tests | 1 | 1 passed |
| `scroll_into_view_nested` tests | 7 | 7 passed |
| `hit_clipped_at_scrolling_box` tests | 13 | 13 passed |
| `transformed_client_rect` tests | 4 | 4 passed |
| `stand_act_transformed` tests | 2 | 2 passed |
| `stand_act_obstructed` tests | 6 | 6 passed |
| `password_caret_click` tests | 1 → 2 | 2 passed |
| mutation controls (`evidence/controls.md`) | 8 | 8 rows |
| merge-surface rows (`evidence/merge-surface.md`) | 7 fixed (F3 uncounted) | 7 |
| `stand-repeat` | 3 of 3 rounds green | 3 of 3 rounds green (277.22 s) |
| coverage floors | 3 of 3 crates | 3 of 3 — escher-driver 98.19 % (floor 97), dioxus-native-dom 84.57 % (floor 83), blitz-test-harness 91.69 % (floor 90) |
| the census over added engine and snapshot lines | 0 | 0 |
| the silence probe over the new and rewritten checks | 0 | 0 |

## The native checkbox and radio (F2, the operator's correction)

After the operator's two corrections inside F2 A — a native `<input type="checkbox">` or `<input type="radio">`
takes neither `aria-checked` nor `aria-pressed`, its native checkedness winning (HTML-AAM, ARIA in HTML) — the
builder reads no author toggle on such an input, `aria-checked` on the five checkable roles otherwise, and
`aria-pressed` everywhere else. `accessibility_states`' fourth test asserts that an unchecked native checkbox, an
unchecked native radio, an unchecked native checkbox carrying `role="switch"` and an unchecked native radio carrying
`role="button"` each read no `toggled` from `aria-checked="true"` and none from `aria-pressed="true"` (count still
5). The native gate's own mutation control — `is_native_checkable` short-circuited off alone — read
`accessibility_states` red at `aria_checked_reads_its_three_values_on_a_checkable_role_alone` (3.2 s), sha256 equal on
restore, green; the F2 row of `controls.md` was re-run on the read's final shape (3.4 s). After the first correction,
entries 3, 4, 5, 15, 17, 18, 21, 22, 23 (`stand-repeat: 3 of 3 rounds green`, 268.48 s), 24, 27 and 28 were re-fired
green; after the second, entries 4, 15, 17, 18, 21, 22 (a11y), 24 (coverage) and 27 (fast) — their lines in the
outcome report.

Measured from the builder's code: native checkedness does not reach the accessibility tree. `accessibility.rs` reads
no `checkbox_input_checked()` and no `CHECKED` state; `toggled` is written from the author attributes alone, so a
native checkbox or radio, checked or not, carries no `toggled` on its tree node. The base's own pin showed it too:
the old fourth test read a native `<input type="checkbox" checked aria-checked="true">` as no state at all. The
snapshot's `checked` is unaffected — it reads the element's native checkedness, not the tree.

## The password caret (F6)

`password_caret_click::a_click_at_one_offset_places_the_caret_by_the_bullets` reads the same caret index for
`iiiiiiii` and `WWWWWWWW` at the 30 px click, in both layout modes, inside the value (1–7); the check asserts the two
equal and does not print the index, which this record therefore does not state. Before the fix the pin read 5 and 2.
