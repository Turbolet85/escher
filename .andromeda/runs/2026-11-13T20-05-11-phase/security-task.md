@task_prose part=chunk
A behaviour-preserving restructure of three functions toward the cognitive-complexity ceiling of 15: blitz-dom's click handler, escher-driver's executor and dioxus-native-dom's UI graph builder.
Logic is pulled out into helpers that stay in the file that holds each function today: no file split, no new module, and no new helper reads over 15.
Behaviour stays the same for every document and every driver call: dispatch order, event sequences, default actions, refusal order, outcomes, span fields, snapshot and graph content are all unchanged.
Readings are taken before and after on the dev host. The existing click, executor and graph checks are re-run. Where a moved arm has no check, a check for it is added before the move.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: blitz-dom pointer event handling: the click default actions (checkbox and radio input and change, button focus and submit, select arms, links, summary, labels) — edits: the handle_click function, plus helpers extracted beside it
@task_file n=2 path=packages/escher-driver/src/execute.rs mark=edits
what: escher-driver's executor: per-verb dispatch, the target check and the fixed refusal order — edits: the execute function, plus helpers extracted beside it
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: dioxus-native-dom's UI graph answer: the nodes, states and declared writes the graph verb returns — edits: the DioxusDocument graph function, plus helpers extracted beside it
@task_item n=1 part=needs
Click default actions on checkbox and radio: the engine fires input then change, in the same order as today.
@task_item n=2 part=needs
A button click focuses the button and submits its form. Enter and Space activation of the focused control end in the same click default actions.
@task_item n=3 part=needs
Select arms of the click handler. Which select opens a list, what a list row takes, what closes the list, multiple-select clicks, the single write path for a select choice, and disabled-option and optgroup rules all keep today's behaviour.
@task_item n=4 part=needs
Link activation from a click. External navigation stays limited to the safe schemes and methods. File URLs load only for file documents. Form submission keeps its scheme and method rule.
@task_item n=5 part=needs
Click handling over inert subtrees, disabled controls and label activation stays as it is.
@task_item n=6 part=needs
The executor's per-verb dispatch over the closed driver verb schema: no verb, argument kind, flag or field is added.
@task_item n=7 part=needs
The executor's shared target check and the fixed order of target refusals: unknown, stale, not-found, disabled, off-screen and covered keep their order and their causes.
@task_item n=8 part=needs
The closed refusal cause set and the refusal JSON fields stay unchanged. A refusal still holds nothing a call supplied, and only the cause name is logged.
@task_item n=9 part=needs
The driver command span: one span per call, its fields (verb, cause, settled, passes, busy, added, removed, changed) and its content-free rule stay unchanged.
@task_item n=10 part=needs
Driver outcomes and caller input are never logged. The restructured executor adds no log line, log field or span.
@task_item n=11 part=needs
No panic on call-supplied input and no NodeId kept between calls. A caller-supplied id is still looked up anew and refused as stale or not-found.
@task_item n=12 part=needs
The actions read changes nothing and shares its target check with the verbs. The actions answer keeps its fields (id, owner, verbs, keyed, nodes).
@task_item n=13 part=needs
The graph answer crossing: nodes and states, the stable element id as the only id, closed state values, and the declared writes of the signal registry (event, target) all stay byte-identical.
@task_item n=14 part=needs
Dispatch is recorded only where a signal registry stands. The reached-handler record keeps no log and no NodeId, stays deduplicated, and survives element removal.
@task_item n=15 part=needs
Values crossing out of the touched code stay masked. Password and file input values keep the masked stand-in, and control states cross only as closed values.
@task_item n=16 part=needs
The stable element id: its derivation, stability, keyed-list segment and unresolved-node form are unchanged by the graph restructure.
@task_standing n=1
convention: A check added for a click arm with no existing coverage takes no new dependency or git dependency, inherits workspace dependency versions, and builds locked.
home: dependency-security
@task_item n=1 part=not_needed
The authentication-authorization area: CI token scopes, publish job guards and the cookie-store feature. The chunk touches no workflow and no auth surface.
@task_item n=2 part=not_needed
The secret-management area: CI secrets, signing material, RUST_LOG as the only environment read, and live-run logins.
@task_item n=3 part=not_needed
Data-protection rows on history in sqlite, the HTTP cache, cookies, TLS, clipboard, the native file dialog, and history database encryption.
@task_item n=4 part=not_needed
Outbound network rows: outbound HTTP through blitz-net or reqwest, per-host concurrency, sub-resource GETs, script fetchers, and the CORS, same-origin, request-timeout and response-size absences.
@task_item n=5 part=not_needed
The MCP server: stdio transport, bounded request lines and results, protocol error codes, initialize revision, the tool_call span, and idle expiry.
@task_item n=6 part=not_needed
The session host and socket: the closed request grammar, socket bounds and connections, the session label, state and recording directories, session error table, and escher-session flags, roles and exit codes.
@task_item n=7 part=not_needed
Recorded and kept check files: their format, owner-only storage, lifecycle, strict untrusted reading, and the coverage answer line.
@task_item n=8 part=not_needed
Cold-agent, agent-run, apt-install, coverage-floor and WPT scripts and workflows, with their argument validation, exit codes and metadata-only logs.
@task_item n=9 part=not_needed
CSS and HTML parsing, the CSSOM script API, @import and iframe nesting limits, font-face sniffing, stylesheet loading, and serialization escaping.
@task_item n=10 part=not_needed
Network tracing and resource-load failure log contracts, the JS console route, and env_logger for the WPT runner.
