@leaf_group group=a11y-plan-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=45
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=15 class=omission
leaf-text: reading `None` past an element on its walk with a `mix-blend-mode` other than `normal`
source: dec-2yo3q4m4 «The text contrast reading reads nothing where an element on its walk has opacity below 1, a non-empty filter, a background-image layer or a mix-blend-mode other than normal.»
replace-with: reading `None` past an element on its walk with opacity below 1, a non-empty filter, a background-image layer or a `mix-blend-mode` other than `normal`
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=23 class=stale
leaf-text: a click on any other element (a CRUD row, the list) still clears focus
source: dec-3p3ll4ik «a click on any other non-interactive element still clears focus.»
replace-with: a click on any other non-interactive element (a CRUD row, the list) still clears focus
@leaf_edit n=3 leaf=.claude/docs/a11y-summary.md line=24 class=omission
leaf-text: a password or file-input value reads the fixed mask `MASKED_VALUE`
source: ctr-37t3bdwi «A password input's value reads MASKED_VALUE where it holds text and "" where it holds none.»
replace-with: a password input's value reads the fixed mask `MASKED_VALUE` where it holds text and `""` where it holds none, a file input's where its `value` attribute is non-empty
