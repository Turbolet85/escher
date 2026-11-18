@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=35 class=omission
leaf-text: so a control enabled after boot needs its place in `ENABLED_TAB_ORDER`,
source: dec-3pl5dhbb «except: A Tab order reached only after a state change, such as the order page with its failure shown, is stated as a local table of the keyboard check, not in the shared table.»
replace-with: so a control enabled after boot needs its place in `ENABLED_TAB_ORDER`, while a Tab order reached only after a state change, such as the order page with its failure shown, is stated as a local table of the keyboard check, not in the shared table,
