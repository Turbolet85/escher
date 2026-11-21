# security-plan · the rows that bind 2026-11-20-driver-debts — picked 197 of 569 (121 by every draw) · rule any · pin 2748

## threat-model-summary · ## Threat Model Summary
decision/caused-text-crossing · Caused text crossing — Each acting verb's answer carries caused on every ratified carrier when the step caused something: one item per panic, boundary error and kept record with its message, location, boundary, cause chain and fields, error as written within the text bound, every other content-named field redacted. · why: A boundary widening to the error text, ratified.
decision/coverage-answer-crossing · Coverage answer crossing — The coverage answer line carries ids, owner ids, event names and the checks and held counts only. · why: A boundary widening, ratified.
decision/coverage-check-files-untrusted · Kept check files read as untrusted — The coverage role reads each caller-named kept check as untrusted input through a bounded read and a strict parse; an unreadable, oversized, non-check or other-app file is refused malformed, holding nothing of the path or the file.
decision/graph-answer-crossing · Graph answer crossing — A graph answer carries the screen's UI graph on the socket, the command line, MCP and a kept check: ids, owner ids, component names, registered state names, handler event names, closed flag words and booleans, never a control value, typed text or NodeId. · why: A boundary widening, ratified.
decision/mcp-result-crossing · MCP result crossing — An MCP result carries ids, accessible names, control values with a password's or file input's as the mask, the snapshot text and an acting call's diff to the client; what a client does onward, a model provider included, is the client's. · why: A boundary widening, ratified.
decision/session-socket-carries-calls · Session socket carries calls and answers — The session socket carries the lifecycle and calls with their answers: an element id and typed text in; ids, accessible names, control values with a password's or file input's as the fixed mask, the snapshot text and the diff out; no NodeId crosses. · why: A boundary widening, ratified.
decision/snapshot-state-readings · Three state readings per control — The snapshot text and the diff hold three state readings per control, selected, pressed and invalid, and a highlighted reading on an open single select's highlighted row, all as closed words and booleans. · why: Inside the ratified crossing, ratified.
contract/driver-session-reply/masked-control-values · driver session socket replies · control values — A password's or file input's control value is carried masked.
contract/driver-session-reply/no-node-id · driver session socket replies · NodeId — No reply carries a NodeId.
contract/mcp-result/masked-control-values · MCP server results · control values — A password's or file input's control value is carried masked.

