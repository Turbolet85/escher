@amendment marker=2026-11-07-ui-graph
title: the cold-agent pipe speaks thirteen verbs
section: §3 Test Harness Contract (test-plan.md: the server and the check's judge, the verdict); §4 Unit Test Strategy (unit-test-strategy.md: the cold-agent pipe)
change:
  - The agent's MCP surface `escher-session mcp` reads the thirteen verbs, and `valid_step` admits only `tool` in the thirteen verbs (was twelve, both).
  - `isolated` holds the init tools exactly the thirteen `mcp__escher__{verb}` tools (was twelve).
  - test_cold_agent.py's fake `escher-session` speaks the thirteen tools and answers `graph` with `{"nodes":[],"states":[]}`; no verdict requires `graph` of a cold agent (was the twelve tools).
why: the chunk added `graph` to the cold-agent pipe's and its test's `VERBS`, the fake answering it with an empty graph, and left the `capability-unused` set unchanged, since `graph` is not required of a cold agent.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: stand_graph joins the stand runs and the target counts
section: §3 Test Harness Contract (test-plan.md: Proof, the newest reading; the test-data bootstrap); §9 CI Integration (ci-integration.md: the legs, stand-repeat); §10 Quality Gates (quality-gates-coverage-targets.md: the flakiness budget); §2 Test Strategy (test-strategy.md: the directory pattern)
change:
  - `run stand`'s `run.end` reads passed 244 · failed 0 · ignored 7 over 52 files, `stand_graph` picked up by its `stand_` prefix with no script change (was 237 · 0 · 7 over 51); the bootstrap's `run stand` reading moves the same way, measured at this chunk (was at 2026-10-31-select-popup).
  - The stand-repeat leg runs one `--test` per `stand_*.rs`, 52 files, `stand_graph` joining (was 51); the flakiness budget's leg reads the 52 files (was 51).
  - `tests/blitz-tests` holds 159 `.rs` files, 158 test targets and the unbuilt `all.rs`, `stand_graph.rs` the latest (was 158 and 157); `tests/common/mod.rs` is read by 22 `stand_*.rs` checks (was 21) and `tests/session_common/mod.rs` by 33 (was 32), `stand_graph.rs` the latest.
why: the chunk added the stand target `stand_graph`, which the stand runs and the stand-repeat glob take by its prefix; `run stand` measured 244 · 0 · 7.
kept: `run all`'s reading, 743 · 0 · 12, stands as measured at 2026-11-04-dioxus-runtime-probe; this chunk did not re-count it.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the stand checks' shared modules gain the graph's drives and readers
section: §3 Test Harness Contract (test-plan.md: blitz-tests stand checks, blitz-tests session checks); §7 Test Data & Fixtures (test-data-fixtures.md: the Graph drives row)
change:
  - `tests/common/mod.rs` states `GRAPH_DRIVES` with its `Drive` enum: each stand task's drives, seven rows, every conditional write driven once so no stand write reads `unconfirmed`, read by `stand_graph`.
  - A new fixture row, Graph drives (stand tasks): Cells clicks A0, C0, A0 and leaves its keyed editor once by Enter and once by Tab, so no stand write reads a flag (`undeclared` or `unconfirmed`).
  - `tests/session_common/mod.rs` gains the builder `graph()` and the reader `graphed(session, call)`, a graph's `UiGraph`, `None` for a refused call or any other outcome.
why: the chunk decided that the stand's drive table drives each conditional write once, and added the graph call's builder and reader to the session checks' module.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: dioxus-native-dom's unit scope and its one test-only shim
section: §1 Test Scope Summary (test-plan.md: the dioxus-native-dom scope line; test-scope-summary.md: the dioxus-native-dom count, the dioxus-native and stylo_taffy search); §3 Test Harness Contract (test-plan.md: the dioxus-native-dom harness)
change:
  - The unit scope names the stable element id and its reserved virtual form, the bridge and its dispatch record, the UI graph reader and the signal registry beside the earlier modules.
  - dioxus-native-dom holds 100 unit tests in eleven files (was 83 in nine): nine in `graph::tests`, under the `accessibility` feature, and four in `signal_registry::tests`, under no gate; the stable element id fifteen (was thirteen), the reserved form's two among them; the snapshot builder twenty-one (was twenty), `clickable` among them; the bridge eight (was seven), the dispatch record among them; the text form writes `clickable` after `focused`, only when true.
  - The test search matches graph.rs and signal_registry.rs too.
  - The bridge tests' harness: no function under test gained a constructor or a `pub`, and its one `cfg(test)` shim, `DioxusDocument::dispatch_to_vdom`, dispatches a chain into the vdom for the dispatch-record test (was: no constructor, `pub` or `cfg(test)` shim).
why: the chunk added the `graph` and `signal_registry` modules, the reserved id form, the bridge's registry dispatch and the snapshot's `clickable`, with their unit tests (`cargo test -p dioxus-native-dom` 100 passed), and the test-only `dispatch_to_vdom`.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: a graph call logs its verb alone and no graph needle
section: §3 Test Harness Contract (test-plan.md: the stand log format); §5 Integration Test Strategy (integration-test-strategy.md: the session host's sink); §1 Test Scope Summary (test-scope-summary.md: the apps' seven_guis common module, the driver-action checks)
change:
  - `graph` is a word of `tool_call`'s `tool` and `command`'s `verb` closed sets, no span field added: a graph call's `command` line records `verb` alone under its `tool_call` parent, and the host's and the `mcp` role's stderr at `trace` hold 0 id, name and graph needles under both builds.
  - `host_log` and `mcp_log` each drive one `graph` call: `host_log` eight calls and 8 command-span lines (was seven and 7), `mcp_log` eleven tool calls and 9 `command` lines (was ten and 8), still 0 needles at `trace`.
  - seven_guis' common module gains `graph_needles()`, each served component and registered state name as a needle, called by `host_log` and `mcp_log` alone.
  - `stand_act_spans` reads the ninth instance verb, `graph`, one call per mode, its `command` line recording `verb` alone.
why: the chunk's obs outcome: the `command` span records `verb` alone and its `tool_call` parents it, and `host_log` and `mcp_log` read 0 id, name and graph needles at `trace` with one graph call each, under both builds.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the graph verb held in session and over MCP in process
section: §5 Integration Test Strategy (test-plan.md: Session and held instance; integration-test-strategy.md: three new bullets, the UI graph, the graph over MCP in process, the graph answer holds no value); §1 Test Scope Summary (test-scope-summary.md: `stand_mcp_tools`)
change:
  - `stand_graph` (6), both layout modes: Counter's `count` declared at boot and confirmed after a click; every stand task's declarations agreeing with its code once `GRAPH_DRIVES` ran; the served screens' handlers, owners and clicks; every graph node a snapshot node, no served id holding `{owner}///{key}`; a graph read changing nothing and holding no typed text; the whole Cells answer within the answer bound; the `graph` verb equal to `DioxusDocument::graph` on every read.
  - `stand_mcp_tools` holds a `graph` tool call in process in both layout modes, answering the reader's graph and parenting one `command` span recording `verb` alone; the file counts 7 (was 6).
  - A `graph` answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value: a sentinel typed into `crud-name` occurs 0 times; an argument to `graph` is refused `malformed` with nothing run.
why: the chunk's acceptance was met against the diff (`stand_graph` 6 passed), as were its arch outcome (no served id holds `///`) and its security outcome (names only, the sentinel 0 times, an argument refused).
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the command line and MCP answer the graph
section: §6 E2E Test Strategy (e2e-test-strategy.md: Command line, MCP); §1 Test Scope Summary (test-scope-summary.md: the apps' `cli_commands` and `mcp_server`); §4 Unit Test Strategy (unit-test-strategy.md: the schema driven through the binary)
change:
  - `cli_commands` counts 16 tests (was 15); each of the nine instance-level verbs is accepted (was eight); `graph_answers_one_line_of_the_ui_graph` runs `graph` as a process and reads one line of the UI graph.
  - `mcp_server` counts 7 tests (was 6); `tools/list` offers thirteen verbs, `graph` after `lint` (was twelve); `a_graph_tool_call_answers_its_line_of_the_ui_graph` reads a `graph` tool call answering its line of the UI graph.
why: the chunk's acceptance names the graph's command-line and MCP checks; `VERBS` reads 13, instance verbs 9, `graph` listed after `lint` because `VERBS` orders it.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the stand's state reaches the UI graph through the fork's registry
section: §5 Integration Test Strategy (integration-test-strategy.md: Dioxus VirtualDom and DioxusDocument, a held signal's readers)
change:
  - Hook-local `use_signal` state still exposes neither link without a fork-level change; the stand now registers each task's signals through `tasks::use_graph_signal`, `use_signal` with dioxus-native-dom's `use_graph_state`, whose entry holds a `ReactiveContext` subscribed to the signal, so their writers and reading components reach the UI graph through the fork's `SignalRegistry`, with no dioxus-core change and no manifest moved, read by `graph::tests` and `stand_graph` (was: hook-local `use_signal` state, where the stand keeps all its state).
why: the chunk registered every task's signals with declarations read off the code, through the registry hook dioxus-native-dom ships; no dependency moved, the manifests unchanged.
kept: the probe's reading of a `GlobalSignal`'s readers stands as measured at 2026-11-04-dioxus-runtime-probe.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the Cells screen and its graph answer, measured
section: §7 Test Data & Fixtures (test-data-fixtures.md: Stand screen over the budget, the new Stand graph answer (Cells) row); §1 Test Scope Summary (test-scope-summary.md: Served pages and the stand's unit tests)
change:
  - The stand's Cells screen reads 53,859 bytes in 740 lines and 6 parts of `SNAPSHOT_TEXT_BUDGET`, as measured at this chunk (was 47,089 bytes and 5 parts, at 2026-11-01-upstream-sync-ui-graph).
  - A new row: `graph` on a fresh `start cells` answers 293,869 bytes, the whole screen in one answer, `graph` taking no argument. This is the stated limit «An unpaged graph», owned by residual `res-7cc5bi4k`, target next, and `stand_graph` holds it within the answer bound in both layout modes.
why: `clickable` lengthened Cells' lines (the forecast 6 parts confirmed); the plan forecast the graph under 200 KB, disproved: 547,685 B first, 522,692 after the operator's readers fix, 293,869 after his writers fix, which recorded the rest as a stated limit and a residual row, target next. The operator, relaying the overseer at this resume, kept it so.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the committed checks' byte sizes dropped
section: §7 Test Data & Fixtures (test-data-fixtures.md: tests/blitz-tests/checks)
change:
  - The two committed checks read `counter-cli.json` (8 lines) and `counter-mcp.json` (11 lines), their byte sizes no longer stated (was 2,929 B and 3,720 B).
why: the chunk re-recorded both checks through `record_checks.sh`, their lines changed in place, and `stand_replay` reads them green; the stated byte sizes were dated.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the unconfirmed flag's stated limit
section: §1 Test Scope Summary (test-scope-summary.md: a new dioxus-native-dom bullet, the UI graph's `unconfirmed` limit)
change:
  - `DioxusDocument::graph` reads a declared write `unconfirmed` once a dispatch of the handler's event passed through its element and no run wrote the state, so a write its handler makes only on a condition reads `unconfirmed` after runs that skipped it, a stated limit no check reaches; the stand's `GRAPH_DRIVES` drives each conditional write once, so no stand write reads `unconfirmed`.
why: this is how the graph reader the chunk shipped fires the flag; the operator's word at the P5 review keeps the conditional write a run skipped as a stated limit.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the snapshot's clickable reading and the CLICKABLE table
section: §1 Test Scope Summary (test-scope-summary.md: the non-lean tasks' measured pin, the new `stand_non_lean_tasks` CLICKABLE bullet)
change:
  - The non-lean tasks' click reading reads `NodeState.clickable`, `NODE_FIELDS` fourteen (was: the snapshot field for it deferred, `NODE_FIELDS` still thirteen).
  - `stand_non_lean_tasks` counts per task the elements reading `clickable` that state no role (`clickable_without_role`) against a 7-row table, `CLICKABLE`, both layout modes: Counter, Flight Booker, Timer, CRUD and Temperature Converter 0 each, Circle Drawer 1, Cells 676, its data cells, `cells-grid` not among them.
why: the chunk built the click reading the capability statement listed as not built; the node keys moved from 13 to 14; the `CLICKABLE` table is a new measured figure.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the driver's verb-table pins read thirteen verbs
section: §1 Test Scope Summary (test-scope-summary.md: escher-driver's unit tests); §4 Unit Test Strategy (unit-test-strategy.md: escher-driver's pins, the list's help and `covered`, the MCP listing test's helpers)
change:
  - schema_tests.rs pins thirteen verbs, nine instance-level, `graph` the ninth after `lint`, no argument, fields `nodes` and `states` (was twelve and eight); twelve field kinds, `GraphNodes` (`graph-nodes`) and `GraphStates` (`graph-states`) the newest (was ten); fourteen node keys, `clickable` after `focused` (was thirteen).
  - `VERBS` reads 13 (was 12), describe's count row `[13, 14, 12, 3, 8, 4, 15]` (was `[12, 13, 12, 3, 8, 4, 15]`).
  - mcp_tests.rs' listing reads the thirteen verbs of the table, `graph` after `lint` (was twelve).
  - json.rs pins the graph's `nodes` and `states` among the outcomes and a node's `clickable` beside `focused`.
  - cli_tests.rs reads a line by its rules in 101 rows, `graph`'s among them (was 64 since 2026-10-10-mcp-surface, 61 until then).
why: the chunk moved `VERBS` 12 to 13, instance verbs 8 to 9, `NODE_FIELDS` 13 to 14, `FieldKind` 10 to 12 and describe's row, and extended the driver's tests' rows (`cargo test -p escher-driver` 100 passed); the citation sweep read the 64 rows false.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: dioxus-native-dom's unit-test groups for the graph
section: §4 Unit Test Strategy (unit-test-strategy.md: five new dioxus-native-dom bullets, the UI graph, the signal registry, the bridge's dispatch record, the snapshot's `clickable`, the reserved id form)
change:
  - `graph::tests` pins the UI graph reader: each node's component; its current listeners, `mounted` included and a removed one gone; a declared write declared before a dispatch and confirmed after; two reader components reading a state the writer does not; an option owned by its select; a prefix declaration naming its elements alone; a drifting declaration reading both flags; a write by an element since gone not handed to its ancestor; a repeated name reading its ordinal, a page with no state reading none.
  - `signal_registry::tests` pins the registry, its mechanism control first: a handler's write recorded against its dispatch and one outside any against none; an entry leaving with its scope, its `Debug` printing counts alone; a declaration naming its element or its prefix; a page with no registered state holding no registry.
  - The bridge test drives a dispatch through the `#[cfg(test)]` `DioxusDocument::dispatch_to_vdom` and reads the dispatch record where a state is registered, a page with none acting as before.
  - `an_element_with_an_app_click_listener_reads_clickable` pins `NodeState.clickable` true where the current listeners hold `click` and the element is no disabled form control.
  - Two `element_id` tests pin the reserved `{owner}///{key}` form over an adversarial page; `virtual_node_id` returns `None` for an empty or `/`-bearing key, and the walk calls it nowhere.
why: these are the unit tests the chunk's new surfaces carry; `stand_graph` disproved the plan's resolution rule (it handed a gone element's write to its ancestor), fixed by stopping at a gone node.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the session contract takes graph as its ninth instance verb
section: §3 → Session lifecycle (contracts/session-lifecycle.md: session-start, session-binary, session-proof)
change:
  - session-start: `snapshot`, `screenshot`, `lint` and `graph` run no step and wait on nothing; `graph` returns `Outcome::Graphed { graph }` with the screen's `UiGraph`; `run` records each answered call of the nine instance verbs, a `lint` and a `graph` among them (was eight).
  - session-binary: thirteen verbs, the nine instance-level ones (was twelve and eight); `graph` the ninth, no argument (an argument refused `malformed`), its accepted answer one line `{"nodes":[…],"states":[…]}`, status `0`; the usage line lists `graph` after `lint`.
  - session-proof: `cli_commands` 16 (was 15), `mcp_server` 7 (was 6); the `graph` verb held over the command line, over MCP and in process by the three graph checks; `stand_graph` 6, the verb's answer equal to `DioxusDocument::graph` on every read.
why: the chunk added the instance verb `graph` after `lint` and `Outcome::Graphed`, which runs no step, the recording routing every verb through `VERBS`; its gates read `stand_graph` 6 passed and escher-driver's 100 unmoved.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: a graph answer crosses the session socket
section: §3 → Session lifecycle (contracts/session-lifecycle.md: session-wire)
change:
  - A `graph` call's answer carries the screen's UI graph out: ids, owner ids, component names, registered state names, handler event names, the declared writes, the closed flag words `undeclared` · `unconfirmed` and booleans, never a value or typed text; still no `NodeId` crosses.
why: a boundary widening, ratified by the founder by dialog in the overseer session and relayed by the operator: at P4, 2026-11-07, his choice of a new verb `graph`, and at P5, 2026-11-08, the authored write declarations. The overseer confirmed by question dialog, 2026-11-08, that this change lies inside those answers. The confirm is provisional, delegated while the founder is away, and his own word in his batch at the epoch boundary supersedes it.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
