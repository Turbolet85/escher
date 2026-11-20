# security-plan · the rows that bind 2026-11-20-codebase-health — picked 221 of 567 (176 by every draw) · rule any · pin 2532

## threat-model-summary · ## Threat Model Summary
decision/actions-answer-crossing · Actions answer crossing — An actions answer carries what each actionable node can take now on the socket, the command line, MCP and a kept check: ids, owner ids, keyed booleans, verb-table words and closed cause words, never a control value, typed text or NodeId. · why: A boundary widening, ratified.
decision/graph-answer-crossing · Graph answer crossing — A graph answer carries the screen's UI graph on the socket, the command line, MCP and a kept check: ids, owner ids, component names, registered state names, handler event names, closed flag words and booleans, never a control value, typed text or NodeId. · why: A boundary widening, ratified.
decision/mcp-result-crossing · MCP result crossing — An MCP result carries ids, accessible names, control values with a password's or file input's as the mask, the snapshot text and an acting call's diff to the client; what a client does onward, a model provider included, is the client's. · why: A boundary widening, ratified.
decision/mcp-server-stdio-only · MCP server on stdio only — The MCP server is a stdio child an MCP client spawns, speaking on the invoking user's stdin and stdout, with no listener, port or socket.
decision/mcp-stdio-no-auth · MCP server over stdio, no authentication — The driver's MCP server runs as a stdio child a client spawns under the invoking user, JSON-RPC 2.0 on stdin and stdout, no listener, port or socket; the spawning client is the trust boundary, with no authentication.
decision/select-list-crossing · Select list crossing limited to bounds and pixels — The select list crosses the ratified carriers only as a row's own bounds in the snapshot, the diff and a layout answer, as the layout answer's select-list placing word, and as pixels in a shot; node state holds no field of it and no expanded node key stands.
decision/session-socket-carries-calls · Session socket carries calls and answers — The session socket carries the lifecycle and calls with their answers: an element id and typed text in; ids, accessible names, control values with a password's or file input's as the fixed mask, the snapshot text and the diff out; no NodeId crosses. · why: A boundary widening, ratified.
decision/snapshot-state-readings · Three state readings per control — The snapshot text and the diff hold three state readings per control, selected, pressed and invalid, as closed words and booleans. · why: Inside the ratified crossing, ratified.
contract/actions-answer/id · The actions answer · id — The node's id.
contract/actions-answer/owner · The actions answer · owner — The node's owner id.
contract/actions-answer/verbs · The actions answer · verbs — A list of entries each holding verb, a word of the verb table, and cause, a closed cause word left out when absent; every listed node carries layout with no cause.
contract/driver-session-reply/masked-control-values · driver session socket replies · control values — A password's or file input's control value is carried masked.
contract/driver-session-reply/no-node-id · driver session socket replies · NodeId — No reply carries a NodeId.
contract/mcp-result/masked-control-values · MCP server results · control values — A password's or file input's control value is carried masked.
decision/capability-statement-mcp-crossing · Committed statement of the MCP crossing — A committed capability statement tells its readers what an MCP client carries onward: the spawning client as the boundary with no authentication, the carried set, and onward use as the client's; it adds no carrier.
decision/driver-session-no-peer-authentication · Driver session and MCP server carry no authentication — The driver session socket and the MCP stdio server carry no handshake, token, secret or peer check, and session start does not check the answering pid; the trust boundary is the owning user or the spawning client process.

