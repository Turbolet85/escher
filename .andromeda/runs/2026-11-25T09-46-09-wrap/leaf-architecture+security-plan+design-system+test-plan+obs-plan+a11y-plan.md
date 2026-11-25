@leaf_group group=architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=CLAUDE.md lines=169
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=122 class=stale
leaf-text: no carrier, snapshot, graph, canvas answer or accessibility tree holding a declared node)
source: dec-eg7d4fr7 «A canvas's virtual node id `{owner}///{key}` and its author-declared name cross the accessibility tree, the snapshot text, the diff, MCP results, kept checks and the socket and command-line answer line»
replace-with: each taken node a child of its canvas's accessibility node and a snapshot node, its id `{owner}///{key}` and declared name crossing the accessibility tree, the snapshot text, the diff, MCP results, kept checks and the answer line, while the graph, actions answer, lint, canvas answer and exploration hold none, and `click` the one verb taking its id, acting on the canvas at its region, every other id verb reading it `not-found`)