## input-validation · ## Input Validation
decision/attribute-text-leaves-by-ratified-carriers · Attribute text leaves only by ratified carriers — Markup attribute values, names and descriptions leave escher only by the ratified carriers. · except: The page's own form submission and events.
decision/boundary-widening-rule · Boundary widening rule — A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
decision/cli-closed-argv-reader · Closed argv reader — The session command line reads argv with a closed reader with fixed rules, and checks a call before the session address is asked for.
decision/cli-echoes-nothing-typed · Command line echoes nothing typed — No command-line ending writes back anything the caller typed, and no path on argv panics.
decision/closed-driver-verb-schema · Closed driver verb schema — Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing. · why: Each widening of the verb table is a widening of a validated surface, ratified.
decision/closed-value-field-no-widening · A closed-value field in a ratified carrier is no widening — A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does. · why: Ratified as the standing rule.
decision/command-schema-no-clock · Command schema reads no clock — The command schema and its validation read no clock.
decision/command-span-content-free · Command span holds no content — One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
decision/disabled-read-by-presence · disabled read by presence on form controls — The disabled attribute is read by its presence, whatever its value, and only on a button, input, select or textarea, for focusability, pointer and keyboard activation and the snapshot's enabled and clickable readings. · why: The presence reading was ratified.
decision/dispatch-recorded-only-with-signal-registry · Dispatch recorded only where a signal registry stands — The Dioxus event handler records a dispatch only where a signal registry is registered; on a page with none, the event's handlers run and nothing is recorded. · why: The dioxus_document tests a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is and the_reached_record_holds_on_a_page_with_no_registered_state pin it
decision/driver-outcomes-never-logged · Driver outcomes never logged — The driver crate logs and fields no command, call, argument, outcome, refusal, screen text, image, finding, session label or id record; an outcome is written only as a command-line answer, an MCP tool result, a session-socket reply or a recording file the caller asked for. · why: Outcomes leave the process only by the ratified crossing.
decision/driver-socket-closed-grammar · Driver socket closed request grammar — The driver session socket takes one request line per connection, parsed against a closed grammar under a size bound, one connection at a time under fixed bounds; anything else is refused with nothing run, and a decoded call runs through the session's one run path alone.
decision/enter-activates-focused-control · Enter activates the focused control — A pressed `Enter` on the focused node dispatches a click on it when it is no disabled form control and is a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`. · why: An engine change for every document and a boundary widening, ratified.
decision/event-is-trusted-by-source · isTrusted by dispatch source — A script-dispatched event reads isTrusted false, and an event the user agent dispatches reads isTrusted true. · why: A boundary widening by upstream merge, ratified.
decision/mask-holds-on-every-carrier · Value mask holds on every carrier — A masked value stays masked on every carrier the snapshot reaches: its text form, a diff, the driver's return, the session socket and MCP tool results. · why: Ratified. · guard: stand_act_filled, stand_mcp_masked, stand_masks_on_new_answers · except: A caused item's message and its error field hold what their author wrote; every other content-named field in it reads as redacted.
decision/masked-input-values · Password and file input values masked — A password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path. · except: The DOM and form submission keep the value.
decision/mcp-bounded-lines · Bounded MCP requests and results — MCP request lines and results are bounded, and tool arguments are typed before validation.
decision/mcp-listing-fixed-text · MCP listing is fixed program text — The MCP tool listing's descriptions are the program's own fixed table text: each verb's help and each result field's name, kind and help. · why: Ratified as the program's own text inside the ratified carrier.
decision/mcp-no-print-or-panic · MCP server never prints or panics — The MCP server has no print, unwrap, expect or panic outside the tests.
decision/multiple-select-keys-and-clicks · Multiple select keys and clicks — In a multiple select, a plain arrow, Home, End or click chooses that row alone, Shift spans from the anchor, Ctrl or Meta moves the current row by key and flips a row by click, and disabled rows are never chosen. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select · except: Ctrl+A is not modelled.
decision/multiple-select-set-exits · Where a multiple select's chosen set leaves — A multiple select's chosen set leaves only as form entries, as input and change values carrying one entry per chosen option, as the snapshot's value of the first chosen option in tree order, and as the options' tree nodes. · why: An engine change for every document and a boundary widening, ratified.
decision/no-echo-of-caller-input · No echo of caller input — No command-line ending writes back anything the caller typed: no path, label, id, value or file content.
decision/no-node-id-between-calls · No NodeId kept between calls — No NodeId is kept between calls.
decision/no-panic-on-call-input · No panic on call-supplied input — Neither the driver command schema's validation nor the executor has unwrap, expect, panic or unreachable on anything a call supplies.
decision/no-process-handle-crosses · No process-local handle crosses with an id — Neither the stable id, the snapshot, the diff nor any driver session, answer or refusal carries a `NodeId`, `ElementId`, `ScopeId` or pointer; only the stable id text crosses.
decision/one-command-span-per-call · One command span per call — Every call handed to the session is wrapped in one INFO span named command under the escher_driver target, entered before validation so a refusal is inside it, holding fixed words, booleans and counts only.
decision/one-validation-path-for-every-carrier · One validation path for every carrier — The session socket, the command line and MCP each pass a call through the schema's validation before anything runs, and nothing public runs a command that did not come from that validation. · why: The crossing to external input surfaces was ratified with this check as the one they all pass through.
decision/paged-answer-closed-decoding · Paged answer decoded by closed fields — The cold-agent check decodes a paged answer only as exactly the fields text, part and parts.
decision/password-file-value-masked · Password and file input values masked — The snapshot reads a password or file input's non-empty value as one fixed marker whatever was typed or chosen, an empty one as empty, never the text, its length or a file's host path. · why: Ratified. · guard: stand_act_filled, stand_mcp_masked · except: The DOM is untouched: a file input's value attribute still holds the host path and form submission still carries it.
decision/reached-handler-record-without-log · Reached-handler record without log or NodeId — Where recording of reached handlers is switched on, each reached handler's stable id and event name is written into a root-context record ahead of the handler's dispatch, with no log site and no `NodeId`.
decision/reached-record-deduplicated-survives-removal · Reached record deduplicated and kept past element removal — Handler coverage relies on the reached record holding the stable id and event of each node whose listeners held a dispatched event, deduplicated, so a handler whose run removes its element still counts. · why: The reached test a_handler_whose_run_removes_its_element_is_recorded_by_its_id pins it; the coverage role reads reached_handlers, each handler once
decision/recorded-check-strict-reader · Recorded check is read strictly as untrusted — A kept check is read as untrusted bytes, at most 1,048,576 bytes and 400 records of closed keys; anything else is refused `malformed` with no fault, holding nothing of the file, with no panic path.
decision/recorded-check-untrusted · Recorded check bytes are untrusted — A kept check is read by a strict reader bounded in bytes and records, with closed keys and schema-named arguments, anything else refused as malformed holding nothing of the file, no panic path; every replayed step is validated again; the coverage role reads checks through the same reader.
decision/replay-through-session-run · Replay runs each step through the session — A replayed check runs each kept step through the session's own validated run alone, runs nothing on a session of another app and stops at the first step whose acceptance or kept answer differs.
decision/select-choice-one-write-path · Every select choice through one write path — Every select choice, by key, click or open list, goes through one write path that marks the changed set and dispatches input then a non-cancelable, bubbling change, a choice of the already chosen option firing nothing. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select, select_popup
decision/select-keys-closed-match · Select keys pass a closed match — A focused select's keys, an open list's keys and a multiple select's keys each pass one closed key match, and every key outside the match does nothing. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select, select_popup
decision/session-socket-bounds · Session socket bounds — One connection at a time; each read and write under 2 s, a client's read of an answer under 30 s, a request line at most 16,384 bytes, a lifecycle reply 128 bytes, an answer 1,048,576 bytes. · guard: an_answer_read_past_its_bound_is_a_timeout
decision/session-socket-closed-grammar · Session socket request is a closed grammar — A session socket connection carries one request line and one reply line, parsed against a closed versioned grammar with strict percent-decoding; anything outside it is refused and runs nothing, and a refusal or broken connection changes nothing.
decision/single-select-selectedness · Single-choice select selectedness — In every document, remote pages included, a select without multiple holds its choice as its options' checked state and settles every recorded select by HTML's selectedness setting algorithm: the last chosen, else the first option not disabled. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select · except: An option of a closed single select has no click arm and stays refused covered; a single select's size is not read.
decision/single-validation-gate · Single command validation gate — Every call, from the socket, the command line or MCP, is checked against a closed static table of verbs and bounded argument kinds, the first failure in a fixed order, and nothing public runs a command that did not come from that validation.
decision/socket-carries-screen-reading · Session socket carries what the screen reads — An answer carries what the screen reads: ids, accessible names, control values, password and file input masked, snapshot text, diff, lint findings, UI graph, layout readings and an acting step's caused text scrubbed by its own field names, error kept, never a NodeId or an attribute value as typed. · why: The crossing was ratified.
decision/space-keyup-activates-focused-control · Space release activates the focused control — A released Space on the focused node dispatches a click for a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is not disabled; a link, a text input and a range input take no Space. · why: An engine change for every document and a boundary widening, ratified.
decision/target-refusal-order · Fixed order of target refusals — A click or type on an element that cannot take it is refused with nothing run, the first of disabled, off-screen, covered that holds, leaving the snapshot and both focus readings as they were; scroll reports in_view instead of refusing. · why: The detection of refusal causes by code was ratified.
contract/cli-exit-codes/0 · CLI exit codes · 0 — Accepted: the answer as one line of JSON on stdout.
contract/cli-flags/part · escher-session flags · --part — Followed by one value, whatever that word is.
contract/cli-roles/coverage · escher-session roles · coverage — coverage <app> <check>... with no --session or --record, else usage; each check read up to 1,048,576 bytes, and an unreadable, oversized, non-check or other-app file refused malformed, status 1, holding nothing of the path or file.
contract/coverage-answer/line · The coverage answer · line — One line {"unexercised":[…],"checks":n,"held":m} on stdout, status 0, holding ids, owner ids, event names and the two counts only.
contract/driver-argument-kinds/part · Driver argument kinds · part — A whole number from 1 to 1000.
contract/driver-session-reply/no-attribute-text · driver session socket replies · attribute text — A reply never carries an attribute's text, except what an author wrote into a record's message or into the `error` field of a `caused` item.
contract/driver-session-reply/state-readings · driver session socket replies · state readings — State readings are carried as closed words and booleans.
contract/driver-verbs/graph · Driver verbs · graph — Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[…],"states":[…]} of ids, owner ids, component names, state names, handler event names, flag words and booleans, never a control value, typed text or NodeId.
contract/escher-session-coverage/answer-line · escher-session coverage role · answer line — The coverage answer holds handler references of ids, owner ids and event names and two counts, no process-local handle, and crosses only as one line on the command line's stdout, with no verb, socket or MCP carrier.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The call was accepted: one uncoloured JSON line on stdout holds the answer, or for the coverage role its one coverage line.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The call was refused: one uncoloured JSON line on stdout holds the refusal, or for the coverage role a malformed refusal.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — A usage error: the usage line goes to stderr and stdout stays empty.
contract/graph-answer/nodes-states · Graph answer · nodes and states — `{"nodes":[…],"states":[…]}` holding ids, component names, state names, handler event names, the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or NodeId.
contract/mcp-tool-arguments/value-kinds · MCP tool arguments · value kinds — A string becomes text, an i64 a number, a bool a flag; any other value is refused malformed before anything runs, and a repeated key collapses to its last value.
contract/mcp-tool-result/size-bound · MCP tool results · size bound — A result's text over 1,048,576 bytes is replaced by the answer-too-large result; a result holding an image is bounded by its whole written form.
contract/recorded-check/answer · Recorded check file · answer — Required and a JSON object; key order is not enforced.
contract/recorded-check/arguments · Recorded check file · arguments — Argument names drawn from that verb's schema.
contract/session-socket-reply/ok-accepted · Session socket reply · ok v3 accepted — The call ran and was accepted; its answer JSON follows on the line.
contract/session-socket-reply/ok-oversize · Session socket reply · ok v3 oversize — The call ran but its answer is over 1,048,576 bytes.
contract/session-socket-reply/ok-refused · Session socket reply · ok v3 refused — The call was refused; the refusal JSON follows on the line.
contract/session-socket-request/call · Session socket request · call — `call v3 <verb> [<name>=<kind>:<value>]…` runs a validated verb, kinds `t` text, `n` whole number, `f` flag, verb, names and text percent-escaped.
contract/snapshot-text/state-words · Snapshot text · state words — Control states are written as the bare words `selected` / `unselected`, `pressed` / `unpressed` and `invalid`; in JSON answers they are booleans.
contract/snapshot-value/masked-value · The snapshot's value · MASKED_VALUE — Eight U+2022 characters, one fixed marker standing for a password or file input's non-empty value on every carrier.
contract/stable-element-id/carriers · The stable element id · carriers — The stable element id leaves by the platform adapter, the driver's answers, MCP results and kept checks, and is written by escher's code to no log.
contract/stable-element-id/no-node-id · The stable element id · NodeId — No `NodeId` crosses any carrier of the stable element id.
decision/button-click-focuses · A button click focuses the button — A click on a `button` of any type or an `input` of type `submit`, `reset` or `button` focuses it and submits its form owner when it is a submit button, a disabled form control stopping the walk first. · why: An engine change for every document and a boundary widening, ratified.
decision/cancelled-pointerup-still-clicks · A cancelled pointerup still clicks — A pointerup whose handler cancels it still clicks and still ends a scrollbar drag. · why: An engine change for every document by upstream merge, ratified. · guard: pointerup_cancelled
decision/closed-pipe-drops-answer · Closed pipe drops the answer — A write to a closed stdout is dropped and the command keeps its exit status, never a panic. · guard: a_closed_stdout_drops_the_answer_and_keeps_the_ending
decision/control-states-closed-values-only · Control states cross as closed values — Control states leave the process only as closed enums or booleans, on the accessibility adapter and in the snapshot's words and JSON booleans, never as the attribute's text, and no unrecognized or long value is written anywhere as typed. · why: A ratified widening of the adapter crossing by a new author-derived class.
decision/descriptions-adapter-only · Descriptions cross only to the accessibility adapter — `aria-describedby` targets and `aria-description` text reach only the platform accessibility adapter; the snapshot reads no description, so no driver or MCP carrier, verb, field, log field or span carries one. · why: Upstream's behaviour, ratified as a boundary widening by upstream merge. · guard: accessibility_description
decision/diff-counts-only-in-logs · Only diff sizes reach a log — A diff's content never reaches a log or event, while its three sizes (counts of added, removed and changed nodes) are recorded on the driver's command span, printed at `info` or below. · why: Ratified: a diff's content never reaches a log while its three sizes may.
decision/driver-type-on-select · Driver type on a select — The driver's type on any select focuses it with no click and no DOM focus events, clears the typeahead, closes any list, and a text opening on a space types nothing. · why: A click on a multiple select would land on the centre row and a Space would flip a row; a boundary widening, ratified.
decision/engine-change-event-checkable · Engine fires change on checkbox and radio — A click on a checkbox always, and on a radio only where it was not checked before, sends `change` after `input` with the same value; the page script runtime synthesizes no `change`, so a script hears one, from the engine. · why: An engine change for every document and a boundary widening, ratified. · guard: checkable_change_event
decision/id-lookup-anew-each-call · Caller-supplied id looked up anew — An acting command that takes a caller-supplied id looks it up on the screen as it reads now, on every call, never by index and keeping no node handle between calls; an id that names nothing is refused with nothing run and no panic.
decision/mcp-over-stdio · MCP over stdio only — The MCP server speaks newline-delimited JSON-RPC 2.0 on the invoking process's stdin and stdout, with no listener, port, socket, authentication or environment read. · why: The MCP carrier and its bounds were ratified as is.
decision/multiple-select-size-rows · Multiple select size sets its rows — A multiple select's size, a whole number above zero capped at 65534, makes it that many rows tall and a key scrolls the next row in; zero or a non-numeric value gives no hint, and a single select's size is unread. · why: A boundary widening, ratified. · guard: size_reads_a_whole_number_above_zero_capped
decision/range-keyboard-actions · Range input key actions — On a focused range input with no `disabled` attribute, `ArrowRight` or `ArrowUp` steps up, `ArrowLeft` or `ArrowDown` steps down, `Home` and `End` go to the minimum and maximum, and every other key does nothing. · why: Ratified.
decision/recording-bounds-end-recording · A failed or overlong recording ends — A failed write or a passed bound of 400 records or 1,048,576 bytes ends the recording, its check removed, the call run and not rolled back and answered `recording-failed`.
decision/recording-dir-never-named · Recording directory is never exposed — No tool call, verb, argument or result field names the recording directory, and nothing of it is logged. · why: Ratified.
decision/recording-directory-never-named · Recording directory never named or logged — No tool call, verb, argument or result field names the recording directory, and nothing of it is logged.
decision/recording-ends-on-failure · Recording ends on failure — A failed write or a passed bound ends the recording.
decision/select-choice-exits · Where a select's choice leaves — A select's choice leaves only as form submission's entries, the input and change events' value, the snapshot's value and the options' accessibility-tree nodes, inside the ratified name and control-value crossings. · why: A boundary widening, ratified.
decision/select-list-keys-and-painted-labels · Single select list keys and painted labels — Enter and Space open a single select's engine-painted list, Space only with no typed text standing, and an option's and a group's label are painted in the open list. · why: Ratified as an engine change for every document and a boundary widening.
decision/select-typeahead-private · Select typeahead kept private and short-lived — Select typeahead keeps the typed text in the document's private buffer, starts afresh for another select or after 1 s of the document's clock, and the text is compared only, never kept past 1 s and never carried out. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select
decision/session-label-only-caller-string · Session label is the one echoed caller string — The session label is never text read from the document and is the one caller string a reply carries.
decision/signal-registry-holds-no-state-value · Signal registry holds no state value — The in-process signal registry takes dioxus-core types alone, holds no value of a state, and has no log site, print, env read or `unwrap` on app input.
decision/stable-id-exits · Ratified exits of the stable id — The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files. · why: Each carrier ratified as a boundary widening.
decision/unknown-target-refused · Call on an unknown or unfit element refused — A call whose id names no element, or an element that cannot take the call, is refused with nothing run.
contract/cli-flags/id · escher-session flags · --id — Followed by one value, whatever that word is.
contract/cli-flags/key · escher-session flags · --key — Followed by one value, whatever that word is.
contract/cli-flags/shift · escher-session flags · --shift — Bare, taking no value.
contract/driver-argument-kinds/flag · Driver argument kinds · flag — True or false.
contract/driver-argument-kinds/id · Driver argument kinds · id — Text of 1 to 1024 bytes.
contract/driver-argument-kinds/key · Driver argument kinds · key — One name of a closed twelve-name list.
contract/driver-call-id/length-bound · Driver call id · length bound — An id a call names, from the command-line `--id` flag, the socket's `call` request or an MCP tool's `id` argument, is bounded at 1 to 1024 bytes, checked in the client before a session is touched and again in the host.
contract/driver-command-span/verb · The driver command span · verb — The verb table's own word, never the caller's verb text.
contract/masked-input-value/masked-stand-in · Masked input values · MASKED_STAND_IN — A recorded masked `type` is kept as `MASKED_STAND_IN`.
contract/mcp-errors/32602 · MCP protocol errors · -32602 — invalid params or unknown tool, with no data: params not an object with a string name and an optional object arguments, or a name outside the verb table.
contract/mcp-initialize/protocol-revision · MCP initialize · protocol revision — Matched against a closed table of four supported revisions; any other revision asked for is answered with the newest of the four.
contract/mcp-request/id · MCP requests · id — Echoed only as an i64 integer or a string of at most 128 bytes, else -32600 with id null; the one client-sent value written back.
contract/mcp-request/line-length · MCP requests · request line length — At most 65,536 bytes; a longer line is read to its terminator, dropped and answered -32700.
contract/mcp-request/notification · MCP requests · a request with no id — Treated as a notification: no reply, and nothing runs.
contract/mcp-server/protocol-error · MCP server · protocol error — A protocol error is a fixed code and message with no data.
contract/mcp-server/request-id · MCP server · request id — The admitted request id is the one client-sent value written back.
contract/mcp-server/transport · MCP server · transport — Newline-delimited JSON-RPC 2.0 on the invoking user's stdin and stdout, with no authentication and no environment read.
contract/mcp-tool-call-span/ending · The MCP tool_call span · ending — One of accepted, refused or error.
contract/mcp-tool-call-span/tool · The MCP tool_call span · tool — The verb table's word for the called tool; the span parents the call's command span.
contract/mcp-tool-result/screenshot · MCP tool results · screenshot — An accepted shot is an image block (base64 PNG, image/png) then a text block of the shot's line without png, isError false; a refused shot is one text block with isError true.
contract/recorded-check/accepted · Recorded check file · accepted — One of the closed record keys, whether the call was accepted.
contract/recorded-check/digest · Recorded check file · digest — 16 lowercase hex characters.
contract/recorded-check/masked · Recorded check file · masked — True on a `type` record alone.
contract/recorded-check/masked-stand-in · A recorded check · MASKED_STAND_IN — The text "masked", with masked:true, stands for a recorded type to a masked or unresolved control.
contract/recorded-check/start-record · Recorded check file · start record — The array opens with `{"tool":"start","arguments":{"app":…},"format":1}`; a check without format reads as format 1, and any other format value, of whatever JSON type, is refused malformed, holding nothing of the bytes.
contract/recorded-check/tool · Recorded check file · tool — One of the eleven instance verbs.
contract/session-errors/state-dir-too-long · Session errors · state-dir-too-long — The state directory path is too long for a socket address; the message states the bound, 94 bytes on Linux and 90 elsewhere, holds no path, and the remedy names a shorter or relative path; status 3.
contract/session-socket-reply/refused-malformed · Session socket reply · refused malformed — A request line the grammar does not read or over 16,384 bytes; nothing runs.
contract/snapshot-text/selected · Snapshot text state words · selected — A bare word written for an element that states it is selected.
contract/snapshot-text/unselected · Snapshot text state words · unselected — A bare word written for an element that states it is not selected.
contract/stable-element-id/unresolved-node · Stable element id · non-element, stale or detached node — A non-element, stale or detached node reads no id, and no path computation panics.

## data-protection · ## Data Protection
decision/coverage-line-no-user-data · Coverage line carries no user data — The coverage answer line holds ids, owner ids, event names and counts only, never a control value, typed text, path or NodeId. · guard: stand_handler_coverage a_typed_sentinel_never_reaches_the_coverage_line
decision/kept-check-answer-whole · Kept check keeps each answer whole — A kept check keeps each answer as the line the driver wrote, masked as on every carrier, with a shot kept as its width and height and never its pixels. · why: Ratified, a shot's pixels excepted.
decision/kept-check-call-content · Kept check call-content class — A kept check holds call content: target ids, press keys, advance ms and text typed into unmasked controls, a masked or unresolved type's text kept as MASKED_STAND_IN. · why: Ratified.
decision/kept-check-graph-answer · Kept check holds the graph answer — A kept graph answer holds ids, owner ids, component names, registered state names, handler event names and closed flag words and booleans, never a control value or typed text. · why: A boundary widening, ratified.
contract/coverage-line/checks · Coverage answer line · checks — The count of checks replayed.
contract/coverage-line/held · Coverage answer line · held — The count of checks that held.
contract/coverage-line/unexercised · Coverage answer line · unexercised — An array of id, owner and event per unexercised handler, owner left out when absent.
contract/kept-check-file/contents · Kept check files · contents — A kept check holds per call its verb, arguments, acceptance, digest and masked answer, never a pixel.
contract/kept-check-file/entry · Kept check file · entry — A check is a JSON array: an opening record of the app label and the format, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer and the answer itself as answer.
decision/committed-counter-checks · Committed stand-Counter checks — The only committed kept checks are the two of the repository's scripted stand-Counter sessions, holding no typed text, host path, name or secret and no `screenshot` step, read only through the strict `Check::from_bytes`. · why: The one exception to a kept check's owner-only medium.
decision/kept-check-actions-answer · Kept check holds the actions answer — A kept actions answer holds ids, owner ids, keyed booleans, the table's verb words and closed cause words, never a control value, typed text or NodeId. · why: A boundary widening, ratified. · guard: stand_replay a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes
decision/kept-check-owner-only · Kept checks are owner-only — A recorded check is written 0600 into a 0700 directory the launcher named, is never logged, and is never removed by escher but for its own check when a recording ends. · why: Ratified. · guard: recording::the_committed_checks_are_what_the_binary_records · except: Two scripted stand-Counter checks, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.

## api-security · ## API Security
decision/bounded-load-wait · Bounded wait on loads in flight — A harness settle never waits on a load in flight, and the driver's acting step waits on one for at most `LOAD_WAIT`. · why: So the absent request timeout cannot hang a settle or a command.
decision/graph-answer-unpaged · The graph answer is unpaged — The graph verb takes no argument and a screen's whole graph is one answer under the 1,048,576-byte answer bound. · why: A part argument would widen the crossing. · guard: stand_graph the_whole_cells_answer_is_within_the_answer_bound
decision/load-wait-bound · Acting commands bounded by LOAD_WAIT — An acting driver command waits on a load in flight for at most LOAD_WAIT, 5 s of wall time, moving no time on the held instance, then returns the load still in flight as settled false with its busy class. · why: The absent request timeout cannot hang a command.
decision/no-wait-verb-fields · No wait argument or result on verbs — No driver verb has an argument or a result field of a wait.
decision/time-moves-only-on-advance · Time moves only on advance — Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph and actions wait on no load.
contract/driver-answer/settled · Driver command answer · settled — A step that ran and did not go quiet is a result, settled false with its busy class, not a refusal; the busy reading names a class only, never a URL, request, node, id, name or value.
contract/mcp-server/help · MCP server · help — The help tool is answered by the server itself from the schema's fixed tables, running nothing on a session.
contract/mcp-server/tool-result · MCP server · tool result — A tool result is bounded at 1,048,576 bytes, one holding an image measured by its whole written form and a text-only one by its text.
contract/session-socket/oversize · Driver session socket · oversize — An answer is at most 1,048,576 bytes on its JSON line; over it the reply is oversize, read by the client as AnswerTooLarge.
contract/mcp-server/request-line · MCP server · request line — A request line is bounded at 65,536 bytes.
contract/session-socket/request-line · Driver session socket · request line — A request line is at most 16,384 bytes.

## dependency-security · ## Dependency Security
decision/serde-json-read-only · serde_json, read only — serde_json, with no features and no preserve_order, reads the MCP server's untrusted stdin, a kept check's bytes and the driver's own answer line for replay, and writes nothing. · why: Chosen as read-only use.
decision/serde-json-untrusted-input-reader · serde_json as the untrusted input reader — serde_json, a workspace dependency with no preserve_order feature, is the reader of the MCP server's untrusted stdin, of a kept check's bytes and of the driver's answer line the replay compares.

## error-handling · ## Error Handling
decision/closed-refusal-cause-set · Closed refusal cause set — A driver command refusal is a typed value whose cause comes from a closed set of eight, with a fault only beside `malformed`; a new verb or argument shape is refused within the set, never by adding a cause. · why: The closed set of eight was ratified.
decision/coverage-refuses-bad-checks-malformed · Coverage refuses an unusable check as malformed — The `coverage` role refuses a check file it cannot read, one past the check bound, one that is no check or a check of another app as `malformed`, holding nothing of the path or the file, and adds no cause.
decision/mcp-errors-fixed-no-data · MCP protocol errors fixed, no data — MCP protocol errors are fixed codes and fixed messages and carry no data member.
decision/refusal-closed-cause-set · Command refusal as a closed set of causes — An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field. · why: So a refusal cannot hold anything a call supplied. · except: A Fault is carried only beside the malformed cause.
decision/refusal-holds-no-call-input · A refusal holds nothing a call supplied — Every refusal's name, meaning and remedy is a fixed static text holding no path, and a fault carries at most the schema's own argument name, so a refusal cannot hold anything a call supplied.
decision/refusal-log-cause-name-only · Only a refusal's cause name is logged — Exactly one thing of a refusal reaches a log: its cause's fixed name, as the `cause` field of the call's driver span; no fault, meaning, remedy or display text is logged.
decision/refusal-logged-by-cause-name · Refusal logged by cause name only — Of an escher-driver command refusal, only its cause's fixed name reaches a log.
decision/stale-node-ids-never-panic · Stale node ids never panic — A stale node id is skipped in layout children and resolves to none on lookup rather than panicking; a stale animation entry for a removed node is skipped on the next resolve. · guard: stale_interaction_state, stale_node_mapping, animations and custom_widget_layout tests
contract/cli-exit-codes/1 · escher-session CLI exit codes · 1 — The call was refused; the refusal JSON is written to stdout.
contract/cli-exit-codes/2 · escher-session CLI exit codes · 2 — Usage: the argv shape is not one the command takes; one fixed usage line on stderr, stdout empty, nothing booted and no state created.
contract/mcp-protocol-errors/invalid-params · MCP protocol errors · invalid params — A fixed code and fixed message with no `data`; only the admitted request id is written back.
contract/mcp-tool-result/is-error · MCP tool result · isError — A refusal or session error is a tool result `{content:[{type:"text",text}], isError: true}`, its text the same JSON as on the command line, a refused `screenshot` included.
contract/refusal-causes/malformed · Driver refusal causes · malformed — The one cause that carries a fault; a call breaking the schema, such as a `graph` or `actions` call with an argument, is refused with it and nothing runs.
contract/refusal-json/cause · Refusal JSON · cause — The refusal's cause name, inside `{"refused":{...}}`, built from the cause's fixed strings.
contract/refusal-json/fault · Refusal JSON · fault — Present only on a `malformed` refusal, naming the fault.
decision/dropped-anonymous-block-clears-parent · A dropped anonymous block clears its children's parent — Dropping an anonymous block clears the layout parent of every child still naming it, so a key press on a control whose block was dropped does not panic and still acts. · why: So the id does not go stale, curing every reader of a stale layout parent. · guard: key_press_after_container_lost_its_box
decision/reach-error-closed-set · Reach errors as a closed set with fixed texts — A reach error is one of a closed set: unknown state, refused at a step with the step's index and its closed refusal cause, or not settled at a step with the busy reading; each reads a fixed text holding nothing a caller supplied. · guard: a_reach_error_reads_fixed_text_naming_no_name_tried
decision/select-row-layout-none-without-styles · Select row layout reads none for an unstyled row — The select list's public readers and writer read a row with no styles as None and add no panic path. · why: The blitz-paint select list painter skips a row whose select_row_layout answers None instead of failing
decision/session-error-fixed-table · Session errors from a fixed table — An escher-driver session error is written from a fixed table, holding nothing a call supplied.
decision/session-errors-fixed-table · Session errors as one fixed table — Each session error's kind, meaning and remedy is one row of a fixed table served by `help`, every text a fixed string holding no path, carried on stdout and in MCP results as the program's own text. · why: Ratified as the program's own fixed text inside the ratified carriers.
decision/try-doc-none-while-borrowed · try_doc answers none while the document is borrowed — Event handling may ask for the document while it is mutably borrowed and gets None instead of a panic. · why: The browser status bar reads the document through try_doc and returns early instead of panicking while event handling holds the borrow
contract/refusal-causes/covered · Driver refusal causes · covered — The target is covered at its hit point; its fixed meaning states the hit's limits, among them that an option of a closed select reads covered, remedied by clicking the select first.
contract/refusal-json/meaning · Refusal JSON · meaning — The cause's fixed meaning text.
contract/refusal-json/remedy · Refusal JSON · remedy — The cause's fixed remedy text.

## logging-monitoring · ## Logging & Monitoring
decision/no-screen-content-in-logs · Screen content stays out of logs — No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer, a coverage check and a step's caused text reach no log line at any level. · guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds
decision/session-output-holds-no-caller-input · Session output holds no caller input — No escher-session output line holds anything the caller typed. · except: The mcp role's echoed request id.
contract/command-span/line · The command span line · line — A sink-installing host writes one closed command span line per call it runs.
contract/command-span/verb · The command span line · verb — Records the instance-level verb; a graph, actions or layout call records the verb alone, plus cause when refused.
contract/escher-session-coverage-line/content · The escher-session coverage role's stdout line · content — The coverage line holds ids, owner ids, event names and two counts, and nothing of a named path.
contract/escher-session-output/caller-text · escher-session output · caller-typed text — No output line echoes a caller's argument but the admitted request id the mcp role echoes; an answer carries each node's current value, so text typed into an unmasked control returns, a masked input reading only the mask.
contract/escher-session-stderr/usage-messages · escher-session stderr · usage messages — escher-session writes usage messages on stderr.
contract/escher-session-stdout/command-line · escher-session stdout · command result line — escher-session writes one uncoloured JSON line per command on stdout.
decision/closed-span-lines · One sink line per closed span — The sink prints one line per closed span whose target the allowlist admits and nothing on creation, record, enter or exit; each span field is judged by the event rule, an outside-target span writes no byte, and a content-named span field's value is never stored. · why: A boundary widening of the sink, ratified as built.
contract/command-span/cause · The command span line · cause — Recorded when the call is refused, as a closed cause word such as fonts-unavailable.
contract/escher-session-mcp-stdout/protocol-lines · The escher-session mcp role's stdout · protocol lines — The mcp role writes JSON-RPC protocol lines only on stdout, and nothing at its end.
contract/escher-session-output/accepted-stderr · escher-session output · stderr of an accepted command — An accepted command's stderr is empty with RUST_LOG unset.
contract/escher-session-output/esc-byte · escher-session output · ESC byte — No output line holds an ESC byte; output is uncoloured whatever the stream is.
contract/escher-session-stdout/mcp-role · escher-session stdout · mcp role — The mcp role writes JSON-RPC lines only on stdout.
contract/tool-call-span/ending · The tool_call span line · ending — One of the closed words accepted, refused or error.
contract/tool-call-span/line · The tool_call span line · line — One closed line per MCP tools/call, the parent of that call's command span, holding tool and ending only.

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
