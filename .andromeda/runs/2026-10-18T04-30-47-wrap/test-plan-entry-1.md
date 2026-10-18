## 2026-10-18-engine-debts — §1 coverage: the five new checks, the fixes' checks restated, the CARRY observed present
**Section:** §1 Test Scope Summary (blitz-paint · escher-driver · dioxus-native-dom · tests/blitz-tests)
**Change:**
- blitz-paint: `password_caret_click` was 1, pinning index 5 and 2 «by the real characters' widths»; now 2 — one index for two values of one length, a double click selecting the whole value.
- escher-driver: the executor covered by fourteen `stand_act_*` files (was thirteen).
- dioxus-native-dom: was 69 unit tests; now 72, three on the toggle's sources and `aria-disabled`; the `disabled` snapshot pin's gloss reads presence on a form control (was «in every document»).
- tests/blitz-tests: `accessibility_states` 4 → 5, `aria-checked` read on the checkable roles alone and neither attribute on a native checkbox or radio; the «observed absent» merged-ARIA clause gives way to `accessibility_merged_states` 3; added `accessibility_active_descendant_unread` 1, `inert_subtree` 4, `transformed_client_rect` 4 and `stand_act_transformed` 2; `hit_clipped_at_scrolling_box` reads a `contain: paint` box stopping the hit, `scrolled_box_absolute_position` a scrolled box where it stands, the transformed box's client rect where its transform moves it — «what stays measured and not fixed» retired; `stand_keyboard`'s `disabled` and `inert` readings restated; the falsy-`inert` clause: a truthy one honoured.
**Why:** the chunk's checks; the merged reads were the CARRY this chunk discharged.
**Kept:** two «observed absent» claims a detector proposed — the password click's shift-click and triple-click branches, and a disabled `summary` or link activating — were returned at Validate, no search behind them; a later chunk's report may carry them.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
