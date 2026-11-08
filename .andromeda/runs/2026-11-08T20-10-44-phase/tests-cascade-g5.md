@carried n=1 label=test-plan/integration-test-strategy fact=fct-t7rg232i
why: the owed replay test and the coverage check must drive their calls through `Session::run` on a held session, each returning after settle with its diff in both layout modes, and a new coverage verb's call must leave the one fixed-words driver line.
quote: a caller-built `Call` run through `Session::run` on a held session — `snapshot`, and `click` · `type` · `press` · `advance` · `scroll`, the first two and the last addressed by stable id alone — returns after settle with the diff of the snapshots taken before and after it, equal to the check's own `before.diff(&after)` and across the two layout modes
@carried n=1 run=2 label=test-plan fact=fct-gq2zcc44
quote: a call run through `Session::run` returns after settle with the diff of the snapshots taken before and after it; a refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied
@carried n=1 run=3 label=test-plan fact=fct-lm4npbmj
quote: with escher's sink installed at `info`, each call run through `Session::run` leaves exactly one `escher_driver` line holding fixed words and counts only, none of the ids, names, values or typed text it handled, and at the default level no driver line
@carried n=2 label=test-plan/integration-test-strategy fact=fct-csattoyz
why: the coverage command must list the handlers exactly as `DioxusDocument::graph` reads them, over the graph `stand_graph` and `GRAPH_DRIVES` pin, which must keep holding unchanged in both layout modes.
quote: `stand_graph` (6) holds, over both layout modes, Counter's `count` declared at boot and confirmed after a click; every stand task's declarations agreeing with its code once its handlers have run (`GRAPH_DRIVES`); the served screens' handlers, owners and clicks; every graph node a snapshot node, no served id holding the reserved `{owner}///{key}` form; a graph read changing nothing and holding no typed text; and the whole Cells answer within the answer bound — the `graph` verb equal to `DioxusDocument::graph` on every read
@carried n=2 run=2 label=test-plan fact=fct-gq2zcc44
quote: the `graph` verb reads as `DioxusDocument::graph` on every served screen in both layout modes and changes nothing (`stand_graph`)
@carried n=3 label=test-plan/integration-test-strategy fact=fct-krn4eywd
why: the owed replay test must keep and compare an `actions` answer as `stand_actions` states it, the verb keeping its shape.
quote: `stand_actions` (8) holds, over the six served screens in both layout modes, each screen's listed actionable nodes with the table's verbs and their counts; every listed cause on the four lean tasks equal to what `Session::run` returns
@carried n=3 run=2 label=test-plan fact=fct-gq2zcc44
quote: the `actions` verb lists each served screen's actionable nodes with the table's verbs, every listed cause what the call returns, in both layout modes, and changes nothing (`stand_actions`)
@carried n=4 label=test-plan/integration-test-strategy fact=fct-kzdrls5d
why: the chunk must write this owed test — a kept `actions` step replayed through `Check::replay` measured, not held by construction — and v030-06 reads verified only on its pass.
quote: no test exercises the replay of a kept `actions` step — `Check::replay` runs every instance-verb step through `Session::run`, so it holds by construction, unmeasured; the replay test is owed, a CARRY on «Handler coverage»
@carried n=5 label=test-plan/integration-test-strategy fact=fct-eja3jr5q
why: the coverage answer must carry ids and handler event names only, never a value, a sentinel occurring 0 times in it and an argument refused `malformed` with nothing run, as the `graph` answer does.
quote: a `graph` answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value — a sentinel typed into `crud-name` occurs 0 times in it — and an argument to `graph` is refused `malformed` with nothing run
@carried n=5 run=2 label=test-plan/integration-test-strategy fact=fct-pocqbqtc
quote: an `actions` answer carries ids, owner ids, the table's verb words, cause words and booleans only, never a value — a sentinel typed into `fixtures::password_labelled` occurs 0 times in it, `MASKED_VALUE` 0 times, and no `NodeId` field is written
@carried n=6 label=test-plan/integration-test-strategy fact=fct-5t64ong4
why: if the coverage read is a new verb, its in-process MCP tool call must join `stand_mcp_tools` through the shared read-tool-call helper, parenting one `command` span recording `verb` alone, in both layout modes.
quote: its body is the shared helper `a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone`, which `an_actions_tool_call_answers_the_readers_nodes_and_parents_a_command_recording_its_verb_alone` runs for an `actions` tool call
@carried n=7 label=test-plan/integration-test-strategy fact=fct-adkwyvky
why: if the coverage read is a new verb, `host_log` and `mcp_log` must drive it too, their call and span-line counts restated, still reading 0 needles at `trace` under both builds.
quote: each also drives one `lint`, one `snapshot` part read, one `graph` call and one `actions` call — `host_log` nine calls and 9 command-span lines, `mcp_log` twelve tool calls and 10 `command` lines — still reading 0 needles on stderr at `trace`
@carried n=7 run=2 label=test-plan fact=fct-kudmubnq
quote: a host that installs escher's sink and the `mcp` role write no id, name or typed text on stderr at `trace`
@carried n=8 label=test-plan/integration-test-strategy fact=fct-xs6og5lj
why: the handlers counted and the exercised record must key on a node's current listeners as the vdom walk reads them, never on the listener attributes that over-report.
quote: the vdom walk's dynamic attributes, `VNode.dynamic_attrs` with `VNode::mounted_dynamic_attribute`, read each element's current listeners exactly, `mounted` included
@carried n=9 label=test-plan/integration-test-strategy fact=fct-3vcgyjbv
why: the stand's states reach the graph only through `SignalRegistry`, so the new exercised record must hold on an app registering no state rather than reading the registry's dispatches.
quote: hook-local `use_signal` state exposes neither link without a fork-level change, `Scope.hooks` being `pub(crate)` (dioxus-core 0.7.10 `scope_context.rs:53`), read from the code, not run, and the stand registers each task's signals through `tasks::use_graph_signal` — `use_signal` with dioxus-native-dom's `use_graph_state`, whose entry holds a `ReactiveContext` subscribed to the signal — so their writers and reading components reach the UI graph through the fork's `SignalRegistry`, with no dioxus-core change and no manifest moved
@carried n=10 label=test-plan/e2e-test-strategy fact=fct-csryo2zo
why: the owed replay test rides the recording route whose kept checks are read byte-equal to what the binary records, and a new coverage verb must answer one line by process and refuse an argument, status `1`, as `actions` does.
quote: `recording`'s `the_committed_checks_are_what_the_binary_records` runs `record_checks.sh` — a command-line session (`start counter --record` · `snapshot` · `click counter-increment` · `advance 600` · `click counter-increment` · `snapshot` · `stop`) and an MCP one (`initialize` … `scroll counter-increment` · `snapshot` · `stop`) — through the binary and reads both kept checks byte-equal to `tests/blitz-tests/checks/counter-{cli,mcp}.json`, the scripts holding no `screenshot` step since a shot's PNG bytes differ by OS
@carried n=10 run=2 label=test-plan/e2e-test-strategy fact=fct-hh5tpkan
quote: `cli_commands`' `actions_answers_one_line_of_the_actionable_nodes` runs `actions` as a process against a started host and reads one JSON line, and an argument to `actions` is refused, status `1`
@carried n=10 run=3 label=test-plan fact=fct-3c5ihsc6
quote: a recorded session run as a process keeps its check, and the committed checks are read byte-equal to what the binary records, the scripts holding no `screenshot` step since a shot's PNG bytes differ by OS
@carried n=11 label=test-plan/e2e-test-strategy fact=fct-gtzxj2kr
why: if the coverage read is a new verb, `mcp_server`'s `tools/list` order and its count of fourteen move to fifteen, and its tool call must answer its line with an argument reading `isError`.
quote: `tools/list` offering the verbs in order — fourteen, `graph` after `lint` and `actions` after `graph` —
@carried n=11 run=2 label=test-plan/e2e-test-strategy fact=fct-wubzr4jw
quote: `mcp_server`'s `actions` tool call answers its line of the actionable nodes, and an argument to `actions` reads `isError`
@carried n=11 run=3 label=test-plan fact=fct-pnkaq5az
quote: the binary's `mcp` role is spoken to as a child over piped stdin through the shared `Speaker` (`mcp_server`, `mcp_log`) and in process in both layout modes (`stand_mcp_tools`, `stand_mcp_masked`)
@carried n=12 label=test-plan/e2e-test-strategy fact=fct-kgita7a3
why: the exercised record added in `dioxus_document.rs` must keep the buffered render's single borrow taken after `render_immediate`, its two key-handler pins staying green in both layout modes.
quote: `DioxusDocument::initial_build` and `Document::poll` render into the crate-private `MutationBuffer` (`buffered_mutations`) with the document unborrowed, `poll` taking `inner.borrow_mut()` (`dioxus_document.rs:318`) once, after `vdom.render_immediate` (`:317`), and replaying through one `MutationWriter`
@carried n=12 run=2 label=test-plan/e2e-test-strategy fact=fct-sys2coyr
quote: a task an event handler spawns while it also dirties a scope is first polled inside `render_immediate`'s `Work::PollTask` arm, the one path that meets a borrow held across `render_immediate`
@carried n=12 run=3 label=test-plan/e2e-test-strategy fact=fct-mealwlke
quote: `mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental` and `mounted_data_in_a_task_a_key_handler_spawned_acts_incremental`, one per layout mode
@binding n=1
**integration-test-strategy (the owed replay) ↔ e2e-test-strategy (recording)**: the owed test records a session holding an `actions` step, keeps it as a check, reads it back through `Check::from_bytes` and replays it through `Check::replay` on a fresh session, each step run through `Session::run` (test-plan/integration-test-strategy fct-kzdrls5d) (test-plan/integration-test-strategy fct-t7rg232i); the kept `actions` answer it compares is the reading `stand_actions` states (test-plan/integration-test-strategy fct-krn4eywd), holding no value or `NodeId` (test-plan/integration-test-strategy fct-pocqbqtc); any check it commits is kept byte-equal to what the binary records, with no `screenshot` step (test-plan/e2e-test-strategy fct-csryo2zo).
@binding n=2
**integration-test-strategy (UI graph) ↔ integration-test-strategy (a node's listeners and held signals)**: the coverage answer lists (graph node id, handler event) pairs exactly as `DioxusDocument::graph` reads them, `stand_graph` unchanged (test-plan/integration-test-strategy fct-csattoyz); a handler is a current listener as the vdom walk reads it, never a listener attribute (test-plan/integration-test-strategy fct-xs6og5lj); and «exercised» is recorded on every app, registry or not, since only registered states reach `SignalRegistry` (test-plan/integration-test-strategy fct-3vcgyjbv); the answer carries ids and event names only, never a value (test-plan/integration-test-strategy fct-eja3jr5q).
@binding n=3
**integration-test-strategy (in-process MCP and host logs) ↔ e2e-test-strategy (command line and MCP)**: if the coverage read is a new verb, it moves together `mcp_server`'s fourteen-verb `tools/list` (test-plan/e2e-test-strategy fct-gtzxj2kr), the command-line and MCP one-line answer and refused-argument pins (test-plan/e2e-test-strategy fct-hh5tpkan) (test-plan/e2e-test-strategy fct-wubzr4jw), `stand_mcp_tools`' shared read helper (test-plan/integration-test-strategy fct-5t64ong4), the one fixed-words driver line (test-plan fct-lm4npbmj) and `host_log`/`mcp_log`'s driven-call and span counts at 0 needles (test-plan/integration-test-strategy fct-adkwyvky).
@binding n=4
**e2e-test-strategy (buffered render) ↔ integration-test-strategy (the exercised record in the bridge)**: the record of (element, event) handlers reached, added in `dioxus_document.rs`, takes no borrow of the document across `render_immediate` (test-plan/e2e-test-strategy fct-sys2coyr), `poll` still borrowing once after it (test-plan/e2e-test-strategy fct-kgita7a3), the two key-handler pins green in both layout modes (test-plan/e2e-test-strategy fct-mealwlke).
