@task_prose part=session
The stand checks' repeated setup (booting a held task, the in-process MCP exchange and its tool-call lines, decoding the MCP tool result, the settled acting step, the both-layout-modes agreement, element geometry reads, the recording check) moved out of each check into the shared test modules.
Each stand check stays one target with the same tests, names and messages; the fold guard read files 197, tests 930, literals 3636 equal before and after.
Helpers whose body or message differs stayed private in their own check; two more checks now declare the session checks' shared module (44 to 46).
Measured: the stand checks' own duplication fell from 75 pairs and 870 lines to 42 and 358; the whole duplication reading fell from 4.05 to 3.74 percent; the upstream-taken cdp-server and vibey-script tests were left as written, the founder's question on them unanswered.
No product source, manifest, CI script, committed check or seven_guis test changed; fast, stand-repeat (3 of 3 rounds) and a11y legs green, CI 17 of 17 green including the windows leg.
@task_file n=1 path=tests/blitz-tests/tests/session_common/mod.rs
what: the session checks' shared module, no test target: helpers every session and stand check shares; now also the tool-call line, the in-process MCP exchange, the MCP tool result decoder, the held lean task and settled step, the both-modes agreement assert, element geometry reads and the unix-gated recording check
@task_file n=2 path=tests/blitz-tests/tests/common/mod.rs
what: the shared fixtures module of blitz-tests, no test target; now also the element marks read from the harness's accessibility roles
@task_file n=3 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: stand check of the refusal causes' error text and remedy served over MCP
@task_file n=4 path=tests/blitz-tests/tests/stand_act_errors.rs
what: stand check of acting-call refusals and their causes, over the session and MCP
@task_file n=5 path=tests/blitz-tests/tests/stand_order_served.rs
what: stand check of the order task served over the session and MCP
@task_file n=6 path=tests/blitz-tests/tests/stand_mcp_masked.rs
what: stand check that a typed password stays masked in MCP answers
@task_file n=7 path=tests/blitz-tests/tests/stand_snapshot_parts.rs
what: stand check of snapshot parts and the snapshot over MCP
@task_file n=8 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: stand check of the MCP role's tools: tools/list, tool results, refused calls, the tool_call span and the no-font and covered exchanges
@task_file n=9 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: stand check that masks hold on every new answer
@task_file n=10 path=tests/blitz-tests/tests/stand_session_state.rs
what: stand check of the held session state (focus and held values)
@task_file n=11 path=tests/blitz-tests/tests/stand_actionable_keys.rs
what: stand check of the actionable rule and keys, read against the harness's element marks
@task_file n=12 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: stand check of the served tasks that are not lean tasks
@task_file n=13 path=tests/blitz-tests/tests/stand_actions.rs
what: stand check of the actions verb's answer over the stand tasks
@task_file n=14 path=tests/blitz-tests/tests/stand_layout.rs
what: stand check of the layout verb's answer over the stand tasks
@task_file n=15 path=tests/blitz-tests/tests/stand_act_diff.rs
what: stand check of the acting call's diff, in both layout modes
@task_file n=16 path=tests/blitz-tests/tests/stand_act_states.rs
what: stand check of changed node states after an acting call, in both layout modes
@task_file n=17 path=tests/blitz-tests/tests/stand_act_obstructed.rs
what: stand check of the covered and off-screen refusals for an obstructed element
@task_file n=18 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: stand check of scrolling and acting on a scrolled element
@task_file n=19 path=tests/blitz-tests/tests/stand_act_transformed.rs
what: stand check that a click lands on a transformed element
@task_file n=20 path=tests/blitz-tests/tests/stand_load_page.rs
what: stand check of the load page stand-in and the settle wait over loads
@task_file n=21 path=tests/blitz-tests/tests/stand_replay.rs
what: stand check of recorded check replay, its stop at a changed step and the unix-gated recording
@task_item n=1 part=touches
The rule that session checks share one module: its shared module now holds the stand checks' boot, MCP exchange, answer-read, settled-step and geometry helpers, and two more checks declare it.
@task_item n=2 part=touches
The rule that shared fixtures sit in one module: the shared fixtures module gained the element marks read.
@task_item n=3 part=touches
The rule that shared test modules are no target and that blitz-tests runs one target per file (one target per behaviour): kept, no target added, renamed or removed, the same tests in each.
@task_item n=4 part=touches
MCP tool result text decoded whole: the decoder moved into the shared module as one helper; the lenient decoder stays beside it, and the MCP tools check keeps its own result assertion on isError.
@task_item n=5 part=touches
Stand checks run in both layout modes and layout assertions in both modes: the modes-agree assertion and the held task's settled step moved into the shared module, still called in both modes.
@task_item n=6 part=touches
Recording checks and socket session checks unix-gated: the recording check helper moved into the shared module under the same unix gate; its off-unix compile was witnessed by the CI windows leg, not on the host.
@task_item n=7 part=touches
MCP role tested as a child and in process: the in-process MCP exchange now goes through one shared helper taking the boot as a parameter; reply counts and messages unchanged.
@task_item n=8 part=touches
Test failures print no argument value and no ids, names or typed text in test names or messages: held, the test name lists and the literal set equal before and after.
@task_item n=9 part=touches
Session and stand logs silent: the shared helpers add no print, subscriber, env read, log site or span; the MCP tools check's covered and no-font exchanges still run under their own recording subscriber.
@task_item n=10 part=touches
Stand checks repeated three rounds with a zero-failure budget, and every requirement witnessed inside stand-repeat: the moved helpers ran green 3 of 3 rounds.
@task_item n=11 part=touches
Cross-OS readings of the served tasks: that check kept its own held-task boot, its body differing from the shared one.
@task_item n=12 part=touches
Shared stand tables in one module and shared graph drives: the stand tables were not moved; a picker should confirm the shared helpers do not restate them.
@task_item n=13 part=touches
Upstream tests kept as written: the upstream-taken cdp-server and vibey-script tests' duplication was left untouched, the founder's question on keeping them still unanswered.
@task_fact n=1 slug=3-test-harness-contract
fact: Setup repeated across the stand checks lives once in the shared test modules; a helper whose body or failure message differs from the shared one stays private in its own check.
@task_item n=1 part=untouched
Product source of the driver, session host, MCP server and harness: no product file changed, so the session wire, session errors, refusal causes, exit codes and command-line contracts stand as they are.
@task_item n=2 part=untouched
CI integration: no CI script, leg membership, cache, package install or trigger changed.
@task_item n=3 part=untouched
Quality gates: coverage floors, mutation testing and the complexity ceiling were not read or moved.
@task_item n=4 part=untouched
WPT runner, its suites, statuses and pinned commit.
@task_item n=5 part=untouched
Agent-run and cold-agent contracts, events, verdicts and exit codes.
@task_item n=6 part=untouched
Committed recorded checks and the recording script: no committed check changed.
@task_item n=7 part=untouched
The seven_guis headless stand's boot functions and timer step, and the seven_guis tests.
@task_item n=8 part=untouched
Harness construction, input, inspection, screenshot, contrast and settle contracts.
@task_item n=9 part=untouched
Lint defects and findings, snapshot node fields and handler coverage contracts.
