@task_prose part=chunk
Adds a new field to the answer of every acting driver call (click, type, press, advance, scroll). When the step causes a handler panic, an error-level log record or a handler's own log record, the field carries it, beside the diff the call already returns.
Each item is tied to the stable element id the step acted on and, through the UI graph, to the handler (owner id and event name) that ran, using the graph's existing node identity.
A handler panic during a step on a held session is caught on the session's run path and returned as the call's outcome, rather than ending the host process.
The same field rides every ratified carrier (in process, the session socket call, the command line, MCP, the kept check recorded and replayed), and the driver's help and the MCP tool listing describe it.
The scrub, the target allowlist, the command and tool_call spans and the sink's printed lines stay as they are. What of a log may enter an answer is the founder's boundary fork.
@task_file n=1 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's run path for an acting command and the command answer it returns — edits: the acting verbs' step and their answer (the new field beside the diff)
@task_file n=2 path=packages/escher-driver/src/session.rs mark=edits
what: the held session and the session's run, through which a hosted call runs — edits: Session::run, which catches a handler panic during a step and returns it as the call's outcome
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver command answer's schema tables (answer fields, refusal causes) — edits: the acting answer's new field
@task_file n=4 path=packages/escher-driver/src/json.rs mark=edits
what: the command answer's written JSON form on escher-session stdout and the session socket — edits: the new answer field's written form
@task_file n=5 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description (the help command) — edits: the description of the acting verbs' answer
@task_file n=6 path=packages/escher-driver/src/mcp.rs mark=edits
what: the escher-session mcp role: its tool listing and its tool_call handling over JSON-RPC lines — edits: the MCP tool listing's description of the new field
@task_file n=7 path=packages/escher-driver/src/recording.rs mark=edits
what: session recording, which writes the kept check file (arguments, answer, answer digest) — edits: the kept answer, where the new field joins it or is held out of the digest
@task_file n=8 path=packages/escher-driver/src/replay.rs mark=edits
what: replay of a kept check, which compares the replayed answer with the kept one — edits: the comparison of the new field
@task_file n=9 path=packages/escher-telemetry/src/lib.rs
what: escher's telemetry sink: the subscriber the native mains install, its target allowlist and its RUST_LOG filter
@task_file n=10 path=packages/escher-telemetry/src/format.rs
what: escher's sink log line format: the event and closed span lines and the field scrub
@task_file n=11 path=packages/escher-telemetry/src/panic.rs
what: escher-telemetry's chaining panic hook and its panic event
@task_file n=12 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph: node identity and the handler identity (owner id and event name) behind the graph answer
@task_file n=13 path=packages/dioxus-native-dom/src/reached.rs
what: the reached record of handlers a step ran, holding ids and event names only
@task_file n=14 path=examples/seven_guis
what: the seven_guis stand app; a minimal fixture app beside it, whose handlers panic, return an error and log
@task_item n=1 part=needs
The driver command answer of the acting verbs gains a new field (panic, error or handler log, each tied to element id and handler). It is a new answer field in a ratified carrier, admitted only as the schema tables and the plan name it.
@task_item n=2 part=needs
What of a log may enter an answer is the founder's boundary fork: any authored text crossing onto a carrier (panic payload, log message, a log field's value) widens the boundary. A new field holding a closed value set does not. The chunk is bounded by the log rulings: content-free log fields, no screen content in logs, and the closed span line.
@task_item n=3 part=needs
Answer-bearing values are never logged. The new answer content appears on no log line, and the types carrying it are never logged in debug form.
@task_item n=4 part=needs
The scrub holds: the target allowlist and the content-named scrub set (url, href, src, html, text, value, attrs, path, request, error) printing redacted on escher targets stay unchanged. Scrubbing stays at the sink, not at call sites.
@task_item n=5 part=needs
The driver's command span and the MCP tool_call span gain no field: their field set and count stay closed, they carry no call content, and the tool_call ending values stay as they are.
@task_item n=6 part=needs
The driver library installs no subscriber and emits no event of its own. Any per-step capture of records must respect this, and must not change what escher's telemetry sink prints, its stream or its RUST_LOG handling.
@task_item n=7 part=needs
Handler panic capture on the session's run path: a hosted call runs through the session's run alone; a caught handler panic becomes the call's outcome. The chaining panic hook's previous-hook behaviour and its panic event stay as they are.
@task_item n=8 part=needs
Exploration's per-path panic catching keeps its contract and its closed report content: its panic ending gains no payload.
@task_item n=9 part=needs
Handler identity through the UI graph and the reached record: owner id and event name, ids and event names only, the graph's existing node identity, never an internal node id.
@task_item n=10 part=needs
No new carrier, listener, socket, port, verb, exit status, session error or host wire version. A new refusal cause only as the refusal cause names and the plan admit. escher-session exit codes, the mcp role exit statuses and the error object stay.
@task_item n=11 part=needs
The kept check file (arguments, answer, answer digest, accepted) and the recording file: whether the new field joins the kept answer and its digest; recording bounds and the private recording directory still apply to what it carries.
@task_item n=12 part=needs
The MCP role's JSON-RPC stdout and tool listing, and the escher-session help command, describe the new field.
@task_item n=13 part=needs
Answers echo nothing the caller typed, and typed text in a recording stays as ruled. The new field carries only what the app's handler caused.
@task_item n=14 part=needs
No panic path on a driver call's arguments. A panic caught here is the app handler's, never the driver's.
@task_item n=15 part=needs
The layout read keeps its contract and opens no span. The actions answer and the graph answer keep their shape, content and logging rules.
@task_item n=16 part=needs
Session logs carry no page content. Escher-session diagnostics stay fixed strings.
@task_standing n=1
convention: each new check runs silently and shares modules; a check reading stderr unsets RUST_LOG; a log check failure names a kind and a count
home: 3-observability-harness-contract
@task_standing n=2
convention: each new fixture or kept check committed holds no secrets and no private data; local state stays local
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: each new test's stand runs headless with no telemetry sink installed
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
Metric coverage: counters, gauges, histograms, exporters and the WPT score publishing. The chunk adds no metric.
@task_item n=2 part=not_needed
The cold-agent pipe: its events, verdict fields, reasons, counted verbs, transcript and server log handling. The chunk does not change the cold-agent harness.
@task_item n=3 part=not_needed
The agent-run harness events (run.start, boot, test, status, cleanup, run.end) and its raw output and log rules. The chunk adds no harness event.
@task_item n=4 part=not_needed
The page script error surface: window error event fields, window.onerror arguments, script error source labels, the embedder drain of script diagnostics, and WPT report messages. These cover page JavaScript, not Dioxus handlers.
@task_item n=5 part=not_needed
Engine document loading rules: a failed stylesheet and the pending critical set, and file loads from non-file documents.
@task_item n=6 part=not_needed
The WASM console panic hook. The chunk reaches the native session only.
@task_item n=7 part=not_needed
Form control and snapshot content rows: select chosen set and list labels, optgroup labels, multiple select form entries, page change events on checkboxes and radios, open list row bounds and shots, and snapshot clickable rows.
@task_item n=8 part=not_needed
CI platform and artifact rows: package-install diagnostic lines, per-leg log and coverage artifacts, and upstream WPT publishing.
@task_item n=9 part=not_needed
The coverage role's line and its shape, owner and logging. The chunk adds nothing to the handler coverage line.