## input-validation · ## Input Validation
decision/accessible-name-exits · Ratified exits of accessible names — Accessible names leave the process only through the platform accessibility adapter, a hosted call's answer in the snapshot text and diff JSON over the session socket and the client's stdout, and MCP tool results, and reach no log. · why: Each carrier ratified.
decision/accessible-names-not-logged · Accessible names kept out of logs — Escher's own code logs no accessible name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level, the third-party record that carried one being dropped whole. · why: That third-party target is outside the sink's allowlist. · guard: mcp_log
decision/actions-keyed-field · Keyed flag in the actions answer — The `actions` verb's answer carries, per actionable node, its id, owner id, a `keyed` boolean and closed verb and cause words, no NodeId. · why: A ratified boundary widening.
decision/actions-read-shares-target-check · Actions read and verbs share one target check — Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused. · why: The stand_actions test every_listed_cause_on_the_lean_tasks_is_what_the_call_returns pins that each listed cause equals the refusal the call returns
decision/aria-label-trimmed-once · One trimmed aria-label read — An `aria-label` non-empty after trimming becomes the node's label and a whitespace-only one names nothing; there is one trimmed, non-empty `aria-label` read. · why: Follows accname-1.2; upstream's unfiltered read was dropped in the merge. · guard: accessibility_label
decision/aria-labelledby-outranks-label · aria-labelledby outranks label and aria-label — An element's `aria-labelledby` targets' text, gathered cycle-guarded, is its computed label, outranking a `<label>` and an `aria-label`; an id naming no element is ignored and `aria-label` is the fallback when the targets hold no text. · why: Upstream's behaviour, ratified at the upstream sync. · guard: accessibility_label
decision/aria-state-lowercase-exact-match · Control-state values matched exactly in lower case — `aria-selected`, `aria-pressed`, `aria-checked` and `aria-invalid` values are trimmed of ASCII whitespace and compared exactly, in lower case, against a closed set; an unrecognized value sets nothing, but for `aria-invalid`, which reads invalid. · why: A choice: the specification as read states no case rule and no browser was measured; the `aria-invalid` fallback is WAI-ARIA 1.2's rule. · except: A select's option reads its chosen state as selected, never `aria-selected`.
decision/attribute-readers-no-log-or-panic · Markup attribute readers without log or panic — Markup attributes, remote pages included, are read from parsed content by in-process readers with no log site or panic path, ARIA states and closed values through closed parses carrying no attribute text.
decision/attribute-text-leaves-by-ratified-carriers · Attribute text leaves only by ratified carriers — Markup attribute values, names and descriptions leave escher only by the ratified carriers. · except: The page's own form submission and events.
decision/cli-closed-argv-reader · Closed argv reader — The session command line reads argv with a closed reader with fixed rules, and checks a call before the session address is asked for.
decision/cli-echoes-nothing-typed · Command line echoes nothing typed — No command-line ending writes back anything the caller typed, and no path on argv panics.
decision/closed-driver-verb-schema · Closed driver verb schema — Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing. · why: Each widening of the verb table is a widening of a validated surface, ratified.
decision/command-schema-no-clock · Command schema reads no clock — The command schema and its validation read no clock.
decision/command-span-content-free · Command span holds no content — One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
decision/control-states-closed-values-only · Control states cross as closed values — Control states leave the process only as closed enums or booleans, on the accessibility adapter and in the snapshot's words and JSON booleans, never as the attribute's text, and no unrecognized or long value is written anywhere as typed. · why: A ratified widening of the adapter crossing by a new author-derived class.
decision/descriptions-adapter-only · Descriptions cross only to the accessibility adapter — `aria-describedby` targets and `aria-description` text reach only the platform accessibility adapter; the snapshot reads no description, so no driver or MCP carrier, verb, field, log field or span carries one. · why: Upstream's behaviour, ratified as a boundary widening by upstream merge. · guard: accessibility_description
decision/diff-counts-only-in-logs · Only diff sizes reach a log — A diff's content never reaches a log or event, while its three sizes (counts of added, removed and changed nodes) are recorded on the driver's command span, printed at `info` or below. · why: Ratified: a diff's content never reaches a log while its three sizes may.
decision/disabled-read-by-presence · disabled read by presence on form controls — The disabled attribute is read by its presence, whatever its value, and only on a button, input, select or textarea, for focusability, pointer and keyboard activation and the snapshot's enabled and clickable readings. · why: The presence reading was ratified.
decision/dispatch-recorded-only-with-signal-registry · Dispatch recorded only where a signal registry stands — The Dioxus event handler records a dispatch only where a signal registry is registered; on a page with none, the event's handlers run and nothing is recorded. · why: The dioxus_document tests a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is and the_reached_record_holds_on_a_page_with_no_registered_state pin it
decision/driver-outcomes-never-logged · Driver outcomes never logged — The driver crate logs and fields no command, call, argument, outcome, refusal, screen text, image, finding, session label or id record; an outcome is written only as a command-line answer, an MCP tool result, a session-socket reply or a recording file the caller asked for. · why: Outcomes leave the process only by the ratified crossing.
decision/id-lookup-anew-each-call · Caller-supplied id looked up anew — An acting command that takes a caller-supplied id looks it up on the screen as it reads now, on every call, never by index and keeping no node handle between calls; an id that names nothing is refused with nothing run and no panic.
decision/mask-holds-on-every-carrier · Value mask holds on every carrier — A masked value stays masked on every carrier the snapshot reaches: its text form, a diff, the driver's return, the session socket and MCP tool results. · why: Ratified. · guard: stand_act_filled, stand_mcp_masked, stand_masks_on_new_answers · except: A caused item's message and its error field hold what their author wrote; every other content-named field in it reads as redacted.
decision/masked-input-values · Password and file input values masked — A password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path. · except: The DOM and form submission keep the value.
decision/mcp-bounded-lines · Bounded MCP requests and results — MCP request lines and results are bounded, and tool arguments are typed before validation.
decision/mcp-no-print-or-panic · MCP server never prints or panics — The MCP server has no print, unwrap, expect or panic outside the tests.
decision/mcp-over-stdio · MCP over stdio only — The MCP server speaks newline-delimited JSON-RPC 2.0 on the invoking process's stdin and stdout, with no listener, port, socket, authentication or environment read. · why: The MCP carrier and its bounds were ratified as is.
decision/merged-aria-states-closed-parse · Closed parse of expanded, disabled and level — `aria-expanded`, `aria-disabled` and `aria-level` are read as closed values carrying none of the attribute's text and exit only on the accessibility adapter; `aria-disabled` moves no snapshot reading. · why: As upstream wrote them, inside the ratified states carrier. · guard: accessibility_merged_states
decision/multiple-select-set-exits · Where a multiple select's chosen set leaves — A multiple select's chosen set leaves only as form entries, as input and change values carrying one entry per chosen option, as the snapshot's value of the first chosen option in tree order, and as the options' tree nodes. · why: An engine change for every document and a boundary widening, ratified.
decision/no-echo-of-caller-input · No echo of caller input — No command-line ending writes back anything the caller typed: no path, label, id, value or file content.
decision/no-panic-on-call-input · No panic on call-supplied input — Neither the driver command schema's validation nor the executor has unwrap, expect, panic or unreachable on anything a call supplies.
decision/no-process-handle-crosses · No process-local handle crosses with an id — Neither the stable id, the snapshot, the diff nor any driver session, answer or refusal carries a `NodeId`, `ElementId`, `ScopeId` or pointer; only the stable id text crosses.
decision/one-actionable-rule · One actionable rule for both readings — The unkeyed-actionable reading and the actionable-nodes reading both read one actionable rule: focusable, an interactive role, or current listeners holding an event other than `mounted`. · why: The actionable test a_listener_removed_on_a_re_render_is_read_by_neither_reader pins it; the driver's actions verb and the unkeyed lint read these readers
decision/one-command-span-per-call · One command span per call — Every call handed to the session is wrapped in one INFO span named command under the escher_driver target, entered before validation so a refusal is inside it, holding fixed words, booleans and counts only.
decision/one-validation-path-for-every-carrier · One validation path for every carrier — The session socket, the command line and MCP each pass a call through the schema's validation before anything runs, and nothing public runs a command that did not come from that validation. · why: The crossing to external input surfaces was ratified with this check as the one they all pass through.
decision/option-disabled-one-rule · One option-disabled rule — Whether an option is disabled is decided by one rule that the readings and form submission share. · why: An engine change for every document, ratified.
decision/password-file-value-masked · Password and file input values masked — The snapshot reads a password or file input's non-empty value as one fixed marker whatever was typed or chosen, an empty one as empty, never the text, its length or a file's host path. · why: Ratified. · guard: stand_act_filled, stand_mcp_masked · except: The DOM is untouched: a file input's value attribute still holds the host path and form submission still carries it.
decision/range-value-exits · Range current value exits — A range input's current value leaves only as a form submission entry, the `input` event's value and the snapshot's control value, and no accessibility node value or action changes. · why: An engine change for every document and a boundary widening, ratified.
decision/screenshot-in-band · Screenshot carried in band — A screenshot's PNG crosses in band, as base64 inside its JSON answer line or as an MCP image block, the whole screen or one element's visible part, a password input painted masked, under the unchanged answer bound. · why: The in-band image was the ratified choice, a boundary widening.
decision/select-choice-exits · Where a select's choice leaves — A select's choice leaves only as form submission's entries, the input and change events' value, the snapshot's value and the options' accessibility-tree nodes, inside the ratified name and control-value crossings. · why: A boundary widening, ratified.
decision/session-layout-read-in-process · In-process layout read on a session — The session's layout read, in process or through the layout verb on any ratified carrier, resolves its id through the run path's id resolution refusing stale or not-found, runs no step, pump, tick or settle, and answers stable id strings, closed words and numbers, never a NodeId, text or value.
decision/single-validation-gate · Single command validation gate — Every call, from the socket, the command line or MCP, is checked against a closed static table of verbs and bounded argument kinds, the first failure in a fixed order, and nothing public runs a command that did not come from that validation.
decision/socket-carries-screen-reading · Session socket carries what the screen reads — An answer carries what the screen reads: ids, accessible names, control values, password and file input masked, snapshot text, diff, lint findings, UI graph, layout readings and an acting step's caused text scrubbed by its own field names, error kept, never a NodeId or an attribute value as typed. · why: The crossing was ratified.
decision/stable-id-exits · Ratified exits of the stable id — The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files. · why: Each carrier ratified as a boundary widening.
decision/stable-id-not-logged · Stable ids kept out of logs — Escher's own code writes the stable id to no log, and a host that installs escher's sink drops third-party style records carrying an element's HTML id whole, printing that value at no `RUST_LOG` level. · why: Those third-party targets are outside the sink's allowlist.
decision/target-refusal-order · Fixed order of target refusals — A click or type on an element that cannot take it is refused with nothing run, the first of disabled, off-screen, covered that holds, leaving the snapshot and both focus readings as they were; scroll reports in_view instead of refusing. · why: The detection of refusal causes by code was ratified.
decision/unknown-id-refused-stale-or-not-found · Unknown id refused as stale or not-found — An id that names no element on the screen as it reads now is refused with nothing run: stale when the session's record of ids holds it, not-found when not.
decision/unknown-target-refused · Call on an unknown or unfit element refused — A call whose id names no element, or an element that cannot take the call, is refused with nothing run.
contract/actions-answer/keyed · The actions answer · keyed — A boolean on each listed node.
contract/actions-answer/nodes · The actions answer · nodes — `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, one entry per actionable node, holding ids, owner ids, `keyed` booleans, verb words and closed cause words, never a control value, typed text or NodeId.
contract/cli-flags/id · escher-session flags · --id — Followed by one value, whatever that word is.
contract/cli-flags/key · escher-session flags · --key — Followed by one value, whatever that word is.
contract/cli-flags/ms · escher-session flags · --ms — Followed by one value, whatever that word is.
contract/cli-flags/part · escher-session flags · --part — Followed by one value, whatever that word is.
contract/cli-flags/record · escher-session flags · --record — Followed by a directory, taken at most once and only on start, the host role and the mcp role; anywhere else, twice or with no value is usage.
contract/cli-flags/session · escher-session flags · --session — Followed by a directory, given exactly once, else usage.
contract/cli-flags/shift · escher-session flags · --shift — Bare, taking no value.
contract/cli-flags/text · escher-session flags · --text — Followed by one value, whatever that word is.
contract/cli-roles/coverage · escher-session roles · coverage — coverage <app> <check>... with no --session or --record, else usage; each check read up to 1,048,576 bytes, and an unreadable, oversized, non-check or other-app file refused malformed, status 1, holding nothing of the path or file.
contract/cli-roles/mcp · escher-session roles · mcp — mcp alone or with one --record <dir> selects the MCP role; any other word after it is usage.
contract/cli-roles/serve · escher-session roles · serve — serve <app> --session <dir> in exactly that shape, optionally with one --record <dir> after it, else usage.
contract/declared-write/event · A DeclaredWrite of the signal registry · event — An event name without the on prefix.
contract/declared-write/target · A DeclaredWrite of the signal registry · target — An element's stable id, or a prefix ending in *.
contract/driver-argument-kinds/flag · Driver argument kinds · flag — True or false.
contract/driver-argument-kinds/id · Driver argument kinds · id — Text of 1 to 1024 bytes.
contract/driver-argument-kinds/key · Driver argument kinds · key — One name of a closed twelve-name list.
contract/driver-argument-kinds/milliseconds · Driver argument kinds · milliseconds — A whole number from 1 to 60000.
contract/driver-argument-kinds/name · Driver argument kinds · name — Text of 1 to 32 bytes of a-z, 0-9 and -.
contract/driver-argument-kinds/part · Driver argument kinds · part — A whole number from 1 to 1000.
contract/driver-argument-kinds/text · Driver argument kinds · text — Text of 0 to 4096 bytes.
contract/driver-call-id/length-bound · Driver call id · length bound — An id a call names, from the command-line `--id` flag, the socket's `call` request or an MCP tool's `id` argument, is bounded at 1 to 1024 bytes, checked in the client before a session is touched and again in the host.
contract/driver-command-span/added · The driver command span · added — The length of the returned diff's added list.
contract/driver-command-span/busy · The driver command span · busy — The schema's class word, unrecorded until it applies.
contract/driver-command-span/cause · The driver command span · cause — The name of the returned refusal's cause, unrecorded until it applies.
contract/driver-command-span/changed · The driver command span · changed — The length of the returned diff's changed list.
contract/driver-command-span/passes · The driver command span · passes — A count, unrecorded until it applies.
contract/driver-command-span/removed · The driver command span · removed — The length of the returned diff's removed list.
contract/driver-command-span/settled · The driver command span · settled — A boolean, unrecorded until it applies.
contract/driver-command-span/verb · The driver command span · verb — The verb table's own word, never the caller's verb text.
contract/driver-id-refusal/not-found · Driver id refusal · not-found — A call naming an id that no element on the current screen carries is refused as `not-found` when the session's record does not hold that id.
contract/driver-id-refusal/stale · Driver id refusal · stale — A call naming an id that no element on the current screen carries is refused as `stale` when the session's record holds that id.
contract/driver-session-reply/no-attribute-text · driver session socket replies · attribute text — A reply never carries an attribute's text, except what an author wrote into a record's message or into the `error` field of a `caused` item.
contract/driver-session-reply/state-readings · driver session socket replies · state readings — State readings are carried as closed words and booleans.
contract/driver-verbs/actions · Driver verbs · actions — Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent, never a control value, typed text or NodeId.
contract/driver-verbs/graph · Driver verbs · graph — Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[…],"states":[…]} of ids, owner ids, component names, state names, handler event names, flag words and booleans, never a control value, typed text or NodeId.
contract/driver-verbs/layout · Driver verbs · layout — Takes one required id, refused malformed without it or with a second argument and not-found or stale for an unknown id; changes nothing, leaves delivered Timer ticks unapplied, and answers the element's placement, shifts and sizes as ids, closed words and numbers.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The call was accepted: one uncoloured JSON line on stdout holds the answer, or for the coverage role its one coverage line.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The call was refused: one uncoloured JSON line on stdout holds the refusal, or for the coverage role a malformed refusal.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — A usage error: the usage line goes to stderr and stdout stays empty.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — A session error: the session-error object on stdout and the session error's fixed message on stderr.
contract/graph-answer/nodes-states · Graph answer · nodes and states — `{"nodes":[…],"states":[…]}` holding ids, component names, state names, handler event names, the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or NodeId.
contract/masked-input-value/masked-stand-in · Masked input values · MASKED_STAND_IN — A recorded masked `type` is kept as `MASKED_STAND_IN`.
contract/mcp-errors/32600 · MCP protocol errors · -32600 — invalid request, with no data: not an object with jsonrpc "2.0" and a string method, or an id of another form.
contract/mcp-errors/32601 · MCP protocol errors · -32601 — method not found, with no data: every method but initialize, ping, tools/list and tools/call.
contract/mcp-errors/32602 · MCP protocol errors · -32602 — invalid params or unknown tool, with no data: params not an object with a string name and an optional object arguments, or a name outside the verb table.
contract/mcp-errors/32700 · MCP protocol errors · -32700 — parse error, with no data: a non-UTF-8, non-JSON or overlong line.
contract/mcp-request/id · MCP requests · id — Echoed only as an i64 integer or a string of at most 128 bytes, else -32600 with id null; the one client-sent value written back.
contract/mcp-request/line-length · MCP requests · request line length — At most 65,536 bytes; a longer line is read to its terminator, dropped and answered -32700.
contract/mcp-request/notification · MCP requests · a request with no id — Treated as a notification: no reply, and nothing runs.
contract/mcp-server/protocol-error · MCP server · protocol error — A protocol error is a fixed code and message with no data.
contract/mcp-server/request-id · MCP server · request id — The admitted request id is the one client-sent value written back.
contract/mcp-server/transport · MCP server · transport — Newline-delimited JSON-RPC 2.0 on the invoking user's stdin and stdout, with no authentication and no environment read.
contract/mcp-tool-call-span/tool · The MCP tool_call span · tool — The verb table's word for the called tool; the span parents the call's command span.
contract/mcp-tool-result/size-bound · MCP tool results · size bound — A result's text over 1,048,576 bytes is replaced by the answer-too-large result; a result holding an image is bounded by its whole written form.
contract/snapshot-text/clickable · Snapshot text state words · clickable — Written after `focused` only when the element's current listeners hold `click` and it is no disabled form control.
contract/snapshot-text/id-field · Snapshot text · id field — Each node's id is written as a quoted `id=` field in the string's Debug form, so the field never holds a line break or a bare `"`; the snapshot has one text form and no JSON form, Display form or parser of the text.
contract/snapshot-text/invalid · Snapshot text state words · invalid — A bare word written for an element that states it is invalid.
contract/snapshot-text/name-field · Snapshot text · name field — Each line's accessible name is written as a quoted string in the string's Debug form, so a name holding a line break or a `"` stays inside its one field.
contract/snapshot-text/pressed · Snapshot text state words · pressed — A bare word written for an element that states it is pressed.
contract/snapshot-text/selected · Snapshot text state words · selected — A bare word written for an element that states it is selected.
contract/snapshot-text/state-words · Snapshot text · state words — Control states are written as the bare words `selected` / `unselected`, `pressed` / `unpressed` and `invalid`; in JSON answers they are booleans.
contract/snapshot-text/unpressed · Snapshot text state words · unpressed — A bare word written for an element that states it is not pressed.
contract/snapshot-text/unselected · Snapshot text state words · unselected — A bare word written for an element that states it is not selected.
contract/snapshot-value/masked-value · The snapshot's value · MASKED_VALUE — Eight U+2022 characters, one fixed marker standing for a password or file input's non-empty value on every carrier.
contract/stable-element-id/carriers · The stable element id · carriers — The stable element id leaves by the platform adapter, the driver's answers, MCP results and kept checks, and is written by escher's code to no log.
contract/stable-element-id/no-node-id · The stable element id · NodeId — No `NodeId` crosses any carrier of the stable element id.
contract/stable-element-id/virtual-node-form · Stable element id · virtual-node form — The id grammar reserves `{owner}///{key}` for a virtual node, spelled for a non-empty key with no `/`; no served id holds `///`, and the anchored path stays the only served id holding `//`.
decision/accessibility-adapter-content-classes · Content classes on the accessibility adapter crossing — The platform accessibility adapter crossing carries four author-derived content classes: element ids, accessible names, control states and descriptions. · why: Each widening ratified; the states by ruling, the names' and descriptions' upstream additions ratified as upstream's behaviour.
decision/anchored-id-tier · Anchored stable-id tier — An unkeyed element's stable id is anchored under the nearest author-keyed element of its own component instance before falling back to its component path or document path. · why: The anchored tier was ratified.
decision/author-id-on-accessibility-nodes · Stable id as accessibility author id — Under the accessibility feature each element node of the accessibility tree carries its stable id as its AccessKit author id, never on a text run, the document root or the synthetic window node, so it leaves the process only through the platform adapter. · why: Ratified; the crossing text runs and labels already take.
decision/closed-pipe-drops-answer · Closed pipe drops the answer — A write to a closed stdout is dropped and the command keeps its exit status, never a panic. · guard: a_closed_stdout_drops_the_answer_and_keeps_the_ending
decision/driver-installs-no-subscriber · Driver installs no subscriber — The driver crate and its command line install no tracing subscriber and read no environment variable; clocks are read only by the host's idle expiry, the MCP server on a request, the session's bounded load wait, and the client's bounded stop and ready waits. · except: An acting step runs under escher-telemetry's scoped log capture, which prints nothing.
decision/driver-no-subscriber-no-env · Driver installs no subscriber and reads no env — The driver crate and its session command line install no tracing subscriber and read no environment variable. · except: An acting step runs under escher-telemetry's scoped log capture, which prints nothing.
decision/driver-socket-closed-grammar · Driver socket closed request grammar — The driver session socket takes one request line per connection, parsed against a closed grammar under a size bound, one connection at a time under fixed bounds; anything else is refused with nothing run, and a decoded call runs through the session's one run path alone.
decision/falsy-boolean-attribute-removed · Falsy boolean attribute removed — A falsy value of a listed boolean attribute clears the attribute rather than writing it. · why: The tests falsy_boolean_attributes_are_removed and every_listed_boolean_attribute_is_removed_when_falsy pin it; snapshot disabled readings depend on it
decision/no-node-id-between-calls · No NodeId kept between calls — No NodeId is kept between calls.
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90. · guard: coverage leg
decision/reached-record-deduplicated-survives-removal · Reached record deduplicated and kept past element removal — Handler coverage relies on the reached record holding the stable id and event of each node whose listeners held a dispatched event, deduplicated, so a handler whose run removes its element still counts. · why: The reached test a_handler_whose_run_removes_its_element_is_recorded_by_its_id pins it; the coverage role reads reached_handlers, each handler once
decision/recording-dir-never-named · Recording directory is never exposed — No tool call, verb, argument or result field names the recording directory, and nothing of it is logged. · why: Ratified.
decision/recording-directory-never-named · Recording directory never named or logged — No tool call, verb, argument or result field names the recording directory, and nothing of it is logged.
decision/select-expanded-from-engine-state · Select expansion from engine state only — An author's `open` on a select and `aria-expanded` on a single select steer nothing: a single select's expanded state is the engine's own closed state, and `open` on a select does not match `:open`. · why: An engine change for every document. · guard: an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match
decision/select-list-label-never-logged · Select list row labels never logged — The select list's public surface is read-only but for one closing writer, and a list row's label, being page text, is never logged.
decision/select-list-live-reader · Open select list read through one live reader — The open select list is one document field, read through one live-list reader everywhere but reconciliation, the removal arm and row hover, which read the field directly.
decision/select-readers-resolve-node-closed-select-none · Select readers resolve the node and skip an open list — Each public select reader resolves its node so a stale id reads none, and the closed-select reader answers none for a select whose list is open. · why: The escher-driver lint and the snapshot's option bounds call closed_select_of and select_list_bounds, relying on none for a stale id or an open list
decision/session-id-record-bounded · Bounded in-process record of read ids — A session keeps a private record of ids its own snapshots read, at most 4096 ids of at most 1024 bytes each, the oldest forgotten first, read by no public item and written to no log, event, socket or file. · why: Kept to tell the stale and not-found refusals apart. · guard: the_record_holds_no_id_longer_than_a_call_can_name
decision/state-dir-path-is-parameter · Session state path is a parameter — The session state directory path is a caller parameter, never read from the environment or defaulted relative to the working directory.
decision/state-directory-path-is-parameter · State directory path is a parameter — The state directory path is a parameter, never an environment read or a working-directory-relative default.
decision/toggle-read-per-role · Toggle state read per role — `aria-checked` sets the toggle only on the five checkable roles and `aria-pressed` only on every other role; a native checkbox or radio reads neither attribute and gives its own checked state under a checkable role and nothing under any other. · why: Ratified.
contract/cli-exit-codes/0 · CLI exit codes · 0 — Accepted: the answer as one line of JSON on stdout.
contract/escher-session-coverage/answer-line · escher-session coverage role · answer line — The coverage answer holds handler references of ids, owner ids and event names and two counts, no process-local handle, and crosses only as one line on the command line's stdout, with no verb, socket or MCP carrier.
contract/mcp-initialize/protocol-revision · MCP initialize · protocol revision — Matched against a closed table of four supported revisions; any other revision asked for is answered with the newest of the four.
contract/mcp-tool-arguments/value-kinds · MCP tool arguments · value kinds — A string becomes text, an i64 a number, a bool a flag; any other value is refused malformed before anything runs, and a repeated key collapses to its last value.
contract/mcp-tool-call-span/ending · The MCP tool_call span · ending — One of accepted, refused or error.
contract/mcp-tool-result/screenshot · MCP tool results · screenshot — An accepted shot is an image block (base64 PNG, image/png) then a text block of the shot's line without png, isError false; a refused shot is one text block with isError true.
contract/recorded-check/arguments · Recorded check file · arguments — Argument names drawn from that verb's schema.
contract/recorded-check/tool · Recorded check file · tool — One of the eleven instance verbs.
contract/session-errors/state-dir-too-long · Session errors · state-dir-too-long — The state directory path is too long for a socket address; the message states the bound, 94 bytes on Linux and 90 elsewhere, holds no path, and the remedy names a shorter or relative path; status 3.

