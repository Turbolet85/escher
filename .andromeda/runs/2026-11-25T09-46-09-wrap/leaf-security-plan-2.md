@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes 
@leaf_edit n=1 leaf=.claude/rules/security.md line=21 class=retired
leaf-text: no `NodeId` crosses, and no snapshot, graph, canvas answer, accessibility tree or MCP result holds a canvas's declared virtual node (its `{owner}///{key}` id produced in process only) — keep it so.
source: dec-eg7d4fr7 «A canvas's virtual node id `{owner}///{key}` and its author-declared name cross the accessibility tree, the snapshot text, the diff, MCP results, kept checks and the socket and command-line answer»
replace-with: no `NodeId` crosses; a canvas's declared virtual node — its `{owner}///{key}` id and author-declared name — crosses the accessibility tree, the snapshot text, the diff, MCP results, kept checks and the socket and command-line answer line, while the graph, actions answer, lint, canvas answer and exploration hold none (a boundary widening, ratified) — keep it so.
