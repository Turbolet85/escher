@task_prose part=chunk
Errors with the action (v030-10): an acting driver call (click, type, press, advance, scroll) whose step causes a handler panic, an error-level log record or a handler's own log record returns it in that call's answer, beside the diff it returns today.
Each returned item is tied to the stable element id the step acted on and, through the UI graph, to the handler (owner id and event name) that ran — the graph's existing node identity, never a raw node handle.
A handler panic during a step on a held session is caught on the run path and returned as the call's outcome rather than ending the host process; the records a step caused are captured bounded per call.
The same answer field rides every ratified carrier of the acting verbs — in process, the session socket's call, the command line, MCP and a kept check recorded and replayed — and the driver's help and the MCP tool listing describe it.
What of a log may enter an answer is the founder's boundary fork; the telemetry scrub and allowlist are held as they are, and the command and tool_call spans gain no field.
@task_file n=1 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's run path for acting calls — edits: the step's run, its panic catch and the capture of what the step caused
@task_file n=2 path=packages/escher-driver/src/session.rs mark=edits
what: the driver session, its held state and the acting call result — edits: Session::run and the acting call's answer
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema — the verb table, the answer's fields and the refusal causes — edits: the acting answer's new field
@task_file n=4 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description, the help answer — edits: the description of the new answer field
@task_file n=5 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's JSON answers, the written form of an answer — edits: the new answer field's written form
@task_file n=6 path=packages/escher-driver/src/mcp.rs mark=edits
what: the escher-session MCP role, its tools/list and tool results — edits: the tool listing's description of the new field
@task_file n=7 path=packages/escher-driver/src/recording.rs mark=edits
what: the session recording, the kept check's record fields — edits: the recorded answer's new field
@task_file n=8 path=packages/escher-driver/src/replay.rs mark=edits
what: check replay through the ordinary call path, the kept check read back — edits: the replayed answer's new field
@task_file n=9 path=packages/escher-telemetry/src/lib.rs
what: the telemetry subscriber, its target allowlist and sink, read for where a step's records can be captured without changing what the sink prints
@task_file n=10 path=packages/escher-telemetry/src/format.rs
what: the telemetry line format and its scrub of named fields to redacted, held as it is
@task_file n=11 path=packages/escher-telemetry/src/panic.rs
what: the telemetry chaining panic hook
@task_file n=12 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph — its node identity, the owner id and event name of each handler, listeners read from the vdom walk
@task_file n=13 path=packages/dioxus-native-dom/src/reached.rs
what: the reached record of handlers a dispatch ran
@task_file n=14 path=examples/seven_guis
what: the seven_guis headless stand, beside which a minimal fixture app whose handler panics, errors and logs is added
@task_file n=15 path=tests/blitz-tests/tests/
what: the blitz-tests integration test targets — a new target for this behaviour, run in both layout modes
@task_item n=1 part=needs
The acting call result of the driver session: its answer gains a field carrying the panic, error-level record or handler log a step caused, beside the existing diff, each item tied to the acted element id and to the handler's owner id and event name.
@task_item n=2 part=needs
A handler panic during an acting step on a held session is caught on the run path and returned as the call's outcome instead of ending the host process; what the instance is after a caught panic (poisoned, rebooted or held) is a behaviour the chunk sets and its checks must state.
@task_item n=3 part=needs
The acting verbs as a set — click, type, press, advance, scroll — and their run validation and settle: each carries the new field, and the field is empty where the step caused nothing.
@task_item n=4 part=needs
The capture of a step's records is bounded per call; the answer stays within the session wire's answer bound, and an over-bound answer meets the existing oversize and answer-too-large handling, never a new error.
@task_item n=5 part=needs
The UI graph's handler identity (owner key id form, owner id and event name) and the reached record are what tie an item to its handler; no raw node handle enters an answer.
@task_item n=6 part=needs
Run carriers: the session socket's call and its graph and acting answers, the command line's actions verb and its output, the MCP role's tool results and the in-process run all carry the same new field with the same content.
@task_item n=7 part=needs
The driver's help and the MCP tools/list describe the new field, and the help on the command line and over MCP stay at parity; the capability statement pinned to the driver schema is held to the schema's new field.
@task_item n=8 part=needs
The kept check: the session recording's record fields and replay through the ordinary call path carry the new field (or hold it out of the digest, a P4 point), and committed recorded checks stay byte-equal to the binary's recording and kept fresh.
@task_item n=9 part=needs
Scrub held: the telemetry target allowlist and scrub of named fields stay as they are, and a step's capture changes nothing of what the sink prints; the sink still drops third-party targets.
@task_item n=10 part=needs
No log line carries the new answer content: the driver's command span and the tool_call span gain no field, the driver's per-call log line keeps fixed words and counts only, and host stderr carries no screen content at any level.
@task_item n=11 part=needs
Answers and logs hold no value: an actions or graph answer holds no typed value and a typed password is never returned; any authored text (panic payload, log message, a log field's value) entering an answer is the founder's boundary fork, bounded by his content-free log rulings.
@task_item n=12 part=needs
No new carrier, listener, socket, verb, exit status or session error: the refusal causes, their order, the session error set and the exit statuses stay closed; a new refusal cause or result field enters only as the schema tables admit.
@task_item n=13 part=needs
No panic path on a driver call's arguments: a refused call changes nothing and echoes no input; a panic caught is the app handler's, never the driver's.
@task_item n=14 part=needs
Exploration keeps its contract: its per-path panic ending carries no payload, and exploration checks in the recording are not widened; the layout read and Session::reach keep theirs.
@task_item n=15 part=needs
A minimal fixture app beside the seven_guis stand gives a handler that panics, one that returns an error and one that logs, since no stand task does so today; it boots offline on the bundled font.
@task_item n=16 part=needs
The new behaviour's proof is a new blitz-tests integration target exercising the in-process run and the session socket, its socket checks owning their host, unix-gated, with host pipes drained and log silence read on a host with a sink.
@task_item n=17 part=needs
Test failures and agent-run output print no argument value and no panic text: the new tests' assertions must not print the captured payloads they compare.
@task_standing n=1
convention: one integration test target per behaviour, one blitz-tests target per file, shared test modules being no target, and every stand check runs in both layout modes
home: test-scope-summary · 1-test-scope-summary
@task_standing n=2
convention: a session check owns its host process, held behind a guard, its child streams to null or drained, and socket checks are unix-gated
home: test-strategy · 2-test-strategy
@task_standing n=3
convention: test data is code — fixtures stated once in one shared module, nothing seeded, a fixture app read by a human carrying no automated assertion of its own
home: 3-test-harness-contract · test-data-fixtures
@task_standing n=4
convention: no sleep or wall-clock read in checks; time moves only by the virtual clock
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=5
convention: new tests are controlled red-before-green, table-driven tests assert their row count, and failures print no argument value
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=6
convention: stand checks run on the bundled real font with no skip and hold a zero-failure budget over the repeated rounds
home: quality-gates-coverage-targets · 10-quality-gates-coverage-targets
@task_standing n=7
convention: hand-written stand-ins only, no mocking library
home: mocking-stubbing-discipline · 8-mocking-stubbing-discipline
@task_item n=1 part=not_needed
WPT conformance — the runner's command line, its outputs, the status set, the pinned WPT commit and the Thai-font reftests: the chunk adds no conformance test and changes no layout or paint.
@task_item n=2 part=not_needed
The cold-agent pipe — its events, verdict, exit codes, tasks, isolation and wrong-call bar: the chunk runs no cold agent and changes nothing of its pipe.
@task_item n=3 part=not_needed
The agent-run verbs, selections, events, state files and exit codes: the chunk adds no verb or selection to that contract.
@task_item n=4 part=not_needed
Harness contrast and lint defects — the contrast formula, the required ratio and the lint finding kinds and order: no reading of contrast or lint is touched.
@task_item n=5 part=not_needed
Paint rows — select list painting, scrollbar thumbs and their fade, box-shadow expectations, pixel tests on a CPU buffer and the screenshot's in-band pixels: the chunk paints nothing new.
@task_item n=6 part=not_needed
CI supply rows — actions pinned to commit SHAs, CI without secrets and read-only permissions, bounded and retried package installs, the apt-install script's exit codes and the upstream-only publishing guard: the chunk adds no CI job.
@task_item n=7 part=not_needed
Mutation tooling rows — where cargo-mutants runs, its nextest profile and the baseline read: the chunk changes none of the mutation run's setup.
@task_item n=8 part=not_needed
Handler coverage limits and the coverage line — bubbling counts, canvas handlers, unreached screens: the chunk changes no coverage counting.
@task_item n=9 part=not_needed
Network stand-ins, the load page fixed answer, the failed-load end of the settle wait and file URL loading: the chunk's step capture touches no load.
@task_item n=10 part=not_needed
Text input specifics — password caret placement by the mask, chosen select row colours, the open pseudo-class, a key after a dropped anonymous block: no editing or form behaviour changes.