## api-security · ## API Security
decision/actions-answer-unpaged · The actions answer is unpaged — The actions verb takes no argument and a screen's whole answer is one answer under the 1,048,576-byte answer bound. · guard: stand_actions the_whole_cells_answer_is_within_the_answer_bound
decision/graph-answer-unpaged · The graph answer is unpaged — The graph verb takes no argument and a screen's whole graph is one answer under the 1,048,576-byte answer bound. · why: A part argument would widen the crossing. · guard: stand_graph the_whole_cells_answer_is_within_the_answer_bound
decision/mcp-server-over-stdio · MCP server over process stdio — The MCP server role serves the driver's verbs as tools over its own stdin and stdout only, binding no listener, port or socket, and answers only the process that spawned it, with no handshake, token, secret or peer check. · why: The crossing was ratified as is.
decision/no-wait-verb-fields · No wait argument or result on verbs — No driver verb has an argument or a result field of a wait.
decision/time-moves-only-on-advance · Time moves only on advance — Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph and actions wait on no load.
contract/mcp-server/request-line · MCP server · request line — A request line is bounded at 65,536 bytes.
contract/mcp-server/tool-result · MCP server · tool result — A tool result is bounded at 1,048,576 bytes, one holding an image measured by its whole written form and a text-only one by its text.
contract/driver-answer/settled · Driver command answer · settled — A step that ran and did not go quiet is a result, settled false with its busy class, not a refusal; the busy reading names a class only, never a URL, request, node, id, name or value.

