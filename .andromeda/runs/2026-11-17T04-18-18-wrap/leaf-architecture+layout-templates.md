@leaf_group group=architecture+layout-templates
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cla
@leaf_edit n=1 leaf=.claude/docs/services/seven_guis.md line=11 class=stale
leaf-text: the `mcp` role lists it as its fourteenth tool, last, `graph` after `lint` and `actions` after `graph`
source: ctr-gcgwos72 «contract/escher-session-mcp/tools · escher-session MCP server · tools/list — Lists the fifteen verbs of the verb table as tools in order, layout after actions and help last»
replace-with: the `mcp` role lists it as its fifteenth tool, last, `graph` after `lint`, `actions` after `graph` and `layout` after `actions`
@leaf_edit n=2 leaf=.claude/docs/services/seven_guis.md line=11 class=omission
leaf-text: over the `mcp` role an accepted `actions` answers its JSON line `{"nodes"}` as one text block.
source: ctr-nikwwkdh «contract/escher-session-layout/answer · escher-session layout · answer — One required --id; one uncoloured line of id, bounds and laid_out (x, y, width, height), placed (by, container, canvas, at, relative)»
replace-with: over the `mcp` role an accepted `actions` answers its JSON line `{"nodes"}` as one text block. The eleventh instance verb, `layout --id <id> --session <dir>`, after `actions`, takes one required `--id` and answers one uncoloured line of `id`, `bounds`, `laid_out`, `placed`, `shifts`, `width` and `height` on stdout, exit 0 — where the element sits and why at its size, ids, closed words and numbers only, changing nothing — kept under `--record` like every instance verb.
