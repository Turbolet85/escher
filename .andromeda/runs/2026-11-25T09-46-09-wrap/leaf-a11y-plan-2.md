@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnabled
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=27 class=omission
leaf-text: a `UiGraph` of one node per snapshot node, names and closed words only
source: dec-45n3iy3a «The read-only UI graph holds one node per snapshot node but a canvas's virtual nodes, which it excepts, so the actions answer and exploration hold none of them; names and closed words only.»
replace-with: a `UiGraph` of one node per snapshot node but a canvas's virtual nodes, which it excepts, so the actions answer and exploration hold none of them, names and closed words only
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=27 class=omission
leaf-text: none on `TextRun`, document-root or `Window` nodes
source: ctr-2peziqew «The form {owner}///{key} is reserved for a canvas's virtual nodes and served in the accessibility tree and, from it, the snapshot and its diff; no element's id holds ///.»
replace-with: none on `TextRun`, document-root or `Window` nodes; a canvas's virtual nodes are served in this tree, and from it the snapshot and its diff, under the reserved form `{owner}///{key}`, which no element's id holds
@leaf_edit n=3 leaf=.claude/rules/a11y.md line=32 class=omission
leaf-text: The snapshot's `enabled` follows the presence reading.
source: ctr-3abkoz7e «The snapshot's enabled field reads the presence of the element's disabled attribute, and on a virtual node the declaration's disabled flag, enabled false where declared disabled.»
replace-with: The snapshot's `enabled` follows the presence reading, and on a canvas's virtual node the declaration's disabled flag, `enabled` false where declared disabled.
