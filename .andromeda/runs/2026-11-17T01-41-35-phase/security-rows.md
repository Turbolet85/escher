# security-plan · the rows that bind 2026-11-17-layout-on-the-carriers — picked 203 of 553 (126 by every draw) · rule any · pin 1580

## threat-model-summary · ## Threat Model Summary
decision/actions-answer-crossing · Actions answer crossing — An actions answer carries what each actionable node can take now on the socket, the command line, MCP and a kept check: ids, owner ids, keyed booleans, verb-table words and closed cause words, never a control value, typed text or NodeId. · why: A boundary widening, ratified.
decision/capability-statement-mcp-crossing · Committed statement of the MCP crossing — A committed capability statement tells its readers what an MCP client carries onward: the spawning client as the boundary with no authentication, the carried set, and onward use as the client's; it adds no carrier.
decision/coverage-check-files-untrusted · Kept check files read as untrusted — The coverage role reads each caller-named kept check as untrusted input through a bounded read and a strict parse; an unreadable, oversized, non-check or other-app file is refused malformed, holding nothing of the path or the file.
decision/mcp-result-crossing · MCP result crossing — An MCP result carries ids, accessible names, control values with a password's or file input's as the mask, the snapshot text and an acting call's diff to the client; what a client does onward, a model provider included, is the client's. · why: A boundary widening, ratified.
decision/session-socket-carries-calls · Session socket carries calls and answers — The session socket carries the lifecycle and calls with their answers: an element id and typed text in; ids, accessible names, control values with a password's or file input's as the fixed mask, the snapshot text and the diff out; no NodeId crosses. · why: A boundary widening, ratified.
contract/actions-answer/verbs · The actions answer · verbs — A list of entries each holding verb, a word of the verb table, and cause, a closed cause word.
contract/driver-session-reply/masked-control-values · driver session socket replies · control values — A password's or file input's control value is carried masked.
contract/driver-session-reply/no-node-id · driver session socket replies · NodeId — No reply carries a NodeId.
contract/mcp-result/masked-control-values · MCP server results · control values — A password's or file input's control value is carried masked.
decision/driver-session-unix-socket-only · Driver session on one Unix socket — The driver session socket binds one Unix-domain socket in a caller-named state directory, never a TCP or UDP port, and exists on unix only, while an in-process session runs on linux, windows and macOS alike.
decision/lint-findings-crossing · Lint findings cross every ratified carrier — A lint's findings cross every ratified carrier as stable ids, closed defect words and numbers, never page text, colour or value.
decision/select-list-crossing · Select list crossing limited to bounds and pixels — The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff and as pixels in a shot; node state holds no field of it and no expanded node key stands.
decision/skip-snapshot-of-unstyled-node · Never-styled nodes not snapshotted — Snapshotting a never-styled node is skipped. · why: Stylo invalidation would otherwise panic.
decision/snapshot-state-readings · Three state readings per control — The snapshot text and the diff hold three state readings per control, selected, pressed and invalid, as closed words and booleans. · why: Inside the ratified crossing, ratified.
contract/actions-answer/id · The actions answer · id — The node's id.
contract/actions-answer/owner · The actions answer · owner — The node's owner id.

