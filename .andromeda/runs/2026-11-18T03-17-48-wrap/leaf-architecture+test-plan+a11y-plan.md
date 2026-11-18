@leaf_group group=architecture+test-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=CLAUDE.md lines=166
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=27 class=stale
leaf-text: resets focus to body
source: dec-2f62jiwg «Removing the focused node drops focus to the root element, the tree's focus naming the window, and runs blur side effects.»
replace-with: drops focus to the root element, the tree's focus naming the window
