@leaf_group group=a11y-plan-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=45
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=23 class=stale
leaf-text: focus resets to body on removal
source: dec-2f62jiwg «Removing the focused node drops focus to the root element, the tree's focus naming the window, and runs blur side effects.»
replace-with: removing the focused node drops focus to the root element, the tree's focus naming the window, and runs blur side effects
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=24 class=omission
leaf-text: keeps focus and takes keys, with no panic
source: dec-3p3pgzo2 «disabled and hidden take no part, so a focused control that no longer shows keeps focus and takes keys while the tree's focus names the window, a named SC 2.4.3 gap.»
replace-with: keeps focus and takes keys while the tree's focus names the window, a named SC 2.4.3 gap, with no panic
