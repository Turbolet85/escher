@leaf_group group=architecture+security-plan+design-system+test-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=69
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=CLAUDE.md lines=169
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=122 class=stale
leaf-text: while the graph, actions answer, lint, canvas answer and exploration hold none,
source: dec-eg7d4fr7 «the graph, actions and layout answers carry the id, never the name; the lint and canvas answer hold none.»
replace-with: while the graph, actions and layout answers carry its id, never its name, exploration holds it, the lint and canvas answer hold none,
@leaf_edit n=2 leaf=CLAUDE.md line=122 class=stale
leaf-text: and `click` the one verb taking its id, acting on the canvas at its region,
source: ctr-7vwjtnb2 «click takes a virtual node id, as layout does: it acts on the canvas at the virtual region»
replace-with: and `click` and `layout` the verbs taking its id, `click` acting on the canvas at its region and `layout` answering it placed by app in its canvas,
@leaf_stands n=1 leaf=CLAUDE.md line=36
because: The line names the canvas read `Session::canvas` in process and as the `canvas` verb and says nothing of how it resolves its id, so the row's move from Session::layout's resolution to the run path's leaves it true.
@leaf_stands n=2 leaf=CLAUDE.md line=122
because: The line's canvas read states no id resolution against Session::layout, and its every other id verb reading a virtual id `not-found` still holds for the canvas verb, which the row now has read a virtual id not-found.
