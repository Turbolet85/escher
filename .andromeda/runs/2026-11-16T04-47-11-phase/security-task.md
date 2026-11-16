@task_prose part=chunk
An acting driver call (click, type, press, advance, scroll) whose step causes a handler panic, an error-level log record or a handler's own log record returns it in that call's answer, beside the diff, each item tied to the stable element id acted on and, through the UI graph, to the handler (owner id and event name).
A handler panic during a step on a held session is caught on the run path and returned as the call's outcome instead of ending the host process.
The records a step causes are captured bounded per call and attributed to the step's target and handler, without changing what the telemetry sink prints.
The new answer field rides every ratified carrier of the acting verbs (in process, session socket call, command line, MCP, kept check recorded and replayed), and the driver's help and MCP tool listing describe it.
What of a log may enter an answer is the founder's boundary fork, bounded by his log rulings; the scrub, the command span and the tool_call span stay as they are.
@task_file n=1 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's run path for the acting verbs — edits: the step run, where a handler panic is caught and a step's caused records join the answer
@task_file n=2 path=packages/escher-driver/src/session.rs mark=edits
what: the held session and its run of a call (Session::run) — edits: the run of a step on a held session and the answer it returns
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the closed driver verb schema and the answer's fields, the refusal causes and result fields — edits: the acting verbs' answer, the new field
@task_file n=4 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description (help) — edits: the description of the acting verbs' answer
@task_file n=5 path=packages/escher-driver/src/json.rs mark=edits
what: the answer's written form on the session socket and command line — edits: the written form of the new answer field
@task_file n=6 path=packages/escher-driver/src/mcp.rs mark=edits
what: the MCP server over stdio, its tool listing and tool results — edits: the tool listing and the acting tools' result
@task_file n=7 path=packages/escher-driver/src/recording.rs mark=edits
what: the recorder of kept checks, the recorded check file — edits: the recorded answer of an acting step
@task_file n=8 path=packages/escher-driver/src/replay.rs mark=edits
what: the replay of a kept check through the session, its strict reader of recorded checks — edits: the compare of a replayed acting answer
@task_file n=9 path=packages/escher-telemetry/src/lib.rs
what: escher's own stderr telemetry sink, the subscriber with its target allowlist, read for where a step's records can be captured
@task_file n=10 path=packages/escher-telemetry/src/format.rs
what: the sink's line format and the scrub of content-named fields on escher_ targets, held unchanged
@task_file n=11 path=packages/escher-telemetry/src/panic.rs
what: the chaining panic hook of the telemetry sink
@task_file n=12 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph and its handler identity (owner id and event name), the signal registry
@task_file n=13 path=packages/dioxus-native-dom/src/reached.rs
what: the reached-handler record, deduplicated, without log or NodeId
@task_item n=1 part=needs
The acting verbs' answer gains a field carrying the panic, error-level log record or handler log record a step caused, beside the diff; each item is tied to the stable element id the step acted on and to the handler through the UI graph's existing node identity, never a NodeId, and no process-local handle crosses with an id.
@task_item n=2 part=needs
A handler panic during a step on a held session is caught on the run path and returned as the call's outcome rather than ending the host process; the driver itself still has no panic path on a call's arguments, and the panic caught is the app handler's.
@task_item n=3 part=needs
The capture of a step's records: bounded per call, attributed to the step's target and handler, built through the tracing subscriber, the reached-handler record or the Dioxus handler result; the driver installs no subscriber and reads no environment beyond RUST_LOG, and the reached record stays without log or NodeId.
@task_item n=4 part=needs
The boundary widening rule: any authored text (a panic payload, a log message, a log field's value) crossing onto a carrier is a widening and the founder's fork; a new field of a closed value set inside a ratified carrier is no widening; the ratified exits of the stable id, accessible names and attribute text bound what may cross.
@task_item n=5 part=needs
The founder's log rulings bound the field: content-free log fields, screen content kept out of logs, one sink line per closed span, and the scrub sets of escher-telemetry.
@task_item n=6 part=needs
Scrub held: the telemetry sink's target allowlist, its dropping of outside targets and its redaction of content-named fields (url, href, src, html, text, value, attrs, path, request, error) on escher_ targets stay as they are; the sink prints nothing new.
@task_item n=7 part=needs
The driver command span and the MCP tool_call span gain no field holding what the answer now carries; no log line carries the new answer content; driver outcomes are never logged and stable ids and accessible names stay out of logs.
@task_item n=8 part=needs
The session socket's call reply (ok v3 accepted and refused forms) carries the new field within its closed grammar, its request-line and oversize bounds.
@task_item n=9 part=needs
The command line's result line carries the new field within the driver command line's answer bounds; session output holds no caller input and echoes nothing typed.
@task_item n=10 part=needs
The MCP acting tools' results carry the new field within the tool result size bound, with isError and fixed protocol errors as they stand; the MCP listing stays fixed program text that describes the field.
@task_item n=11 part=needs
The kept check: the recorded acting answer and its replay through the session hold the new field as the kept check's contents, digest and strict untrusted reader admit; kept checks stay owner-only and the recording directory never named.
@task_item n=12 part=needs
The value mask holds on every carrier: a password or file input value never reaches the new field through a captured record or panic payload.
@task_item n=13 part=needs
No new carrier, listener, socket, port, verb, exit status or session error; a new refusal cause or result field only as the closed refusal cause set, the fixed session error table and the closed verb schema admit, with refusals holding nothing a call supplied and one validation path for every carrier.
@task_item n=14 part=needs
The layout read, explore and Session::reach keep their contracts; explore's per-path panic ending is not widened to carry a payload (exploration report holds closed content only), and reach errors stay a closed set with fixed texts.
@task_item n=15 part=needs
The acting verbs' answer stays unpaged and acting commands stay bounded by the load wait; settling and the virtual clock are as they are.
@task_item n=16 part=needs
The proof: a minimal fixture app beside the seven_guis stand whose handlers panic, return an error and log, and a new blitz-tests target exercising the field on each carrier in both layout modes.
@task_standing n=1
convention: each crate's line coverage holds its per-crate floor; new driver and telemetry code is covered by its tests
home: input-validation
@task_standing n=2
convention: a new test target takes no tracing and no style in its harness unless its own measure needs it; dependencies stay workspace-inherited and builds locked
home: dependency-security
@task_standing n=3
convention: every new log line goes through escher's own allowlist sink, by an escher_ target, with content-named fields scrubbed
home: logging-monitoring
@task_standing n=4
convention: a new CI job or test target runs across the CI matrix with every job run
home: error-handling
@task_item n=1 part=not_needed
Authentication and authorization rows (read-only CI tokens, publish job guards, cookie store feature): the chunk touches no credential, token or publish path.
@task_item n=2 part=not_needed
Outbound network rows in API security (fetch methods, sub-resource GETs, per-host limits, CORS and same-origin absences, external navigation schemes, iframe navigation): the chunk makes no request and takes no URL.
@task_item n=3 part=not_needed
Browser data rows in data protection (history in sqlite, HTTP cache, cookies, TLS, clipboard, native file dialog): the chunk stores no browsing data.
@task_item n=4 part=not_needed
Secret management rows (CI secrets, signing material, live-run login, live evidence): the chunk adds no secret and reads no new environment variable.
@task_item n=5 part=not_needed
Engine rendering and parsing rows (CSS declarations and selectors, HTML parsing, inline SVG serialization, border radii, inline-box heights, font sources, stylesheet loads): the chunk changes no parse, style or paint.
@task_item n=6 part=not_needed
Control behaviour rows (select list keys, rows and painting, range input actions, password painting, button and Enter or Space activation, multiple select): the chunk changes no control's behaviour.
@task_item n=7 part=not_needed
WPT runner and its post-results workflow rows: the chunk adds nothing to the WPT run.
@task_item n=8 part=not_needed
Cold-agent harness rows (its prompt, call allowlist, transcripts, recordings, wrong-call bar) and the apt-install script rows: the chunk changes neither harness.
