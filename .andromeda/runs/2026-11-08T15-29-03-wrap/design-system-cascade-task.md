@task_prose part=session
The session added the driver verb actions: one read of every actionable node on the screen, each with the verbs it can take now and the refusal each would meet.
The verbs come from the schema's verb table (the instance rows taking an id: click, type, scroll, screenshot), with no closed list. Each pair is decided by the executor's own validate and a new shared per-verb target check, check_target, which all four id-taking arms now call.
The bridge gained actionable_nodes and ActionableNode, with one shared actionable rule. The unkeyed-actionable check's listener leg moved from the data-dioxus-id mark to the exact current listeners; that mark-leg reading is retired, and the mark itself stays.
Counts moved: VERBS 13 to 14, instance verbs 9 to 10, FieldKind 12 to 13 (action-nodes), the stand 244 to 253 passed over 53 files, escher-driver modules 17 to 18 and re-exports 57 to 59.
Measured: ACTIONABLE_AT_BOOT 2, 7, 3, 11, 678, 2. The Cells actions answer is 106,088 bytes for 678 nodes, and 461 cells read off-screen for click, type and screenshot.
The unpaged actions answer is a stated limit beside the unpaged graph, owned by the same residual res-7cc5bi4k, now widened.
@task_file n=1 path=packages/escher-driver/src/actions.rs
@task_file n=2 path=tests/blitz-tests/tests/stand_actions.rs
@task_file n=3 path=packages/dioxus-native-dom/src/actionable.rs
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=5 path=packages/escher-driver/src/execute.rs
@task_file n=6 path=packages/escher-driver/src/schema.rs
@task_file n=7 path=packages/escher-driver/src/schema/spec.rs
@task_file n=8 path=packages/escher-driver/src/schema_tests.rs
@task_file n=9 path=packages/escher-driver/src/command.rs
@task_file n=10 path=packages/escher-driver/src/json.rs
@task_file n=11 path=packages/escher-driver/src/describe.rs
@task_file n=12 path=packages/escher-driver/src/lib.rs
@task_file n=13 path=packages/escher-driver/src/mcp.rs
@task_file n=14 path=packages/escher-driver/src/mcp_tests.rs
@task_file n=15 path=packages/escher-driver/src/cli_tests.rs
@task_file n=16 path=packages/escher-driver/src/recording_tests.rs
@task_file n=17 path=tests/blitz-tests/tests/session_common/mod.rs
@task_file n=18 path=tests/blitz-tests/tests/common/mod.rs
@task_file n=19 path=tests/blitz-tests/tests/stand_act_spans.rs
@task_file n=20 path=tests/blitz-tests/tests/stand_act_scroll.rs
@task_file n=21 path=tests/blitz-tests/tests/stand_mcp_tools.rs
@task_file n=22 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
@task_file n=23 path=examples/seven_guis/tests/cli_commands.rs
@task_file n=24 path=examples/seven_guis/tests/mcp_server.rs
@task_file n=25 path=examples/seven_guis/tests/host_log.rs
@task_file n=26 path=examples/seven_guis/tests/mcp_log.rs
@task_file n=27 path=examples/seven_guis/tests/common/mod.rs
@task_file n=28 path=scripts/cold-agent.sh
@task_file n=29 path=.github/scripts/test_cold_agent.py
@task_file n=30 path=docs/capabilities.md
@task_file n=31 path=README.md
@task_item n=1 part=touches
The CLI surface's pending unpaged-graph limit (the cli-unpaged-graph group, fact fct-bzczuomv): its residual res-7cc5bi4k now also owns the unpaged actions answer. That answer takes no argument and answers the whole screen at once, 106,088 bytes for the 678 Cells nodes. The text must name the actions answer beside the graph's.
@task_item n=2 part=touches
The CLI surface's component patterns (cli-output): a new answer line for actions joins the one-JSON-line-per-command output, written as nodes with id, owner, keyed and verbs (verb, cause), leaving out owner and cause when absent. It carries only ids, verb words, closed cause words and booleans: never a value, typed text or NodeId.
@task_item n=3 part=touches
The CLI surface's graph pattern (cli-graph): the graph and actions answers now both carry a result field named nodes, of different field kinds (the graph's nodes and action-nodes). Any text describing the graph answer as the only node listing, or any count of verbs or field kinds on the command line, moves (verbs fourteen, field kinds thirteen).
@task_item n=4 part=touches
Any count of driver verbs, tools or command-line commands in design-system moves from thirteen to fourteen, and any count of instance verbs from nine to ten (actions the tenth, after graph). The describe count row reads 14, 14, 12, 3, 8, 4, 15.
@task_item n=5 part=touches
Typography loading (screenshot refused fonts-unavailable with nothing painted): the refusal is now decided by the shared check_target, through BaseDocument::can_paint_text, after in_view and an empty-crop check and before painting. The actions read reports it as the screenshot verb's cause without painting. The rule stands, but who decides it moved, and the read now surfaces it.
@task_item n=6 part=touches
Motion and clock text (the driver's advance moves the VirtualClock; click, type and press leave it where it was): actions is a read that moves no clock, opens no list and calls no pump, tick, dispatch or scroll-into-view. A clause listing which verbs leave the clock alone may need actions added.
@task_item n=7 part=touches
Depth and hit-testing, indirectly: the actions read reports covered for click and type through the unchanged action_point (disabled, then off-screen, then covered). A covered select option reads covered with its select as owner until the list opens. The hit-testing rule itself is unchanged.
@task_item n=8 part=touches
Motion's into-view and scroll text, indirectly: the actions read says scroll is accepted on every listed node (all 678 Cells nodes take scroll) and meets no target check. The last CRUD row reads off-screen until a scroll.
@task_item n=9 part=touches
The CLI surface's exit-status and stdout statement, indirectly: an actions call with any argument is refused malformed, so exit status 1 on the command line and isError over MCP. The four exit statuses and the JSON-only stdout are unchanged and now cover one more verb.
@task_item n=10 part=touches
The subjects comment's cli entries (cli-output, cli-graph, cli-unpaged-graph): if the actions answer gets a group of its own, the subjects list gains it.
@task_fact n=1 slug=surface
fact: The driver command line answers actions with one JSON line, {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}. It lists every actionable node in snapshot pre-order: focusable, interactive-role or carrying a current non-mounted listener, or owned by another node. Each node carries the table's id-taking instance verbs and the cause each would be refused with; owner and cause are left out when absent.
@task_fact n=2 slug=surface
fact: The actions answer is unpaged: it takes no argument and answers the whole screen at once. On Cells that is 106,088 bytes for 678 nodes, within the 1,048,576-byte answer bound. It is a stated limit owned by residual res-7cc5bi4k beside the unpaged graph.
@task_fact n=3 slug=surface
fact: The actions answer reports whether each call would be accepted, not what it would do. A node's verbs are decided by the same validate and target check the executor uses, so a listed cause equals what the call returns. A node that resolves to no element reads stale.
@task_item n=1 part=untouched
Color palette, core colors, surface scale, text hierarchy, semantic colors and border progression: no colour or token changed.
@task_item n=2 part=untouched
The open select list's engine colour constants, the SelectListView highlight pair: unchanged.
@task_item n=3 part=untouched
Spacing, border radius and their untokenized values: the session touched no layout or radius value.
@task_item n=4 part=untouched
Iconography, browser toolbar icons, cursor keywords and favicons: unchanged.
@task_item n=5 part=untouched
The desktop-native, mobile-native and web-spa surfaces (platform, toolkit, tokens, navigation, titlebar, URL bar, canvas): no app shell or platform build changed.
@task_item n=6 part=untouched
The WPT runner and the paint_bench and screenshot examples on the CLI surface, with their owo-colors output: unchanged.
@task_item n=7 part=untouched
The blitz-tests system-fonts feature and the WASM bundled-font context rule: unchanged.
@task_item n=8 part=untouched
The harness settle and the animation runtime (Settled, the animating flag) and the driver's load wait holding the clock still: unchanged.
@task_item n=9 part=untouched
The NOT YET MEASURED and NO RECORDED INTENT placeholders (brand, anti-patterns, self-validation, decisions log, project-wide scales, CLI navigation and terminal width): the session measured none of them.
