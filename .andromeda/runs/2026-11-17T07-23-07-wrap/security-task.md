@task_prose part=session
The session added a way to render one component alone, with given props, at the headless stand's pinned viewport and font, and read it as inside its app: a public test-harness constructor and mount, and a stand boot that provides the timer-ticks root context.
The isolated render is an in-process test API only: no driver verb, argument, carrier, MCP tool, session socket call, kept-check key, span, log field, IPC method, socket, port or env var was added, and no driver or session-host code changed (the founder's F1 answer A, only inside the process).
The cold-agent pipe's verb list moved from fourteen to fifteen, layout after actions, so the pipe's isolation tool set is the fifteen escher MCP tools; layout is an uncounted read like graph and actions, the counted verbs stay lint, screenshot and advance, and the verdict shape is unchanged.
On a live cold-agent run the layout answer now reaches the model provider: ids, closed words and numbers, no text or value; no live run fired.
A unit test confirms that a document built after the first build reads one root error-boundary mark through the fallback; the test harness takes the accessibility feature of the native DOM crate for its test build only, with no new crate and the lockfile unchanged.
The session brings the founder's answer that the attribute-text rule of the driver session replies takes an exception (inputs I2 item 1), left to this wrap to land since no reply changed in code.
@task_file n=1 path=tests/blitz-tests/tests/stand_component_alone.rs
what: a new integration test of the headless stand: a component rendered alone reads as on the stand, node for node, in both layout modes; prints nothing and installs no subscriber
@task_file n=2 path=packages/blitz-test-harness/src/harness.rs
what: the test harness's Harness construction, edited: a new constructor and mount of one component alone with a props maker, and its first unit test
@task_file n=3 path=packages/blitz-test-harness/src/lib.rs
what: the test harness's public surface, edited: re-exports the alone mount
@task_file n=4 path=packages/blitz-test-harness/Cargo.toml
what: the test harness's dependency manifest, edited: a dev-dependency on the native DOM crate with its accessibility feature, workspace-inherited, for the test build only
@task_file n=5 path=examples/seven_guis/src/stand.rs
what: the headless stand's boots, edited: a new boot of a component alone with the timer-ticks root context; no existing boot changed
@task_file n=6 path=packages/dioxus-native-dom/src/boundary.rs
what: the error boundary of the Dioxus native DOM, edited: one unit test added, a document built after the first build reads the root boundary's error
@task_file n=7 path=scripts/cold-agent.sh
what: the cold-agent pipe, edited: its verb list and isolation tool set now fifteen, layout after actions
@task_file n=8 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's contract test and fake server, edited: the verb list at fifteen, the fake answers layout as an uncounted read, one test added
@task_file n=9 path=docs/capabilities.md
what: the committed statement of capabilities, edited: a subsection on a component alone with its witnesses line
@task_item n=1 part=touches
contract/driver-session-reply/no-attribute-text takes the founder's exception (inputs I2 item 1); the row is stale until the exception lands, though no reply changed in code.
@task_item n=2 part=touches
decision/cold-agent-call-validation: the cold-agent call and task allowlist's verb set moved from fourteen to fifteen, layout added after actions; any cell naming fourteen verbs or tools is stale.
@task_item n=3 part=touches
decision/cold-agent-check-validation: an agent-written check's tool must now be one of the fifteen tools, layout among them.
@task_item n=4 part=touches
decision/cold-agent-replay-isolation: the isolation check's tool set on the fresh servers is the fifteen escher MCP tools.
@task_item n=5 part=touches
decision/cold-agent-screen-to-provider: on a live run the layout answer now also reaches the model provider, ids, closed words and numbers only.
@task_item n=6 part=touches
decision/cold-agent-wrong-call-bar, decision/cold-agent-check-reader-counts-only and decision/cold-agent-checks-read-for-counts-only: layout is an uncounted read; the counted verbs stay lint, screenshot and advance; the verdict shape gains no field.
@task_item n=7 part=touches
decision/cold-agent-fixed-prompt and decision/cold-agent-events-metadata-only: rules over the cold-agent pipe whose verb list changed; any listed verb set or tool count in them moves to fifteen.
@task_item n=8 part=touches
contract/driver-verbs/layout and decision/kept-check-layout-answer: the layout verb is now offered on the cold-agent pipe and a replayed agent-written check may hold a layout call.
@task_item n=9 part=touches
contract/mcp-tool-call-span/tool and contract/tool-call-span/tool: a layout call on the pipe writes the tool_call span the server already writes; no new span or field.
@task_item n=10 part=touches
decision/stable-id-exits and decision/accessible-name-exits: the cold-agent pipe is a further route by which stable ids in a layout answer reach the model provider; no accessible name or text crosses in a layout answer.
@task_item n=11 part=touches
decision/boundary-widening-rule and decision/one-validation-path-for-every-carrier: the isolated render adds no carrier and widens no boundary; the founder's F1 answer A keeps it in process.
@task_item n=12 part=touches
decision/session-layout-read-in-process: a component alone is read in process only, beside the in-process layout read; it never reaches a session.
@task_item n=13 part=touches
decision/root-boundary-err-crossing: a document built after the first build now reads one root error-boundary mark through the fallback, confirmed by a unit test.
@task_item n=14 part=touches
decision/test-harness-no-tracing-no-style: the test harness gains a dev-dependency on the native DOM crate with its accessibility feature for its test build; it takes no tracing and no style.
@task_item n=15 part=touches
decision/workspace-inherited-dependency-versions and decision/locked-builds: the new dev-dependency is workspace-inherited; no new crate, lockfile unchanged, every locked gate green.
@task_item n=16 part=touches
decision/free-disk-only-on-stand-repeat and decision/blitz-tests-not-published: a new stand integration target joins the stand run and the repeated stand leg by its glob, inside the unpublished test crate.
@task_item n=17 part=touches
decision/committed-counter-checks: the stand's Counter is now also read alone, its subtree rows, bounds and a click equal to the in-app reading.
@task_item n=18 part=touches
decision/time-moves-only-on-advance: the timer read alone moves only by ticks of the root's timer-ticks context; without the context the Timer falls back to a wall-clock delay.
@task_item n=19 part=touches
decision/capability-statement-mcp-crossing: the committed statement of capabilities gains a subsection on a component alone; the MCP crossing it states is unchanged.
@task_fact n=1 slug=input-validation
fact: Rendering a component alone with given props is an in-process test API only, on the test harness and the headless stand; it crosses no carrier, no driver verb, MCP tool, session socket call or kept check (the founder's F1 answer A).
@task_fact n=2 slug=threat-model-summary
fact: The layout answer crosses to the model provider through the cold-agent pipe on a live run, as an uncounted read holding ids, closed words and numbers, no text or control value.
@task_item n=1 part=untouched
The driver session socket, its grammar, bounds, replies other than the attribute-text exception, and the session host: no reply, call or lifecycle changed.
@task_item n=2 part=untouched
The MCP server's transport, request and result bounds, protocol errors and listing: no tool added to the server and no result shape changed.
@task_item n=3 part=untouched
Refusal causes, session errors and the error-handling tables: no cause, text or exit code added.
@task_item n=4 part=untouched
Outbound requests, script fetching, CORS and same-origin absences, iframe and import limits, navigation schemes: no network path reached.
@task_item n=5 part=untouched
HTML and CSS parsing, select lists, range and password inputs, masking of control values: no engine behaviour changed.
@task_item n=6 part=untouched
The log sink, telemetry allowlist and escher target scrubbing: no span, field or target added.
@task_item n=7 part=untouched
Authentication, secret management, CI tokens, publishing, signing and the dependency audit gate: none changed.
@task_item n=8 part=untouched
Recording, kept-check files on disk and the session state directory: no file format, lifecycle or permission changed.