## dependency-security · ## Dependency Security
decision/serde-json-read-only · serde_json, read only — serde_json, with no features and no preserve_order, reads the MCP server's untrusted stdin, a kept check's bytes and the driver's own answer line for replay, and writes nothing. · why: Chosen as read-only use.
decision/serde-json-untrusted-input-reader · serde_json as the untrusted input reader — serde_json, a workspace dependency with no preserve_order feature, is the reader of the MCP server's untrusted stdin, of a kept check's bytes and of the driver's answer line the replay compares.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project)
decision/telemetry-sink-target-allowlist · Telemetry sink target allowlist — escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level. · except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed

## secret-management · ## Secret Management
decision/driver-reads-only-rust-log · Driver reads only RUST_LOG — The escher-driver library and the escher-session binary read no environment variable beyond RUST_LOG.

## error-handling · ## Error Handling
decision/closed-refusal-cause-set · Closed refusal cause set — A driver command refusal is a typed value whose cause comes from a closed set of eight, with a fault only beside `malformed`; a new verb or argument shape is refused within the set, never by adding a cause. · why: The closed set of eight was ratified.
decision/mcp-errors-fixed-no-data · MCP protocol errors fixed, no data — MCP protocol errors are fixed codes and fixed messages and carry no data member.
decision/refusal-closed-cause-set · Command refusal as a closed set of causes — An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field. · why: So a refusal cannot hold anything a call supplied. · except: A Fault is carried only beside the malformed cause.
decision/refusal-holds-no-call-input · A refusal holds nothing a call supplied — Every refusal's name, meaning and remedy is a fixed static text holding no path, and a fault carries at most the schema's own argument name, so a refusal cannot hold anything a call supplied.
decision/refusal-log-cause-name-only · Only a refusal's cause name is logged — Exactly one thing of a refusal reaches a log: its cause's fixed name, as the `cause` field of the call's driver span; no fault, meaning, remedy or display text is logged.
decision/refusal-logged-by-cause-name · Refusal logged by cause name only — Of an escher-driver command refusal, only its cause's fixed name reaches a log.
contract/cli-exit-codes/2 · escher-session CLI exit codes · 2 — Usage: the argv shape is not one the command takes; one fixed usage line on stderr, stdout empty, nothing booted and no state created.
contract/mcp-protocol-errors/invalid-params · MCP protocol errors · invalid params — A fixed code and fixed message with no `data`; only the admitted request id is written back.
contract/mcp-protocol-errors/invalid-request · MCP protocol errors · invalid request — A fixed code and fixed message with no `data`; only the admitted request id is written back.
contract/mcp-protocol-errors/method-not-found · MCP protocol errors · method not found — A fixed code and fixed message with no `data`; only the admitted request id is written back.
contract/mcp-protocol-errors/parse-error · MCP protocol errors · parse error — A fixed code and fixed message with no `data`; only the admitted request id is written back.
contract/mcp-protocol-errors/unknown-tool · MCP protocol errors · unknown tool — A fixed code and fixed message with no `data`; no reply holds the tool name.
contract/mcp-tool-result/is-error · MCP tool result · isError — A refusal or session error is a tool result `{content:[{type:"text",text}], isError: true}`, its text the same JSON as on the command line, a refused `screenshot` included.
contract/refusal-causes/covered · Driver refusal causes · covered — The target is covered at its hit point; its fixed meaning states the hit's limits, among them that an option of a closed select reads covered, remedied by clicking the select first.
contract/refusal-causes/disabled · Driver refusal causes · disabled — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/fonts-unavailable · Driver refusal causes · fonts-unavailable — One of the eight closed causes, returned by the executor, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/malformed · Driver refusal causes · malformed — The one cause that carries a fault; a call breaking the schema, such as a `graph` or `actions` call with an argument, is refused with it and nothing runs.
contract/refusal-causes/not-found · Driver refusal causes · not-found — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/off-screen · Driver refusal causes · off-screen — The element lies outside the visible part, including outside a scrolling box that holds it; its remedy names the `scroll` verb.
contract/refusal-causes/stale · Driver refusal causes · stale — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/unknown-verb · Driver refusal causes · unknown-verb — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-json/cause · Refusal JSON · cause — The refusal's cause name, inside `{"refused":{...}}`, built from the cause's fixed strings.
contract/refusal-json/fault · Refusal JSON · fault — Present only on a `malformed` refusal, naming the fault.
contract/refusal-json/meaning · Refusal JSON · meaning — The cause's fixed meaning text.
contract/refusal-json/remedy · Refusal JSON · remedy — The cause's fixed remedy text.
decision/session-error-fixed-table · Session errors from a fixed table — An escher-driver session error is written from a fixed table, holding nothing a call supplied.
decision/session-errors-fixed-table · Session errors as one fixed table — Each session error's kind, meaning and remedy is one row of a fixed table served by `help`, every text a fixed string holding no path, carried on stdout and in MCP results as the program's own text. · why: Ratified as the program's own fixed text inside the ratified carriers.
decision/stale-node-ids-never-panic · Stale node ids never panic — A stale node id is skipped in layout children and resolves to none on lookup rather than panicking; a stale animation entry for a removed node is skipped on the next resolve. · guard: stale_interaction_state, stale_node_mapping, animations and custom_widget_layout tests
contract/cli-exit-codes/1 · escher-session CLI exit codes · 1 — The call was refused; the refusal JSON is written to stdout.
contract/cli-exit-codes/3 · CLI exit codes · 3 — Session error: the error object with kind, message and remedy on stdout and its fixed message on stderr; a boot that fails ends with its session error.
contract/session-error-json/kind · Session error JSON · kind — One fixed kebab-case word per session error variant, inside `{"error":{...}}`.
contract/session-error-json/message · Session error JSON · message — The error's fixed meaning, holding no path.
contract/session-error-json/remedy · Session error JSON · remedy — The error's fixed remedy text.

