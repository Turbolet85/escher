## 2026-10-18-engine-debts — the blitz-tests scope: five new checks, five restated
**Section:** §Existing Scopes → blitz-tests
**Change:**
- `accessibility_states`: was «`aria-checked` setting none of the three, 4 tests»; now `aria-checked` on a checkable role alone, neither attribute on a native checkbox or radio, a 5,500-character value nowhere, 5 tests.
- Added: `accessibility_merged_states` 3, `accessibility_active_descendant_unread` 1, `inert_subtree` 4, `transformed_client_rect` 4, and the fourteenth driver-action file `stand_act_transformed` 2.
- `hit_clipped_at_scrolling_box`: a `contain: paint` box stops the hit (was «still hit through»); `scrolled_box_absolute_position`: a scrolled box read where it stands (was «pinned as it reads»).
- `stand_keyboard`: a `disabled` attribute skips a form control and not a `<div disabled tabindex=0>` (was «of any value skipped, the limit pinned»); an inert subtree skipped by Tab (was «the engine honouring no `inert`»); the falsy-attribute clause: a truthy `inert` honoured, a falsy one cleared.
- `password_caret_click`: was 1 test pinning index 5 and 2; now 2 tests, one index for two values of one length and a double click selecting the whole value.
- `session_common` readers: was twenty-one with the thirteen `stand_act_*`; now twenty-two, fourteen.
**Why:** the chunk's new and rewritten checks; their fixes PROVISIONAL, delegate operator, as the batch entry states.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
