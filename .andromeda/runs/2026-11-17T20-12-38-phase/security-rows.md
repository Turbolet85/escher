# security-plan · the rows that bind 2026-11-17-stand-requirement-sweep — picked 176 of 555 (101 by every draw) · rule any · pin 1850

## threat-model-summary · ## Threat Model Summary
decision/actions-answer-crossing · Actions answer crossing — An actions answer carries what each actionable node can take now on the socket, the command line, MCP and a kept check: ids, owner ids, keyed booleans, verb-table words and closed cause words, never a control value, typed text or NodeId. · why: A boundary widening, ratified.
decision/caused-text-crossing · Caused text crossing — Each acting verb's answer carries caused on every ratified carrier when the step caused something: one item per panic, boundary error and kept record with its message, location, boundary, cause chain and fields, error as written within the text bound, every other content-named field redacted. · why: A boundary widening to the error text, ratified.
decision/graph-answer-crossing · Graph answer crossing — A graph answer carries the screen's UI graph on the socket, the command line, MCP and a kept check: ids, owner ids, component names, registered state names, handler event names, closed flag words and booleans, never a control value, typed text or NodeId. · why: A boundary widening, ratified.
decision/mcp-result-crossing · MCP result crossing — An MCP result carries ids, accessible names, control values with a password's or file input's as the mask, the snapshot text and an acting call's diff to the client; what a client does onward, a model provider included, is the client's. · why: A boundary widening, ratified.
decision/session-socket-carries-calls · Session socket carries calls and answers — The session socket carries the lifecycle and calls with their answers: an element id and typed text in; ids, accessible names, control values with a password's or file input's as the fixed mask, the snapshot text and the diff out; no NodeId crosses. · why: A boundary widening, ratified.
contract/driver-session-reply/masked-control-values · driver session socket replies · control values — A password's or file input's control value is carried masked.
contract/mcp-result/masked-control-values · MCP server results · control values — A password's or file input's control value is carried masked.
decision/coverage-answer-crossing · Coverage answer crossing — The coverage answer line carries ids, owner ids, event names and the checks and held counts only. · why: A boundary widening, ratified.
decision/coverage-check-files-untrusted · Kept check files read as untrusted — The coverage role reads each caller-named kept check as untrusted input through a bounded read and a strict parse; an unreadable, oversized, non-check or other-app file is refused malformed, holding nothing of the path or the file.
decision/driver-session-unix-socket-only · Driver session on one Unix socket — The driver session socket binds one Unix-domain socket in a caller-named state directory, never a TCP or UDP port, and exists on unix only, while an in-process session runs on linux, windows and macOS alike.
decision/file-url-only-for-file-documents · File URLs load only for file documents — A file: URL loads only for a file: document or a top-level load; for any other document, remote pages included, script fetch, module import and script src fail, blitz-net refuses engine loads before a disk read, and the browser app names no file: favicon and drops a file: navigation silently.
decision/open-list-shot-pixels · Open select list shot carries labels as pixels — A shot of an open select list carries the page's option labels and group labels as pixels, on the socket and MCP carriers alike, the shot's fields unmoved. · why: A boundary widening, ratified. · guard: cli_commands, pinning a hosted shot equal to an in-process one
decision/root-boundary-err-crossing · Root-boundary Err crossing — A handler Err the app's root error boundary takes crosses as screen content, through Dioxus's default boundary rendering of its Display, and in caused as one error item holding its message and cause chain under the root boundary.
decision/select-list-crossing · Select list crossing limited to bounds and pixels — The select list crosses the ratified carriers only as a row's own bounds in the snapshot, the diff and a layout answer, as the layout answer's select-list placing word, and as pixels in a shot; node state holds no field of it and no expanded node key stands.
contract/actions-answer/id · The actions answer · id — The node's id.
contract/actions-answer/owner · The actions answer · owner — The node's owner id.
contract/actions-answer/verbs · The actions answer · verbs — A list of entries each holding verb, a word of the verb table, and cause, a closed cause word left out when absent; every listed node carries layout with no cause.