## input-validation · ## Input Validation
decision/actions-read-changes-nothing · Actions read runs nothing and moves no session state — The actions read runs nothing and changes no snapshot, focus reading or diff, so it can be called without moving the session. · why: The stand_actions test the_read_changes_nothing_and_holds_no_typed_text_or_mask pins it; agents call the actions verb between steps over socket and MCP
decision/actions-read-shares-target-check · Actions read and verbs share one target check — Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused. · why: The stand_actions test every_listed_cause_on_the_lean_tasks_is_what_the_call_returns pins that each listed cause equals the refusal the call returns
decision/boundary-widening-rule · Boundary widening rule — A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
decision/cli-closed-argv-reader · Closed argv reader — The session command line reads argv with a closed reader with fixed rules, and checks a call before the session address is asked for.
decision/cli-echoes-nothing-typed · Command line echoes nothing typed — No command-line ending writes back anything the caller typed, and no path on argv panics.
decision/closed-driver-verb-schema · Closed driver verb schema — Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing. · why: Each widening of the verb table is a widening of a validated surface, ratified.
decision/closed-pipe-drops-answer · Closed pipe drops the answer — A write to a closed stdout is dropped and the command keeps its exit status, never a panic. · guard: a_closed_stdout_drops_the_answer_and_keeps_the_ending
decision/closed-value-field-no-widening · A closed-value field in a ratified carrier is no widening — A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does. · why: Ratified as the standing rule.
decision/command-schema-no-clock · Command schema reads no clock — The command schema and its validation read no clock.
decision/command-span-content-free · Command span holds no content — One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
decision/descriptions-adapter-only · Descriptions cross only to the accessibility adapter — `aria-describedby` targets and `aria-description` text reach only the platform accessibility adapter; the snapshot reads no description, so no driver or MCP carrier, verb, field, log field or span carries one. · why: Upstream's behaviour, ratified as a boundary widening by upstream merge. · guard: accessibility_description
decision/driver-installs-no-subscriber · Driver installs no subscriber — The driver crate and its command line install no tracing subscriber and read no environment variable; clocks are read only by the host's idle expiry, the MCP server on a request, the session's bounded load wait, and the client's bounded stop and ready waits. · except: An acting step runs under escher-telemetry's scoped log capture, which prints nothing.
decision/driver-socket-closed-grammar · Driver socket closed request grammar — The driver session socket takes one request line per connection, parsed against a closed grammar under a size bound, one connection at a time under fixed bounds; anything else is refused with nothing run, and a decoded call runs through the session's one run path alone.
decision/id-lookup-anew-each-call · Caller-supplied id looked up anew — An acting command that takes a caller-supplied id looks it up on the screen as it reads now, on every call, never by index and keeping no node handle between calls; an id that names nothing is refused with nothing run and no panic.
decision/id-record-bounded-and-session-fed · Bounded id record fed by the session — The session keeps no node handle between calls, only a record of at most 4096 id texts fed from its own snapshots and never from a call's argument.
decision/mask-holds-on-every-carrier · Value mask holds on every carrier — A masked value stays masked on every carrier the snapshot reaches: its text form, a diff, the driver's return, the session socket and MCP tool results. · why: Ratified. · guard: stand_act_filled, stand_mcp_masked · except: A caused item's message and its error field hold what their author wrote; every other content-named field in it reads as redacted.
decision/masked-input-values · Password and file input values masked — A password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path. · except: The DOM and form submission keep the value.
decision/mcp-bounded-lines · Bounded MCP requests and results — MCP request lines and results are bounded, and tool arguments are typed before validation.
decision/mcp-listing-fixed-text · MCP listing is fixed program text — The MCP tool listing's descriptions are the program's own fixed table text: each verb's help and each result field's name, kind and help. · why: Ratified as the program's own text inside the ratified carrier.
decision/no-echo-of-caller-input · No echo of caller input — No command-line ending writes back anything the caller typed: no path, label, id, value or file content.
decision/no-node-id-between-calls · No NodeId kept between calls — No NodeId is kept between calls.
decision/no-panic-on-call-input · No panic on call-supplied input — Neither the driver command schema's validation nor the executor has unwrap, expect, panic or unreachable on anything a call supplies.
decision/no-process-handle-crosses · No process-local handle crosses with an id — Neither the stable id, the snapshot, the diff nor any driver session, answer or refusal carries a `NodeId`, `ElementId`, `ScopeId` or pointer; only the stable id text crosses.
decision/one-command-span-per-call · One command span per call — Every call handed to the session is wrapped in one INFO span named command under the escher_driver target, entered before validation so a refusal is inside it, holding fixed words, booleans and counts only.
decision/one-validation-path-for-every-carrier · One validation path for every carrier — The session socket, the command line and MCP each pass a call through the schema's validation before anything runs, and nothing public runs a command that did not come from that validation. · why: The crossing to external input surfaces was ratified with this check as the one they all pass through.
decision/password-file-value-masked · Password and file input values masked — The snapshot reads a password or file input's non-empty value as one fixed marker whatever was typed or chosen, an empty one as empty, never the text, its length or a file's host path. · why: Ratified. · guard: stand_act_filled, stand_mcp_masked · except: The DOM is untouched: a file input's value attribute still holds the host path and form submission still carries it.
decision/recorded-check-strict-reader · Recorded check is read strictly as untrusted — A kept check is read as untrusted bytes, at most 1,048,576 bytes and 400 records of closed keys; anything else is refused `malformed` with no fault, holding nothing of the file, with no panic path.
decision/recorded-check-untrusted · Recorded check bytes are untrusted — A kept check is read by a strict reader bounded in bytes and records, with closed keys and schema-named arguments, anything else refused as malformed holding nothing of the file, no panic path; every replayed step is validated again; the coverage role reads checks through the same reader.
decision/recording-dir-never-named · Recording directory is never exposed — No tool call, verb, argument or result field names the recording directory, and nothing of it is logged. · why: Ratified.
decision/recording-dir-owner-only · Recording directory is owner-only — The recording directory is created mode 0700 and a present one open to group or others is refused `record-dir-not-private` with status 3, nothing written and nothing spawned. · why: Ratified.
decision/recording-directory-never-named · Recording directory never named or logged — No tool call, verb, argument or result field names the recording directory, and nothing of it is logged.
decision/recording-directory-owner-only · Owner-only recording directory — The recording directory is created 0700, a group- or other-open one is refused with nothing written or spawned, each check is created 0600 and never replaced, and recording is unix only.
decision/recording-never-replaces-files · Recording never replaces or removes foreign files — A recorded check file is created 0600 with create-new under the smallest free number, rewritten whole through a temp file and rename; no file is replaced and a file the recorder did not create is never removed.
decision/replay-through-session-run · Replay runs each step through the session — A replayed check runs each kept step through the session's own validated run alone, runs nothing on a session of another app and stops at the first step whose acceptance or kept answer differs.
decision/session-id-record-bounded · Bounded in-process record of read ids — A session keeps a private record of ids its own snapshots read, at most 4096 ids of at most 1024 bytes each, the oldest forgotten first, read by no public item and written to no log, event, socket or file. · why: Kept to tell the stale and not-found refusals apart. · guard: the_record_holds_no_id_longer_than_a_call_can_name
decision/session-layout-read-in-process · In-process layout read on a session — The session's layout read takes its id from Rust code alone, resolves it through the run path's id resolution refusing stale or not-found, runs no step, pump, tick or settle, and answers stable id strings, closed words and numbers, never a NodeId, text or value.
decision/session-socket-bounds · Session socket bounds — One connection at a time; each read and write under 2 s, a client's read of an answer under 30 s, a request line at most 16,384 bytes, a lifecycle reply 128 bytes, an answer 1,048,576 bytes. · guard: an_answer_read_past_its_bound_is_a_timeout
decision/session-socket-closed-grammar · Session socket request is a closed grammar — A session socket connection carries one request line and one reply line, parsed against a closed versioned grammar with strict percent-decoding; anything outside it is refused and runs nothing, and a refusal or broken connection changes nothing.
decision/single-validation-gate · Single command validation gate — Every call, from the socket, the command line or MCP, is checked against a closed static table of verbs and bounded argument kinds, the first failure in a fixed order, and nothing public runs a command that did not come from that validation.
decision/socket-carries-screen-reading · Session socket carries what the screen reads — An answer carries what the screen reads, ids, accessible names, control values with password and file input masked, snapshot text, diff, lint findings, UI graph, and an acting step's caused text scrubbed by the answer's own field names, error kept, never a NodeId or an attribute value as typed. · why: The crossing was ratified.
decision/stable-id-exits · Ratified exits of the stable id — The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files. · why: Each carrier ratified as a boundary widening.
decision/stable-id-not-logged · Stable ids kept out of logs — Escher's own code writes the stable id to no log, and a host that installs escher's sink drops third-party style records carrying an element's HTML id whole, printing that value at no `RUST_LOG` level. · why: Those third-party targets are outside the sink's allowlist.
decision/target-refusal-order · Fixed order of target refusals — A click or type on an element that cannot take it is refused with nothing run, the first of disabled, off-screen, covered that holds, leaving the snapshot and both focus readings as they were; scroll reports in_view instead of refusing. · why: The detection of refusal causes by code was ratified.
decision/unknown-id-refused-stale-or-not-found · Unknown id refused as stale or not-found — An id that names no element on the screen as it reads now is refused with nothing run: stale when the session's record of ids holds it, not-found when not.
decision/unknown-target-refused · Call on an unknown or unfit element refused — A call whose id names no element, or an element that cannot take the call, is refused with nothing run.
contract/actions-answer/nodes · The actions answer · nodes — `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, one entry per actionable node, holding ids, owner ids, `keyed` booleans, verb words and closed cause words, never a control value, typed text or NodeId.
contract/cli-flags/id · escher-session flags · --id — Followed by one value, whatever that word is.
contract/cli-flags/session · escher-session flags · --session — Followed by a directory, given exactly once, else usage.
contract/driver-argument-kinds/id · Driver argument kinds · id — Text of 1 to 1024 bytes.
contract/driver-call-id/length-bound · Driver call id · length bound — An id a call names, from the command-line `--id` flag, the socket's `call` request or an MCP tool's `id` argument, is bounded at 1 to 1024 bytes, checked in the client before a session is touched and again in the host.
contract/driver-id-refusal/not-found · Driver id refusal · not-found — A call naming an id that no element on the current screen carries is refused as `not-found` when the session's record does not hold that id.
contract/driver-id-refusal/stale · Driver id refusal · stale — A call naming an id that no element on the current screen carries is refused as `stale` when the session's record holds that id.
contract/driver-session-reply/no-attribute-text · driver session socket replies · attribute text — A reply never carries an attribute's text.
contract/driver-session-reply/state-readings · driver session socket replies · state readings — State readings are carried as closed words and booleans.
contract/driver-verbs/actions · Driver verbs · actions — Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent, never a control value, typed text or NodeId.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The call was accepted: one uncoloured JSON line on stdout holds the answer, or for the coverage role its one coverage line.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The call was refused: one uncoloured JSON line on stdout holds the refusal, or for the coverage role a malformed refusal.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — A usage error: the usage line goes to stderr and stdout stays empty.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — A session error: the session-error object on stdout and the session error's fixed message on stderr.
contract/mcp-errors/32602 · MCP protocol errors · -32602 — invalid params or unknown tool, with no data: params not an object with a string name and an optional object arguments, or a name outside the verb table.
contract/mcp-tool-arguments/value-kinds · MCP tool arguments · value kinds — A string becomes text, an i64 a number, a bool a flag; any other value is refused malformed before anything runs, and a repeated key collapses to its last value.
contract/mcp-tool-call-span/ending · The MCP tool_call span · ending — One of accepted, refused or error.
contract/mcp-tool-call-span/tool · The MCP tool_call span · tool — The verb table's word for the called tool; the span parents the call's command span.
contract/mcp-tool-result/size-bound · MCP tool results · size bound — A result's text over 1,048,576 bytes is replaced by the answer-too-large result; a result holding an image is bounded by its whole written form.
contract/recorded-check/accepted · Recorded check file · accepted — One of the closed record keys, whether the call was accepted.
contract/recorded-check/answer · Recorded check file · answer — Required and a JSON object; key order is not enforced.
contract/recorded-check/arguments · Recorded check file · arguments — Argument names drawn from that verb's schema.
contract/recorded-check/digest · Recorded check file · digest — 16 lowercase hex characters.
contract/recorded-check/tool · Recorded check file · tool — One of the ten instance verbs.
contract/session-socket-reply/ok-accepted · Session socket reply · ok v3 accepted — The call ran and was accepted; its answer JSON follows on the line.
contract/session-socket-reply/ok-oversize · Session socket reply · ok v3 oversize — The call ran but its answer is over 1,048,576 bytes.
contract/session-socket-reply/ok-refused · Session socket reply · ok v3 refused — The call was refused; the refusal JSON follows on the line.
contract/session-socket-reply/refused-malformed · Session socket reply · refused malformed — A request line the grammar does not read or over 16,384 bytes; nothing runs.
contract/session-socket-request/call · Session socket request · call — `call v3 <verb> [<name>=<kind>:<value>]…` runs a validated verb, kinds `t` text, `n` whole number, `f` flag, verb, names and text percent-escaped.
contract/snapshot-value/masked-value · The snapshot's value · MASKED_VALUE — Eight U+2022 characters, one fixed marker standing for a password or file input's non-empty value on every carrier.
contract/stable-element-id/carriers · The stable element id · carriers — The stable element id leaves by the platform adapter, the driver's answers, MCP results and kept checks, and is written by escher's code to no log.
contract/stable-element-id/no-node-id · The stable element id · NodeId — No `NodeId` crosses any carrier of the stable element id.
contract/stable-element-id/stability · Stable element id · stability — A stable id is never a list index, pointer, hash, clock or process-local value, so a row reads the same id across a re-render, a remount and a second process.
contract/stable-element-id/unresolved-node · Stable element id · non-element, stale or detached node — A non-element, stale or detached node reads no id, and no path computation panics.
decision/accessible-name-exits · Ratified exits of accessible names — Accessible names leave the process only through the platform accessibility adapter, a hosted call's answer in the snapshot text and diff JSON over the session socket and the client's stdout, and MCP tool results, and reach no log. · why: Each carrier ratified.
decision/actions-keyed-field · Keyed flag in the actions answer — The `actions` verb's answer carries, per actionable node, its id, owner id, a `keyed` boolean and closed verb and cause words, no NodeId. · why: A ratified boundary widening.
decision/anchored-id-tier · Anchored stable-id tier — An unkeyed element's stable id is anchored under the nearest author-keyed element of its own component instance before falling back to its component path or document path. · why: The anchored tier was ratified.
decision/aria-label-trimmed-once · One trimmed aria-label read — An `aria-label` non-empty after trimming becomes the node's label and a whitespace-only one names nothing; there is one trimmed, non-empty `aria-label` read. · why: Follows accname-1.2; upstream's unfiltered read was dropped in the merge. · guard: accessibility_label
decision/aria-labelledby-outranks-label · aria-labelledby outranks label and aria-label — An element's `aria-labelledby` targets' text, gathered cycle-guarded, is its computed label, outranking a `<label>` and an `aria-label`; an id naming no element is ignored and `aria-label` is the fallback when the targets hold no text. · why: Upstream's behaviour, ratified at the upstream sync. · guard: accessibility_label
decision/aria-state-lowercase-exact-match · Control-state values matched exactly in lower case — `aria-selected`, `aria-pressed`, `aria-checked` and `aria-invalid` values are trimmed of ASCII whitespace and compared exactly, in lower case, against a closed set; an unrecognized value sets nothing, but for `aria-invalid`, which reads invalid. · why: A choice: the specification as read states no case rule and no browser was measured; the `aria-invalid` fallback is WAI-ARIA 1.2's rule. · except: A select's option reads its chosen state as selected, never `aria-selected`.
decision/attribute-text-leaves-by-ratified-carriers · Attribute text leaves only by ratified carriers — Markup attribute values, names and descriptions leave escher only by the ratified carriers. · except: The page's own form submission and events.
decision/author-id-on-accessibility-nodes · Stable id as accessibility author id — Under the accessibility feature each element node of the accessibility tree carries its stable id as its AccessKit author id, never on a text run, the document root or the synthetic window node, so it leaves the process only through the platform adapter. · why: Ratified; the crossing text runs and labels already take.
decision/cold-agent-check-validation · Agent-written check validated before replay — Only the reply's last fenced json block is taken, at most 65,536 bytes and 400 steps, each step of closed keys and a known verb; a check outside these is never replayed, and the check is never printed. · why: A boundary widening to a new input class, ratified.
decision/control-states-closed-values-only · Control states cross as closed values — Control states leave the process only as closed enums or booleans, on the accessibility adapter and in the snapshot's words and JSON booleans, never as the attribute's text, and no unrecognized or long value is written anywhere as typed. · why: A ratified widening of the adapter crossing by a new author-derived class.
decision/disabled-read-by-presence · disabled read by presence on form controls — The disabled attribute is read by its presence, whatever its value, and only on a button, input, select or textarea, for focusability, pointer and keyboard activation and the snapshot's enabled and clickable readings. · why: The presence reading was ratified.
decision/driver-no-subscriber-no-env · Driver installs no subscriber and reads no env — The driver crate and its session command line install no tracing subscriber and read no environment variable. · except: An acting step runs under escher-telemetry's scoped log capture, which prints nothing.
decision/driver-outcomes-never-logged · Driver outcomes never logged — The driver crate logs and fields no command, call, argument, outcome, refusal, screen text, image, finding, session label or id record; an outcome is written only as a command-line answer, an MCP tool result, a session-socket reply or a recording file the caller asked for. · why: Outcomes leave the process only by the ratified crossing.
decision/falsy-boolean-attribute-removed · Falsy boolean attribute removed — A falsy value of a listed boolean attribute clears the attribute rather than writing it. · why: The tests falsy_boolean_attributes_are_removed and every_listed_boolean_attribute_is_removed_when_falsy pin it; snapshot disabled readings depend on it
decision/inert-subtree · Inert subtree — An element carrying `inert`, or under an ancestor that does, is not focusable, takes no hit, leaves the accessibility tree and the snapshot, and a node holding focus when it comes under `inert` gives focus up; presence only, no value read. · why: An engine change for every document, ratified. · guard: inert_subtree
decision/lint-findings-closed-content · Lint findings hold no page content — A lint finding holds ids, closed defect words and numbers only, never page text, colour or a control value, and no log site, print, span or field carries a finding. · why: A ratified boundary widening.
decision/mcp-no-print-or-panic · MCP server never prints or panics — The MCP server has no print, unwrap, expect or panic outside the tests.
decision/merged-aria-states-closed-parse · Closed parse of expanded, disabled and level — `aria-expanded`, `aria-disabled` and `aria-level` are read as closed values carrying none of the attribute's text and exit only on the accessibility adapter; `aria-disabled` moves no snapshot reading. · why: As upstream wrote them, inside the ratified states carrier. · guard: accessibility_merged_states
decision/multiple-select-set-exits · Where a multiple select's chosen set leaves — A multiple select's chosen set leaves only as form entries, as input and change values carrying one entry per chosen option, as the snapshot's value of the first chosen option in tree order, and as the options' tree nodes. · why: An engine change for every document and a boundary widening, ratified.
decision/one-actionable-rule · One actionable rule for both readings — The unkeyed-actionable reading and the actionable-nodes reading both read one actionable rule: focusable, an interactive role, or current listeners holding an event other than `mounted`. · why: The actionable test a_listener_removed_on_a_re_render_is_read_by_neither_reader pins it; the driver's actions verb and the unkeyed lint read these readers
decision/range-value-exits · Range current value exits — A range input's current value leaves only as a form submission entry, the `input` event's value and the snapshot's control value, and no accessibility node value or action changes. · why: An engine change for every document and a boundary widening, ratified.
decision/select-choice-exits · Where a select's choice leaves — A select's choice leaves only as form submission's entries, the input and change events' value, the snapshot's value and the options' accessibility-tree nodes, inside the ratified name and control-value crossings. · why: A boundary widening, ratified.
decision/select-readers-resolve-node-closed-select-none · Select readers resolve the node and skip an open list — Each public select reader resolves its node so a stale id reads none, and the closed-select reader answers none for a select whose list is open. · why: The escher-driver lint and the snapshot's option bounds call closed_select_of and select_list_bounds, relying on none for a stale id or an open list
decision/session-label-only-caller-string · Session label is the one echoed caller string — The session label is never text read from the document and is the one caller string a reply carries.
decision/session-socket-unix-only · Session socket is Unix only — The driver session socket exists on Unix only; elsewhere a session is `Unsupported`.
decision/state-dir-path-is-parameter · Session state path is a parameter — The session state directory path is a caller parameter, never read from the environment or defaulted relative to the working directory.
decision/state-directory-path-is-parameter · State directory path is a parameter — The state directory path is a parameter, never an environment read or a working-directory-relative default.
decision/text-extent-read-only-none-when-unresolved · Text extent reader read-only with none answers — The text extent reader is read-only and answers none for an id that does not resolve or a node with no inline layout. · why: The text_extent test an_element_with_no_inline_content_reads_none pins the none answers; the escher-driver lint and stand_lint read it for truncation
decision/toggle-read-per-role · Toggle state read per role — `aria-checked` sets the toggle only on the five checkable roles and `aria-pressed` only on every other role; a native checkbox or radio reads neither attribute and gives its own checked state under a checkable role and nothing under any other. · why: Ratified.
decision/visible-region-through-transforms · Visible region read through transforms — A clipping box's visible region is read through its transforms, exact under translate and scale, a rotation reading the rotated box's bounding box, and a mapped box with a non-finite edge answers none. · why: An engine change for every document, ratified. · guard: visible_region_transformed, stand_act_transformed, transformed_client_rect
contract/actions-answer/keyed · The actions answer · keyed — A boolean on each listed node.
contract/cli-exit-codes/0 · CLI exit codes · 0 — Accepted: the answer as one line of JSON on stdout.
contract/cli-flags/record · escher-session flags · --record — Followed by a directory, taken at most once and only on start, the host role and the mcp role; anywhere else, twice or with no value is usage.
contract/driver-command-span/cause · The driver command span · cause — The name of the returned refusal's cause, unrecorded until it applies.
contract/driver-command-span/verb · The driver command span · verb — The verb table's own word, never the caller's verb text.
contract/session-socket-reply/ok-recording-failed · Session socket reply · ok v3 recording-failed — The call ran and the host's recording ended; the client reads it as a recording-failed session error.
contract/snapshot-text/clickable · Snapshot text state words · clickable — Written after `focused` only when the element's current listeners hold `click` and it is no disabled form control.
contract/snapshot-text/id-field · Snapshot text · id field — Each node's id is written as a quoted `id=` field in the string's Debug form, so the field never holds a line break or a bare `"`; the snapshot has one text form and no JSON form, Display form or parser of the text.
contract/snapshot-text/invalid · Snapshot text state words · invalid — A bare word written for an element that states it is invalid.
contract/snapshot-text/name-field · Snapshot text · name field — Each line's accessible name is written as a quoted string in the string's Debug form, so a name holding a line break or a `"` stays inside its one field.
contract/snapshot-text/pressed · Snapshot text state words · pressed — A bare word written for an element that states it is pressed.
contract/snapshot-text/selected · Snapshot text state words · selected — A bare word written for an element that states it is selected.
contract/snapshot-text/state-words · Snapshot text · state words — Control states are written as the bare words `selected` / `unselected`, `pressed` / `unpressed` and `invalid`; in JSON answers they are booleans.
contract/snapshot-text/unpressed · Snapshot text state words · unpressed — A bare word written for an element that states it is not pressed.
contract/snapshot-text/unselected · Snapshot text state words · unselected — A bare word written for an element that states it is not selected.

