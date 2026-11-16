@task_prose part=chunk
Builds errors with the action (v030-10): an acting driver call (click, type, press, advance, scroll) whose step causes a handler panic, an error-level log record or a handler's own log record returns it in that call's answer, beside the diff it returns today.
Each item is tied to the stable element id the step acted on and, through the UI graph, to the handler (owner id and event name) that ran; a handler panic on a held session is caught on the run path and returned as the call's outcome instead of ending the host.
The same answer field rides every ratified carrier (in process, the session socket's call, the command line, MCP, a recorded and replayed kept check), and help and the MCP tool listing describe it.
What of a log may enter an answer is the founder's boundary fork; the telemetry scrub, the command and tool_call spans and every log line stay as they are.
@task_file n=1 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's run path for the acting verbs — edits: the step's run path, where a handler panic is caught and the step's records captured
@task_file n=2 path=packages/escher-driver/src/session.rs mark=edits
what: the held session and its run of a step (Session::run), the in-process carrier — edits: Session::run
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's answer schema tables — the answer fields, refusal causes and result fields each carrier admits — edits: the acting verbs' answer, gaining the new field
@task_file n=4 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description, the help answer — edits: the acting verbs' answer description
@task_file n=5 path=packages/escher-driver/src/json.rs mark=edits
what: the answer's written form on the command line and the session socket's call — edits: the acting verbs' answer
@task_file n=6 path=packages/escher-driver/src/mcp.rs mark=edits
what: the session's MCP server over stdio, its tools/list and tool results — edits: the tool listing and the acting tools' result
@task_file n=7 path=packages/escher-driver/src/recording.rs mark=edits
what: the session recording, the kept check written as check-{n}.json — edits: the recorded acting answer
@task_file n=8 path=packages/escher-driver/src/replay.rs mark=edits
what: the replay of a kept check against a fresh boot — edits: the replayed acting answer and its comparison
@task_file n=9 path=packages/escher-telemetry/src/lib.rs
what: escher-telemetry's subscriber, its target allowlist — read for where a step's records can be captured without changing what the sink prints
@task_file n=10 path=packages/escher-telemetry/src/format.rs
what: escher-telemetry's line format and its scrub of url, href, src, html, text, value, attrs, path, request and error to [redacted] on escher_* targets
@task_file n=11 path=packages/escher-telemetry/src/panic.rs
what: escher-telemetry's chaining panic hook
@task_file n=12 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph — its node identity and the handler identity (owner id and event name) an item is tied to
@task_file n=13 path=packages/dioxus-native-dom/src/reached.rs
what: the reached record of a step — which handler a step's event reached
@task_item n=1 part=needs
The escher-session actions answer: an acting call's answer gains one field carrying the handler panic, error-level log record or handler log record the step caused, beside the diff it returns today; a step that causes none carries it empty or absent as the schema decides.
@task_item n=2 part=needs
Each item in the new field is tied to the stable element id the step acted on, never an engine NodeId, and through the UI graph's existing node identity to the handler that ran (owner id and event name).
@task_item n=3 part=needs
A handler panic during a step on a held session is caught on the run path and returned as the call's outcome rather than ending the host process; what the held instance is after a caught panic (poisoned, rebooted or held) must stay consistent with a held instance reading as a fresh boot and with the session roles sharing the stand boot.
@task_item n=4 part=needs
The capture of what a step caused is bounded per call and attributed to the step's target and handler; the answer stays inside the MCP server's result bound.
@task_item n=5 part=needs
The same field on every carrier the acting verbs ride: in process, the session socket's call, the escher-session command line form, the MCP tool results, and the kept check recorded as check-{n}.json and replayed; whether it joins the kept answer's digest or is held out is P4's.
@task_item n=6 part=needs
The escher-session help answer and the MCP tools/list describe the new field.
@task_item n=7 part=needs
What of a log may enter an answer is the founder's boundary fork: any authored text (a panic payload, a log message, a log field's value) crossing onto a carrier is a boundary widening, while a new field of a closed value set inside a ratified carrier is not; the fork is bounded by his log rulings (content-free log fields, no screen content in logs, the closed-span line, the telemetry scrub sets) and by caller input never being written back.
@task_item n=8 part=needs
Scrub held: the telemetry target allowlist and scrub stay as they are, the capture changes nothing the sink prints, the driver's command span and the MCP tool_call span gain no field holding what an answer now carries, and no log line, the session host log included, carries the new answer content.
@task_item n=9 part=needs
No new carrier, listener, socket, port, verb, exit status or session error: the escher-session exit codes and refusals stay as they are, and a new refusal cause or result field enters only as the schema tables admit and the plan names.
@task_item n=10 part=needs
No panic path on a driver call's arguments: the panic caught is the app handler's, never the driver's own.
@task_item n=11 part=needs
The layout read, explore and Session::reach keep their contracts; explore's per-path panic ending is not widened to carry a payload (the closed content of the exploration report).
@task_item n=12 part=needs
The proof needs a minimal fixture app beside the stand, since no stand task panics or logs from a handler today; the stand's tasks and its served tasks stay as they are.
@task_item n=13 part=needs
A new blitz-tests target proves the behaviour in both layout modes, over the acting verbs on a held session and over the kept check's record and replay.
@task_standing n=1
convention: each new test target is reachable through the agent-run.sh test contract — its usage, stdout, run log, exit 2 and the Windows pass-through
home: surface-cli
@task_standing n=2
convention: each new fixture app's controls are keyed, their stable ids carried as attributes alone and their names, roles and states by attributes alone, and the fixture lints clean at boot
home: surface-desktop-native
@task_standing n=3
convention: each new headless test of a session boots on the pinned headless viewport
home: surface-desktop-native
@task_item n=1 part=not_needed
The ShellProvider window chrome controls (drag, maximize, minimize, decorations, close): the chunk touches no window chrome.
@task_item n=2 part=not_needed
The seven tasks' stable element ids (cell, circle, counter, crud, flight, load, task-card, temp, timer): the chunk adds, renames or removes no stand id; its proof's elements live in a fixture beside the stand.
@task_item n=3 part=not_needed
The stand's visual and layout decisions (accent colour, fixed document skeleton, flight type select at boot, CRUD list as one keyboard region, open list rows' own bounds, load page's network provider, visible region through clipping boxes, into-view scroll): the chunk changes no stand look, layout, hit or scroll behaviour.
@task_item n=4 part=not_needed
The cold-agent pipe and its decisions (its usage, stdout, record dir, exit 2 and 3, prompt naming no id, building nothing): the chunk runs no cold agent.
@task_item n=5 part=not_needed
The multi_doc and paint_bench command lines (arguments, environment knobs, backend, stdout): benches the chunk does not reach.
@task_item n=6 part=not_needed
The escher-session screenshot, snapshot (and its --part), lint and coverage answers: the chunk's new field rides the acting verbs alone.
@task_item n=7 part=not_needed
The arbitrary index.html templates absence: the chunk loads no document template.
@task_item n=8 part=not_needed
The mobile-native and web-spa surfaces: unmeasured, and the chunk reaches neither.
