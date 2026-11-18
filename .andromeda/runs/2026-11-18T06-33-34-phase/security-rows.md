# security-plan · the rows that bind 2026-11-18-cold-agent-bar — picked 129 of 555 (70 by every draw) · rule any · pin 2041

## threat-model-summary · ## Threat Model Summary
decision/actions-answer-crossing · Actions answer crossing — An actions answer carries what each actionable node can take now on the socket, the command line, MCP and a kept check: ids, owner ids, keyed booleans, verb-table words and closed cause words, never a control value, typed text or NodeId. · why: A boundary widening, ratified.
decision/caused-text-crossing · Caused text crossing — Each acting verb's answer carries caused on every ratified carrier when the step caused something: one item per panic, boundary error and kept record with its message, location, boundary, cause chain and fields, error as written within the text bound, every other content-named field redacted. · why: A boundary widening to the error text, ratified.
decision/cold-agent-screen-to-provider · Cold-agent runs carry the screen to the provider — A cold-agent live run carries the stand's screen, element ids, accessible names and control values in snapshot texts and diffs, and layout answers of ids, closed words, numbers, through the claude client to the model provider; the reply returns as an agent-written check the pipe parses and replays. · why: A boundary widening, ratified.
decision/graph-answer-crossing · Graph answer crossing — A graph answer carries the screen's UI graph on the socket, the command line, MCP and a kept check: ids, owner ids, component names, registered state names, handler event names, closed flag words and booleans, never a control value, typed text or NodeId. · why: A boundary widening, ratified.
decision/open-list-shot-pixels · Open select list shot carries labels as pixels — A shot of an open select list carries the page's option labels and group labels as pixels, on the socket and MCP carriers alike, the shot's fields unmoved. · why: A boundary widening, ratified. · guard: cli_commands, pinning a hosted shot equal to an in-process one
decision/root-boundary-err-crossing · Root-boundary Err crossing — A handler Err the app's root error boundary takes crosses as screen content, through Dioxus's default boundary rendering of its Display, and in caused as one error item holding its message and cause chain under the root boundary.
decision/select-list-crossing · Select list crossing limited to bounds and pixels — The select list crosses the ratified carriers only as a row's own bounds in the snapshot, the diff and a layout answer, as the layout answer's select-list placing word, and as pixels in a shot; node state holds no field of it and no expanded node key stands.
contract/actions-answer/id · The actions answer · id — The node's id.
contract/actions-answer/owner · The actions answer · owner — The node's owner id.
contract/actions-answer/verbs · The actions answer · verbs — A list of entries each holding verb, a word of the verb table, and cause, a closed cause word left out when absent; every listed node carries layout with no cause.
decision/coverage-answer-crossing · Coverage answer crossing — The coverage answer line carries ids, owner ids, event names and the checks and held counts only. · why: A boundary widening, ratified.
decision/coverage-check-files-untrusted · Kept check files read as untrusted — The coverage role reads each caller-named kept check as untrusted input through a bounded read and a strict parse; an unreadable, oversized, non-check or other-app file is refused malformed, holding nothing of the path or the file.
decision/mcp-result-crossing · MCP result crossing — An MCP result carries ids, accessible names, control values with a password's or file input's as the mask, the snapshot text and an acting call's diff to the client; what a client does onward, a model provider included, is the client's. · why: A boundary widening, ratified.
decision/mcp-results-client-responsibility · MCP results are the client's — What an MCP client does with the results it receives, screenshots and lint findings among them, a model provider included, is the client's responsibility.
contract/driver-session-reply/no-node-id · driver session socket replies · NodeId — No reply carries a NodeId.

