@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=67
last: - 2026-11-13: A control that deletes a loop's advance statemen
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own l
@leaf_edit n=1 leaf=.claude/rules/testing.md line=36 class=omission
leaf-text: keep the scripts free of a `screenshot` step (a shot's PNG bytes differ by OS).
source: dec-fcya4gfl «The six served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes»
replace-with: keep the scripts free of a `screenshot` step (a shot's PNG bytes differ by OS). The six served tasks are read on every CI OS by one stand check, `stand_cross_os_readings`, which compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file.
