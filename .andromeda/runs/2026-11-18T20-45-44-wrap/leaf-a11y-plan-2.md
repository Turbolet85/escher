@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=25 class=omission
leaf-text: `aria-level` (a number, with the tag and role defaults)
source: ctr-4ard553y «contract/aria-property-reads/aria-level · ARIA name and property reads · aria-level — aria-level is read as its number less one, headings keeping their default levels.»
replace-with: `aria-level` (read as its number less one, headings keeping their default levels)
