@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=CLAUDE.md lines=169
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=36 class=omission
leaf-text: and the layout read (`Session::layout`, in process and as the `layout` verb)
source: ctr-elrfiwmi «A read answering a CanvasReading of one canvas in process, resolving its id as Session::layout does, refusing fonts-unavailable where the document cannot paint text, changing nothing and opening no span of its own.»
replace-with: the layout read (`Session::layout`, in process and as the `layout` verb) and the canvas read (`Session::canvas`, in process and as the `canvas` verb)
@leaf_edit n=2 leaf=CLAUDE.md line=112 class=omission
leaf-text: `layout --id` · `status` · `stop`
source: ctr-bdnv7itl «The verb canvas takes --id and the optional bare --full and --part.»
replace-with: `layout --id` · `canvas --id [--full] [--part]` · `status` · `stop`
@leaf_edit n=3 leaf=CLAUDE.md line=113 class=stale
leaf-text: its tools the fifteen verbs, `graph` after `lint`, `actions` after `graph`, `layout` after `actions`, `help` last
source: ctr-s4mfww2m «The verb table holds sixteen verbs, `graph` listed after `lint`, `actions` after `graph`, `layout` after `actions` and `canvas` after `layout`.»
replace-with: its tools the sixteen verbs, `graph` after `lint`, `actions` after `graph`, `layout` after `actions`, `canvas` after `layout`, `help` last
@leaf_edit n=4 leaf=CLAUDE.md line=122 class=stale
leaf-text: one verb table of fifteen verbs in three levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` · `screenshot` · `lint` · `graph` · `actions` · `layout` asked of the held instance
source: ctr-yimmfwol «The command and refusal schema is stated once as 'static data: sixteen verbs, eight closed causes.»
replace-with: one verb table of sixteen verbs in three levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` · `screenshot` · `lint` · `graph` · `actions` · `layout` · `canvas` asked of the held instance
@leaf_edit n=5 leaf=CLAUDE.md line=122 class=stale
leaf-text: `snapshot`, `screenshot`, `lint`, `graph`, `actions` and `layout` waiting on none
source: ctr-s46mb5gp «The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph`, `actions` and `canvas` wait on nothing.»
replace-with: `snapshot`, `screenshot`, `lint`, `graph`, `actions`, `layout` and `canvas` waiting on none
@leaf_edit n=6 leaf=CLAUDE.md line=122 class=stale
leaf-text: `escher_driver::serve_mcp`, the driver's fifteen verbs as MCP tools
source: ctr-stdygm64 «The sixteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.»
replace-with: `escher_driver::serve_mcp`, the driver's sixteen verbs as MCP tools
@leaf_edit n=7 leaf=CLAUDE.md line=122 class=stale
leaf-text: `help`, the fifteenth and last verb
source: ctr-stdygm64 «The sixteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.»
replace-with: `help`, the sixteenth and last verb
@leaf_edit n=8 leaf=CLAUDE.md line=122 class=retired
leaf-text: its content crossing the driver's carriers only as pixels in a screenshot, on every ratified carrier, with no new carrier, verb, field, exit, refusal cause, session error, log field or span, a ratified boundary widening
source: dec-uojpjchi «A canvas's app-painted content crosses the driver's carriers, the session socket, the command line, MCP and a kept check, as the canvas verb's answer of draw commands beside a screenshot's pixels»
replace-with: its content crossing the driver's carriers, the session socket, the command line, MCP and a kept check, as the `canvas` verb's answer of draw commands beside a screenshot's pixels, with no new carrier, exit, cause, session error or span field, a ratified boundary widening
@leaf_edit n=9 leaf=CLAUDE.md line=122 class=omission
leaf-text: and an actions answer's ids, owner ids, verb words, closed cause words and booleans at rest
source: dec-37vb5ccc «A kept check holds a canvas step and its whole answer of closed words, numbers and 64-bit digests, never a glyph id, text or NodeId, and its replay stops naming digest when a canvas digest differs.»
replace-with: and an actions answer's ids, owner ids, verb words, closed cause words and booleans, and a canvas answer's closed words, numbers and 64-bit digests at rest
