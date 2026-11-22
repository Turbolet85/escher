@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnabled
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=19 class=stale
leaf-text: the focus fixup firing no `blur`, a single select's `size` unread, `aria-activedescendant` unread
source: ctr-2upfv5xp «contract/accessibility-tree/select-role · the accessibility tree · select — A select reads ListBox where it is a list box, multiple or of display size above 1, else ComboBox;»
replace-with: the focus fixup firing no `blur`, `aria-activedescendant` unread
