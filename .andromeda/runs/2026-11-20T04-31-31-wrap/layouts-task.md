@task_prose part=session
The session moved the repeated boot, act and answer-read setup of the stand checks into shared test helpers of the blitz-tests crate; no product source, manifest, CI script or committed check changed.
Each stand check keeps one target a file and the same tests, names and messages (fold guard PASS, files 197, tests 930); no stand check was added, renamed or removed.
The shared session test module gains the MCP tool-call line, the served MCP exchange, the MCP tool-result decoder, the lean and held served-task boot, the settled step, the incremental-versus-full agreement assertion, the element geometry readers (edges, centre, area, inside, pinned viewport) and the unix-only recording check; the shared harness module gains per-element role marks.
Measured: the stand checks' duplication fell from 75 pairs / 870 lines to 42 / 358, the whole-tree reading from 4.05 to 3.74 percent; the upstream-taken tests' share untouched.
Nothing retired; no decision, contract item, id, refusal, exit code, served task or MCP tool changed; gates fast, stand-repeat and a11y green, CI 17 of 17 green including the windows leg.
@task_file n=1 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared setup of the session stand checks — boots the stand and holds a served task, frames and serves MCP tool calls over stdio, decodes MCP tool results, takes a settled act step, reads element bounds against the pinned headless viewport, and replays a session recording as a check; edited
@task_file n=2 path=tests/blitz-tests/tests/common/mod.rs
what: the shared harness helpers of the stand checks over the rendered document — now also per-element accessibility role marks; edited
@task_file n=3 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand check on the text of the act refusals and errors the session answers; edited
@task_file n=4 path=tests/blitz-tests/tests/stand_act_errors.rs
what: the stand check on act refusals — off-screen, unknown verb and the other refused acts over the command line and MCP; edited
@task_file n=5 path=tests/blitz-tests/tests/stand_order_served.rs
what: the stand check on the order page served as a task, its controls built to cause; edited
@task_file n=6 path=tests/blitz-tests/tests/stand_mcp_masked.rs
what: the stand check that the MCP server's answers mask caller input; edited
@task_file n=7 path=tests/blitz-tests/tests/stand_snapshot_parts.rs
what: the stand check on the snapshot answer and its --part selection; edited
@task_file n=8 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand check on the escher-session MCP server — tools/list, tool results, the result bound, protocol revisions and the fonts-unavailable refusal; edited
@task_file n=9 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: the stand check that new answers keep masking caller input; edited
@task_file n=10 path=tests/blitz-tests/tests/stand_session_state.rs
what: the stand check on session state held across acts, the focused element among it; edited
@task_file n=11 path=tests/blitz-tests/tests/stand_actionable_keys.rs
what: the stand check that every stand control is keyed and actionable by its id; edited
@task_file n=12 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: the stand check on the served tasks booted outside the lean set; edited
@task_file n=13 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check on the escher-session actions answer for each served task; edited
@task_file n=14 path=tests/blitz-tests/tests/stand_layout.rs
what: the stand check on the escher-session layout answer for each served task; edited
@task_file n=15 path=tests/blitz-tests/tests/stand_act_diff.rs
what: the stand check on the snapshot diff an act returns, incremental and full modes agreeing; edited
@task_file n=16 path=tests/blitz-tests/tests/stand_act_states.rs
what: the stand check on control states after acts, a held instance reading as a fresh boot; edited
@task_file n=17 path=tests/blitz-tests/tests/stand_act_obstructed.rs
what: the stand check on acts against obstructed or clipped elements, the visible region through clipping boxes; edited
@task_file n=18 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check on scrolling an element into view across every holding scroller; edited
@task_file n=19 path=tests/blitz-tests/tests/stand_act_transformed.rs
what: the stand check that clicks on transformed elements land; edited
@task_file n=20 path=tests/blitz-tests/tests/stand_load_page.rs
what: the stand check on the load page, the one task with a network provider, fetching and reading done; edited
@task_file n=21 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand check that a session recording replays as a check, kept and stopped at a failing step; edited
@task_item n=1 part=touches
Rows whose guard or why names a stand check target as witness: every target still stands under the same name with the same tests, so no guard is renamed or moved; a picker confirms the guard cell still holds.
@task_item n=2 part=touches
Headless stand pinned viewport: the pinned viewport the checks read is now one shared reader instead of per-check copies; its value is unchanged.
@task_item n=3 part=touches
Held instance reads as a fresh boot: the held served-task boot and its settled step now come from one shared helper used by the act diff and act states checks; the decision's claim is unchanged.
@task_item n=4 part=touches
Session roles share the stand boot: the stand boot and served-task hold used by the session checks is now one shared helper; claim unchanged.
@task_item n=5 part=touches
Session recording as a replayable check, and the escher-session recording contract (check file, record dir): the recording check helper moved to the shared module, unix-only as before, witnessed green on the windows CI leg; no contract item moved.
@task_item n=6 part=touches
Visible region through clipping boxes and into-view scroll moves every holding scroller: the element edges, centre, area and inside readers these checks use moved to the shared module; behaviour unchanged.
@task_item n=7 part=touches
escher-session MCP server contract items (tools/list, result bound, stdio, end, protocol revisions) and the fonts-unavailable refusal: their checks now frame tool calls and decode results through the shared exchange and decoder; replies and messages unchanged.
@task_item n=8 part=touches
Caller input never written back: the masking checks now use the shared MCP tool-result decoder; no message or literal changed.
@task_item n=9 part=touches
Load page alone has a network provider and order page controls built to cause: their checks folded repeated setup and a shared tail assertion; the tasks they guard are unchanged.
@task_item n=10 part=touches
Every stand control keyed and stand names, roles and states by attributes alone: the shared harness now reads per-element role marks for the keyed-control check; no id or role changed.
@task_item n=11 part=touches
escher-session actions, layout and snapshot answers and the --part selection: their checks read answers through the shared helpers; no answer shape changed.
@task_item n=12 part=touches
escher-session refusals (off-screen, unknown-verb) and act error text: their checks share the tool-call and serve helpers; refusal codes and texts unchanged.
@task_item n=13 part=untouched
ShellProvider window chrome controls: no window chrome, decoration, maximise, minimise or close behaviour was reached.
@task_item n=14 part=untouched
Stand stable element ids and the stand accent colour: no stand app markup, id or style changed; the stand app itself is untouched.
@task_item n=15 part=untouched
Fixed document skeleton with main mount and the absence of arbitrary index.html templates: no document template or mount changed.
@task_item n=16 part=untouched
agent-run.sh and cold-agent.sh contracts and the cold-agent decisions: no script, usage, exit code, stdout or record dir changed.
@task_item n=17 part=untouched
multi_doc and paint_bench command lines: no argument, environment variable or output changed.
@task_item n=18 part=untouched
The mobile-native, web-spa and decisions-log areas: nothing in the session reaches them.
