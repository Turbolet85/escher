@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=70
last: - 2026-11-14: The Integration bullet's counts are stale — it
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own l
@leaf_edit n=1 leaf=.claude/rules/testing.md line=23 class=omission
leaf-text: a named mutation of what it guards turns it red, and the restored tree reads green again.
source: dec-7jkoc3jb «A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again; a control that removes a bound loosens it to twice the budget rather than removing it»
replace-with: a named mutation of what it guards turns it red, and the restored tree reads green again; a control that removes a bound loosens it to twice the budget rather than removing it, so the mutated run stays bounded.
