@leaf_group group=a11y-plan-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=45
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=15 class=stale
leaf-text: for a `multiple` select's rows, `#000000`
source: dec-2r7uwa2k «A list box's chosen rows, multiple or single, read #000000 on #c8c8c8, Chromium's inactive selected pair, hardcoded in the default sheet.»
replace-with: for a list box's chosen rows, `multiple` or single, `#000000`
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=19 class=stale
leaf-text: a single select's `ComboBox` writes `expanded`
source: ctr-32zx75uf «A drop-down single select reads ComboBox and writes expanded from its list's open state; a single list box reads ListBox, no expanded, active descendant or multiselectable; an author's open or aria-expan»
replace-with: a drop-down single select's (display size 1) `ComboBox` writes `expanded`
@leaf_edit n=3 leaf=.claude/docs/a11y-summary.md line=19 class=omission
leaf-text: a `multiple` select's `ListBox` writes `set_multiselectable`
source: ctr-2upfv5xp «A select reads ListBox where it is a list box, multiple or of display size above 1, else ComboBox; its options, under an optgroup or display none too, stay as ListBoxOption nodes, each selected true or f»
replace-with: a `multiple` select's `ListBox` writes `set_multiselectable`, and a single list box (a single select of display size above 1) reads `ListBox` with no `expanded`, active descendant or multiselectable, an author's `open` or `aria-expanded` steering it neither
@leaf_edit n=4 leaf=.claude/docs/a11y-summary.md line=20 class=stale
leaf-text: every option of a single-choice select but its chosen one
source: dec-3r272kl2 «why: Every option of a drop-down single select but its chosen one computes display none.»
replace-with: every option of a drop-down single select but its chosen one
@leaf_edit n=5 leaf=.claude/docs/a11y-summary.md line=21 class=stale
leaf-text: on every element but a single select, which reads none and writes `expanded` from its list's open state
source: ctr-4baai6me «aria-expanded is read as true or false and nothing else on every element but a single select, which reads no author attribute: a drop-down writes its list's open state instead, a single list box writes no»
replace-with: on every element but a single select, which reads none: a drop-down writes `expanded` from its list's open state, a single list box writes no `expanded`
@leaf_edit n=6 leaf=.claude/docs/a11y-summary.md line=23 class=stale
leaf-text: Select: a focused single-choice `select` (no `multiple`)
source: ctr-3sl57pcy «Enter, and a Space with no typed text standing, open a focused drop-down's list on the chosen option and fire nothing; a disabled, inert, hidden or empty select, or a single list box, opens nothing.»
replace-with: Select: a focused drop-down `select` (no `multiple`, display size 1)
@leaf_edit n=7 leaf=.claude/docs/a11y-summary.md line=23 class=stale
leaf-text: a disabled, inert, hidden or empty select opening nothing
source: ctr-3sl57pcy «Enter, and a Space with no typed text standing, open a focused drop-down's list on the chosen option and fire nothing; a disabled, inert, hidden or empty select, or a single list box, opens nothing.»
replace-with: a disabled, inert, hidden or empty select, or a single list box, opening nothing
@leaf_edit n=8 leaf=.claude/docs/a11y-summary.md line=23 class=omission
leaf-text: an `optgroup`'s options are the select's own in tree order
source: ctr-puvj6ral «On a focused single list box that is no disabled form control, the arrows, Home and End step and choose, a printable key with no Ctrl, Alt or Meta is typeahead, the reached row is scrolled into view; Enter»
replace-with: a focused single list box (a single select of display size above 1) that is no disabled form control takes the arrows, Home and End to step and choose and a printable key with no Ctrl, Alt or Meta as typeahead, the reached row scrolled into view, while Enter, a Space with no typed text and every other key do nothing and no list opens; an `optgroup`'s options are the select's own in tree order
@leaf_edit n=9 leaf=.claude/docs/a11y-summary.md line=23 class=stale
leaf-text: and on a single select also opens its list or closes one standing
source: ctr-ecq7ujy7 «A click on a single list box's row focuses the select, clears its typeahead and chooses that row alone whatever modifier is held; a disabled select or row changes nothing.»
replace-with: and on a drop-down single select also opens its list or closes one standing, a single list box opening none and a click on its row choosing that row alone whatever modifier is held, a disabled select or row changing nothing
@leaf_edit n=10 leaf=.claude/docs/a11y-summary.md line=24 class=stale
leaf-text: a closed select's options under it with its select's client rect
source: ctr-2uai45dy «A closed drop-down's options read under it, each with the select's client rect as its bounds; a list box, multiple or of display size above 1, is no closed select and its rows read their own bounds.»
replace-with: a closed drop-down's options under it with its select's client rect
@leaf_edit n=11 leaf=.claude/docs/a11y-summary.md line=24 class=stale
leaf-text: while a `multiple` select's rows and groups read their own client rects
source: ctr-2uai45dy «A closed drop-down's options read under it, each with the select's client rect as its bounds; a list box, multiple or of display size above 1, is no closed select and its rows read their own bounds.»
replace-with: while a list box's rows and groups, `multiple` or of display size above 1, read their own client rects
