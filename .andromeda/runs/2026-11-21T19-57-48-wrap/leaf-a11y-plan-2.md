@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=24 class=stale
leaf-text: which every option of a single-choice select but its chosen one computes and a `multiple` select's rows do not
source: dec-3r272kl2 «decision/hidden-content-excluded · Hidden content kept out of the tree — An aria-hidden true node is kept but marked hidden, while hidden, inert, display none, visibility hidden and visibility collapse »
replace-with: which every option of a drop-down single select (display size 1) but its chosen one computes and a list box's rows, `multiple` or of display size above 1, do not
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=25 class=stale
leaf-text: (a boolean, on every element but a single select, which reads none and writes `expanded` from its list's open state)
source: ctr-4baai6me «aria-expanded is read as true or false and nothing else on every element but a single select, which reads no author attribute: a drop-down writes its list's open state instead, a single list box writes no expanded.»
replace-with: (a boolean, on every element but a single select, which reads none: a drop-down writes `expanded` from its list's open state, a single list box writes no `expanded`)
@leaf_edit n=3 leaf=.claude/rules/a11y.md line=26 class=stale
leaf-text: A `select` (role `ComboBox`, `ListBox` for `multiple`)
source: ctr-2upfv5xp «A select reads ListBox where it is a list box, multiple or of display size above 1, else ComboBox; its options, under an optgroup or display none too, stay as ListBoxOption nodes»
replace-with: A `select` (role `ListBox` where it is a list box, `multiple` or of display size above 1, else `ComboBox`)
@leaf_edit n=4 leaf=.claude/rules/a11y.md line=26 class=stale
leaf-text: a closed select's options under it, each with its select's client rect as its `bounds`
source: ctr-2uai45dy «A closed drop-down's options read under it, each with the select's client rect as its bounds; a list box, multiple or of display size above 1, is no closed select and its rows read their own bounds.»
replace-with: a closed drop-down's options under it, each with its select's client rect as its `bounds` (a list box, `multiple` or of display size above 1, is no closed select and its rows read their own bounds)
@leaf_edit n=5 leaf=.claude/rules/a11y.md line=26 class=stale
leaf-text: A single select's tree node takes `expanded` from a closed bool of engine state, reading no `aria-expanded`,
source: ctr-32zx75uf «A drop-down single select reads ComboBox and writes expanded from its list's open state; a single list box reads ListBox, no expanded, active descendant or multiselectable; an author's open or aria-expanded»
replace-with: A drop-down single select's tree node takes `expanded` from a closed bool of engine state, reading no `aria-expanded` (a single list box's node reading `ListBox`, with no `expanded`, active descendant or multiselectable),
@leaf_edit n=6 leaf=.claude/rules/a11y.md line=33 class=stale
leaf-text: and on a single select also opens its list or closes one standing
source: dec-2kmkpxdz «A closed drop-down single select (display size 1) opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root; a single list box opens none.»
replace-with: and on a drop-down single select (display size 1) also opens its list or closes one standing, a single list box opening none
@leaf_edit n=7 leaf=.claude/rules/a11y.md line=33 class=omission
leaf-text: Shift spanning and a disabled row changing nothing)
source: ctr-ecq7ujy7 «A click on a single list box's row focuses the select, clears its typeahead and chooses that row alone whatever modifier is held; a disabled select or row changes nothing.»
replace-with: Shift spanning and a disabled row changing nothing; a click on a single list box's row focusing the select, clearing its typeahead and choosing that row alone whatever modifier is held, a disabled select or row changing nothing)
@leaf_edit n=8 leaf=.claude/rules/a11y.md line=33 class=stale
leaf-text: a disabled, inert, hidden or empty select opening nothing
source: ctr-3sl57pcy «Enter, and a Space with no typed text standing, open a focused drop-down's list on the chosen option and fire nothing; a disabled, inert, hidden or empty select, or a single list box, opens nothing.»
replace-with: a disabled, inert, hidden or empty select, or a single list box, opening nothing
@leaf_edit n=9 leaf=.claude/rules/a11y.md line=33 class=omission
leaf-text: A focused `multiple` select that is no disabled form control takes ArrowDown, ArrowUp, Home and End:
source: ctr-puvj6ral «On a focused single list box that is no disabled form control, the arrows, Home and End step and choose, a printable key with no Ctrl, Alt or Meta is typeahead, the reached row is scrolled into view; Ent»
replace-with: A focused single list box that is no disabled form control takes the arrows, Home and End, stepping and choosing, and a printable key with no Ctrl, Alt or Meta as typeahead, the reached row scrolled into view, while Enter, a Space with no typed text and every other key do nothing and no list opens. A focused `multiple` select that is no disabled form control takes ArrowDown, ArrowUp, Home and End:
@leaf_edit n=10 leaf=.claude/rules/a11y.md line=38 class=stale
leaf-text: `select[multiple] > option:checked` and `select[multiple] > optgroup > option:checked` read `#000000` on `#c8c8c8`
source: dec-2r7uwa2k «A list box's chosen rows, multiple or single, read #000000 on #c8c8c8, Chromium's inactive selected pair, hardcoded in the default sheet.»
replace-with: a list box's chosen rows, `multiple` or single, read `#000000` on `#c8c8c8`
