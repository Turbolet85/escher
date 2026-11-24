@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=27 class=omission
leaf-text: a `UiGraph` of one node per snapshot node, names and closed words only,
source: dec-35ymmlqw «The text form, the diff, the text parts, MCP tool results, the node keys' help and the screenshot read only the one snapshot model, with no second role or name mapping; the canvas read, which reads»
replace-with: a `UiGraph` of one node per snapshot node, names and closed words only — the canvas read, which reads one canvas element's draw commands, being no reader of the snapshot model —,
