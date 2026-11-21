@leaf_group group=design-system
@leaf_read leaf=.claude/docs/design-summary.md lines=43
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/la
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=16 class=omission
leaf-text: a `multiple` select's chosen row `#c8c8c8` background with `#000000` text (12.56:1)
source: dec-flfsya6d «A list box's chosen row, of a multiple select or of a single select sized above 1, is painted with Chromium's inactive selected pair, a #c8c8c8 background with #000000 text, 12.56:1.»
replace-with: a list box's chosen row (a `multiple` select's, or a single select's sized above 1) `#c8c8c8` background with `#000000` text (12.56:1)
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=36 class=stale
leaf-text: and no second OS window: a click, Enter, or a Space with no typed text standing opens it
source: dec-etsv2ebq «A drop-down single select's list opens on a click, Enter, or a Space with no typed text standing; a single select sized above 1 opens no list on any of them.»
replace-with: and no second OS window: a click, Enter, or a Space with no typed text standing opens a drop-down single select's list, and a single select sized above 1 opens no list on any of them
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=36 class=stale
leaf-text: `size` is read on a `multiple` select and stays unread on a single select
source: dec-ets3bn7h «A disabled, inert or hidden select, a select with no row to show, a multiple select and a single select sized above 1 open no list.»
replace-with: `size` is read on every select, a single select sized above 1 being a list box that, like a `multiple` select, opens no list
