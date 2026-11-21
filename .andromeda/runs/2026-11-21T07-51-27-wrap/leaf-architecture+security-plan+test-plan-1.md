@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=6 class=omission
leaf-text: the closed flag words `undeclared` and `unconfirmed` and booleans, never a value, typed text or `NodeId` (a ratified boundary widening), and since
source: dec-rrd735pk «The UI graph crosses socket call, command line and MCP and rests in a kept check as the graph verb's answer, whole or in parts»
replace-with: the closed flag words `undeclared` and `unconfirmed`, booleans and, read in parts, the part counts `part` and `parts`, never a value, typed text or `NodeId` (a ratified boundary widening), and since
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=16 class=stale
leaf-text: a bubbling dispatch counts every ancestor listener it passed, a `stopPropagation` unread
source: ctr-rparzix5 «A bubbling dispatch that still propagates counts every listener it passed; a dispatch a listener stops, or a handler's panic unwinds, counts its innermost listener alone.»
replace-with: a bubbling dispatch that still propagates counts every listener it passed, and one a listener stops, or a handler's panic unwinds, counts its innermost listener alone
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=18 class=stale
leaf-text: `graph` (since 2026-11-07-ui-graph) takes no argument — an argument refused `malformed` with nothing run — and answers `Outcome::Graphed`, the screen's graph read through the read-only `DioxusDocument::graph`
source: ctr-bsbxq5uz «Returns `Outcome::Graphed` holding the screen's whole `UiGraph` when no part is asked and the answer is within the snapshot budget; else `Outcome::GraphPart`, one part of whole nodes with its `part`»
replace-with: `graph` (since 2026-11-07-ui-graph) takes one optional `part`, a whole number from 1 to 1000 — a part of 0, of 1001 or of text refused `malformed` with nothing run — and answers `Outcome::Graphed`, the screen's whole graph read through the read-only `DioxusDocument::graph`, when no part is asked and the answer is within the snapshot budget, else `Outcome::GraphPart`, one part of whole nodes with its `part` and `parts`, the states in part 1 alone and a part past the last empty
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=18 class=stale
leaf-text: (shift when asked; it carries no Ctrl, so a set of rows of a `multiple` select that are not adjacent cannot be built through the driver, as its help states)
source: ctr-ublownab «Takes key and optional shift and ctrl, each false when absent; presses on what holds focus, ctrl held as the host's action key, Meta on macOS and Control elsewhere.»
replace-with: (shift and ctrl when asked, each false when absent; ctrl is held as the host's action key, Meta on macOS and Control elsewhere)
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=19 class=omission
leaf-text: `Graphed { graph: UiGraph }` (since 2026-11-07-ui-graph, dioxus-native-dom's `UiGraph`)
source: ctr-bsbxq5uz «else `Outcome::GraphPart`, one part of whole nodes with its `part` and `parts`, the states in part 1 alone and a part past the last empty.»
replace-with: `Graphed { graph: UiGraph }` (since 2026-11-07-ui-graph, dioxus-native-dom's `UiGraph`, the whole graph) · `GraphPart` (one part of whole nodes with its `part` and `parts`, the states in part 1 alone and a part past the last empty)
@leaf_edit n=6 leaf=.claude/docs/services/escher-driver.md line=20 class=omission
leaf-text: a `graph`'s outcome `{"nodes":[…],"states":[…]}` (since 2026-11-07-ui-graph)
source: ctr-amy33am2 «a whole answer is one line `{"nodes":[…],"states":[…]}`, a part one line `{"nodes":[…],"states":[…],"part":n,"parts":m}` with `states` in part 1 alone, status `0`.»
replace-with: a `graph`'s outcome `{"nodes":[…],"states":[…]}` (since 2026-11-07-ui-graph), a part's `{"nodes":[…],"states":[…],"part":n,"parts":m}` with `states` in part 1 alone
@leaf_edit n=7 leaf=.claude/docs/services/escher-driver.md line=20 class=stale
leaf-text: state and event names, closed flag words and booleans only, never a value
source: ctr-s26oh3sp «The answer `{"nodes":[…],"states":[…]}`, a part's adding `part` and `parts`, holds ids, component, state and event names, closed flag words, booleans and the part counts only, never a value.»
replace-with: state and event names, closed flag words, booleans and the part counts only, never a value
@leaf_edit n=8 leaf=.claude/docs/services/escher-driver.md line=24 class=stale
leaf-text: `graph` (the ninth, since 2026-11-07-ui-graph: no argument, an argument refused `malformed` with nothing run; result `nodes` (`FieldKind::GraphNodes`) and `states` (`FieldKind::GraphStates`), the screen's UI graph read whole)
source: ctr-ua4czbc6 «One optional part (1 to 1000); result nodes and states, plus part and parts when read in parts, the screen's UI graph whole when within the snapshot budget and no part is asked; dispatches nothing, runs no step.»
replace-with: `graph` (the ninth, since 2026-11-07-ui-graph: one optional `part` (1 to 1000), anything else refused `malformed` with nothing run; result `nodes` (`FieldKind::GraphNodes`) and `states` (`FieldKind::GraphStates`), plus `part` and `parts` when read in parts, the screen's UI graph whole when within the snapshot budget and no part is asked)
@leaf_edit n=9 leaf=.claude/docs/services/escher-driver.md line=24 class=stale
leaf-text: `press` (`key`, optional `shift`)
source: ctr-ublownab «Takes key and optional shift and ctrl, each false when absent; presses on what holds focus, ctrl held as the host's action key, Meta on macOS and Control elsewhere.»
replace-with: `press` (`key`, optional `shift` and `ctrl`)
@leaf_edit n=10 leaf=.claude/docs/services/escher-driver.md line=24 class=stale
leaf-text: `snapshot`'s, the seventh kind
source: ctr-tsfek65q «A whole number from 1 to 1000, the snapshot or graph part asked for.»
replace-with: `snapshot`'s and `graph`'s, the seventh kind
@leaf_edit n=11 leaf=.claude/docs/services/escher-driver.md line=25 class=stale
leaf-text: `Graph` (validated from `("graph", [])`, since 2026-11-07-ui-graph)
source: ctr-s3qxxjb3 «`graph` takes one optional `part`, a whole number from 1 to 1000; a part of 0, of 1001 or of text is refused `malformed` with nothing run.»
replace-with: `Graph` (validated from `("graph", [])` or with one `part` from 1 to 1000, since 2026-11-07-ui-graph)
@leaf_edit n=12 leaf=.claude/docs/services/escher-driver.md line=28 class=stale
leaf-text: `--shift` bare
source: ctr-suwkkat5 «--id, --text, --key, --ms and --part each take one value, --shift and --ctrl are bare, and --session <dir> exactly once; the call is validated before the session is asked for.»
replace-with: `--shift` and `--ctrl` bare
@leaf_edit n=13 leaf=.claude/docs/services/escher-driver.md line=28 class=omission
leaf-text: with `snapshot [--part <part>]`, `layout --id <id>` after `actions`
source: ctr-aj725x7q «Usage reads `snapshot [--part <part>]`, `graph [--part <part>]` after `lint`, press's `[--ctrl]`, `start <app> [--record <dir>]`»
replace-with: with `snapshot [--part <part>]`, `graph [--part <part>]` after `lint`, press's `[--ctrl]`, `layout --id <id>` after `actions`
@leaf_edit n=14 leaf=.claude/docs/services/escher-driver.md line=32 class=omission
leaf-text: and rests in a kept check as the `graph` verb's answer: stable ids
source: dec-rrd735pk «The UI graph crosses socket call, command line and MCP and rests in a kept check as the graph verb's answer, whole or in parts»
replace-with: and rests in a kept check as the `graph` verb's answer, whole or in parts: stable ids
@leaf_edit n=15 leaf=.claude/docs/services/escher-driver.md line=32 class=stale
leaf-text: the app authors, the closed flag words `undeclared` and `unconfirmed` and booleans
source: dec-rrd735pk «stable ids, owner ids, component, state and handler event names, declared writes, flags undeclared and unconfirmed, booleans, part counts; no value, typed text or NodeId.»
replace-with: the app authors, the closed flag words `undeclared` and `unconfirmed`, booleans and a part's `part` and `parts` counts
@leaf_edit n=16 leaf=.claude/docs/services/escher-driver.md line=62 class=stale
leaf-text: the whole Cells answer within the answer bound; picked up by the stand-repeat leg by its prefix)
source: dec-cmklkx6x «guard: stand_graph cells_reads_in_parts_each_within_the_budget_and_together_the_whole_graph»
replace-with: the Cells graph read in parts, each within the budget and together the whole graph; picked up by the stand-repeat leg by its prefix)
