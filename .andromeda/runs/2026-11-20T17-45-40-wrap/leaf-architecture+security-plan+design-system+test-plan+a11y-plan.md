@leaf_group group=architecture+security-plan+design-system+test-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=CLAUDE.md lines=166
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=31 class=stale
leaf-text: and a highlight or a wheel move inside the list marks nothing.
source: dec-vvnxavu4 «a wheel elsewhere marks it on close, and a highlight move to another row, by key or pointer, marks it; a move to the same row and a wheel move inside the list mark nothing.»
replace-with: and a highlight move to another row, by key or pointer, marks its select, while a move to the same row or a wheel move inside the list marks nothing.
@leaf_edit n=2 leaf=CLAUDE.md line=122 class=retired
leaf-text: a highlighted row's contrast unread
source: dec-2rwoal42 «The visual lint's contrast reads an open list row's plain pair even when the row is highlighted, so a highlighted row's contrast is unread by the lint, a stated limit.»
replace-with: a highlighted row's contrast read at its painted pair, the engine's highlight colours
@leaf_edit n=3 leaf=CLAUDE.md line=122 class=omission
leaf-text: a row's own `bounds` crossing in the snapshot and the diff
source: dec-rt2rzqbq «The select list crosses the carriers only as a row's bounds and closed bool highlighted in the snapshot and diff, differs naming either»
replace-with: a row's own `bounds` and the closed bool `highlighted` crossing in the snapshot and the diff, `differs` naming either
@leaf_edit n=4 leaf=CLAUDE.md line=122 class=stale
leaf-text: node key, log field or span
source: dec-rt2rzqbq «no new carrier, exit, cause, session error, log field or span.»
replace-with: log field or span
@leaf_edit n=5 leaf=CLAUDE.md line=122 class=omission
leaf-text: and `Harness::text_contrast`
source: dec-4cdysvgx «The driver's visual lint reads contrast through the harness's readers, the highlight reader first and the text contrast where it answers nothing, with the stand's exempt rule»
replace-with: and the harness's `Harness::highlight_contrast` first, `Harness::text_contrast` where it answers nothing
