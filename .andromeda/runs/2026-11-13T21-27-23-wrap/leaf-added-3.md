@leaf_group group=added-3
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnabled
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=33 class=omission
leaf-text: a CRUD row or the list — still clears focus;
source: ctr-523awgmf «With no node focused, the harness's focused reading answers the root html element, never none, so a check that a click clears focus asserts the root element.»
replace-with: a CRUD row or the list — still clears focus (with no node focused the harness's focused reading answers the root html element, never none, so a check that a click clears focus asserts the root element);
