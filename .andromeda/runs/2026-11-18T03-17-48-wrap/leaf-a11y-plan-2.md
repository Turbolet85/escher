@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnabled
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=32 class=omission
leaf-text: keeps focus and takes keys, a select
source: dec-3p3pgzo2 «so a focused control that no longer shows keeps focus and takes keys while the tree's focus names the window, a named SC 2.4.3 gap.»
replace-with: keeps focus and takes keys while the tree's focus names the window, a named SC 2.4.3 gap, a select
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=33 class=stale
leaf-text: removing the focused node resets focus to body and runs blur side effects
source: dec-2f62jiwg «Removing the focused node drops focus to the root element, the tree's focus naming the window, and runs blur side effects.»
replace-with: removing the focused node drops focus to the root element, the tree's focus naming the window, and runs blur side effects
