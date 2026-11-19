@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dir
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/sta
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_edit n=1 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: a nested box at once, the requested behaviour for the viewport alone
source: ctr-zara73ad «scroll_into_view scrolls every scrolling box on the target's chain, skipping a box whose scroll min and max are both zero, nested ones instantly with no scroll event, then the viewport»
replace-with: a nested box at once with no scroll event, the requested behaviour for the viewport alone
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: sanitized to `[min, max]` and the nearest grid value
source: dec-t3244gzx «every value sanitized to [min, max] and the nearest step, the greater on a tie.»
replace-with: sanitized to `[min, max]` and the nearest grid value, the greater on a tie
@leaf_edit n=3 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: sets the value from the hit's fraction of the content box
source: dec-szcsd6ey «A main-button pointerdown on a range input sets its value from the hit's fraction of the content box, clamped to [0, 1]; a click focuses it and changes no value.»
replace-with: sets the value from the hit's fraction of the content box, clamped to `[0, 1]`,
@leaf_edit n=4 leaf=.claude/docs/services/blitz-paint.md line=18 class=omission
leaf-text: and a double click selects the whole value
source: dec-smz43iv7 «A click in a password input places the caret by its painted bullets, one per character, a shift-click extending the selection and two or more clicks selecting all»
replace-with: a shift-click extends the selection and two or more clicks select the whole value
@leaf_edit n=5 leaf=.claude/docs/conventions.md line=39 class=omission
leaf-text: which flushes deferred work on `Drop`
source: dec-xmlwfni7 «DOM mutations go through `DocumentMutator`, which flushes deferred work on drop and requests a redraw only for in-document mutations.»
replace-with: which flushes deferred work on `Drop` and requests a redraw only for in-document mutations
