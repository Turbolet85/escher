@disposition n=1
row: stretched fct-prxcxiqp tests/blitz-tests/tests/session_common/mod.rs:119-158 → 119-172 — changed inside
disposition: holds (`shot(session, call)` returning `Option<Shot>` stands at 164-169 inside the range)
@disposition n=2
row: stretched fct-prxcxiqp tests/blitz-tests/tests/session_common/mod.rs:1-408 → 1-423 — changed inside
disposition: claim false — raised at Validate (the module now also holds the `graph()` builder and the `graphed(session, call)` reader, `act` returning `None` for a graph too, none of them named; and `stand_graph` is a new reader declaring `mod session_common;`, so the stated reader count is one short)
@disposition n=3
row: stretched fct-xxpbx2ay examples/seven_guis/tests/mcp_log.rs:53-315 → 55-335 — changed inside
disposition: holds (mcp_log still exercises the format at `RUST_LOG=trace`, a graph call and graph needles added)
@disposition n=4
row: stretched fct-xxpbx2ay examples/seven_guis/tests/host_log.rs:33-608 → 34-626 — changed inside
disposition: holds (host_log still reads no id, no name and no typed text on stderr at trace)
@disposition n=5
row: stretched fct-xxpbx2ay tests/blitz-tests/tests/stand_act_spans.rs:1-24 → 1-24 — changed inside
disposition: holds (the header still states one closed command span line per call; eight verbs read nine)
@disposition n=6
row: held fct-dp2mr6q3 .github/workflows/wpt-post-results.yml:43-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Pos…»
disposition: holds — stands (43-49 is the `Post results to PR description` step through its `run` line)
@disposition n=7
row: stretched fct-csryo2zo examples/seven_guis/tests/cli_commands.rs:134-664 → 135-692 — changed inside
disposition: claim false — raised at Validate (cli_commands gained `graph_answers_one_line_of_the_ui_graph`, so its count of 15 is one short, and the instance-level verbs are nine, not eight)
@disposition n=8
row: stretched fct-gtzxj2kr examples/seven_guis/tests/mcp_server.rs:308-338 → 310-362 — changed inside
disposition: holds (the screenshot tool call test stands at 310-341; the range now also takes in the graph tool call test)
@disposition n=9
row: stretched fct-gtzxj2kr examples/seven_guis/tests/mcp_server.rs:100-259 → 101-261 — changed inside
disposition: claim false — raised at Validate (tools/list now offers thirteen verbs, not twelve, `graph` among them before `start`)
@disposition n=10
row: stretched fct-gtzxj2kr examples/seven_guis/tests/mcp_server.rs:261-381 → 263-405 — changed inside
disposition: claim false — raised at Validate (mcp_server now holds 7 tests, not 6: `a_graph_tool_call_answers_its_line_of_the_ui_graph` added inside this range and unnamed)
@disposition n=11
row: moved fct-kgita7a3 dioxus_document.rs:305 → 318 — listed, not written: suffix
disposition: holds — re-pointed by hand to 318 (`let mut inner = self.inner.borrow_mut();`)
@disposition n=12
row: moved fct-kgita7a3 dioxus_document.rs:304 → 317 — listed, not written: naked
disposition: holds — re-pointed by hand to 317 (`self.vdom.render_immediate(&mut self.buffered_mutations);`)
@disposition n=13
row: stretched fct-2tbofxbl examples/seven_guis/tests/cli_commands.rs:134-216 → 135-217 — changed inside
disposition: holds (`each_task_starts_answers_and_stops` still starts, reads and stops the six served tasks)
@disposition n=14
row: stretched fct-adkwyvky examples/seven_guis/tests/mcp_log.rs:53-315 → 55-335 — changed inside
disposition: holds (0 sentinel, id and name needles and one tool_call line per call still asserted)
@disposition n=15
row: stretched fct-adkwyvky examples/seven_guis/tests/host_log.rs:33-608 → 34-626 — changed inside
disposition: claim false — raised at Validate (host_log now drives eight calls, a `graph` added, not seven, and mcp_log eleven tool calls, not ten)
@disposition n=16
row: stretched fct-adkwyvky tests/blitz-tests/tests/stand_act_spans.rs:1-24 → 1-24 — changed inside
disposition: holds (stand_act_spans still runs `type` with the sink over a capture and reads no typed text, id or name)
@disposition n=17
row: stretched fct-k5y3m6cu examples/seven_guis/tests/common/mod.rs:371-544 → 371-573 — changed inside
disposition: claim false — raised at Validate (the log checks' fragment now also holds `graph_needles()`, called by host_log and mcp_log, and the list does not name it)
@disposition n=18
row: stretched fct-k5y3m6cu examples/seven_guis/tests/mcp_server.rs:308-338 → 310-362 — changed inside
disposition: holds (the screenshot tool call test stands at 310-341)
@disposition n=19
row: stretched fct-k5y3m6cu examples/seven_guis/tests/host_log.rs:33-608 → 34-626 — changed inside
disposition: holds (host_log still holds 5 tests, the graph call added inside `host_flow`)
@disposition n=20
row: stretched fct-k5y3m6cu examples/seven_guis/tests/cli_commands.rs:134-664 → 135-692 — changed inside
disposition: claim false — raised at Validate (cli_commands gained `graph_answers_one_line_of_the_ui_graph`, so the stated 15 tests is one short)
