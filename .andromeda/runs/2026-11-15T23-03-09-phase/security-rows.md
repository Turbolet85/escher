# security-plan · the rows that bind 2026-11-15-session-binary-smoke — picked 141 of 550 (86 by every draw) · rule any · pin 1209

## threat-model-summary · ## Threat Model Summary
decision/driver-access-by-filesystem-permissions · Driver session access by filesystem permissions — The driver session's state directory is created 0700, a present one open to its group or others is refused, and the socket is 0600, so access to the session is the owning user's.
decision/driver-session-no-peer-authentication · Driver session and MCP server carry no authentication — The driver session socket and the MCP stdio server carry no handshake, token, secret or peer check, and session start does not check the answering pid; the trust boundary is the owning user or the spawning client process.
decision/driver-session-unix-socket-only · Driver session on one Unix socket — The driver session socket binds one Unix-domain socket in a caller-named state directory, never a TCP or UDP port, and exists on unix only, while an in-process session runs on linux, windows and macOS alike.
decision/session-socket-carries-calls · Session socket carries calls and answers — The session socket carries the lifecycle and calls with their answers: an element id and typed text in; ids, accessible names, control values with a password's or file input's as the fixed mask, the snapshot text and the diff out; no NodeId crosses. · why: A boundary widening, ratified.
contract/driver-session-reply/masked-control-values · driver session socket replies · control values — A password's or file input's control value is carried masked.
contract/driver-session-reply/no-node-id · driver session socket replies · NodeId — No reply carries a NodeId.