## data-protection · ## Data Protection
decision/kept-check-actions-answer · Kept check holds the actions answer — A kept actions answer holds ids, owner ids, keyed booleans, the table's verb words and closed cause words, never a control value, typed text or NodeId. · why: A boundary widening, ratified. · guard: stand_replay a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes
decision/kept-check-answer-whole · Kept check keeps each answer whole — A kept check keeps each answer as the line the driver wrote, masked as on every carrier, with a shot kept as its width and height and never its pixels. · why: Ratified, a shot's pixels excepted.
decision/kept-check-call-content · Kept check call-content class — A kept check holds call content: target ids, press keys, advance ms and text typed into unmasked controls, a masked or unresolved type's text kept as MASKED_STAND_IN. · why: Ratified.
decision/kept-check-owner-only · Kept checks are owner-only — A recorded check is written 0600 into a 0700 directory the launcher named, is never logged, and is never removed by escher but for its own check when a recording ends. · why: Ratified. · guard: recording::the_committed_checks_are_what_the_binary_records · except: Two scripted stand-Counter checks, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.
decision/kept-checks-owner-only · Kept checks owner-only and uncommitted — A kept check stays in an owner-only medium, is never logged and is never committed. · except: The two committed checks of the repository's own scripted stand-Counter sessions.
contract/kept-check-file/contents · Kept check files · contents — A kept check holds per call its verb, arguments, acceptance, digest and masked answer, never a pixel.
contract/kept-check-file/entry · Kept check file · entry — A check is a JSON array: an opening record of the app label and the format, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer and the answer itself as answer.
contract/kept-check-file/medium · Kept check files · medium — Under `--record <dir>`, `Session::recording_into` or an exploration's record directory, one `check-{n}.json` with mode `0600` per recorded session, each explored path its own, is written in a `0700` directory the launcher named.
decision/committed-counter-checks · Committed stand-Counter checks — The only committed kept checks are the two of the repository's scripted stand-Counter sessions, holding no typed text, host path, name or secret and no `screenshot` step, read only through the strict `Check::from_bytes`. · why: The one exception to a kept check's owner-only medium.

