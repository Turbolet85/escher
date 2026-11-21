@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=62
last: - Architecture: `doc:architecture` (the records store) · dir
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=45
last: - `doc:architecture` (the records store) · `.claude/docs/stac
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=36
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=6 class=omission
leaf-text: integration tests one file per behaviour in `tests/blitz-tests/tests/`, each its own test target
source: dec-u3zljheg «blitz-vibey-script keeps its own integration tests in its tests directory»
replace-with: integration tests one file per behaviour in `tests/blitz-tests/tests/` (blitz-vibey-script keeping its own integration tests in its own `tests/` directory), each its own test target
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: a node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives it up at `DocumentMutator::flush` through `BaseDocument::clear_focus`, firing no DOM `blur` or `focusout`, the fixup reading neither `disabled` nor `hidden`.
source: dec-sg6war4y «At flush, focus on a node that has become inert, a subtree moved under an inert box included, and after each layout, focus on a node that no longer shows, is cleared with no blur or focusout fired»
replace-with: a node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives it up at `DocumentMutator::flush`, and after each layout a focused node that no longer shows gives it up, both through `BaseDocument::clear_focus`, firing no DOM `blur` or `focusout`; a `display: contents` control and a control beside a hidden sibling keep focus.
@leaf_edit n=3 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: Beside these, eight more ratified direct engine changes for every document — `visible_region` through transforms, a non-invertible transform's no hit, the focus fixup under `inert`, the tree's focus falling back to the window
source: dec-z4rgfluk «focus fixups under inert and after layout, Tab skipping what does not show, tree focus falling back to the window, native toggled, multiple select, optgroup and checkbox and radio change»
replace-with: Beside these, nine more ratified direct engine changes for every document — `visible_region` through transforms, a non-invertible transform's no hit, the focus fixups under `inert` and after layout, Tab skipping what does not show, the tree's focus falling back to the window
@leaf_edit n=4 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: A dropped anonymous block clears its children's `layout_parent` beside those eight
source: dec-z4rgfluk «focus fixups under inert and after layout, Tab skipping what does not show, tree focus falling back to the window, native toggled, multiple select, optgroup and checkbox and radio change»
replace-with: A dropped anonymous block clears its children's `layout_parent` beside those nine
@leaf_edit n=5 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: focus stays and the key still acts, `DocumentMutator::flush` reading `is_inert()` alone.
source: dec-sllwljwg «so a key press on a control whose anonymous block was dropped does not panic, and once re-focused the key acts.»
replace-with: once re-focused the key acts.
@leaf_edit n=6 leaf=.claude/docs/services/blitz-dom.md line=19 class=stale
leaf-text: and, with no select list standing, NOT by node creation
source: dec-ut2clwr7 «a page scroll or a move that closes an open select list marks its select at the next resolve; a focus given up after a layout marks its node»
replace-with: and by a focus given up after a layout, which marks its node — and, with no select list standing and no focus given up, NOT by node creation
