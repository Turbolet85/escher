@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=83
last: - Fonts missing in headless tests → `system-fonts` feature / 
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=38
last: - `.claude/rules/verification-harness.md` · `.claude/docs/test
@leaf_edit n=1 leaf=.claude/docs/commands.md line=40 class=stale
leaf-text: given only the escher server's fourteen tools
source: dec-vnrtz2ue «decision/cold-agent-verbs-without-layout · Cold-agent pipe's verb list without layout — The cold-agent pipe's verb list holds the fourteen driver verbs other than `layout`.»
replace-with: given only fourteen of the escher server's tools, every driver verb but `layout` (provisional)
@leaf_edit n=2 leaf=.claude/docs/commands.md line=58 class=omission
leaf-text: the whole screen unpaged, status `0`) · `status` each run
source: ctr-xifju45c «contract/driver-verbs/layout · driver verbs · layout — Instance level, after actions; takes one required id; result id, bounds, laid_out, placed, shifts, width and height, each always present; reads where the»
replace-with: the whole screen unpaged, status `0`) · `layout --id <id>` (the eleventh instance verb, after `actions`; its accepted answer one line of `id`, `bounds`, `laid_out`, `placed`, `shifts`, `width` and `height`, ids, closed words and numbers only, reading where the element sits and why at its size, changing nothing and running no step) · `status` each run
@leaf_edit n=3 leaf=.claude/docs/commands.md line=59 class=stale
leaf-text: its tools the fourteen driver verbs
source: ctr-stdygm64 «contract/mcp-server/tools-list · MCP server · tools/list — The fifteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.»
replace-with: its tools the fifteen driver verbs
@leaf_edit n=4 leaf=.claude/docs/commands.md line=59 class=omission
leaf-text: then `actions`, answering its line of the actionable nodes, an argument to it answered `isError`)
source: ctr-s4mfww2m «The verb table holds fifteen verbs, `graph` listed after `lint`, `actions` after `graph` and `layout` after `actions`.»
replace-with: then `actions`, answering its line of the actionable nodes, an argument to it answered `isError`, then `layout`, taking one required `id` and answering its line of the element's bounds, placing and sizes)