## api-security · ## API Security
decision/actions-answer-unpaged · The actions answer is unpaged — The actions verb takes no argument and a screen's whole answer is one answer under the 1,048,576-byte answer bound. · guard: stand_actions the_whole_cells_answer_is_within_the_answer_bound
decision/no-wait-verb-fields · No wait argument or result on verbs — No driver verb has an argument or a result field of a wait.
decision/session-socket-one-bounded-connection · One bounded connection at a time — The local session socket serves one connection at a time under fixed read, write and size bounds.
decision/time-moves-only-on-advance · Time moves only on advance — Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph and actions wait on no load.
contract/driver-command-line/answer-bounds · Driver command line · answer bounds — A started host has 30 s to answer and each read of a call's answer is bounded at 30 s; these bound a transport and wait on no load.
contract/mcp-server/help · MCP server · help — The help tool is answered by the server itself from the schema's fixed tables, running nothing on a session.
contract/mcp-server/tool-result · MCP server · tool result — A tool result is bounded at 1,048,576 bytes, one holding an image measured by its whole written form and a text-only one by its text.
contract/session-socket/connections · Driver session socket · connections — One connection is served at a time, with a 2 s read and write bound.
contract/session-socket/oversize · Driver session socket · oversize — An answer is at most 1,048,576 bytes on its JSON line; over it the reply is oversize, read by the client as AnswerTooLarge.
contract/session-socket/request-line · Driver session socket · request line — A request line is at most 16,384 bytes.
decision/mcp-server-over-stdio · MCP server over process stdio — The MCP server role serves the driver's verbs as tools over its own stdin and stdout only, binding no listener, port or socket, and answers only the process that spawned it, with no handshake, token, secret or peer check. · why: The crossing was ratified as is.
decision/session-socket-sole-listener · Session socket as the one listener — The driver session's Unix-domain socket is the project's one listener, serving the session lifecycle and driver calls and their answers.
contract/mcp-server/request-line · MCP server · request line — A request line is bounded at 65,536 bytes.
contract/session-wire/recording-failed · Driver session wire · ok v3 recording-failed — Wire v3 replies include the fixed reply ok v3 recording-failed.

