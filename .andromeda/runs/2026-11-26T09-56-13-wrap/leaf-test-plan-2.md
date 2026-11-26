@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=79
last: - 2026-11-25: A change of the driver's fixed program text — 
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own l
@leaf_edit n=1 leaf=.claude/rules/testing.md line=35 class=stale
leaf-text: A Circle Drawer check that needs circles apart writes `data-circles` through the mutator rather than clicking — driver clicks on Circle Drawer all land at the canvas's centre.
source: dec-vib2fltb «A Circle Drawer check that reads circles apart at widget level writes `data-circles` through the mutator; a check whose click or key acts on circles apart adds them by harness clicks off the centre»
replace-with: A Circle Drawer check that reads circles apart at widget level writes `data-circles` through the mutator; a check whose click or key acts on circles apart adds them by harness clicks off the centre, since a `data-circles` write sets no app state — driver clicks on Circle Drawer all land at the canvas's centre.
