# The re-read — 2026-10-24-a11y-re-read

Measured by /implement run `.andromeda/runs/2026-10-24T21-09-21-implement/`, 2026-10-24T21:10Z–21:21Z.

## The three stand a11y checks

| check | widened `a11y` leg (workspace build, entry 1) | per-package run (entry 7) | `stand-repeat` (per-package build, entry 14) |
|---|---|---|---|
| stand_keyboard | 14 passed, 0 failed | 14 passed, 0 failed | 14 passed, 0 failed in each of rounds 1, 2, 3 |
| stand_contrast | 5 passed, 0 failed | 5 passed, 0 failed | 5 passed, 0 failed in each of rounds 1, 2, 3 |
| stand_accessibility_ids | 8 passed, 0 failed | 8 passed, 0 failed | 8 passed, 0 failed in each of rounds 1, 2, 3 |

No row read red, so step 4's stop did not fire.

## The tables are unedited

The preservation guard (entry 11) printed `guard: allowed 2 — .github/scripts/ci-leg.sh,
.github/scripts/test_ci_workflows.py · companions excluded 0 · changed 2 · excluded 0`, then `guard: clean`, over
`packages tests apps wpt .github scripts examples` against db585b711. So `tests/blitz-tests/tests/common/mod.rs`
(`tab_order`, `ENABLED_TAB_ORDER`, `controls`) and `tests/blitz-tests/tests/stand_contrast.rs` (`TEXT_ROWS`,
`STATE_ROWS`) were not edited, and nothing was re-baselined. `ci.yml`, the stand markup and its CSS are unchanged too.

## 0.2.0's engine changes the re-read covers (scope.md §0.2.0's engine changes)

- 2026-10-18-engine-debts
  - `inert`: skipped by Tab and left out of the tree. Witnessed by `stand_keyboard`'s `an_inert_subtree_is_skipped_by_tab`
    and by `inert_subtree`, now in the leg.
  - `disabled` read on a form control alone, for focusability and the activation keys. Witnessed by `stand_keyboard`'s
    `a_disabled_attribute_skips_a_form_control_and_not_a_div` and `a_button_disabled_after_it_took_focus_takes_no_key`,
    with its stand Tab walks (`tab_order`, `ENABLED_TAB_ORDER`) and Enter/Space rows unchanged.
  - `aria-checked` on the five checkable roles. Witnessed by `accessibility_states`'
    `aria_checked_reads_its_three_values_on_a_checkable_role_alone` and
    `aria_pressed_on_a_checkable_role_reads_nothing_and_a_long_aria_checked_is_nowhere`, now in the leg.
  - The `contain: paint` hit stop, the transformed client rect and the password click caret. Not a11y readings. They
    stay witnessed in `stand-repeat` (`hit_clipped_at_scrolling_box`, `transformed_client_rect`, `password_caret_click`,
    all green in its three rounds). The stand's `stand_accessibility_ids` tree and `stand_contrast` rows read unchanged
    across them.
- 2026-10-18-one-virtual-clock: the double-click window on the document's clock. The stand's Enter/Space and Tab rows in
  `stand_keyboard` read unchanged under the harness's `VirtualClock`, and `select_interaction`'s
  `typed_text_is_kept_for_one_second_of_the_documents_clock` reads the document's clock, now in the leg.
- 2026-10-20-select-by-id: the select's keyboard and typeahead model, and a click focusing it. The stand holds no select
  (scope.md, the premise-corrected bullet). Witnessed on fixtures by `select_interaction` (7) and through the driver by
  `stand_act_select` (5), both now in the leg.
- 2026-10-17-upstream-sync-engine-debts: upstream's `aria-labelledby`, `aria-describedby`, `aria-description`,
  `aria-expanded`, `aria-disabled` and `aria-level` reads. Witnessed by `accessibility_label` (13),
  `accessibility_description` (3) and `accessibility_merged_states` (3), now in the leg. `accessibility_active_descendant_unread`
  (1) pins the unread `aria-activedescendant` as a gap. The stand's tree in `stand_accessibility_ids` reads unchanged,
  since no stand markup carries these attributes.
- 2026-10-24-upstream-sync-polish-and-ship: ten upstream commits cherry-picked at `f9cbb2756`. Covered by all fourteen
  leg files and the three stand checks reading green on the tree that holds them. The stand graph holds neither
  platform adapter: the `cargo tree` probe (entry 9) read 0.

SC 1.4.3 holds on every non-exempt stand row: `stand_contrast` is green in the leg and in all three `stand-repeat` rounds,
and its exempt class (`enabled == Some(false)`) is unedited (the guard).