## logging-monitoring · ## Logging & Monitoring
decision/closed-span-lines · One sink line per closed span — The sink prints one line per closed span whose target the allowlist admits and nothing on creation, record, enter or exit; each span field is judged by the event rule, an outside-target span writes no byte, and a content-named span field's value is never stored. · why: A boundary widening of the sink, ratified as built.
decision/no-screen-content-in-logs · Screen content stays out of logs — No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer, a coverage check and a step's caused text reach no log line at any level. · guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds
decision/session-output-holds-no-caller-input · Session output holds no caller input — No escher-session output line holds anything the caller typed. · except: The mcp role's echoed request id.
contract/command-span/cause · The command span line · cause — Recorded when the call is refused, as a closed cause word such as fonts-unavailable.
contract/command-span/line · The command span line · line — A sink-installing host writes one closed command span line per call it runs.
contract/command-span/verb · The command span line · verb — Records the instance-level verb; a graph, actions or layout call records the verb alone, plus cause when refused.
contract/escher-session-mcp-stdout/protocol-lines · The escher-session mcp role's stdout · protocol lines — The mcp role writes JSON-RPC protocol lines only on stdout, and nothing at its end.
contract/escher-session-output/caller-text · escher-session output · caller-typed text — No output line echoes a caller's argument but the admitted request id the mcp role echoes; an answer carries each node's current value, so text typed into an unmasked control returns, a masked input reading only the mask.
contract/escher-session-stderr/usage-messages · escher-session stderr · usage messages — escher-session writes usage messages on stderr.
contract/escher-session-stdout/mcp-role · escher-session stdout · mcp role — The mcp role writes JSON-RPC lines only on stdout.
contract/tool-call-span/ending · The tool_call span line · ending — One of the closed words accepted, refused or error.
contract/tool-call-span/line · The tool_call span line · line — One closed line per MCP tools/call, the parent of that call's command span, holding tool and ending only.
contract/tool-call-span/tool · The tool_call span line · tool — The tool table's word for the call, a closed word outside the content-named fields; absent for an unknown tool.
contract/escher-session-output/accepted-stderr · escher-session output · stderr of an accepted command — An accepted command's stderr is empty with RUST_LOG unset.

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
absence/served-network-api — A served network API in a default build: no default feature set serves a TCP or UDP port
overview/api-security — No served network API exists in a default build: no default feature set serves a TCP or UDP port. The one default listener is local, the driver session's Unix-domain socket, and the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results. The opt-in devtools feature builds a CDP server listening on 127.0.0.1 with no authentication. The remaining controls govern outbound requests and in-document navigation.

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
