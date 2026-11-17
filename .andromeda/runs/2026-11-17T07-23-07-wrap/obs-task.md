@task_prose part=session
The session added an in-process way to render one component alone, with given props, at the headless stand's pinned 800 by 600 viewport and font: a harness constructor, a public mount function and an isolated stand boot.
It added no verb, argument, carrier, MCP tool, session socket call, kept-check key, span or log field; no driver or session-host file changed.
The cold-agent pipe's verb list went from fourteen to fifteen, layout after actions; the counted set stays lint, screenshot and advance, and layout is an uncounted read like graph and actions; the verdict shape is unchanged, no layout_calls field, capability-unused unchanged.
It measured that a component alone has no shell CSS so its root element sizes to its content, and that a vdom rebuilt in place reads one Root error mark through the boundary fallback.
Tests added: a new stand integration target (five tests, both layout modes, it prints nothing and installs no subscriber), one harness unit test, one boundary unit test, one cold-agent contract test (38 in all); the capability statement gained a section.
@task_file n=1 path=tests/blitz-tests/tests/stand_component_alone.rs
what: a new headless stand check: a component rendered alone reads as inside its app, node for node, in both layout modes; silent, no sink
@task_file n=2 path=packages/blitz-test-harness/src/harness.rs
what: the test harness's Harness construction; gains the isolated constructor from_component_alone, the public vdom_alone mount and its first unit test
@task_file n=3 path=packages/blitz-test-harness/src/lib.rs
what: the test harness crate's public surface; re-exports vdom_alone
@task_file n=4 path=packages/blitz-test-harness/Cargo.toml
what: the test harness crate's manifest; a dev-dependency turns the accessibility feature on for its test build only
@task_file n=5 path=examples/seven_guis/src/stand.rs
what: the headless stand's boots; gains boot_alone, a component alone with the stand's timer-ticks root context, no sink installed
@task_file n=6 path=packages/dioxus-native-dom/src/boundary.rs
what: the error-boundary registry and its root error mark; one unit test added, a vdom rebuilt in place reads one Root mark
@task_file n=7 path=scripts/cold-agent.sh
what: the cold-agent pipe; its VERBS list now fifteen with layout, its isolation tool set the fifteen mcp escher tools
@task_file n=8 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's contract check and its fake server; VERBS fifteen, the fake answers layout as an uncounted read
@task_file n=9 path=docs/capabilities.md
what: the capability statement, updated by hand; a section on a component alone with its witnesses line
@task_item n=1 part=touches
decision/graph-and-actions-not-counted-verbs is stale: layout now joins graph and actions as a verb the cold-agent pipe offers and does not count.
@task_item n=2 part=touches
contract/cold-agent-counted-verbs advance, lint and screenshot: the counted set is unchanged at three; a picker should confirm no row names layout as counted or the offered verbs as fourteen.
@task_item n=3 part=touches
contract/cold-agent-verdict/capability-unused and contract/cold-agent-verdict-reasons/capability-unused: unchanged in rule, but any cell naming the pipe's tool set as fourteen tools is now fifteen.
@task_item n=4 part=touches
decision/verdict-fields-closed and the cold-agent verdict field contracts: no layout_calls field was added; the closed field set holds, a layout call moves no verdict counter.
@task_item n=5 part=touches
decision/cold-agent-server-log-counted-only and decision/cold-agent-server-log-read-by-span-name: the server log's tool_call spans may now carry tool layout; it is read and not counted.
@task_item n=6 part=touches
contract/layout-answer/content and contract/layout-answer/logging: the layout answer now reaches the model provider on a live cold-agent run, holding ids, closed words and numbers only.
@task_item n=7 part=touches
decision/layout-read-opens-no-span and contract/command-span/layout: a layout call on the cold-agent pipe is instrumented only by the tool_call span the server already writes; no span added.
@task_item n=8 part=touches
decision/checks-silent: covers the new stand integration check and the new harness and boundary unit tests, which print nothing.
@task_item n=9 part=touches
decision/headless-stand-no-sink: covers the stand's new isolated boot, which installs no subscriber and no sink.
@task_item n=10 part=touches
decision/capability-statement-updated-by-hand: the statement gained a section by hand for a component alone, naming its witnesses.
@task_item n=11 part=touches
decision/cold-agent-log-no-user-content, decision/cold-agent-events-carry-no-user-content and decision/cold-agent-verdict-holds-no-content: layout's answer carries no page text or value, so these hold with layout on the pipe.
@task_item n=12 part=touches
decision/cold-agent-kept-checks-at-rest, decision/cold-agent-kept-checks-stay-local and contract/kept-check rows: an agent-written check's step tool may now be layout; no kept-check key added.
@task_item n=13 part=touches
contract/tool-call-span/tool and contract/escher-driver-tool-call-span/tool: layout, already a driver tool, is now one the cold-agent pipe lets the agent call; no new tool name.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: On a live cold-agent run the layout answer crosses to the model provider in the transcript, carrying ids, closed words and numbers and no page text or value.
@task_item n=1 part=untouched
The telemetry sink, its log line format, scrub set, target allowlist, RUST_LOG handling and resource keys: no sink, field or target changed.
@task_item n=2 part=untouched
The driver's command span fields, refusal cause names and tool_call endings: no driver code changed, no verb, cause or ending added.
@task_item n=3 part=untouched
escher-session roles, exit codes, host wire, coverage line, recording file and session lifecycle: no session host or carrier changed.
@task_item n=4 part=untouched
Script error surfaces, window error events, WPT reporting and the panic hook: untouched.
@task_item n=5 part=untouched
Metrics, telemetry backends and OTel export absences: the session added no metric, exporter or backend.
@task_item n=6 part=untouched
CI integration rows (artifacts, workflows, cleanup scope, recording bounds) and agent-run harness events: no CI step or agent-run event changed.
@task_item n=7 part=untouched
Select, optgroup, snapshot, page-event and mutation-buffer PII rows: no engine reading of the page changed.
