@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=57
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=13 class=omission
leaf-text: inspection (`query`, `layout_rect`, `text_content`, `focused`, `hit`, `dom_string`)
source: dec-qygu7ld5 «decision/box-presence-read-with-display · Box presence read with computed display — A check asking whether a node has a box reads the node's computed display beside its client rect.»
replace-with: inspection (`query`, `layout_rect`, `text_content`, `focused`, `hit`, `dom_string`; a check asking whether a node has a box reads the node's computed display beside its client rect, since a node a restyle turns `display: none` keeps the client rect it last laid out at)
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=18 class=omission
leaf-text: **Test-data bootstrap:** nothing is seeded — the fixtures are code:
source: dec-mopr7eaf «decision/moved-fixture-select-controlled · Fixture select whose choice moves is controlled — A Dioxus fixture select whose choice a check moves is written as a controlled select.»
replace-with: **Test-data bootstrap:** nothing is seeded — the fixtures are code (a Dioxus fixture select whose choice a check moves written as a controlled select, since a fixture's `selected` literal is re-written on any re-render of its component, re-seeding the choice):
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=44 class=omission
leaf-text: a named mutation of what it guards turning it red and the restored tree reading green
source: dec-7jkoc3jb «decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, a case it leaves green widened until it does, and the restored tree reads green;»
replace-with: a named mutation of what it guards turning it red, a case it leaves green widened until it does, and the restored tree reading green