## dependency-security · ## Dependency Security
decision/serde-json-read-only · serde_json, read only — serde_json, with no features and no preserve_order, reads the MCP server's untrusted stdin, a kept check's bytes and the driver's own answer line for replay, and writes nothing. · why: Chosen as read-only use.
decision/serde-json-untrusted-input-reader · serde_json as the untrusted input reader — serde_json, a workspace dependency with no preserve_order feature, is the reader of the MCP server's untrusted stdin, of a kept check's bytes and of the driver's answer line the replay compares.

## error-handling · ## Error Handling
decision/stale-node-ids-never-panic · Stale node ids never panic — A stale node id is skipped in layout children and resolves to none on lookup rather than panicking; a stale animation entry for a removed node is skipped on the next resolve. · guard: stale_interaction_state, stale_node_mapping, animations and custom_widget_layout tests
contract/cli-exit-codes/1 · escher-session CLI exit codes · 1 — The call was refused; the refusal JSON is written to stdout.
contract/cli-exit-codes/2 · escher-session CLI exit codes · 2 — Usage: the argv shape is not one the command takes; one fixed usage line on stderr, stdout empty, nothing booted and no state created.
contract/mcp-tool-result/is-error · MCP tool result · isError — A refusal or session error is a tool result `{content:[{type:"text",text}], isError: true}`, its text the same JSON as on the command line, a refused `screenshot` included.
contract/refusal-causes/malformed · Driver refusal causes · malformed — The one cause that carries a fault; a call breaking the schema, such as a `graph` or `actions` call with an argument, is refused with it and nothing runs.
contract/refusal-causes/not-found · Driver refusal causes · not-found — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/stale · Driver refusal causes · stale — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
decision/closed-refusal-cause-set · Closed refusal cause set — A driver command refusal is a typed value whose cause comes from a closed set of eight, with a fault only beside `malformed`; a new verb or argument shape is refused within the set, never by adding a cause. · why: The closed set of eight was ratified.
decision/mcp-errors-fixed-no-data · MCP protocol errors fixed, no data — MCP protocol errors are fixed codes and fixed messages and carry no data member.
decision/refusal-closed-cause-set · Command refusal as a closed set of causes — An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field. · why: So a refusal cannot hold anything a call supplied. · except: A Fault is carried only beside the malformed cause.
decision/refusal-holds-no-call-input · A refusal holds nothing a call supplied — Every refusal's name, meaning and remedy is a fixed static text holding no path, and a fault carries at most the schema's own argument name, so a refusal cannot hold anything a call supplied.
decision/refusal-log-cause-name-only · Only a refusal's cause name is logged — Exactly one thing of a refusal reaches a log: its cause's fixed name, as the `cause` field of the call's driver span; no fault, meaning, remedy or display text is logged.
decision/refusal-logged-by-cause-name · Refusal logged by cause name only — Of an escher-driver command refusal, only its cause's fixed name reaches a log.
decision/session-error-fixed-table · Session errors from a fixed table — An escher-driver session error is written from a fixed table, holding nothing a call supplied.
decision/session-errors-fixed-table · Session errors as one fixed table — Each session error's kind, meaning and remedy is one row of a fixed table served by `help`, every text a fixed string holding no path, carried on stdout and in MCP results as the program's own text. · why: Ratified as the program's own fixed text inside the ratified carriers.
contract/cli-exit-codes/3 · CLI exit codes · 3 — Session error: the error object with kind, message and remedy on stdout and its fixed message on stderr; a boot that fails ends with its session error.
contract/mcp-protocol-errors/unknown-tool · MCP protocol errors · unknown tool — A fixed code and fixed message with no `data`; no reply holds the tool name.
contract/query-apis/missing-element · Query APIs · missing element — Query APIs return Result<Option<NodeId>>, and a missing element is None.
contract/refusal-causes/unknown-verb · Driver refusal causes · unknown-verb — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-json/cause · Refusal JSON · cause — The refusal's cause name, inside `{"refused":{...}}`, built from the cause's fixed strings.
contract/refusal-json/fault · Refusal JSON · fault — Present only on a `malformed` refusal, naming the fault.

