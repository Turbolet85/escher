@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=76
last: - 2026-11-21: A kept check read back by `Check::from_bytes` 
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=23 class=omission
leaf-text: turns it red, and the restored tree reads green again;
source: dec-7jkoc3jb «A new check is controlled: a named mutation of what it guards turns it red, a case it leaves green widened until it does, and the restored tree reads green;»
replace-with: turns it red, a case it leaves green widened until it does, and the restored tree reads green;
@leaf_edit n=2 leaf=.claude/rules/testing.md line=35 class=omission
leaf-text: blitz-html would be a circular dev-dependency.
source: dec-mopr7eaf «A Dioxus fixture select whose choice a check moves is written as a controlled select.»
replace-with: blitz-html would be a circular dev-dependency. A Dioxus fixture select whose choice a check moves is written as a controlled select (a fixture's `selected` literal is re-written on any re-render of its component, re-seeding the choice).
@leaf_edit n=3 leaf=.claude/rules/verification-harness.md line=37 class=omission
leaf-text: so a check that a click clears focus asserts the root element.
source: dec-qygu7ld5 «A check asking whether a node has a box reads the node's computed display beside its client rect.»
replace-with: so a check that a click clears focus asserts the root element. A check asking whether a node has a box reads the node's computed display beside its client rect (a node a restyle turns `display: none` keeps the client rect it last laid out at).
