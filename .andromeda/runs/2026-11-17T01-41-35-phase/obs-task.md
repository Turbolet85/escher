@task_prose part=chunk
Adds `layout`, the fifteenth driver verb and eleventh instance verb: one required stable element id in, one line out — the element's layout explanation as stable ids, closed words and numbers — run through the session's run with no step, pump, settle or load wait, changing nothing.
Carries it on every ratified carrier: the session socket's call (wire unchanged), the escher-session command line, an MCP tool `layout` listed after `actions` and before `help`, and a kept check that records and replays the step; `help` describes it with its result fields.
Gives an element the engine cannot explain a closed word of its own instead of answering Placer Block, and makes the actions answer list the verb word `layout` for every node.
Moves the verb counts from 14 to 15 in the schema, the MCP tool listing and the docs that state the verb list, and re-reads the twelve cross-OS answer digests (snapshot, lint, actions) on linux, windows and macOS.
Claims capability v030-11 «Layout explained» and retires the architecture decision that kept the layout read in-process only; adds no new refusal cause, session error, exit status or node key.
@task_file n=1 path=packages/escher-driver/src/layout.rs mark=edits
what: the driver's layout read, the session's layout explanation of one element — edits: `unexplained()`, the mapping of an element the engine cannot explain
@task_file n=2 path=docs/capabilities.md mark=edits
what: the capability statement, the public list of the driver's verbs and its verb count, updated by hand
@task_file n=3 path=README.md mark=edits
what: the project README, its tool count of the driver's verbs
@task_file n=4 path=CLAUDE.md mark=edits
what: the agent instructions that state the driver's verb list — edits: the «fourteen verbs» count
@task_file n=5 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the capability statement check that holds README's tool count against the verb list
@task_item n=1 part=needs
The driver's command span for the new verb: it records `verb` = `layout` and, when refused, `cause` alone — no id, no reading, no count of the answer; the span's existing field count and closed-span line stay as they are.
@task_item n=2 part=needs
The MCP tool_call span for the new `layout` tool: it holds `tool` and `ending` (accepted, refused, error) as for every tool, with the command span as its child.
@task_item n=3 part=needs
The layout read opens no span of its own and the driver emits no event and installs no subscriber: the verb is seen only through the one command span per driver command.
@task_item n=4 part=needs
The hosted call runs through the session's run alone: the new verb's executor arm goes through that run, with no step, pump, tick, settle or load wait, and leaves the instance unchanged.
@task_item n=5 part=needs
The layout answer is never logged, never put in a span field or a debug print: like a part's answer, the graph answer and the actions answer, it leaves only in its answer line.
@task_item n=6 part=needs
Answers echo nothing the caller typed: the layout answer carries stable ids, closed words and numbers, never a node id, page text, authored style value, colour or control value.
@task_item n=7 part=needs
Refusal cause names for the new verb: an id the screen does not read answers `stale` or `not-found`, a malformed or missing id answers the existing validation cause (`malformed`); no new cause, session error, exit status or node key.
@task_item n=8 part=needs
The unexplained element's answer: an element the engine reads no layout reason for gets a closed word of its own instead of Placer Block, before the layout answer crosses any carrier.
@task_item n=9 part=needs
The actions answer gains the verb word `layout` for every node it lists, with no cause; the actions read still changes nothing in the document and its answer shape stays closed.
@task_item n=10 part=needs
escher-session stdout and the command answer: the command-line reader `layout --id {id} --session {dir}` writes one answer line in the JSON written form; diagnostics stay fixed strings and the exit codes 0 to 3 keep their meaning.
@task_item n=11 part=needs
The escher-session mcp role: the tool listing gains `layout` after `actions` and before `help`, its stdout stays JSON-RPC lines only, and its exit statuses are unchanged.
@task_item n=12 part=needs
The kept check file and the recording: a recorded `layout` step keeps its arguments, its answer whole and the answer digest; replay takes `tool` = `layout` among the instance verbs; the recording stays within its bounds and in its private directory.
@task_item n=13 part=needs
The `help` command describes the new verb from the tables with its result fields; the help tool_call span is unchanged.
@task_item n=14 part=needs
The host wire stays v3: the socket's call carries the new verb inside it with no wire change.
@task_item n=15 part=needs
The capability statement is updated by hand: the verb list and the count move from fourteen to fifteen in the public docs, and the capability check reads them back.
@task_item n=16 part=needs
The twelve cross-OS answer digests are re-read on linux x86_64, windows x86_64 and macOS aarch64: the actions digests move with the new verb word, the snapshot and lint digests do not.
@task_item n=17 part=needs
Committed kept checks hold no private data: any kept check the chunk adds for the layout step carries no secret, page text or local state.
@task_standing n=1
convention: every new check is silent on success and shares its helpers through the shared modules
home: 3-observability-harness-contract
@task_standing n=2
convention: every check that asserts an empty stderr unsets RUST_LOG for its child
home: 3-observability-harness-contract
@task_standing n=3
convention: every log check that fails names a kind and a count, never the content it read
home: 3-observability-harness-contract
@task_standing n=4
convention: every committed kept check or fixture holds no secret, page content or local state
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
Metric coverage: metric emission, exporters and the in-process timings and counters; the chunk adds no counter, gauge or histogram.
@task_item n=2 part=not_needed
Error capture and reporting in the engine: the window error event, window.onerror, script error source labels, the script diagnostics drain and the panic hooks; the chunk touches no script error path.
@task_item n=3 part=not_needed
WPT reports and WPT score publishing; the chunk does not touch the WPT runner.
@task_item n=4 part=not_needed
The agent-run harness events and log; the chunk adds no agent-run event.
@task_item n=5 part=not_needed
The cold-agent pipe: its events, its verdict fields and reasons, its counted verbs and its transcript; counting the layout read for a cold agent is out of this chunk.
@task_item n=6 part=not_needed
The telemetry sink's installation and resource keys, the OTel export absence and the telemetry backend absence; the chunk installs no sink and adds no resource key.
@task_item n=7 part=not_needed
Engine tracing behind its cargo feature and the engine log sites; the chunk changes no engine tracing.
@task_item n=8 part=not_needed
Form-control privacy rows: the select chosen set, select list labels, optgroup labels, multiple select form entries and page script change events; the layout answer carries no control value.
@task_item n=9 part=not_needed
CI package-install diagnostics and the CI log and coverage artifact uploads; the chunk adds no CI job.
@task_item n=10 part=not_needed
The session lifecycle's idle expiry and the session state directory; the new verb changes no session lifetime.
