@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=27 class=omission
leaf-text: the canvas read, which reads one canvas element's draw commands, being no reader of the snapshot model
source: dec-45plribb «the canvas read, which reads one canvas element's draw commands, and the virtual-node read, which reads one canvas's declared nodes, are no readers of the snapshot model.»
replace-with: the canvas read, which reads one canvas element's draw commands, and the virtual-node read, which reads one canvas's declared nodes, being no readers of the snapshot model