## input-validation · ## Input Validation
decision/accessible-name-exits · Ratified exits of accessible names — Accessible names leave the process only through the platform accessibility adapter, a hosted call's answer in the snapshot text and diff JSON over the session socket and the client's stdout, and MCP tool results, and reach no log. · why: Each carrier ratified.
decision/accessible-names-not-logged · Accessible names kept out of logs — Escher's own code logs no accessible name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level, the third-party record that carried one being dropped whole. · why: That third-party target is outside the sink's allowlist. · guard: mcp_log
decision/agent-run-selection-allowlist · agent-run.sh selection reaches cargo only as a test stem — agent-run.sh exits 2 with usage before any cargo call on an unknown verb, a wrong argument count or a selection other than the fixed words `stand`, `all` or `session`, matched before the file-stem arm, or an existing test-file stem matching `^[a-z0-9_]+$`.
decision/attribute-readers-no-log-or-panic · Markup attribute readers without log or panic — Markup attributes, remote pages included, are read from parsed content by in-process readers with no log site or panic path, ARIA states and closed values through closed parses carrying no attribute text.
decision/attribute-text-leaves-by-ratified-carriers · Attribute text leaves only by ratified carriers — Markup attribute values, names and descriptions leave escher only by the ratified carriers. · except: The page's own form submission and events.
decision/boundary-widening-rule · Boundary widening rule — A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
decision/cli-echoes-nothing-typed · Command line echoes nothing typed — No command-line ending writes back anything the caller typed, and no path on argv panics.
decision/closed-driver-verb-schema · Closed driver verb schema — Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing. · why: Each widening of the verb table is a widening of a validated surface, ratified.
decision/closed-value-field-no-widening · A closed-value field in a ratified carrier is no widening — A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does. · why: Ratified as the standing rule.
decision/command-span-content-free · Command span holds no content — One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
decision/coverage-floor-input-validation · Coverage floor checker validates and prints no paths — coverage_floor.py, stdlib only, refuses a malformed or repeated floor or a crate with no record with exit 2, and prints only crate directories and counts, never a source or lcov path. · guard: CoverageFloorTest
decision/coverage-floor-prints-no-paths · Coverage floor prints no paths — The coverage floor check prints only crate directories and counts, never a source or lcov path.
decision/diff-counts-only-in-logs · Only diff sizes reach a log — A diff's content never reaches a log or event, while its three sizes (counts of added, removed and changed nodes) are recorded on the driver's command span, printed at `info` or below. · why: Ratified: a diff's content never reaches a log while its three sizes may.
decision/driver-installs-no-subscriber · Driver installs no subscriber — The driver crate and its command line install no tracing subscriber and read no environment variable; clocks are read only by the host's idle expiry, the MCP server on a request, the session's bounded load wait, and the client's bounded stop and ready waits. · except: An acting step runs under escher-telemetry's scoped log capture, which prints nothing.
decision/driver-no-subscriber-no-env · Driver installs no subscriber and reads no env — The driver crate and its session command line install no tracing subscriber and read no environment variable. · except: An acting step runs under escher-telemetry's scoped log capture, which prints nothing.
decision/driver-outcomes-never-logged · Driver outcomes never logged — The driver crate logs and fields no command, call, argument, outcome, refusal, screen text, image, finding, session label or id record; an outcome is written only as a command-line answer, an MCP tool result, a session-socket reply or a recording file the caller asked for. · why: Outcomes leave the process only by the ratified crossing.
decision/mask-holds-on-every-carrier · Value mask holds on every carrier — A masked value stays masked on every carrier the snapshot reaches: its text form, a diff, the driver's return, the session socket and MCP tool results. · why: Ratified. · guard: stand_act_filled, stand_mcp_masked · except: A caused item's message and its error field hold what their author wrote; every other content-named field in it reads as redacted.
decision/masked-input-values · Password and file input values masked — A password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path. · except: The DOM and form submission keep the value.
decision/no-echo-of-caller-input · No echo of caller input — No command-line ending writes back anything the caller typed: no path, label, id, value or file content.
decision/one-command-span-per-call · One command span per call — Every call handed to the session is wrapped in one INFO span named command under the escher_driver target, entered before validation so a refusal is inside it, holding fixed words, booleans and counts only.
decision/password-file-value-masked · Password and file input values masked — The snapshot reads a password or file input's non-empty value as one fixed marker whatever was typed or chosen, an empty one as empty, never the text, its length or a file's host path. · why: Ratified. · guard: stand_act_filled, stand_mcp_masked · except: The DOM is untouched: a file input's value attribute still holds the host path and form submission still carries it.
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90. · guard: coverage leg
decision/reached-handler-record-without-log · Reached-handler record without log or NodeId — Where recording of reached handlers is switched on, each reached handler's stable id and event name is written into a root-context record ahead of the handler's dispatch, with no log site and no `NodeId`.
decision/recorded-type-masked · Recorded type to a masked control kept masked — A recorded type whose element resolves to a masked control or to none is kept as a stand-in marked masked, never the text, its length or a digest of it, while any other type's text is kept as it crossed. · why: Ratified.
decision/select-list-label-never-logged · Select list row labels never logged — The select list's public surface is read-only but for one closing writer, and a list row's label, being page text, is never logged.
decision/session-id-record-bounded · Bounded in-process record of read ids — A session keeps a private record of ids its own snapshots read, at most 4096 ids of at most 1024 bytes each, the oldest forgotten first, read by no public item and written to no log, event, socket or file. · why: Kept to tell the stale and not-found refusals apart. · guard: the_record_holds_no_id_longer_than_a_call_can_name
decision/session-layout-read-in-process · In-process layout read on a session — The session's layout read, in process or through the layout verb on any ratified carrier, resolves its id through the run path's id resolution refusing stale or not-found, runs no step, pump, tick or settle, and answers stable id strings, closed words and numbers, never a NodeId, text or value.
decision/socket-carries-screen-reading · Session socket carries what the screen reads — An answer carries what the screen reads: ids, accessible names, control values, password and file input masked, snapshot text, diff, lint findings, UI graph, layout readings and an acting step's caused text scrubbed by its own field names, error kept, never a NodeId or an attribute value as typed. · why: The crossing was ratified.
decision/stable-id-exits · Ratified exits of the stable id — The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files. · why: Each carrier ratified as a boundary widening.
decision/stable-id-not-logged · Stable ids kept out of logs — Escher's own code writes the stable id to no log, and a host that installs escher's sink drops third-party style records carrying an element's HTML id whole, printing that value at no `RUST_LOG` level. · why: Those third-party targets are outside the sink's allowlist.
contract/actions-answer/nodes · The actions answer · nodes — `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, one entry per actionable node, holding ids, owner ids, `keyed` booleans, verb words and closed cause words, never a control value, typed text or NodeId.
contract/agent-run-cli/2 · agent-run.sh exit codes · 2 — A verb outside boot run status cleanup logs, a wrong argument count, or a selection other than stand, all, session or an existing test-file stem prints usage and exits 2 before any cargo call.
contract/cli-flags/id · escher-session flags · --id — Followed by one value, whatever that word is.
contract/cli-flags/record · escher-session flags · --record — Followed by a directory, taken at most once and only on start, the host role and the mcp role; anywhere else, twice or with no value is usage.
contract/cli-flags/session · escher-session flags · --session — Followed by a directory, given exactly once, else usage.
contract/cli-flags/text · escher-session flags · --text — Followed by one value, whatever that word is.
contract/cli-roles/coverage · escher-session roles · coverage — coverage <app> <check>... with no --session or --record, else usage; each check read up to 1,048,576 bytes, and an unreadable, oversized, non-check or other-app file refused malformed, status 1, holding nothing of the path or file.
contract/cli-roles/mcp · escher-session roles · mcp — mcp alone or with one --record <dir> selects the MCP role; any other word after it is usage.
contract/cli-roles/serve · escher-session roles · serve — serve <app> --session <dir> in exactly that shape, optionally with one --record <dir> after it, else usage.
contract/coverage-floor-cli/2 · coverage_floor.py exit codes · 2 — A floor not matching CRATE_DIR=PERCENT (a percent at most 100, no crate twice), or a crate with no SF record, exits 2.
contract/driver-argument-kinds/id · Driver argument kinds · id — Text of 1 to 1024 bytes.
contract/driver-call-id/length-bound · Driver call id · length bound — An id a call names, from the command-line `--id` flag, the socket's `call` request or an MCP tool's `id` argument, is bounded at 1 to 1024 bytes, checked in the client before a session is touched and again in the host.
contract/driver-command-span/added · The driver command span · added — The length of the returned diff's added list.
contract/driver-command-span/busy · The driver command span · busy — The schema's class word, unrecorded until it applies.
contract/driver-command-span/cause · The driver command span · cause — The name of the returned refusal's cause, unrecorded until it applies.
contract/driver-command-span/changed · The driver command span · changed — The length of the returned diff's changed list.
contract/driver-command-span/passes · The driver command span · passes — A count, unrecorded until it applies.
contract/driver-command-span/removed · The driver command span · removed — The length of the returned diff's removed list.
contract/driver-command-span/settled · The driver command span · settled — A boolean, unrecorded until it applies.
contract/driver-command-span/verb · The driver command span · verb — The verb table's own word, never the caller's verb text.
contract/driver-verbs/actions · Driver verbs · actions — Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent, never a control value, typed text or NodeId.
contract/driver-verbs/graph · Driver verbs · graph — Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[…],"states":[…]} of ids, owner ids, component names, state names, handler event names, flag words and booleans, never a control value, typed text or NodeId.
contract/driver-verbs/layout · Driver verbs · layout — Takes one required id, refused malformed without it or with a second argument and not-found or stale for an unknown id; changes nothing, leaves delivered Timer ticks unapplied, and answers the element's placement, shifts and sizes as ids, closed words and numbers.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The call was accepted: one uncoloured JSON line on stdout holds the answer, or for the coverage role its one coverage line.
contract/graph-answer/nodes-states · Graph answer · nodes and states — `{"nodes":[…],"states":[…]}` holding ids, component names, state names, handler event names, the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or NodeId.
contract/masked-input-value/masked-stand-in · Masked input values · MASKED_STAND_IN — A recorded masked `type` is kept as `MASKED_STAND_IN`.
contract/recorded-check/masked · Recorded check file · masked — True on a `type` record alone.
contract/recorded-check/masked-stand-in · A recorded check · MASKED_STAND_IN — The text "masked", with masked:true, stands for a recorded type to a masked or unresolved control.
contract/snapshot-value/masked-value · The snapshot's value · MASKED_VALUE — Eight U+2022 characters, one fixed marker standing for a password or file input's non-empty value on every carrier.
contract/stable-element-id/carriers · The stable element id · carriers — The stable element id leaves by the platform adapter, the driver's answers, MCP results and kept checks, and is written by escher's code to no log.
decision/actions-keyed-field · Keyed flag in the actions answer — The `actions` verb's answer carries, per actionable node, its id, owner id, a `keyed` boolean and closed verb and cause words, no NodeId. · why: A ratified boundary widening.
decision/actions-read-changes-nothing · Actions read runs nothing and moves no session state — The actions read runs nothing and changes no snapshot, focus reading or diff, so it can be called without moving the session. · why: The stand_actions test the_read_changes_nothing_and_holds_no_typed_text_or_mask pins it; agents call the actions verb between steps over socket and MCP
decision/actions-read-shares-target-check · Actions read and verbs share one target check — Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused. · why: The stand_actions test every_listed_cause_on_the_lean_tasks_is_what_the_call_returns pins that each listed cause equals the refusal the call returns
decision/control-states-closed-values-only · Control states cross as closed values — Control states leave the process only as closed enums or booleans, on the accessibility adapter and in the snapshot's words and JSON booleans, never as the attribute's text, and no unrecognized or long value is written anywhere as typed. · why: A ratified widening of the adapter crossing by a new author-derived class.
decision/descriptions-adapter-only · Descriptions cross only to the accessibility adapter — `aria-describedby` targets and `aria-description` text reach only the platform accessibility adapter; the snapshot reads no description, so no driver or MCP carrier, verb, field, log field or span carries one. · why: Upstream's behaviour, ratified as a boundary widening by upstream merge. · guard: accessibility_description
decision/external-navigation-get-http-mailto · External navigation limited to safe schemes — Navigation handed to an external browser is limited to GET requests with http, https or mailto schemes.
decision/external-navigation-schemes · External navigation limited — Navigation handed to an external browser is limited to GET requests with http, https or mailto schemes.
decision/failed-stylesheet-blocks-no-render · Failed stylesheet blocks no render — In every document, remote pages included, a stylesheet that fails to load, is empty, is not CSS or is not valid UTF-8 blocks no render. · guard: failed_stylesheet_unblocks_rendering, empty_or_non_css_stylesheet_unblocks_rendering
decision/file-urls-only-from-file-documents · file URLs load only from file documents — A file: URL is read only for a file: document or a top-level load: a script fetch, module import or script src, every engine stylesheet, @import, font, image or iframe load, and in the browser app a favicon and a link navigation; blitz-net refuses before a disk read, a navigation drops silently. · why: A narrowing, not a boundary widening. · guard: remote_document_cannot_load_file_urls
decision/fixture-name-closed-table · Fixture state name checked against a closed table — A fixture state name is compared whole against the app's closed table of named states before any session starts; any other name is refused as an unknown state and nothing is booted.
decision/hintless-font-face-source-sniffed · Hint-less font-face source fetched and sniffed — An @font-face source whose URL cannot be resolved is skipped; a source with no format hint and no extension is fetched and its format sniffed from its bytes. · why: Ratified as upstream's behaviour, with a check asked for. · guard: font_face_sniffed_source
decision/html-parsed-scripting-disabled · HTML parsed with scripting disabled — HTML is parsed with scripting disabled.
decision/html-parsed-without-scripting · HTML parsed with scripting disabled — HTML is parsed with the parser's scripting flag disabled.
decision/lint-findings-closed-content · Lint findings hold no page content — A lint finding holds ids, closed defect words and numbers only, never page text, colour or a control value, and no log site, print, span or field carries a finding. · why: A ratified boundary widening.
decision/mcp-no-print-or-panic · MCP server never prints or panics — The MCP server has no print, unwrap, expect or panic outside the tests.
decision/multiple-select-keys-and-clicks · Multiple select keys and clicks — In a multiple select, a plain arrow, Home, End or click chooses that row alone, Shift spans from the anchor, Ctrl or Meta moves the current row by key and flips a row by click, and disabled rows are never chosen. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select · except: Ctrl+A is not modelled.
decision/no-agent-urls-through-font-path · No agent URL through font sources — escher routes no agent- or user-supplied URL through the font-face source path.
decision/one-validation-path-for-every-carrier · One validation path for every carrier — The session socket, the command line and MCP each pass a call through the schema's validation before anything runs, and nothing public runs a command that did not come from that validation. · why: The crossing to external input surfaces was ratified with this check as the one they all pass through.
decision/optgroup-options-and-label · Optgroup options and label — A select's options include its optgroup children's options in tree order, and a group's non-empty label is its name in the accessibility tree and a header painted in an open list. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select, select_popup, select_popup_painted
decision/password-painted-masked · Password input painted masked — A password input is painted as one bullet per character in every document and through every renderer that paints with the engine's painter, so a typed password never leaves as pixels in a screenshot. · why: An engine change for every document, ratified. · guard: password_painted_masked, stand_act_screenshot · except: The frame depends on the value's length; a file input's paint is unchanged.
decision/presentational-hints-typed-parse · Presentational hint attributes read by a typed parse — The font element's face and size, valign, cellspacing and cellpadding are read as presentational hints by a typed parse, and a value that does not parse is ignored. · why: Upstream's behaviour, ratified as an engine change for every document.
decision/recorded-check-untrusted · Recorded check bytes are untrusted — A kept check is read by a strict reader bounded in bytes and records, with closed keys and schema-named arguments, anything else refused as malformed holding nothing of the file, no panic path; every replayed step is validated again; the coverage role reads checks through the same reader.
decision/select-choice-one-write-path · Every select choice through one write path — Every select choice, by key, click or open list, goes through one write path that marks the changed set and dispatches input then a non-cancelable, bubbling change, a choice of the already chosen option firing nothing. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select, select_popup
decision/select-keys-closed-match · Select keys pass a closed match — A focused select's keys, an open list's keys and a multiple select's keys each pass one closed key match, and every key outside the match does nothing. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select, select_popup
decision/select-list-painted-over-page · Select list painted over the page — The open select list is painted after the root's render and before the devtools overlays, a non-finite list box left unpainted. · why: An engine change for every document and a boundary widening, ratified.
decision/select-typeahead-private · Select typeahead kept private and short-lived — Select typeahead keeps the typed text in the document's private buffer, starts afresh for another select or after 1 s of the document's clock, and the text is compared only, never kept past 1 s and never carried out. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select
decision/session-socket-bounds · Session socket bounds — One connection at a time; each read and write under 2 s, a client's read of an answer under 30 s, a request line at most 16,384 bytes, a lifecycle reply 128 bytes, an answer 1,048,576 bytes. · guard: an_answer_read_past_its_bound_is_a_timeout
decision/session-socket-closed-grammar · Session socket request is a closed grammar — A session socket connection carries one request line and one reply line, parsed against a closed versioned grammar with strict percent-decoding; anything outside it is refused and runs nothing, and a refusal or broken connection changes nothing.
decision/session-socket-unix-only · Session socket is Unix only — The driver session socket exists on Unix only; elsewhere a session is `Unsupported`.
decision/single-validation-gate · Single command validation gate — Every call, from the socket, the command line or MCP, is checked against a closed static table of verbs and bounded argument kinds, the first failure in a fixed order, and nothing public runs a command that did not come from that validation.
decision/state-dir-path-is-parameter · Session state path is a parameter — The session state directory path is a caller parameter, never read from the environment or defaulted relative to the working directory.
decision/state-directory-path-is-parameter · State directory path is a parameter — The state directory path is a parameter, never an environment read or a working-directory-relative default.
decision/visible-region-open-list-row · Visible region of an open select list row — For a row or a group of an open select list, the visible region answers the list's scrollport. · why: An engine change for every document, ratified.
contract/actions-answer/keyed · The actions answer · keyed — A boolean on each listed node.
contract/cli-exit-codes/0 · CLI exit codes · 0 — Accepted: the answer as one line of JSON on stdout.
contract/coverage-answer/line · The coverage answer · line — One line {"unexercised":[…],"checks":n,"held":m} on stdout, status 0, holding ids, owner ids, event names and the two counts only.
contract/driver-argument-kinds/text · Driver argument kinds · text — Text of 0 to 4096 bytes.
contract/driver-session-reply/no-attribute-text · driver session socket replies · attribute text — A reply never carries an attribute's text, except what an author wrote into a record's message or into the `error` field of a `caused` item.
contract/escher-session-coverage/answer-line · escher-session coverage role · answer line — The coverage answer holds handler references of ids, owner ids and event names and two counts, no process-local handle, and crosses only as one line on the command line's stdout, with no verb, socket or MCP carrier.
contract/mcp-server/transport · MCP server · transport — Newline-delimited JSON-RPC 2.0 on the invoking user's stdin and stdout, with no authentication and no environment read.
contract/mcp-tool-arguments/value-kinds · MCP tool arguments · value kinds — A string becomes text, an i64 a number, a bool a flag; any other value is refused malformed before anything runs, and a repeated key collapses to its last value.
contract/mcp-tool-call-span/ending · The MCP tool_call span · ending — One of accepted, refused or error.
contract/mcp-tool-call-span/tool · The MCP tool_call span · tool — The verb table's word for the called tool; the span parents the call's command span.
contract/recorded-check/answer · Recorded check file · answer — Required and a JSON object; key order is not enforced.
contract/session-socket-reply/ok-accepted · Session socket reply · ok v3 accepted — The call ran and was accepted; its answer JSON follows on the line.
contract/session-socket-request/call · Session socket request · call — `call v3 <verb> [<name>=<kind>:<value>]…` runs a validated verb, kinds `t` text, `n` whole number, `f` flag, verb, names and text percent-escaped.

## data-protection · ## Data Protection
decision/kept-check-actions-answer · Kept check holds the actions answer — A kept actions answer holds ids, owner ids, keyed booleans, the table's verb words and closed cause words, never a control value, typed text or NodeId. · why: A boundary widening, ratified. · guard: stand_replay a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes
decision/kept-check-answer-whole · Kept check keeps each answer whole — A kept check keeps each answer as the line the driver wrote, masked as on every carrier, with a shot kept as its width and height and never its pixels. · why: Ratified, a shot's pixels excepted.
decision/kept-check-call-content · Kept check call-content class — A kept check holds call content: target ids, press keys, advance ms and text typed into unmasked controls, a masked or unresolved type's text kept as MASKED_STAND_IN. · why: Ratified.
decision/kept-check-graph-answer · Kept check holds the graph answer — A kept graph answer holds ids, owner ids, component names, registered state names, handler event names and closed flag words and booleans, never a control value or typed text. · why: A boundary widening, ratified.
decision/kept-check-layout-answer · Kept check holds the layout answer — A kept check holds a layout step and its whole answer of ids, closed words and numbers, never a control value, typed text or NodeId, and its replay stops on a changed placed value. · guard: stand_replay a_kept_layout_step_replays_and_a_changed_one_stops_naming_placed
decision/log-sink-redacts-and-allowlists · Log sink redacts and allowlists targets — Escher's own log sink redacts records and drops a record from a target outside escher-telemetry's target allowlist.
contract/kept-check-file/contents · Kept check files · contents — A kept check holds per call its verb, arguments, acceptance, digest and masked answer, never a pixel.
decision/coverage-line-no-user-data · Coverage line carries no user data — The coverage answer line holds ids, owner ids, event names and counts only, never a control value, typed text, path or NodeId. · guard: stand_handler_coverage a_typed_sentinel_never_reaches_the_coverage_line
contract/kept-check-file/entry · Kept check file · entry — A check is a JSON array: an opening record of the app label and the format, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer and the answer itself as answer.
contract/kept-check-file/medium · Kept check files · medium — Under `--record <dir>`, `Session::recording_into` or an exploration's record directory, one `check-{n}.json` with mode `0600` per recorded session, each explored path its own, is written in a `0700` directory the launcher named.

## api-security · ## API Security
decision/document-abort-signal-cancels-fetches · Document abort signal cancels its fetches — Aborting the configured AbortSignal cancels every in-flight fetch tied to the document. · why: The browser DocumentLoader aborts the previous page's controller on each navigation and in abort_current, and iframe reloads abort the prior generation
decision/actions-answer-unpaged · The actions answer is unpaged — The actions verb takes no argument and a screen's whole answer is one answer under the 1,048,576-byte answer bound. · guard: stand_actions the_whole_cells_answer_is_within_the_answer_bound
decision/external-targets-to-system-browser · External targets open in the system browser — Navigation to external targets goes to the system browser.
decision/graph-answer-unpaged · The graph answer is unpaged — The graph verb takes no argument and a screen's whole graph is one answer under the 1,048,576-byte answer bound. · why: A part argument would widen the crossing. · guard: stand_graph the_whole_cells_answer_is_within_the_answer_bound
decision/no-wait-verb-fields · No wait argument or result on verbs — No driver verb has an argument or a result field of a wait.
contract/session-socket/oversize · Driver session socket · oversize — An answer is at most 1,048,576 bytes on its JSON line; over it the reply is oversize, read by the client as AnswerTooLarge.
contract/session-wire/recording-failed · Driver session wire · ok v3 recording-failed — Wire v3 replies include the fixed reply ok v3 recording-failed.

## dependency-security · ## Dependency Security
decision/cargo-llvm-cov-coverage-tool · cargo-llvm-cov as the coverage tool — cargo-llvm-cov 0.9.1 is the coverage tool installed in CI.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project)
decision/telemetry-sink-target-allowlist · Telemetry sink target allowlist — escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level. · except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed

## secret-management · ## Secret Management
contract/environment-variables/rust-log · Environment variables · RUST_LOG — Holds the tracing filter directives escher-telemetry reads, defaulting to warn when unset or unparsable.
decision/driver-reads-only-rust-log · Driver reads only RUST_LOG — The escher-driver library and the escher-session binary read no environment variable beyond RUST_LOG.

## error-handling · ## Error Handling
decision/closed-refusal-cause-set · Closed refusal cause set — A driver command refusal is a typed value whose cause comes from a closed set of eight, with a fault only beside `malformed`; a new verb or argument shape is refused within the set, never by adding a cause. · why: The closed set of eight was ratified.
decision/coverage-refuses-bad-checks-malformed · Coverage refuses an unusable check as malformed — The `coverage` role refuses a check file it cannot read, one past the check bound, one that is no check or a check of another app as `malformed`, holding nothing of the path or the file, and adds no cause.
decision/refusal-closed-cause-set · Command refusal as a closed set of causes — An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field. · why: So a refusal cannot hold anything a call supplied. · except: A Fault is carried only beside the malformed cause.
decision/refusal-holds-no-call-input · A refusal holds nothing a call supplied — Every refusal's name, meaning and remedy is a fixed static text holding no path, and a fault carries at most the schema's own argument name, so a refusal cannot hold anything a call supplied.
decision/refusal-log-cause-name-only · Only a refusal's cause name is logged — Exactly one thing of a refusal reaches a log: its cause's fixed name, as the `cause` field of the call's driver span; no fault, meaning, remedy or display text is logged.
decision/refusal-logged-by-cause-name · Refusal logged by cause name only — Of an escher-driver command refusal, only its cause's fixed name reaches a log.
contract/refusal-causes/covered · Driver refusal causes · covered — The target is covered at its hit point; its fixed meaning states the hit's limits, among them that an option of a closed select reads covered, remedied by clicking the select first.

## logging-monitoring · ## Logging & Monitoring
decision/closed-span-lines · One sink line per closed span — The sink prints one line per closed span whose target the allowlist admits and nothing on creation, record, enter or exit; each span field is judged by the event rule, an outside-target span writes no byte, and a content-named span field's value is never stored. · why: A boundary widening of the sink, ratified as built.
decision/engine-target-field-allowlist · Engine log targets keep only allowlisted fields — In escher's telemetry sink an engine target keeps only seven allowlisted fields, and every other field is written as [redacted]. · except: An allowlisted log.file field can still carry a host path.
decision/engine-target-safe-fields · Engine targets print only safe fields — An event whose target starts with blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer or js_console prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line; every other field, the message included, prints as [redacted].
decision/escher-target-content-fields-withheld · escher_ targets withhold content-named fields — In escher's telemetry sink an escher_ target keeps all fields but twelve content-named ones.
decision/escher-target-content-scrub · Escher targets scrub content-named fields — An event whose target starts with escher_ prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which print as [redacted].
decision/escher-telemetry-allowlist-sink · escher's allowlist telemetry sink — Both seven_guis binaries log through escher's own stderr-only allowlist subscriber. · except: The sink's scrubbing reaches that sink only, escher's scoped capture hiding its own field set: upstream's fmt::init subscribers, env_logger and the panic hook stay unscrubbed.
decision/no-screen-content-in-logs · Screen content stays out of logs — No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer, a coverage check and a step's caused text reach no log line at any level. · guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds
decision/other-log-targets-dropped · Other log targets dropped whole — In escher's telemetry sink any target other than an engine or escher_ target is dropped whole at every level, and spans follow the same rule.
decision/outside-target-dropped · Outside targets dropped whole — An event or bridged log record from a target outside the allowlist is not printed at all, zero bytes, at every level including WARN and ERROR, whatever RUST_LOG names. · why: The named cost: a third-party WARN or ERROR no longer prints either.
decision/session-output-holds-no-caller-input · Session output holds no caller input — No escher-session output line holds anything the caller typed. · except: The mcp role's echoed request id.
decision/stderr-only-telemetry-sink · Escher's own stderr telemetry sink — Both seven_guis binaries, the native app and the escher-session driver host, install escher's own telemetry sink first thing: a stderr-only subscriber whose formatter applies an allowlist of targets. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed, and the chained std panic hook still prints the raw panic message to stderr.
contract/command-span/cause · The command span line · cause — Recorded when the call is refused, as a closed cause word such as fonts-unavailable.
contract/command-span/line · The command span line · line — A sink-installing host writes one closed command span line per call it runs.
contract/command-span/verb · The command span line · verb — Records the instance-level verb; a graph, actions or layout call records the verb alone, plus cause when refused.
contract/escher-session-output/caller-text · escher-session output · caller-typed text — No output line echoes a caller's argument but the admitted request id the mcp role echoes; an answer carries each node's current value, so text typed into an unmasked control returns, a masked input reading only the mask.
contract/tool-call-span/ending · The tool_call span line · ending — One of the closed words accepted, refused or error.
contract/tool-call-span/line · The tool_call span line · line — One closed line per MCP tools/call, the parent of that call's command span, holding tool and ending only.
contract/tool-call-span/tool · The tool_call span line · tool — The tool table's word for the call, a closed word outside the content-named fields; absent for an unknown tool.
decision/js-console-to-debug-log · JS console routed to the debug log — Script console output (log, info, warn, error) goes to the log crate at debug level under target js_console, never to stdout or stderr. · why: Keeps stdout and stderr clean.
decision/tracing-feature-propagates · One tracing feature across the stack — Turning on dioxus-native's tracing feature also turns on tracing in dioxus-native-dom, blitz-shell, blitz-dom, blitz-html and blitz-net.
decision/tracing-for-engine-logging · tracing for engine logging — Engine crates log through the tracing crate behind an optional tracing feature, and their logging macros compile only with that feature on.
contract/escher-session-coverage-line/content · The escher-session coverage role's stdout line · content — The coverage line holds ids, owner ids, event names and two counts, and nothing of a named path.
contract/escher-session-output/esc-byte · escher-session output · ESC byte — No output line holds an ESC byte; output is uncoloured whatever the stream is.
contract/escher-session-stdout/command-line · escher-session stdout · command result line — escher-session writes one uncoloured JSON line per command on stdout.
contract/resource-load-failure-log/waiting-nodes · Resource load failure log · waiting_nodes — A resource load failure log carries structured field waiting_nodes.

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
