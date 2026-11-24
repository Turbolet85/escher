@task_prose part=chunk
Builds a read of one canvas element's content as data: the draw commands a widget's paint records into its scene (fill, stroke, glyph run, push layer, push clip layer, push mask layer, pop layer, box shadow), named by stable element id, on the held instance, running no step and changing nothing.
The read has a summary level (a count per command kind and the canvas's size, or a closed digest) and a full level (one line or object per command in paint order, layers as nesting).
It is deterministic and host-independent (same bytes across reads, processes and the linux, windows and macOS CI hosts, numbers at a stated rounding) and diffable (a step that changes nothing reads an empty diff).
A driver verb carries it, a sixteenth verb or an argument of an existing one, on the carriers the founder rules; the dump's size on Circle Drawer's canvas is measured at each level against the snapshot's 10,000-byte budget.
Every fork (what crosses, notably authored text in glyph runs) is written to the founder's forks evidence and the phase stops, no default taken.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: the paint path that builds widget scenes inside the scene paint and drops them after it — edits: paint_scene, build_custom_widget_scenes, process_custom_widget_node
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs
what: the Widget trait, whose paint returns a recorded scene
@task_file n=3 path=packages/blitz-test-harness/src/ mark=edits
what: the test harness's reads of a document, beside its screenshot — edits: a read of one canvas's scene
@task_file n=4 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's verb table and argument schema — edits: a new verb or an argument of an existing one
@task_file n=5 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver's verb specs — edits: the new verb's or argument's spec
@task_file n=6 path=packages/escher-driver/src/command.rs mark=edits
what: the driver command — edits: the new verb or argument
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's command execution, under the one command span — edits: the canvas read's execution
@task_file n=8 path=packages/escher-driver/src/json.rs mark=edits
what: the driver command answer's writer — edits: the canvas read's result shape
@task_file n=9 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's help text — edits: the new verb's or argument's help
@task_file n=10 path=packages/escher-driver/src/mcp.rs mark=edits
what: the driver's MCP tools and tool_call span — edits: the MCP tool for the canvas read
@task_file n=11 path=packages/escher-driver/src/recording.rs mark=edits
what: the session recording and its kept check files — edits: the kept check of the canvas read
@task_file n=12 path=packages/escher-driver/src/replay.rs mark=edits
what: the replay of kept checks — edits: replay of the canvas read's kept check
@task_file n=13 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: Circle Drawer, the stand's one canvas widget
@task_file n=14 path=examples/seven_guis/src/stand.rs
what: the seven_guis stand that serves the tasks
@task_file n=15 path=tests/blitz-tests/tests/ mark=edits
what: the integration checks — edits: the canvas read's checks (determinism, diff, size)
@task_file n=16 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared tables and shared test module — edits: the stand's shared tables
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement — edits: the new verb's entry
@task_file n=18 path=README.md mark=edits
what: the README — edits: the tool count
@task_file n=19 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the capability statement check that the README's tool count matches
@task_item n=1 part=needs
The new driver verb (or argument) and its answer: a new canvas command answer shape, its levels, written by the driver's answer writer and carried on the escher-session stdout and the MCP stdout.
@task_item n=2 part=needs
The driver's command span: the new verb runs under the one span per driver command, its verb field gains the new verb's name, and its field set and field count stay closed.
@task_item n=3 part=needs
The MCP tool_call span over the new tool: its tool field names the new tool and its ending (accepted, refused, error) follows the existing rule, with the command span as its child.
@task_item n=4 part=needs
The dump's content never enters a span field, a log line or an event: the canvas answer, like the graph, layout and actions answers, leaves only in its answer; driver spans carry no call content and the driver emits no event and installs no subscriber.
@task_item n=5 part=needs
What crosses in the dump: glyph runs carry glyph ids and font data that stand for the authored text, so the text-bearing part of the answer is content under the scrub and no-page-content rules; what may reach a model provider is the founder's fork.
@task_item n=6 part=needs
Font data and image data in a command cross only by reference (a digest or an index), never inline, bounding the answer's size and keeping it host-independent.
@task_item n=7 part=needs
The read changes nothing in the document and runs no step on the held instance, as the actions read does.
@task_item n=8 part=needs
Refusal causes the new verb answers with: an unknown or stale element id (not-found, stale), a malformed argument, an element that is no canvas — named from the closed refusal cause set or as a new cause the set must gain.
@task_item n=9 part=needs
The kept check file for the new verb: its arguments, answer and answer digest written and replayed deterministically, with no private data in a committed check.
@task_item n=10 part=needs
The recording file: the canvas read's answer is recorded within the recording bounds in the private recording directory.
@task_item n=11 part=needs
escher-session's help and the capability statement name the new verb, the statement updated by hand and the README's tool count kept true by its check.
@task_item n=12 part=needs
Determinism of the dump across the linux x86_64, windows x86_64 and macOS aarch64 CI hosts, numbers at a stated rounding, glyph positions measured not assumed; the canvas cross-OS shot digest check stays a watch.
@task_item n=13 part=needs
Size measured: the dump of Circle Drawer's canvas at boot and after circles are added, at each level, stated in bytes against the snapshot's 10,000-byte budget.
@task_item n=14 part=needs
Diffable: two reads before and after a step compare line by line, and a step that changes nothing reads an empty diff.
@task_item n=15 part=needs
The founder's fork evidence holds no raw log carrying a temp or home path; a probe worktree stands outside the build directory and no plan step deletes it, so no build-directory sweep reaches it.
@task_item n=16 part=needs
Circle Drawer is not served by escher-session, so the verb reaches no stand canvas on the command line, MCP or a kept check until it is served or a fixture serves one; the carrier fork depends on it.
@task_standing n=1
convention: each new check is silent and its failures name a kind and a count, never content; empty-stderr checks unset RUST_LOG
home: 3-observability-harness-contract
@task_standing n=2
convention: each new committed kept check or fixture holds no secrets and no private data
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: each new test that runs a headless stand installs no sink and logs no answer
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
Metric emission, metrics exporters and the in-process timing counters: the chunk adds no counter, gauge or histogram.
@task_item n=2 part=not_needed
Telemetry backends and OTel export, Sentry or an error-reporting client: none is added.
@task_item n=3 part=not_needed
Script error reporting: window error events, window.onerror, script error source labels, the script diagnostics drain and the WPT report.
@task_item n=4 part=not_needed
Engine tracing behind its cargo feature and the escher-telemetry startup and panic events in native mains: the chunk installs no sink and changes no main.
@task_item n=5 part=not_needed
Form-control rows: select chosen sets, select list labels, optgroup labels, multiple select form entries and page script change events.
@task_item n=6 part=not_needed
The agent-run harness events and its raw log: the chunk adds no agent-run harness event.
@task_item n=7 part=not_needed
WPT score publishing upstream and the package-install diagnostic line in CI.
@task_item n=8 part=not_needed
The session lifecycle's idle expiry and the host wire version: the read is one more verb on the held session.
