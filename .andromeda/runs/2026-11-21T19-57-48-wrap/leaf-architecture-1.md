@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dir
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/stack
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/servi
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/servi
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/servi
@leaf_edit n=1 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: a single select's `size` is unread — a `multiple` select's hints a `height`
source: dec-yvsz7ugd «decision/select-size-hint · Select size as a height hint — A select's `size` is read in every document: a `multiple` select's, or a single select's above 1, hints its height in rows»
replace-with: a single select of display size 1 stays a drop-down — a `multiple` select's `size`, or a single select's above 1, hints a `height`
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: a focused `multiple` select takes its keys through `press_multiple_select_key`:
source: dec-7ef5o35w «A single select whose size parses above 1 is an in-flow list box in every document, display size 1 keeping the drop-down: it opens no list, arrows, Home, End and typeahead choose and scroll the row into view»
replace-with: a single select whose `size` parses above 1 is an in-flow list box (display size 1 keeping the drop-down): it opens no list, arrows, Home, End and typeahead choose and scroll the row into view, a row click chooses that row alone whatever modifier, and nothing need be chosen; a focused `multiple` select takes its keys through `press_multiple_select_key`:
@leaf_edit n=3 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: a focused single-choice select, its list closed,
source: dec-slku37au «decision/select-keyboard-model · Select keyboard model — A focused closed drop-down, a single select of display size 1, takes arrow keys and Home/End to enabled options»
replace-with: a focused drop-down, a single select of display size 1, its list closed,
@leaf_edit n=4 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: and on a single select opens its list or closes one standing
source: ctr-vjhbvxr6 «contract/select-list-default-actions/click-select · Select list default actions · click on select — Clicking a drop-down, a single select of display size 1, opens its list, or closes one standing»
replace-with: and on a drop-down, a single select of display size 1, opens its list or closes one standing
@leaf_edit n=5 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: an option of a closed single select, under a group too, is not clicked
source: dec-yyacsgv5 «decision/closed-select-options-covered · Closed select options refused as covered — An option of a closed drop-down, a single select of display size 1, under a group too, takes no click»
replace-with: an option of a closed drop-down, a single select of display size 1, under a group too, is not clicked
@leaf_edit n=6 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: a single select opens a list the engine paints over the page
source: dec-sl5sqsqp «decision/select-list-painted-overlay · Select list painted over the page — A drop-down, a single select of display size 1, opens a list the engine paints over the page»
replace-with: a drop-down, a single select of display size 1, opens a list the engine paints over the page
@leaf_edit n=7 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: a click on a `multiple` select's row focusing that select
source: dec-svdas4ye «decision/click-focuses-button · A click focuses a button — A click on a button of any type or a submit, reset or button input focuses it, then submits the form owner for a submit button;»
replace-with: a click on a list box's row focusing that select
@leaf_edit n=8 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: the `multiple` select, `optgroup`, and `change`
source: dec-z4rgfluk «tree focus falling back to window, native toggled, multiple select, list box, optgroup, checkbox and radio change are engine changes for all documents.»
replace-with: the `multiple` select, the list box, `optgroup`, and `change`
@leaf_edit n=9 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: Beside these, nine more ratified
source: dec-z4rgfluk «tree focus falling back to window, native toggled, multiple select, list box, optgroup, checkbox and radio change are engine changes for all documents.»
replace-with: Beside these, ten more ratified
@leaf_edit n=10 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: beside those nine
source: dec-z4rgfluk «tree focus falling back to window, native toggled, multiple select, list box, optgroup, checkbox and radio change are engine changes for all documents.»
replace-with: beside those ten
@leaf_edit n=11 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: holds the engine-default chosen-row pair `background-color: #c8c8c8; color: #000000` (12.56:1, hardcoded since the engine holds no tokens)
source: dec-sg4oulq7 «A list box's chosen row is painted with a #c8c8c8 background and #000000 text, a multiple select's from the default stylesheet and a single list box's as presentational hints.»
replace-with: holds the engine-default chosen-row pair `background-color: #c8c8c8; color: #000000` for a `multiple` select's rows (12.56:1, hardcoded since the engine holds no tokens), a single list box's chosen row taking the same pair as presentational hints
@leaf_edit n=12 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: a single select excepted, which reads none and writes `expanded` from its list
source: ctr-vxiqwugd «Read as a boolean on every element but a single select, which reads none; a drop-down writes expanded from whether its list is open, and a single list box writes none.»
replace-with: a single select excepted, which reads none, a drop-down writing `expanded` from its list and a single list box writing none
@leaf_edit n=13 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: `select_of(&self, node: NodeId) -> Option<NodeId>` (an option's or group's select)
source: ctr-ouyxjsed «contract/select-readers/is-list-box · BaseDocument select readers · is_list_box — Public and read-only: true for a select with multiple or a display size above 1; a stale or non-select id reads false.»
replace-with: `select_of(&self, node: NodeId) -> Option<NodeId>` (an option's or group's select) and `is_list_box` (true for a select with `multiple` or a display size above 1, false for a stale or non-select id)
@leaf_edit n=14 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: with no blanket `RestyleHint::restyle_subtree()`
source: dec-upk4tmyw «decision/targeted-restyle-on-mutation · Targeted restyle on mutation — Attribute changes, node removal and insertion restyle through targeted attribute-change and child-list-change restyles»
replace-with: with no blanket `RestyleHint::restyle_subtree()` but for a `size` or `multiple` write on a select, which restyles its descendants since both decide its rows' presentational hints
@leaf_edit n=15 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: as CSSOM View's `getBoundingClientRect` does (ratified by the founder, 2026-10-18)
source: ctr-z5p2evku «get_client_bounding_rect reads an element where its transforms move it, and a node a restyle turned display none keeps the client rect it was last laid out at.»
replace-with: as CSSOM View's `getBoundingClientRect` does (ratified by the founder, 2026-10-18), and `get_client_bounding_rect` keeping, for a node a restyle turned display none, the client rect it was last laid out at
@leaf_edit n=16 leaf=.claude/docs/services/blitz-dom.md line=19 class=stale
leaf-text: on a `multiple` select or a single select's open list, a click on a row
source: dec-7ef5o35w «A single select whose size parses above 1 is an in-flow list box in every document, display size 1 keeping the drop-down: it opens no list»
replace-with: on a list box or a drop-down's open list, a click on a row
