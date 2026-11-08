@disposition n=1
row: stretched fct-og7dxjok tests/blitz-tests/tests/common/mod.rs:17-174 → 17-185 — changed inside
disposition: holds (the tables remain stated expectation and no mapping; the range now also holds the new ACTIONABLE_AT_BOOT table at 93-102, which the fact does not name)
@disposition n=2
row: stretched fct-prxcxiqp@c1446 tests/blitz-tests/tests/session_common/mod.rs:119-172 → 119-186 — changed inside
disposition: holds (the builders and `shot` stand there; the new `actions()` builder and `actioned` reader inside the range are unnamed by the fact)
@disposition n=3
row: stretched fct-prxcxiqp@c1690 tests/blitz-tests/tests/session_common/mod.rs:134-147 → 134-161 — changed inside
disposition: holds (`graphed` stands at 153-159, with `actions()` and `actioned` now inside the range)
@disposition n=4
row: stretched fct-prxcxiqp tests/blitz-tests/tests/session_common/mod.rs:1-423 → 1-438 — changed inside
disposition: holds (the module is still the shared session harness with no test; `actions()` and `actioned` are new and unnamed, and the new reader `stand_actions` is not counted)
@disposition n=5
row: stretched fct-xxpbx2ay@c4830 examples/seven_guis/tests/mcp_log.rs:55-335 → 57-352 — changed inside
disposition: holds (mcp_log still reads the role's stderr at trace under both builds, now also with an `actions` call and its needles)
@disposition n=6
row: stretched fct-xxpbx2ay examples/seven_guis/tests/host_log.rs:34-626 → 35-641 — changed inside
disposition: holds (host_log still reads the host at trace with no id, name or typed text on stderr; an `actions` call was added)
@disposition n=7
row: stretched fct-xxpbx2ay tests/blitz-tests/tests/stand_act_spans.rs:1-24 → 1-24 — changed inside
disposition: holds (the header still states one closed command line per call; nine verbs now read ten)
@disposition n=8
row: held fct-dp2mr6q3 .github/workflows/wpt-post-results.yml:43-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Pos…»
disposition: holds — stands (lines 43-49 are the step that posts the results to the PR description)
@disposition n=9
row: stretched fct-csryo2zo examples/seven_guis/tests/cli_commands.rs:135-692 → 136-716 — changed inside
disposition: claim false — raised at Validate (`cli_commands (16)` is false: `actions_answers_one_line_of_the_actionable_nodes` at 495-516 makes 17 tests on each platform, and the binary now has ten instance-level verbs where the fact says nine)
@disposition n=10
row: stretched fct-gtzxj2kr@c522 examples/seven_guis/tests/mcp_server.rs:310-362 → 312-390 — changed inside
disposition: holds (the screenshot test at 312-342 still answers an image block then a text block with isError false, and a refused shot answers one text block with isError true)
@disposition n=11
row: stretched fct-gtzxj2kr@c2000 examples/seven_guis/tests/mcp_server.rs:101-261 → 102-263 — changed inside
disposition: claim false — raised at Validate (the Counter-flow test now asserts that tools/list offers fourteen verbs with `actions` right after `graph`, where the fact says thirteen)
@disposition n=12
row: stretched fct-gtzxj2kr examples/seven_guis/tests/mcp_server.rs:263-405 → 265-433 — changed inside
disposition: claim false — raised at Validate (`mcp_server (7)` is false: `an_actions_tool_call_answers_its_line_of_the_actionable_nodes` at 366-390 inside this range makes eight tests)
@disposition n=13
row: stretched fct-5t64ong4 tests/blitz-tests/tests/stand_mcp_tools.rs:771-843 → 772-867 — changed inside
disposition: holds (the graph test now runs a shared read helper in both layout modes, its tool_call parenting one command that records `verb` alone; the range also holds the actions test)
@disposition n=14
row: stretched fct-2tbofxbl examples/seven_guis/tests/cli_commands.rs:135-217 → 136-218 — changed inside
disposition: holds (start, status and stop on the six tasks, and a command after stop ending in status 3, stand at 136-218)
@disposition n=15
row: stretched fct-adkwyvky examples/seven_guis/tests/mcp_log.rs:55-335 → 57-352 — changed inside
disposition: holds (0 sentinel, id and name needles at trace, one tool_call line per call, and protocol lines only on stdout)
@disposition n=16
row: stretched fct-adkwyvky examples/seven_guis/tests/host_log.rs:34-626 → 35-641 — changed inside
disposition: holds (the host check stands; note that the fact's separately cited count of eight calls is now nine, since `actions` was added)
@disposition n=17
row: stretched fct-adkwyvky tests/blitz-tests/tests/stand_act_spans.rs:1-24 → 1-24 — changed inside
disposition: holds (the in-process capture still reads no typed text, id or name on any driver line)
@disposition n=18
row: stretched fct-k5y3m6cu@c1600 examples/seven_guis/tests/common/mod.rs:371-573 → 371-597 — changed inside
disposition: holds (the log checks' fragment stands, called by host_log and mcp_log alone; the new `actions_needles` at 454-476 inside the range is unnamed)
@disposition n=19
row: stretched fct-k5y3m6cu examples/seven_guis/tests/mcp_server.rs:310-362 → 312-390 — changed inside
disposition: holds (the screenshot tool call answers an image block then a text block at 312-342)
@disposition n=20
row: stretched fct-k5y3m6cu examples/seven_guis/tests/host_log.rs:34-626 → 35-641 — changed inside
disposition: holds (host_log still holds 5 tests reading the host at trace with 0 needles)
