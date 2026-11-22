@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=33 class=omission
leaf-text: its rows one height
source: dec-2qmhfbqn «An open select list's rows take one height, the tallest row's line with the select's own one-line height as the floor, so a row with a larger authored font is painted whole.»
replace-with: its rows one height, the tallest row's line with the select's own one-line height as the floor, so a row with a larger authored font is painted whole