## logging-monitoring · ## Logging & Monitoring
decision/no-screen-content-in-logs · Screen content stays out of logs — No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer, a coverage check and a step's caused text reach no log line at any level. · guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds
decision/session-output-holds-no-caller-input · Session output holds no caller input — No escher-session output line holds anything the caller typed. · except: The mcp role's echoed request id.
contract/command-span/cause · The command span line · cause — Recorded when the call is refused, as a closed cause word such as fonts-unavailable.
contract/command-span/line · The command span line · line — A sink-installing host writes one closed command span line per call it runs.
contract/command-span/verb · The command span line · verb — Records the instance-level verb; a graph or actions call records the verb alone, plus cause when refused.
contract/escher-session-output/caller-text · escher-session output · caller-typed text — No output line echoes a caller's argument but the admitted request id the mcp role echoes; an answer carries each node's current value, so text typed into an unmasked control returns, a masked input reading only the mask.
contract/escher-session-stderr/usage-messages · escher-session stderr · usage messages — escher-session writes usage messages on stderr.
contract/escher-session-stdout/command-line · escher-session stdout · command result line — escher-session writes one uncoloured JSON line per command on stdout.
contract/tool-call-span/ending · The tool_call span line · ending — One of the closed words accepted, refused or error.
contract/tool-call-span/line · The tool_call span line · line — One closed line per MCP tools/call, the parent of that call's command span, holding tool and ending only.
contract/tool-call-span/tool · The tool_call span line · tool — The tool table's word for the call, a closed word outside the content-named fields; absent for an unknown tool.
decision/cold-agent-recordings-owner-only · Cold-agent recordings owner-only — The cold-agent live server records its kept checks into an owner-only directory, read for booleans and counts only, never printed, copied or committed, and removed at cleanup; the replay children run without recording.
contract/escher-session-mcp-stdout/protocol-lines · The escher-session mcp role's stdout · protocol lines — The mcp role writes JSON-RPC protocol lines only on stdout, and nothing at its end.
contract/escher-session-output/accepted-stderr · escher-session output · stderr of an accepted command — An accepted command's stderr is empty with RUST_LOG unset.
contract/escher-session-output/esc-byte · escher-session output · ESC byte — No output line holds an ESC byte; output is uncoloured whatever the stream is.
contract/escher-session-stderr/error-messages · escher-session stderr · error messages — escher-session writes error messages on stderr.
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