## input-validation · ## Input Validation
decision/actions-keyed-field · Keyed flag in the actions answer — The `actions` verb's answer carries, per actionable node, its id, owner id, a `keyed` boolean and closed verb and cause words, no NodeId. · why: A ratified boundary widening.
decision/actions-read-changes-nothing · Actions read runs nothing and moves no session state — The actions read runs nothing and changes no snapshot, focus reading or diff, so it can be called without moving the session. · why: The stand_actions test the_read_changes_nothing_and_holds_no_typed_text_or_mask pins it; agents call the actions verb between steps over socket and MCP
decision/cold-agent-call-validation · cold-agent.sh call and task allowlist — cold-agent.sh exits 2 with usage on stderr and empty stdout before any precondition or `claude` call on an unknown verb, a wrong argument count or a task outside the allowlist `stand`.
decision/cold-agent-check-reader-counts-only · Cold-agent recording reader keeps counts only — The cold-agent pipe's reader of its live child's kept checks holds bools and counts only, reads an unreadable directory as no check kept, and never prints or copies a check's content.
decision/cold-agent-checks-never-printed · Cold-agent checks never printed — The cold-agent check file is never printed, and the live child's kept checks are read for booleans and counts only and never printed.
decision/cold-agent-fixed-prompt · Cold-agent task as a fixed prompt — A cold-agent task reaches the agent session only as the fixed prompt, naming no element id and no app slug. · guard: test_the_prompt_names_no_stand_id
decision/command-span-content-free · Command span holds no content — One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
decision/mask-holds-on-every-carrier · Value mask holds on every carrier — A masked value stays masked on every carrier the snapshot reaches: its text form, a diff, the driver's return, the session socket and MCP tool results. · why: Ratified. · guard: stand_act_filled, stand_mcp_masked, stand_masks_on_new_answers · except: A caused item's message and its error field hold what their author wrote; every other content-named field in it reads as redacted.
decision/no-process-handle-crosses · No process-local handle crosses with an id — Neither the stable id, the snapshot, the diff nor any driver session, answer or refusal carries a `NodeId`, `ElementId`, `ScopeId` or pointer; only the stable id text crosses.
decision/one-actionable-rule · One actionable rule for both readings — The unkeyed-actionable reading and the actionable-nodes reading both read one actionable rule: focusable, an interactive role, or current listeners holding an event other than `mounted`. · why: The actionable test a_listener_removed_on_a_re_render_is_read_by_neither_reader pins it; the driver's actions verb and the unkeyed lint read these readers
decision/one-command-span-per-call · One command span per call — Every call handed to the session is wrapped in one INFO span named command under the escher_driver target, entered before validation so a refusal is inside it, holding fixed words, booleans and counts only.
decision/screen-pixels-in-band · Screen pixels in band — A screenshot's PNG travels inside the answer itself as base64 text, on the command line and as an MCP image block. · why: Ratified as the in-band crossing.
decision/screenshot-in-band · Screenshot carried in band — A screenshot's PNG crosses in band, as base64 inside its JSON answer line or as an MCP image block, the whole screen or one element's visible part, a password input painted masked, under the unchanged answer bound. · why: The in-band image was the ratified choice, a boundary widening.
decision/select-list-open-gate · Which select opens a list — A disabled, inert or hidden select, a select with no row to show, and one under a non-invertible transform open no list. · why: An engine change for every document and a boundary widening, ratified. · guard: a_disabled_inert_hidden_singular_or_empty_select_opens_nothing
decision/select-readers-resolve-node-closed-select-none · Select readers resolve the node and skip an open list — Each public select reader resolves its node so a stale id reads none, and the closed-select reader answers none for a select whose list is open. · why: The escher-driver lint and the snapshot's option bounds call closed_select_of and select_list_bounds, relying on none for a stale id or an open list
decision/session-layout-read-in-process · In-process layout read on a session — The session's layout read, in process or through the layout verb on any ratified carrier, resolves its id through the run path's id resolution refusing stale or not-found, runs no step, pump, tick or settle, and answers stable id strings, closed words and numbers, never a NodeId, text or value.
contract/cold-agent-cli/2 · cold-agent.sh exit codes · 2 — A verb outside run status cleanup logs, a wrong argument count or a task outside the allowlist stand exits 2 before any precondition or claude call.
contract/driver-command-span/cause · The driver command span · cause — The name of the returned refusal's cause, unrecorded until it applies.
contract/driver-verbs/actions · Driver verbs · actions — Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent, never a control value, typed text or NodeId.
contract/driver-verbs/graph · Driver verbs · graph — Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[…],"states":[…]} of ids, owner ids, component names, state names, handler event names, flag words and booleans, never a control value, typed text or NodeId.
contract/driver-verbs/layout · Driver verbs · layout — Takes one required id, refused malformed without it or with a second argument and not-found or stale for an unknown id; changes nothing, leaves delivered Timer ticks unapplied, and answers the element's placement, shifts and sizes as ids, closed words and numbers.
contract/mcp-tool-call-span/ending · The MCP tool_call span · ending — One of accepted, refused or error.
contract/mcp-tool-call-span/tool · The MCP tool_call span · tool — The verb table's word for the called tool; the span parents the call's command span.
decision/actions-read-shares-target-check · Actions read and verbs share one target check — Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused. · why: The stand_actions test every_listed_cause_on_the_lean_tasks_is_what_the_call_returns pins that each listed cause equals the refusal the call returns
decision/boundary-widening-rule · Boundary widening rule — A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
decision/closed-driver-verb-schema · Closed driver verb schema — Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing. · why: Each widening of the verb table is a widening of a validated surface, ratified.
decision/closed-value-field-no-widening · A closed-value field in a ratified carrier is no widening — A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does. · why: Ratified as the standing rule.
decision/fixture-name-closed-table · Fixture state name checked against a closed table — A fixture state name is compared whole against the app's closed table of named states before any session starts; any other name is refused as an unknown state and nothing is booted.
decision/id-lookup-anew-each-call · Caller-supplied id looked up anew — An acting command that takes a caller-supplied id looks it up on the screen as it reads now, on every call, never by index and keeping no node handle between calls; an id that names nothing is refused with nothing run and no panic.
decision/mcp-bounded-lines · Bounded MCP requests and results — MCP request lines and results are bounded, and tool arguments are typed before validation.
decision/no-echo-of-caller-input · No echo of caller input — No command-line ending writes back anything the caller typed: no path, label, id, value or file content.
decision/reached-handler-record-without-log · Reached-handler record without log or NodeId — Where recording of reached handlers is switched on, each reached handler's stable id and event name is written into a root-context record ahead of the handler's dispatch, with no log site and no `NodeId`.
decision/reached-record-deduplicated-survives-removal · Reached record deduplicated and kept past element removal — Handler coverage relies on the reached record holding the stable id and event of each node whose listeners held a dispatched event, deduplicated, so a handler whose run removes its element still counts. · why: The reached test a_handler_whose_run_removes_its_element_is_recorded_by_its_id pins it; the coverage role reads reached_handlers, each handler once
decision/recorded-check-strict-reader · Recorded check is read strictly as untrusted — A kept check is read as untrusted bytes, at most 1,048,576 bytes and 400 records of closed keys; anything else is refused `malformed` with no fault, holding nothing of the file, with no panic path.
decision/recorded-check-untrusted · Recorded check bytes are untrusted — A kept check is read by a strict reader bounded in bytes and records, with closed keys and schema-named arguments, anything else refused as malformed holding nothing of the file, no panic path; every replayed step is validated again; the coverage role reads checks through the same reader.
decision/recording-bounds-end-recording · A failed or overlong recording ends — A failed write or a passed bound of 400 records or 1,048,576 bytes ends the recording, its check removed, the call run and not rolled back and answered `recording-failed`.
decision/recording-dir-owner-only · Recording directory is owner-only — The recording directory is created mode 0700 and a present one open to group or others is refused `record-dir-not-private` with status 3, nothing written and nothing spawned. · why: Ratified.
decision/seeded-source-in-process-seed-only · Seeded source takes its seed from in-process code only — The seeded source of randomness takes its seed only from in-process code, a root context the virtual DOM's builder gives else the default seed 0, and outside any run seeds from the host's per-process random keys, reading no environment variable, file or carrier.
decision/select-list-closing · What closes an open select list — An open select list closes on a press outside it, a page scroll or a move of its box, while a list a click just opened survives a focus rule's restyle. · why: An engine change for every document and a boundary widening, ratified. · guard: escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event
decision/select-list-painted-over-page · Select list painted over the page — The open select list is painted after the root's render and before the devtools overlays, a non-finite list box left unpainted. · why: An engine change for every document and a boundary widening, ratified.
decision/session-id-record-bounded · Bounded in-process record of read ids — A session keeps a private record of ids its own snapshots read, at most 4096 ids of at most 1024 bytes each, the oldest forgotten first, read by no public item and written to no log, event, socket or file. · why: Kept to tell the stale and not-found refusals apart. · guard: the_record_holds_no_id_longer_than_a_call_can_name
decision/socket-carries-screen-reading · Session socket carries what the screen reads — An answer carries what the screen reads: ids, accessible names, control values, password and file input masked, snapshot text, diff, lint findings, UI graph, layout readings and an acting step's caused text scrubbed by its own field names, error kept, never a NodeId or an attribute value as typed. · why: The crossing was ratified.
decision/stable-id-exits · Ratified exits of the stable id — The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files. · why: Each carrier ratified as a boundary widening.
decision/stable-id-not-logged · Stable ids kept out of logs — Escher's own code writes the stable id to no log, and a host that installs escher's sink drops third-party style records carrying an element's HTML id whole, printing that value at no `RUST_LOG` level. · why: Those third-party targets are outside the sink's allowlist.
decision/target-refusal-order · Fixed order of target refusals — A click or type on an element that cannot take it is refused with nothing run, the first of disabled, off-screen, covered that holds, leaving the snapshot and both focus readings as they were; scroll reports in_view instead of refusing. · why: The detection of refusal causes by code was ratified.
decision/unknown-id-refused-stale-or-not-found · Unknown id refused as stale or not-found — An id that names no element on the screen as it reads now is refused with nothing run: stale when the session's record of ids holds it, not-found when not.
decision/visible-region-open-list-row · Visible region of an open select list row — For a row or a group of an open select list, the visible region answers the list's scrollport. · why: An engine change for every document, ratified.
contract/actions-answer/keyed · The actions answer · keyed — A boolean on each listed node.
contract/actions-answer/nodes · The actions answer · nodes — `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, one entry per actionable node, holding ids, owner ids, `keyed` booleans, verb words and closed cause words, never a control value, typed text or NodeId.
contract/cli-roles/coverage · escher-session roles · coverage — coverage <app> <check>... with no --session or --record, else usage; each check read up to 1,048,576 bytes, and an unreadable, oversized, non-check or other-app file refused malformed, status 1, holding nothing of the path or file.
contract/coverage-answer/line · The coverage answer · line — One line {"unexercised":[…],"checks":n,"held":m} on stdout, status 0, holding ids, owner ids, event names and the two counts only.
contract/driver-command-span/verb · The driver command span · verb — The verb table's own word, never the caller's verb text.
contract/driver-id-refusal/not-found · Driver id refusal · not-found — A call naming an id that no element on the current screen carries is refused as `not-found` when the session's record does not hold that id.
contract/driver-id-refusal/stale · Driver id refusal · stale — A call naming an id that no element on the current screen carries is refused as `stale` when the session's record holds that id.
contract/driver-session-reply/no-attribute-text · driver session socket replies · attribute text — A reply never carries an attribute's text, except what an author wrote into a record's message or into the `error` field of a `caused` item.
contract/driver-session-reply/state-readings · driver session socket replies · state readings — State readings are carried as closed words and booleans.
contract/escher-session-coverage/answer-line · escher-session coverage role · answer line — The coverage answer holds handler references of ids, owner ids and event names and two counts, no process-local handle, and crosses only as one line on the command line's stdout, with no verb, socket or MCP carrier.
contract/graph-answer/nodes-states · Graph answer · nodes and states — `{"nodes":[…],"states":[…]}` holding ids, component names, state names, handler event names, the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or NodeId.
contract/mcp-server/transport · MCP server · transport — Newline-delimited JSON-RPC 2.0 on the invoking user's stdin and stdout, with no authentication and no environment read.

## data-protection · ## Data Protection
decision/cold-agent-checks-read-for-counts-only · Cold-agent checks read for bools and counts only — The cold-agent pipe's live child keeps its checks in an owner-only record directory, read by the pipe for bools and counts only, never printed, copied or committed, and removed by its cleanup.
decision/cold-agent-records-private · Cold-agent recordings stay private — The cold-agent pipe records into an owner-only directory, never prints, copies or commits the checks, reads them for bools and counts only and removes them with its state on cleanup.
decision/log-sink-redacts-and-allowlists · Log sink redacts and allowlists targets — Escher's own log sink redacts records and drops a record from a target outside escher-telemetry's target allowlist.

## api-security · ## API Security
decision/actions-answer-unpaged · The actions answer is unpaged — The actions verb takes no argument and a screen's whole answer is one answer under the 1,048,576-byte answer bound. · guard: stand_actions the_whole_cells_answer_is_within_the_answer_bound
decision/graph-answer-unpaged · The graph answer is unpaged — The graph verb takes no argument and a screen's whole graph is one answer under the 1,048,576-byte answer bound. · why: A part argument would widen the crossing. · guard: stand_graph the_whole_cells_answer_is_within_the_answer_bound
decision/no-wait-verb-fields · No wait argument or result on verbs — No driver verb has an argument or a result field of a wait.
decision/time-moves-only-on-advance · Time moves only on advance — Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph and actions wait on no load.
contract/driver-answer/settled · Driver command answer · settled — A step that ran and did not go quiet is a result, settled false with its busy class, not a refusal; the busy reading names a class only, never a URL, request, node, id, name or value.
decision/bounded-load-wait · Bounded wait on loads in flight — A harness settle never waits on a load in flight, and the driver's acting step waits on one for at most `LOAD_WAIT`. · why: So the absent request timeout cannot hang a settle or a command.
decision/load-wait-bound · Acting commands bounded by LOAD_WAIT — An acting driver command waits on a load in flight for at most LOAD_WAIT, 5 s of wall time, moving no time on the held instance, then returns the load still in flight as settled false with its busy class. · why: The absent request timeout cannot hang a command.
contract/mcp-server/tool-result · MCP server · tool result — A tool result is bounded at 1,048,576 bytes, one holding an image measured by its whole written form and a text-only one by its text.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project)
decision/telemetry-sink-target-allowlist · Telemetry sink target allowlist — escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level. · except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed

## secret-management · ## Secret Management
decision/live-evidence-holds-no-credentials · Live evidence holds no credentials — No committed live evidence, artifact, event or verdict carries a key, auth header, credential-file name, token field, e-mail address or the operator's login.
decision/live-runs-use-operator-login · Live runs use the operator's own login — The cold-agent live run and the MCP live trial reach the model provider only through the operator's own Claude Code login held outside the repository: no API key, no env read by the pipe, no CI secret, no new secret source and no auth library. · why: Ratified.
contract/environment-variables/rust-log · Environment variables · RUST_LOG — Holds the tracing filter directives escher-telemetry reads, defaulting to warn when unset or unparsable.

## error-handling · ## Error Handling
decision/closed-refusal-cause-set · Closed refusal cause set — A driver command refusal is a typed value whose cause comes from a closed set of eight, with a fault only beside `malformed`; a new verb or argument shape is refused within the set, never by adding a cause. · why: The closed set of eight was ratified.
decision/refusal-closed-cause-set · Command refusal as a closed set of causes — An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field. · why: So a refusal cannot hold anything a call supplied. · except: A Fault is carried only beside the malformed cause.
decision/refusal-holds-no-call-input · A refusal holds nothing a call supplied — Every refusal's name, meaning and remedy is a fixed static text holding no path, and a fault carries at most the schema's own argument name, so a refusal cannot hold anything a call supplied.
decision/refusal-log-cause-name-only · Only a refusal's cause name is logged — Exactly one thing of a refusal reaches a log: its cause's fixed name, as the `cause` field of the call's driver span; no fault, meaning, remedy or display text is logged.
decision/refusal-logged-by-cause-name · Refusal logged by cause name only — Of an escher-driver command refusal, only its cause's fixed name reaches a log.
contract/refusal-causes/covered · Driver refusal causes · covered — The target is covered at its hit point; its fixed meaning states the hit's limits, among them that an option of a closed select reads covered, remedied by clicking the select first.
contract/refusal-causes/disabled · Driver refusal causes · disabled — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/fonts-unavailable · Driver refusal causes · fonts-unavailable — One of the eight closed causes, returned by the executor, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/malformed · Driver refusal causes · malformed — The one cause that carries a fault; a call breaking the schema, such as a `graph` or `actions` call with an argument, is refused with it and nothing runs.
contract/refusal-causes/not-found · Driver refusal causes · not-found — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/off-screen · Driver refusal causes · off-screen — The element lies outside the visible part, including outside a scrolling box that holds it; its remedy names the `scroll` verb.
contract/refusal-causes/stale · Driver refusal causes · stale — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/unknown-verb · Driver refusal causes · unknown-verb — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
decision/coverage-refuses-bad-checks-malformed · Coverage refuses an unusable check as malformed — The `coverage` role refuses a check file it cannot read, one past the check bound, one that is no check or a check of another app as `malformed`, holding nothing of the path or the file, and adds no cause.
contract/refusal-json/cause · Refusal JSON · cause — The refusal's cause name, inside `{"refused":{...}}`, built from the cause's fixed strings.
contract/refusal-json/fault · Refusal JSON · fault — Present only on a `malformed` refusal, naming the fault.
contract/refusal-json/meaning · Refusal JSON · meaning — The cause's fixed meaning text.
contract/refusal-json/remedy · Refusal JSON · remedy — The cause's fixed remedy text.

## logging-monitoring · ## Logging & Monitoring
decision/closed-span-lines · One sink line per closed span — The sink prints one line per closed span whose target the allowlist admits and nothing on creation, record, enter or exit; each span field is judged by the event rule, an outside-target span writes no byte, and a content-named span field's value is never stored. · why: A boundary widening of the sink, ratified as built.
decision/cold-agent-events-metadata-only · Cold-agent events hold run metadata only — The cold-agent pipe's events and verdict carry counts, booleans and closed words, never transcript text, a tool argument or answer, a reply or the check.
decision/cold-agent-logs-metadata-only · Cold-agent pipe logs run metadata only — The cold-agent pipe's events and verdict carry counts, booleans and closed words only: no content-named field and no transcript text, tool argument, tool answer, reply or check. · guard: The scrub-set key check and four planted markers, under the shim
decision/cold-agent-record-unprinted · Cold-agent record kept unprinted — The cold-agent server log, check, raw transcript and the live child's kept checks are never printed; the transcript stays unscrubbed and gitignored, and each committed copy has its host paths masked.
decision/cold-agent-recordings-owner-only · Cold-agent recordings owner-only — The cold-agent live server records its kept checks into an owner-only directory, read for booleans and counts only, never printed, copied or committed, and removed at cleanup; the replay children run without recording.
decision/cold-agent-server-output-unprinted · Cold-agent server output never printed — The cold-agent pipe reads its server's stderr log only by counting span lines and writes the agent's check as parsed, printing or copying neither; the replay children's stderr goes to /dev/null.
decision/cold-agent-transcript-raw · Cold-agent raw transcript kept unscrubbed — The cold-agent session's raw transcript stays unscrubbed by design, is never printed and is gitignored; each committed copy has its host paths, the session dir and the home prefix, masked.
decision/cold-agent-wrong-call-bar · Cold-agent wrong-call bar — The cold-agent verdict decides its wrong calls against a 5 % bar, failing with the reason wrong-calls-over-bar above it.
decision/no-screen-content-in-logs · Screen content stays out of logs — No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer, a coverage check and a step's caused text reach no log line at any level. · guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds
contract/command-span/cause · The command span line · cause — Recorded when the call is refused, as a closed cause word such as fonts-unavailable.
contract/command-span/line · The command span line · line — A sink-installing host writes one closed command span line per call it runs.
contract/command-span/verb · The command span line · verb — Records the instance-level verb; a graph, actions or layout call records the verb alone, plus cause when refused.
contract/tool-call-span/ending · The tool_call span line · ending — One of the closed words accepted, refused or error.
contract/tool-call-span/line · The tool_call span line · line — One closed line per MCP tools/call, the parent of that call's command span, holding tool and ending only.
contract/tool-call-span/tool · The tool_call span line · tool — The tool table's word for the call, a closed word outside the content-named fields; absent for an unknown tool.
decision/escher-target-content-fields-withheld · escher_ targets withhold content-named fields — In escher's telemetry sink an escher_ target keeps all fields but twelve content-named ones.
decision/escher-target-content-scrub · Escher targets scrub content-named fields — An event whose target starts with escher_ prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which print as [redacted].
decision/escher-telemetry-allowlist-sink · escher's allowlist telemetry sink — Both seven_guis binaries log through escher's own stderr-only allowlist subscriber. · except: The sink's scrubbing reaches that sink only, escher's scoped capture hiding its own field set: upstream's fmt::init subscribers, env_logger and the panic hook stay unscrubbed.
decision/stderr-only-telemetry-sink · Escher's own stderr telemetry sink — Both seven_guis binaries, the native app and the escher-session driver host, install escher's own telemetry sink first thing: a stderr-only subscriber whose formatter applies an allowlist of targets. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed, and the chained std panic hook still prints the raw panic message to stderr.
contract/escher-session-coverage-line/content · The escher-session coverage role's stdout line · content — The coverage line holds ids, owner ids, event names and two counts, and nothing of a named path.
contract/escher-session-mcp-stdout/protocol-lines · The escher-session mcp role's stdout · protocol lines — The mcp role writes JSON-RPC protocol lines only on stdout, and nothing at its end.
contract/escher-session-stdout/mcp-role · escher-session stdout · mcp role — The mcp role writes JSON-RPC lines only on stdout.

# security-plan · its absences, names and overviews, whole

## threat-model-summary · ## Threat Model Summary
> NOT YET MEASURED — no slice states an application-level threat model; data classification, auth model, infrastructure and compliance triggers were not gathered for this summary (the stored-data and auth facts the reading did reach are in the sections below).
absence/same-origin-cors-checks — Same-origin and CORS checks on script fetches

## authentication-authorization · ## Authentication & Authorization
absence/application-authentication — Application authentication or authorization
absence/authentication-authorization — Authentication or authorization code

## input-validation · ## Input Validation
absence/dioxus-native-form-validation — Form data validation in the Dioxus native bridge: form data always reads valid
absence/same-origin-and-cors-checks — Same-origin and CORS checks on script fetches
absence/url-taking-driver-verb — A driver verb that takes a URL

## data-protection · ## Data Protection
> NOT YET MEASURED — HSTS, TLS version requirements and certificate pinning: no slice gathered them. ¶ > NOT YET MEASURED — key management, backups, retention periods and anonymization: no slice gathered them.
absence/database-encryption — Encryption of the history database

## api-security · ## API Security
absence/content-security-policy — CSP enforcement
absence/cors-checks — CORS checks on outbound requests
absence/cors-same-origin-policy — A CORS or same-origin policy
absence/request-timeout — A request timeout on outbound requests
absence/response-body-size-limit — A response body size limit on outbound requests
absence/response-size-limit — A response size limit on outbound requests
absence/same-origin-checks — Same-origin checks on outbound requests
absence/served-network-api — A served network API: no binary serves a TCP or UDP port
overview/api-security — No served network API exists: no binary serves a TCP or UDP port. The one listener is local, the driver session's Unix-domain socket, and the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results. The remaining controls govern outbound requests and in-document navigation.

## dependency-security · ## Dependency Security
> NOT YET MEASURED — a critical-CVE response SLA: none is stated. The lockfile's advisory state is read on every push by the audit leg, within cargo-deny's resolved graph (Audit tool above). ¶ Supply chain integrity › > NOT YET MEASURED — SBOM generation and base image scanning: no slice gathered them.
absence/automated-dependency-updates — Automated dependency update tooling, such as dependabot
absence/signed-artifacts — Signed release artifacts produced by the fork

## secret-management · ## Secret Management
> NOT YET MEASURED — development secret storage (beyond the fact that the cold-agent run's login is held by the `claude` CLI; its at-rest location is unmeasured), secret scanning in CI, rotation cadence and access auditing: no slice gathered them.

## error-handling · ## Error Handling
absence/error-reporting-integration — An error reporting integration

## logging-monitoring · ## Logging & Monitoring
> NOT YET MEASURED — security-event logging (auth, authorization or validation failures), log retention, access controls and tamper evidence: no slice gathered them.
absence/monitoring-and-alerts — Monitoring beyond the stderr log: no metrics, no alerting and no telemetry export

## compliance-controls · ## Compliance Controls
> NO RECORDED INTENT

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## security-decisions-log · ## Security Decisions Log
> NO RECORDED INTENT