## input-validation · ## Input Validation
decision/accessible-name-exits · Ratified exits of accessible names — Accessible names leave the process only through the platform accessibility adapter, a hosted call's answer in the snapshot text and diff JSON over the session socket and the client's stdout, and MCP tool results, and reach no log. · why: Each carrier ratified.
decision/accessible-names-not-logged · Accessible names kept out of logs — Escher's own code logs no accessible name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level, the third-party record that carried one being dropped whole. · why: That third-party target is outside the sink's allowlist. · guard: mcp_log
decision/agent-run-selection-allowlist · agent-run.sh selection reaches cargo only as a test stem — agent-run.sh exits 2 with usage before any cargo call on an unknown verb, a wrong argument count or a selection other than `stand`, `all` or an existing test-file stem matching `^[a-z0-9_]+$`.
decision/attribute-text-leaves-by-ratified-carriers · Attribute text leaves only by ratified carriers — Markup attribute values, names and descriptions leave escher only by the ratified carriers. · except: The page's own form submission and events.
decision/cli-closed-argv-reader · Closed argv reader — The session command line reads argv with a closed reader with fixed rules, and checks a call before the session address is asked for.
decision/cli-echoes-nothing-typed · Command line echoes nothing typed — No command-line ending writes back anything the caller typed, and no path on argv panics.
decision/closed-driver-verb-schema · Closed driver verb schema — Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing. · why: Each widening of the verb table is a widening of a validated surface, ratified.
decision/descriptions-adapter-only · Descriptions cross only to the accessibility adapter — `aria-describedby` targets and `aria-description` text reach only the platform accessibility adapter; the snapshot reads no description, so no driver or MCP carrier, verb, field, log field or span carries one. · why: Upstream's behaviour, ratified as a boundary widening by upstream merge. · guard: accessibility_description
decision/driver-installs-no-subscriber · Driver installs no subscriber — The driver crate and its command line install no tracing subscriber and read no environment variable; clocks are read only by the host's idle expiry, the MCP server on a request, the session's bounded load wait, and the client's bounded stop and ready waits.
decision/driver-no-subscriber-no-env · Driver installs no subscriber and reads no env — The driver crate and its session command line install no tracing subscriber and read no environment variable.
decision/driver-socket-closed-grammar · Driver socket closed request grammar — The driver session socket takes one request line per connection, parsed against a closed grammar under a size bound, one connection at a time under fixed bounds; anything else is refused with nothing run, and a decoded call runs through the session's one run path alone.
decision/fixture-name-closed-table · Fixture state name checked against a closed table — A fixture state name is compared whole against the app's closed table of named states before any session starts; any other name is refused as an unknown state and nothing is booted.
decision/no-echo-of-caller-input · No echo of caller input — No command-line ending writes back anything the caller typed: no path, label, id, value or file content.
decision/one-validation-path-for-every-carrier · One validation path for every carrier — The session socket, the command line and MCP each pass a call through the schema's validation before anything runs, and nothing public runs a command that did not come from that validation. · why: The crossing to external input surfaces was ratified with this check as the one they all pass through.
decision/session-label-form · Session label form — A session label is 1 to 32 bytes of a-z, 0-9 and -, checked before the caller's boot runs; it is never document text, and is the one caller string a reply carries.
decision/session-socket-bounds · Session socket bounds — One connection at a time; each read and write under 2 s, a client's read of an answer under 30 s, a request line at most 16,384 bytes, a lifecycle reply 128 bytes, an answer 1,048,576 bytes. · guard: an_answer_read_past_its_bound_is_a_timeout
decision/session-socket-closed-grammar · Session socket request is a closed grammar — A session socket connection carries one request line and one reply line, parsed against a closed versioned grammar with strict percent-decoding; anything outside it is refused and runs nothing, and a refusal or broken connection changes nothing.
decision/session-socket-unix-only · Session socket is Unix only — The driver session socket exists on Unix only; elsewhere a session is `Unsupported`.
decision/single-validation-gate · Single command validation gate — Every call, from the socket, the command line or MCP, is checked against a closed static table of verbs and bounded argument kinds, the first failure in a fixed order, and nothing public runs a command that did not come from that validation.
decision/state-dir-deletes-only-own · Session cleanup deletes only its own files — Stopping or idle expiry removes the socket file and then the directory, non-recursively, and nothing else is ever deleted.
decision/state-dir-owner-only · Session state directory is owner-only — The session state directory is created mode 0700 and a present one granting group or others any bit is refused; the socket file is set 0600.
decision/state-dir-path-is-parameter · Session state path is a parameter — The session state directory path is a caller parameter, never read from the environment or defaulted relative to the working directory.
decision/state-directory-long-path-refused · Too-long state directory path refused — A state directory path too long for a socket address is refused with a fixed message holding no path.
decision/state-directory-non-recursive-delete · Non-recursive session cleanup — Only the socket file and then the state directory are ever deleted, non-recursively.
decision/state-directory-path-is-parameter · State directory path is a parameter — The state directory path is a parameter, never an environment read or a working-directory-relative default.
contract/agent-run-cli/2 · agent-run.sh exit codes · 2 — A verb outside boot run status cleanup logs, a wrong argument count, or a selection other than stand, all or an existing test-file stem prints usage and exits 2 before any cargo call.
contract/cli-flags/id · escher-session flags · --id — Followed by one value, whatever that word is.
contract/cli-flags/key · escher-session flags · --key — Followed by one value, whatever that word is.
contract/cli-flags/ms · escher-session flags · --ms — Followed by one value, whatever that word is.
contract/cli-flags/part · escher-session flags · --part — Followed by one value, whatever that word is.
contract/cli-flags/session · escher-session flags · --session — Followed by a directory, given exactly once, else usage.
contract/cli-flags/shift · escher-session flags · --shift — Bare, taking no value.
contract/cli-flags/text · escher-session flags · --text — Followed by one value, whatever that word is.
contract/cli-roles/serve · escher-session roles · serve — serve <app> --session <dir> in exactly that shape, optionally with one --record <dir> after it, else usage.
contract/driver-argument-kinds/id · Driver argument kinds · id — Text of 1 to 1024 bytes.
contract/driver-argument-kinds/key · Driver argument kinds · key — One name of a closed twelve-name list.
contract/driver-argument-kinds/milliseconds · Driver argument kinds · milliseconds — A whole number from 1 to 60000.
contract/driver-argument-kinds/part · Driver argument kinds · part — A whole number from 1 to 1000.
contract/driver-argument-kinds/text · Driver argument kinds · text — Text of 0 to 4096 bytes.
contract/driver-call-id/length-bound · Driver call id · length bound — An id a call names, from the command-line `--id` flag, the socket's `call` request or an MCP tool's `id` argument, is bounded at 1 to 1024 bytes, checked in the client before a session is touched and again in the host.
contract/driver-session-reply/state-readings · driver session socket replies · state readings — State readings are carried as closed words and booleans.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The call was accepted: one uncoloured JSON line on stdout holds the answer, or for the coverage role its one coverage line.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The call was refused: one uncoloured JSON line on stdout holds the refusal, or for the coverage role a malformed refusal.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — A usage error: the usage line goes to stderr and stdout stays empty.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — A session error: the session-error object on stdout and the session error's fixed message on stderr.
contract/session-errors/state-dir-too-long · Session errors · state-dir-too-long — The state directory path is too long for a socket address; the message states the bound, 94 bytes on Linux and 90 elsewhere, holds no path, and the remedy names a shorter or relative path; status 3.
contract/session-socket-reply/ok-accepted · Session socket reply · ok v3 accepted — The call ran and was accepted; its answer JSON follows on the line.
contract/session-socket-reply/ok-refused · Session socket reply · ok v3 refused — The call was refused; the refusal JSON follows on the line.
contract/session-socket-request/call · Session socket request · call — `call v3 <verb> [<name>=<kind>:<value>]…` runs a validated verb, kinds `t` text, `n` whole number, `f` flag, verb, names and text percent-escaped.
contract/session-socket-request/hello · Session socket request · hello — `hello v3` asks for the session's lifecycle reply.
contract/session-socket-request/stop · Session socket request · stop — `stop v3` stops the session once its reply is written.
decision/boundary-widening-rule · Boundary widening rule — A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
decision/closed-value-field-no-widening · A closed-value field in a ratified carrier is no widening — A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does. · why: Ratified as the standing rule.
decision/command-span-content-free · Command span holds no content — One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
decision/diff-counts-only-in-logs · Only diff sizes reach a log — A diff's content never reaches a log or event, while its three sizes (counts of added, removed and changed nodes) are recorded on the driver's command span, printed at `info` or below. · why: Ratified: a diff's content never reaches a log while its three sizes may.
decision/driver-outcomes-never-logged · Driver outcomes never logged — The driver crate logs and fields no command, call, argument, outcome, refusal, screen text, image, finding, session label or id record; an outcome is written only as a command-line answer, an MCP tool result, a session-socket reply or a recording file the caller asked for. · why: Outcomes leave the process only by the ratified crossing.
decision/no-panic-on-call-input · No panic on call-supplied input — Neither the driver command schema's validation nor the executor has unwrap, expect, panic or unreachable on anything a call supplies.
decision/session-label-only-caller-string · Session label is the one echoed caller string — The session label is never text read from the document and is the one caller string a reply carries.
decision/socket-carries-screen-reading · Session socket carries what the screen reads — An answer carries what the screen reads, ids, accessible names, control values with password and file input masked, snapshot text, diff, lint findings, UI graph, never a NodeId or an attribute value as typed. · why: The crossing was ratified.
decision/stable-id-exits · Ratified exits of the stable id — The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files. · why: Each carrier ratified as a boundary widening.
decision/stable-id-not-logged · Stable ids kept out of logs — Escher's own code writes the stable id to no log, and a host that installs escher's sink drops third-party style records carrying an element's HTML id whole, printing that value at no `RUST_LOG` level. · why: Those third-party targets are outside the sink's allowlist.
decision/target-refusal-order · Fixed order of target refusals — A click or type on an element that cannot take it is refused with nothing run, the first of disabled, off-screen, covered that holds, leaving the snapshot and both focus readings as they were; scroll reports in_view instead of refusing. · why: The detection of refusal causes by code was ratified.
decision/unknown-id-refused-stale-or-not-found · Unknown id refused as stale or not-found — An id that names no element on the screen as it reads now is refused with nothing run: stale when the session's record of ids holds it, not-found when not.
decision/unknown-target-refused · Call on an unknown or unfit element refused — A call whose id names no element, or an element that cannot take the call, is refused with nothing run.
contract/actions-answer/keyed · The actions answer · keyed — A boolean on each listed node.
contract/actions-answer/nodes · The actions answer · nodes — `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, one entry per actionable node, holding ids, owner ids, `keyed` booleans, verb words and closed cause words, never a control value, typed text or NodeId.
contract/cli-exit-codes/0 · CLI exit codes · 0 — Accepted: the answer as one line of JSON on stdout.
contract/diff-node/clickable · Diff node keys · clickable — A boolean always written beside `focused`, true where the element's listeners hold `click` and it is no disabled form control; a changed node's `differs` can name it.
contract/diff-node/differs · Diff node keys · differs — On a changed node only, after `bounds`: a list of the node-key names that changed; an added node is written without it.
contract/diff-node/invalid · Diff node keys · invalid — A boolean, always written.
contract/diff-node/pressed · Diff node keys · pressed — A boolean, written only where the element states it.
contract/diff-node/selected · Diff node keys · selected — A boolean, written only where the element states it.
contract/driver-argument-kinds/flag · Driver argument kinds · flag — True or false.
contract/driver-argument-kinds/name · Driver argument kinds · name — Text of 1 to 32 bytes of a-z, 0-9 and -.
contract/driver-id-refusal/not-found · Driver id refusal · not-found — A call naming an id that no element on the current screen carries is refused as `not-found` when the session's record does not hold that id.
contract/driver-id-refusal/stale · Driver id refusal · stale — A call naming an id that no element on the current screen carries is refused as `stale` when the session's record holds that id.
contract/driver-session-label/format · driver session label · format — A label is 1 to 32 bytes of a-z, 0-9 and -, checked before the caller's boot runs.
contract/driver-session-reply/no-attribute-text · driver session socket replies · attribute text — A reply never carries an attribute's text.
contract/driver-verbs/actions · Driver verbs · actions — Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent, never a control value, typed text or NodeId.
contract/driver-verbs/graph · Driver verbs · graph — Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[…],"states":[…]} of ids, owner ids, component names, state names, handler event names, flag words and booleans, never a control value, typed text or NodeId.
contract/graph-answer/nodes-states · Graph answer · nodes and states — `{"nodes":[…],"states":[…]}` holding ids, component names, state names, handler event names, the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or NodeId.
contract/session-socket-reply/ok-oversize · Session socket reply · ok v3 oversize — The call ran but its answer is over 1,048,576 bytes.

## api-security · ## API Security
decision/host-idle-expiry · Host idle expiry — A host that has answered no request for IDLE_EXPIRY, 1,800 s, ends exactly as on stop; every answered request, refused or malformed included, restarts the count, and a connection broken before its request is whole does not. · why: The answer to the orphaned-host question: a host whose client died without stop kept running.
decision/session-host-idle-expiry · Session host idle expiry — A session host that answers no request for its idle expiry ends as on `stop`.
decision/session-socket-one-bounded-connection · One bounded connection at a time — The local session socket serves one connection at a time under fixed read, write and size bounds.
decision/session-socket-owner-only · Session socket reachable by the owning user only — Access to the driver session socket is the filesystem's alone: the state directory is 0700, the socket file 0600, a directory open to its group or others is refused, and there is no handshake secret, token or peer-credential check.
decision/session-socket-sole-listener · Session socket as the one listener — The driver session's Unix-domain socket is the project's one listener, serving the session lifecycle and driver calls and their answers.
contract/driver-answer/settled · Driver command answer · settled — A step that ran and did not go quiet is a result, settled false with its busy class, not a refusal; the outcome names a class only, never a URL, request, node, id, name or value.
contract/driver-command-line/answer-bounds · Driver command line · answer bounds — A started host has 30 s to answer and each read of a call's answer is bounded at 30 s; these bound a transport and wait on no load.
contract/session-socket/connections · Driver session socket · connections — One connection is served at a time, with a 2 s read and write bound.
contract/session-socket/lifecycle-reply · Driver session socket · lifecycle reply — A lifecycle reply is at most 128 bytes on its JSON line.
contract/session-socket/stop · Driver session socket · stop — stop removes the socket and the state directory and writes no record.
decision/actions-answer-unpaged · The actions answer is unpaged — The actions verb takes no argument and a screen's whole answer is one answer under the 1,048,576-byte answer bound. · guard: stand_actions the_whole_cells_answer_is_within_the_answer_bound
decision/bounded-load-wait · Bounded wait on loads in flight — A harness settle never waits on a load in flight, and the driver's acting step waits on one for at most `LOAD_WAIT`. · why: So the absent request timeout cannot hang a settle or a command.
decision/graph-answer-unpaged · The graph answer is unpaged — The graph verb takes no argument and a screen's whole graph is one answer under the 1,048,576-byte answer bound. · why: A part argument would widen the crossing. · guard: stand_graph the_whole_cells_answer_is_within_the_answer_bound
decision/load-wait-bound · Acting commands bounded by LOAD_WAIT — An acting driver command waits on a load in flight for at most LOAD_WAIT, 5 s of wall time, moving no time on the held instance, then returns the load still in flight as settled false with its busy class. · why: The absent request timeout cannot hang a command.
decision/no-wait-verb-fields · No wait argument or result on verbs — No driver verb has an argument or a result field of a wait.
decision/settle-never-waits-on-loads · Settle never waits on a load — A harness settle reports a load in flight at once as not settled with the class Loads, waits on no clock and moves none, and its pass loop is bounded at 64 passes. · why: The absent request timeout cannot hang a settle.
decision/time-moves-only-on-advance · Time moves only on advance — Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph and actions wait on no load.
contract/session-socket/oversize · Driver session socket · oversize — An answer is at most 1,048,576 bytes on its JSON line; over it the reply is oversize, read by the client as AnswerTooLarge.
contract/session-socket/request-line · Driver session socket · request line — A request line is at most 16,384 bytes.

## dependency-security · ## Dependency Security
decision/locked-builds · Locked builds — Every build, the flake, the bundle and every CI cargo leg, passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock

## secret-management · ## Secret Management
decision/driver-reads-only-rust-log · Driver reads only RUST_LOG — The escher-driver library and the escher-session binary read no environment variable beyond RUST_LOG.
decision/live-evidence-holds-no-credentials · Live evidence holds no credentials — No committed live evidence, artifact, event or verdict carries a key, auth header, credential-file name, token field, e-mail address or the operator's login.
contract/environment-variables/rust-log · Environment variables · RUST_LOG — Holds the tracing filter directives escher-telemetry reads, defaulting to warn when unset or unparsable.

## error-handling · ## Error Handling
decision/closed-refusal-cause-set · Closed refusal cause set — A driver command refusal is a typed value whose cause comes from a closed set of eight, with a fault only beside `malformed`; a new verb or argument shape is refused within the set, never by adding a cause. · why: The closed set of eight was ratified.
decision/refusal-closed-cause-set · Command refusal as a closed set of causes — An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field. · why: So a refusal cannot hold anything a call supplied. · except: A Fault is carried only beside the malformed cause.
decision/refusal-holds-no-call-input · A refusal holds nothing a call supplied — Every refusal's name, meaning and remedy is a fixed static text holding no path, and a fault carries at most the schema's own argument name, so a refusal cannot hold anything a call supplied.
decision/session-error-fixed-table · Session errors from a fixed table — An escher-driver session error is written from a fixed table, holding nothing a call supplied.
decision/session-errors-fixed-table · Session errors as one fixed table — Each session error's kind, meaning and remedy is one row of a fixed table served by `help`, every text a fixed string holding no path, carried on stdout and in MCP results as the program's own text. · why: Ratified as the program's own fixed text inside the ratified carriers.
contract/cli-exit-codes/1 · escher-session CLI exit codes · 1 — The call was refused; the refusal JSON is written to stdout.
contract/cli-exit-codes/2 · escher-session CLI exit codes · 2 — Usage: the argv shape is not one the command takes; one fixed usage line on stderr, stdout empty, nothing booted and no state created.
contract/cli-exit-codes/3 · CLI exit codes · 3 — Session error: the error object with kind, message and remedy on stdout and its fixed message on stderr; a boot that fails ends with its session error.
contract/refusal-json/cause · Refusal JSON · cause — The refusal's cause name, inside `{"refused":{...}}`, built from the cause's fixed strings.
contract/session-error-json/kind · Session error JSON · kind — One fixed kebab-case word per session error variant, inside `{"error":{...}}`.
decision/reach-error-closed-set · Reach errors as a closed set with fixed texts — A reach error is one of a closed set: unknown state, refused at a step with the step's index and its closed refusal cause, or not settled at a step with the busy reading; each reads a fixed text holding nothing a caller supplied. · guard: a_reach_error_reads_fixed_text_naming_no_name_tried
decision/refusal-log-cause-name-only · Only a refusal's cause name is logged — Exactly one thing of a refusal reaches a log: its cause's fixed name, as the `cause` field of the call's driver span; no fault, meaning, remedy or display text is logged.
decision/refusal-logged-by-cause-name · Refusal logged by cause name only — Of an escher-driver command refusal, only its cause's fixed name reaches a log.
contract/refusal-causes/malformed · Driver refusal causes · malformed — The one cause that carries a fault; a call breaking the schema, such as a `graph` or `actions` call with an argument, is refused with it and nothing runs.
contract/refusal-causes/unknown-verb · Driver refusal causes · unknown-verb — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-json/fault · Refusal JSON · fault — Present only on a `malformed` refusal, naming the fault.
contract/refusal-json/meaning · Refusal JSON · meaning — The cause's fixed meaning text.
contract/refusal-json/remedy · Refusal JSON · remedy — The cause's fixed remedy text.
contract/session-error-json/message · Session error JSON · message — The error's fixed meaning, holding no path.
contract/session-error-json/remedy · Session error JSON · remedy — The error's fixed remedy text.

## logging-monitoring · ## Logging & Monitoring
decision/agent-run-events-metadata-only · Agent-run events hold harness metadata only — The agent-run test contract's JSON-line events carry no content-named field, only harness metadata, and never captured test output or panic text.
decision/agent-run-raw-log-unprinted · Agent-run raw log kept unprinted — The agent-run raw cargo and libtest output log is unscrubbed, never printed by logs, and gitignored.
decision/session-output-holds-no-caller-input · Session output holds no caller input — No escher-session output line holds anything the caller typed. · except: The mcp role's echoed request id.
contract/escher-session-output/accepted-stderr · escher-session output · stderr of an accepted command — An accepted command's stderr is empty with RUST_LOG unset.
contract/escher-session-output/caller-text · escher-session output · caller-typed text — No output line echoes a caller's argument but the admitted request id the mcp role echoes; an answer carries each node's current value, so text typed into an unmasked control returns, a masked input reading only the mask.
contract/escher-session-output/esc-byte · escher-session output · ESC byte — No output line holds an ESC byte; output is uncoloured whatever the stream is.
contract/escher-session-stderr/error-messages · escher-session stderr · error messages — escher-session writes error messages on stderr.
contract/escher-session-stderr/usage-messages · escher-session stderr · usage messages — escher-session writes usage messages on stderr.
contract/escher-session-stdout/command-line · escher-session stdout · command result line — escher-session writes one uncoloured JSON line per command on stdout.
decision/no-screen-content-in-logs · Screen content stays out of logs — No driver call puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer and a coverage check reach no log line at any level. · guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds

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
