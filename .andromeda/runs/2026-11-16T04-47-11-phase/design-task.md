@task_prose part=chunk
An acting driver verb (click, type, press, advance, scroll) whose step causes a handler panic, an error-level log record or a handler's own log record returns it in that call's answer, beside the diff it returns today.
Each returned item is tied to the stable element id the step acted on and, through the UI graph, to the handler (owner id and event name) that ran.
A handler panic during a step on a held session is caught on the run path and returned as the call's outcome instead of ending the host process.
The new answer field rides every ratified carrier of the acting verbs: in process, the session socket's call, the command line, MCP and the kept check (recorded and replayed); help and the MCP tool listing describe it.
What authored text of a log or panic may enter an answer is the founder's boundary fork; the telemetry scrub and allowlist stay as they are.
@task_file n=1 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's run path, where an acting verb's step runs and its answer is built — edits: the run path's panic catch and the step's capture
@task_file n=2 path=packages/escher-driver/src/session.rs mark=edits
what: the held session and its run of a step, the session socket's call — edits: Session::run
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's answer schema, its tables of answer fields, refusal causes and exit statuses — edits: the acting verbs' answer field
@task_file n=4 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description, the help answer — edits: the description of the new answer field
@task_file n=5 path=packages/escher-driver/src/json.rs mark=edits
what: the answer's written form on the command line and the socket — edits: the new answer field's written form
@task_file n=6 path=packages/escher-driver/src/mcp.rs mark=edits
what: the MCP role, its tool listing and tool results — edits: the tool listing's description and the acting tools' results
@task_file n=7 path=packages/escher-driver/src/recording.rs mark=edits
what: the kept check's recording of an action and its answer — edits: the recorded answer
@task_file n=8 path=packages/escher-driver/src/replay.rs mark=edits
what: the kept check's replay and its comparison of answers — edits: the replayed answer's comparison
@task_file n=9 path=packages/escher-telemetry/src/lib.rs
what: the telemetry subscriber, its escher_* target allowlist, read for where a step's records can be captured without changing what the sink prints
@task_file n=10 path=packages/escher-telemetry/src/format.rs
what: the telemetry line format and its scrub of url, href, src, html, text, value, attrs, path, request and error as redacted
@task_file n=11 path=packages/escher-telemetry/src/panic.rs
what: the chaining panic hook of the telemetry
@task_file n=12 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph, its node identity and its handler identity (owner id and event name)
@task_file n=13 path=packages/dioxus-native-dom/src/reached.rs
what: the reached record of a step, what node and handler an action reached
@task_item n=1 part=needs
The acting verbs click, type, press and advance (and scroll): their answer gains a field carrying the panic, error or handler log the step caused, beside the diff, and how each verb's step is settled and answered on the virtual clock stays as it is.
@task_item n=2 part=needs
A handler panic in a step is caught on the run path and becomes the call's outcome rather than ending the host process; what the session instance is after a caught panic (held, poisoned or rebooted) is a new state the answer and later calls must reflect.
@task_item n=3 part=needs
Attribution: each item in the answer is tied to the stable element id the step acted on and, through the UI graph's existing node identity, to the handler's owner id and event name — the graph's node identity, never an internal node id; the UI graph line's content and the actions line's nodes are the identities it reuses.
@task_item n=4 part=needs
The capture of a step's records is bounded per call, so an answer's new field has a bound on how many items and how much it holds.
@task_item n=5 part=needs
The escher-session commands and their answers: the acting commands' answers carry the new field on the command line and through the socket's call; the help answer describes it.
@task_item n=6 part=needs
The MCP role: the acting tools' results carry the new field, the tool listing and the help tool result describe it, and the MCP stdout stays one result line per call.
@task_item n=7 part=needs
The kept check, recorded and replayed: the new field joins the kept answer or is held out of its digest, and a replay's comparison of answers must treat it accordingly.
@task_item n=8 part=needs
Boundary rule: any authored text (a panic payload, a log message, a log field's value) crossing onto a carrier is a boundary widening and is the founder's fork; a new field of a closed value set inside a ratified carrier is not a widening, so the field's content is either a closed set or awaits his word.
@task_item n=9 part=needs
Indirect, through the telemetry: the target allowlist and scrub hold unchanged, the driver's command span and the MCP tool_call span gain no field holding what an answer now carries, and no log line carries the new answer content; the founder's content-free log rulings bound what a log may yield.
@task_item n=10 part=needs
No new carrier, listener, socket, port, verb, exit status or session error; a new refusal cause or result field enters only as the schema's tables admit, and exit statuses stay read from the schema's table.
@task_item n=11 part=needs
No panic path on a driver call's arguments: the panic caught here is the app's handler's, never the driver's own.
@task_item n=12 part=needs
The driver's output stays read by an agent, not a person: the new field's written form is plain, uncoloured and machine-read on the command line and MCP.
@task_item n=13 part=needs
The layout read, explore and Session::reach keep their contracts; explore's per-path panic ending is not widened to carry a payload.
@task_item n=14 part=needs
The proof: a minimal fixture app beside the seven_guis stand whose handlers panic, return an error and log, since no stand task does; a new blitz test target proving the behaviour in both layout modes.
@task_standing n=1
convention: each new test of the driver runs against a minimal fixture where the stand's tasks lack a case, built with system fonts, and runs in both layout modes
home: typography · surface
@task_standing n=2
convention: each new answer field of the driver is listed in the schema's table, described by help and the MCP tool listing, and written the same on every ratified carrier
home: surface · surface-cli
@task_standing n=3
convention: each new log line or span field stays content-free under the telemetry scrub and carries no screen content
home: surface
@task_item n=1 part=not_needed
Color palette rows (engine colours, color-scheme recascade, select list colours, base color): the chunk paints nothing and changes no colour.
@task_item n=2 part=not_needed
Spacing and depth strategy rows (margins, table cell hints, z-index paint levels, hit-testing at paint clips, stacking contexts): no layout, paint or hit-test order changes.
@task_item n=3 part=not_needed
Border radius and iconography rows (cursor icons, cursor resolution, favicon): untouched.
@task_item n=4 part=not_needed
The select list's rows (its events, its accessibility tree, its screenshot and snapshot rows, its open gestures, its overlay and motion): the chunk changes no select behaviour.
@task_item n=5 part=not_needed
The screenshot and snapshot verbs and their refusals: they are reads, not acting verbs, and gain no new field.
@task_item n=6 part=not_needed
The web-spa surface and WASM font context: the chunk works on the native driver session only.
@task_item n=7 part=not_needed
WPT runner output and result colours: the WPT runner is not reached.
@task_item n=8 part=not_needed
Shell provider rows (maximize, minimize, decorations, window drag, cursor) and the JS window object's sizes: no window or script runtime change.
@task_item n=9 part=not_needed
Motion rows on smooth scrolling, overlay scrollbar fades and animation on the virtual clock: no timing or scroll animation changes.
@task_item n=10 part=not_needed
The coverage line and coverage check files, and the escher-session tasks of the stand (counter, crud, timer, cells, flight-booker, load): no coverage or stand task changes.
