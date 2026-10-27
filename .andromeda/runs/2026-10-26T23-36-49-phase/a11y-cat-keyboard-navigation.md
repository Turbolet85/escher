# a11y · Keyboard Navigation — category read

## Carried
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:100) The refactored `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` must still assert that `select_interaction` runs three rounds in `stand-repeat` by the leg's named `--test` list (its `STAND_REPEAT_NAMED` row) and that `stand_act_select` reaches it by its `stand_` prefix: the table walk may not drop either keyboard check's stand-repeat row.
  > both in the `a11y` leg (§9), both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list (.github/scripts/ci-leg.sh:54-58)
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:97) The sweep's witness rows name keyboard checks by file and test name: `stand_keyboard`'s disabled-attribute and inert-subtree tests and `stand_act_keys`. The refactor must keep asserting each of those rows, with the same file and test name, and must rename or move no keyboard test, so the harness this plan states stays where it is cited.
  > a `disabled` attribute skipping a form control and not a `<div disabled tabindex=0>`; the CRUD list taking focus and its keys moving the selection; the Apple delete binding deleting on every OS; and an inert subtree skipped by Tab
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:97) The `stand_act_keys` witness the sweep names must still be asserted after the refactor, with the same file and test, as this plan's driver-level keyboard check.
  > through the driver `stand_act_keys` presses `enter` and `space` on a Tab-focused `counter-increment` (tests/blitz-tests/tests/stand_act_keys.rs:131-165)

## Bindings
- **The sweep's keyboard witness rows ↔ the plan body §9 CI Integration**: the `stand-repeat` named list the sweep asserts must keep the a11y checks this plan runs three times. `select_interaction` and `inert_subtree` are named targets, and `stand_keyboard` and `stand_act_select` are `stand_*` files. Category side (.andromeda/a11y-plan-detail/keyboard-navigation.md:100); body side (.andromeda/a11y-plan.md:237).
- **The sweep's keyboard witness rows ↔ the plan body §5 Test harness pattern**: the `stand_keyboard` tests the sweep names are the harness this body's §5 states. (.andromeda/a11y-plan-detail/keyboard-navigation.md:97) · (.andromeda/a11y-plan.md:140)
