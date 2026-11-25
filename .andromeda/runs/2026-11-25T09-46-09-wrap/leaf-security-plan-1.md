@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=32 class=omission
leaf-text: | Snapshot content | stable ids, accessible names,
source: dec-eg7d4fr7 «decision/virtual-node-crossing · Virtual node crossing — A canvas's virtual node id `{owner}///{key}` and its author-declared name cross the accessibility tree, the snapshot text, the diff, MCP results»
replace-with: | Snapshot content | stable ids, accessible names (since 2026-11-25-virtual-nodes-as-elements a canvas's virtual node id `{owner}///{key}` and its author-declared name included, crossing the accessibility tree, the snapshot text, the diff, MCP results, kept checks and the socket and command-line answer line, while the graph, actions answer, lint, canvas answer and exploration hold none — a boundary widening, ratified),
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=23 class=omission
leaf-text: a `click` or a `type` on a target that cannot take it
source: dec-irbkdmgs «decision/target-refusal-order · Fixed order of target refusals — A click or type on an element, or a click on a virtual node, that cannot take it is refused with nothing run, the first of disabled, off-screen, covered that holds»
replace-with: a `click` or a `type` on an element, or a `click` on a virtual node (which reads `covered` only by an element over its region), that cannot take it
