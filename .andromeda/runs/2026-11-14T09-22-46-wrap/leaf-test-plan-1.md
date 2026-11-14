@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: asserted unconditionally, no font skip;
source: dec-7prkrwp5 «Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · guard: stand_bundled_font_families»
replace-with: asserted unconditionally under the bundled DejaVu Sans with system fonts off, no font skip, guarded by `stand_bundled_font_families`;
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=26 class=omission
leaf-text: so a check compared byte for byte across OSes holds no shot —,
source: dec-fcya4gfl «Cross-OS readings of the served tasks — The six served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot»
replace-with: so a check compared byte for byte across OSes holds no shot, and the six served tasks are read on every CI OS by one stand check, `stand_cross_os_readings`, comparing the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file —,
