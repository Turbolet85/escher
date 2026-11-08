@leaf_group group=masters-1
@leaf_read leaf=CLAUDE.md lines=153
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=33 class=omission
leaf-text: and lists the actionable elements that read no author key (`DioxusDocument::unkeyed_actionable`)
source: fct-d5zuqqfb «beside it the read-only `DioxusDocument::actionable_nodes` lists, in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an `ActionableNode`»
replace-with: and lists the actionable elements that read no author key (`DioxusDocument::unkeyed_actionable`) and, read-only, every UI graph node the actionable rule holds for or the graph gives an owner, in snapshot pre-order, as an `ActionableNode` (id, owner, `keyed` where the id holds no `/`) (`DioxusDocument::actionable_nodes`), both reading the one actionable rule — focusable, an interactive role, or current listeners holding an event other than `mounted`
@leaf_edit n=2 leaf=CLAUDE.md line=37 class=stale
leaf-text: `snapshot`, `screenshot`, `lint` and `graph` on nothing
source: fct-g75i5t3p «and `snapshot`, `screenshot`, `lint`, `graph` and `actions`, which run no step, wait on nothing.»
replace-with: `snapshot`, `screenshot`, `lint`, `graph` and `actions` on nothing
@leaf_edit n=3 leaf=CLAUDE.md line=37 class=stale
leaf-text: no field added, `VERBS` holding thirteen)
source: fct-6m5fkd33 «and `VERBS` holds fourteen, `graph` listed after `lint` and `actions` after `graph`»
replace-with: no field added, `VERBS` holding fourteen)
@leaf_edit n=4 leaf=CLAUDE.md line=37 class=omission
leaf-text: booleans, never a value — inside its answer line (a ratified boundary widening) — an element's selected
source: fct-nb3g6bqn «a UI graph's names, closed flag words and booleans, and an actions answer's owner ids, verb words, closed cause words and booleans cross, never a `NodeId`»
replace-with: booleans, never a value — inside its answer line (a ratified boundary widening) and an `actions` answer — ids, owner ids, `keyed` booleans, the table's verb words and closed cause words, never a value, typed text or `NodeId` — inside its answer line (a ratified boundary widening) — an element's selected
@leaf_edit n=5 leaf=CLAUDE.md line=37 class=stale
leaf-text: `VERBS`, thirteen verbs in three levels (`snapshot` · `click` · `type` · `press` · `advance` · `scroll` · `screenshot` · `lint` · `graph` asked of the held instance
source: fct-c5e4c7ao «the verbs are the fourteen of `VERBS` — `snapshot`, `click --id`, `type --id --text`, `press --key [--shift]`, `advance --ms`, `scroll --id`, `screenshot [--id <id>]`, `lint`, `graph`, `actions`»
replace-with: `VERBS`, fourteen verbs in three levels (`snapshot` · `click` · `type` · `press` · `advance` · `scroll` · `screenshot` · `lint` · `graph` · `actions` asked of the held instance
@leaf_edit n=6 leaf=CLAUDE.md line=37 class=omission
leaf-text: unpaged, its node ids feeding the record of ids;
source: fct-rqe4xefm «each with the verb table's id-taking instance verbs and the cause each would be refused with, `owner` and `cause` left out when absent»
replace-with: unpaged, its node ids feeding the record of ids, and `actions` — no argument (one refused `malformed` with nothing run), no step, changing nothing — `Outcome::Actions`, written `{"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}`, unpaged, each actionable node in snapshot pre-order with each id-taking instance verb and the cause `check_target` would refuse it with (a node resolving to no element `stale`), `owner` and `cause` left out when absent, every id it lists feeding the record of ids;
@leaf_edit n=7 leaf=CLAUDE.md line=37 class=stale
leaf-text: `tools/list` the thirteen verbs of `VERBS` (`lint` after `screenshot`, `graph` after `lint`)
source: fct-t7zq5feb «answers the fourteen verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`, then `graph`, then `actions`), `help` last»
replace-with: `tools/list` the fourteen verbs of `VERBS` (`lint` after `screenshot`, `graph` after `lint`, `actions` after `graph`)
@leaf_edit n=8 leaf=CLAUDE.md line=38 class=stale
leaf-text: the nine instance verbs
source: fct-exqct2ck «over the fourteen verbs of the driver's verb table, in three levels — the ten asked of the held instance»
replace-with: the ten instance verbs
@leaf_edit n=9 leaf=CLAUDE.md line=44 class=omission
leaf-text: and a UI graph's ids, component, state and handler event names, closed flag words and booleans — each widening ratified
source: fct-z5ttw3ol «a UI graph's ids, component, state and handler event names, closed flag words and booleans, an actions answer's ids, owner ids, the table's verb words, closed cause words and `keyed` booleans»
replace-with: and a UI graph's ids, component, state and handler event names, closed flag words and booleans and an actions answer's ids, owner ids, the table's verb words, closed cause words and `keyed` booleans — each widening ratified
@leaf_edit n=10 leaf=CLAUDE.md line=44 class=omission
leaf-text: a lint's findings and a `graph` answer's UI graph, at rest
source: fct-g4yyfogl «an `actions` answer's ids, owner ids, the table's verb words, closed cause words and booleans at rest»
replace-with: a lint's findings, a `graph` answer's UI graph and an `actions` answer's ids, owner ids, verb words, closed cause words and booleans, at rest
@leaf_edit n=11 leaf=CLAUDE.md line=83 class=omission
leaf-text: `packages/escher-driver/src/execute.rs` · the visual lint
source: fct-z3b2jzgi «The actions read: `Session::actions(&mut self) -> Vec<ActionNode>` reads one snapshot and one `element_ids` map per call»
replace-with: `packages/escher-driver/src/execute.rs` (the shared per-verb target check `check_target`) · the actions read `packages/escher-driver/src/actions.rs` (`Session::actions`, as the `actions` verb; check `tests/blitz-tests/tests/stand_actions.rs`) · the visual lint
@leaf_edit n=12 leaf=CLAUDE.md line=102 class=omission
leaf-text: `screenshot [--id]` · `lint` · `graph` · `status` · `stop`
source: fct-c5e4c7ao «`screenshot [--id <id>]`, `lint`, `graph`, `actions`, `start <task> [--record <dir>]`, `status`, `stop`»
replace-with: `screenshot [--id]` · `lint` · `graph` · `actions` · `status` · `stop`
@leaf_edit n=13 leaf=CLAUDE.md line=103 class=stale
leaf-text: its tools the thirteen verbs, `graph` after `lint`, `help` last
source: fct-t7zq5feb «answers the fourteen verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`, then `graph`, then `actions`), `help` last»
replace-with: its tools the fourteen verbs, `graph` after `lint`, `actions` after `graph`, `help` last
@leaf_edit n=14 leaf=CLAUDE.md line=112 class=stale
leaf-text: one verb table of thirteen verbs in three levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` · `screenshot` · `lint` · `graph` asked of the held instance
source: fct-6m5fkd33 «and `VERBS` holds fourteen, `graph` listed after `lint` and `actions` after `graph`»
replace-with: one verb table of fourteen verbs in three levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` · `screenshot` · `lint` · `graph` · `actions` asked of the held instance
@leaf_edit n=15 leaf=CLAUDE.md line=112 class=stale
leaf-text: `snapshot`, `screenshot`, `lint` and `graph` waiting on none
source: fct-g75i5t3p «and `snapshot`, `screenshot`, `lint`, `graph` and `actions`, which run no step, wait on nothing.»
replace-with: `snapshot`, `screenshot`, `lint`, `graph` and `actions` waiting on none
@leaf_edit n=16 leaf=CLAUDE.md line=112 class=stale
leaf-text: the driver's thirteen verbs as MCP tools
source: fct-svo6bgmr «the `escher-session mcp` role also serves the driver's verbs as MCP tools — fourteen»
replace-with: the driver's fourteen verbs as MCP tools
@leaf_edit n=17 leaf=CLAUDE.md line=112 class=omission
leaf-text: a lint's findings and a UI graph's names, closed flag words and booleans at rest
source: fct-7p2g4lnp «a lint's findings, a UI graph's names, closed flag words and booleans, and an actions answer's ids, owner ids, verb words, closed cause words and booleans at rest»
replace-with: a lint's findings, a UI graph's names, closed flag words and booleans, and an actions answer's ids, owner ids, verb words, closed cause words and booleans at rest
