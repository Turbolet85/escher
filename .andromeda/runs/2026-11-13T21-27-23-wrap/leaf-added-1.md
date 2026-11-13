@leaf_group group=added-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=45
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=13 class=omission
leaf-text: **Focus harness:** `Harness::focused()` / `hovered()`.
source: ctr-523awgmf «With no node focused, the harness's focused reading answers the root html element, never none, so a check that a click clears focus asserts the root element.»
replace-with: **Focus harness:** `Harness::focused()` / `hovered()` — with no node focused, `focused()` answers the root `html` element, never none, so a check that a click clears focus asserts the root element.
