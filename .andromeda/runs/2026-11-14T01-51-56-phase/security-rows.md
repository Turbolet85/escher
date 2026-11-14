# security-plan · the rows that bind 2026-11-14-seeded-randomness — picked 184 of 546 (109 by every draw) · rule any · pin 741

## threat-model-summary · ## Threat Model Summary
decision/bootstrap-native-helper-removed · Bootstrap native helper hidden from pages — The URLSearchParams bootstrap's native search-params helper is deleted from globalThis before any page script runs.
decision/capability-statement-mcp-crossing · Committed statement of the MCP crossing — A committed capability statement tells its readers what an MCP client carries onward: the spawning client as the boundary with no authentication, the carried set, and onward use as the client's; it adds no carrier.
decision/coverage-check-files-untrusted · Kept check files read as untrusted — The coverage role reads each caller-named kept check as untrusted input through a bounded read and a strict parse; an unreadable, oversized, non-check or other-app file is refused malformed, holding nothing of the path or the file.
decision/driver-session-unix-socket-only · Driver session on one Unix socket — The driver session binds one Unix-domain socket in a caller-named state directory, never a TCP or UDP port, and runs on unix only.
decision/mcp-server-stdio-only · MCP server on stdio only — The MCP server is a stdio child an MCP client spawns, speaking on the invoking user's stdin and stdout, with no listener, port or socket.
decision/mcp-stdio-no-auth · MCP server over stdio, no authentication — The driver's MCP server runs as a stdio child a client spawns under the invoking user, JSON-RPC 2.0 on stdin and stdout, no listener, port or socket; the spawning client is the trust boundary, with no authentication.
decision/session-socket-carries-calls · Session socket carries calls and answers — The session socket carries the lifecycle and calls with their answers: an element id and typed text in; ids, accessible names, control values with a password's or file input's as the fixed mask, the snapshot text and the diff out; no NodeId crosses. · why: A boundary widening, ratified.
decision/snapshot-state-readings · Three state readings per control — The snapshot text and the diff hold three state readings per control, selected, pressed and invalid, as closed words and booleans. · why: Inside the ratified crossing, ratified.
decision/driver-session-no-peer-authentication · Driver session and MCP server carry no authentication — The driver session socket and the MCP stdio server carry no handshake, token, secret or peer check, and session start does not check the answering pid; the trust boundary is the owning user or the spawning client process.
decision/skip-snapshot-of-unstyled-node · Never-styled nodes not snapshotted — Snapshotting a never-styled node is skipped. · why: Stylo invalidation would otherwise panic.
contract/driver-session-reply/no-node-id · driver session socket replies · NodeId — No reply carries a NodeId.

## input-validation · ## Input Validation
decision/advance-moves-virtual-clock-exactly · Advance moves the virtual clock exactly — The advance verb, accepted on every session, moves the session's virtual clock by exactly the milliseconds asked inside one settled step and reports exactly that time. · why: Ratified.
decision/anchored-id-tier · Anchored stable-id tier — An unkeyed element's stable id is anchored under the nearest author-keyed element of its own component instance before falling back to its component path or document path. · why: The anchored tier was ratified.
decision/boundary-widening-rule · Boundary widening rule — A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
decision/cli-closed-argv-reader · Closed argv reader — The session command line reads argv with a closed reader with fixed rules, and checks a call before the session address is asked for.
decision/cli-echoes-nothing-typed · Command line echoes nothing typed — No command-line ending writes back anything the caller typed, and no path on argv panics.
decision/closed-driver-verb-schema · Closed driver verb schema — Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing. · why: Each widening of the verb table is a widening of a validated surface, ratified.
decision/closed-value-field-no-widening · A closed-value field in a ratified carrier is no widening — A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does. · why: Ratified as the standing rule.
decision/command-schema-no-clock · Command schema reads no clock — The command schema and its validation read no clock.
decision/command-span-content-free · Command span holds no content — One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
decision/driver-installs-no-subscriber · Driver installs no subscriber — The driver crate and its command line install no tracing subscriber and read no environment variable; clocks are read only by the host's idle expiry, the MCP server on a request, the session's bounded load wait, and the client's bounded stop and ready waits.
decision/driver-no-subscriber-no-env · Driver installs no subscriber and reads no env — The driver crate and its session command line install no tracing subscriber and read no environment variable.
decision/driver-outcomes-never-logged · Driver outcomes never logged — The driver crate logs and fields no command, call, argument, outcome, refusal, screen text, image, finding, session label or id record; an outcome is written only as a command-line answer, an MCP tool result, a session-socket reply or a recording file the caller asked for. · why: Outcomes leave the process only by the ratified crossing.
decision/driver-socket-closed-grammar · Driver socket closed request grammar — The driver session socket takes one request line per connection, parsed against a closed grammar under a size bound, one connection at a time under fixed bounds; anything else is refused with nothing run, and a decoded call runs through the session's one run path alone.
decision/mcp-bounded-lines · Bounded MCP requests and results — MCP request lines and results are bounded, and tool arguments are typed before validation.
decision/mcp-listing-fixed-text · MCP listing is fixed program text — The MCP tool listing's descriptions are the program's own fixed table text: each verb's help and each result field's name, kind and help. · why: Ratified as the program's own text inside the ratified carrier.
decision/mcp-no-print-or-panic · MCP server never prints or panics — The MCP server has no print, unwrap, expect or panic outside the tests.
decision/mcp-over-stdio · MCP over stdio only — The MCP server speaks newline-delimited JSON-RPC 2.0 on the invoking process's stdin and stdout, with no listener, port, socket, authentication or environment read. · why: The MCP carrier and its bounds were ratified as is.
decision/no-echo-of-caller-input · No echo of caller input — No command-line ending writes back anything the caller typed: no path, label, id, value or file content.
decision/no-panic-on-call-input · No panic on call-supplied input — Neither the driver command schema's validation nor the executor has unwrap, expect, panic or unreachable on anything a call supplies.
decision/one-command-span-per-call · One command span per call — Every call handed to the session is wrapped in one INFO span named command under the escher_driver target, entered before validation so a refusal is inside it, holding fixed words, booleans and counts only.
decision/one-validation-path-for-every-carrier · One validation path for every carrier — The session socket, the command line and MCP each pass a call through the schema's validation before anything runs, and nothing public runs a command that did not come from that validation. · why: The crossing to external input surfaces was ratified with this check as the one they all pass through.
decision/recorded-check-strict-reader · Recorded check is read strictly as untrusted — A kept check is read as untrusted bytes, at most 1,048,576 bytes and 400 records of closed keys; anything else is refused `malformed` with no fault, holding nothing of the file, with no panic path.
decision/recorded-check-untrusted · Recorded check bytes are untrusted — A kept check is read by a strict reader bounded in bytes and records, with closed keys and schema-named arguments, anything else refused as malformed holding nothing of the file, no panic path; every replayed step is validated again; the coverage role reads checks through the same reader.
decision/replay-through-session-run · Replay runs each step through the session — A replayed check runs each kept step through the session's own validated run alone, runs nothing on a session of another app and stops at the first step whose acceptance or kept answer differs.
decision/search-params-native-parser-hidden · Native search-params parser hidden from pages — The native search-params parser is deleted from the global object before any page script runs. · why: An engine change for every document, ratified. · guard: url_search_params
decision/session-socket-bounds · Session socket bounds — One connection at a time; each read and write under 2 s, a client's read of an answer under 30 s, a request line at most 16,384 bytes, a lifecycle reply 128 bytes, an answer 1,048,576 bytes. · guard: an_answer_read_past_its_bound_is_a_timeout
decision/session-socket-closed-grammar · Session socket request is a closed grammar — A session socket connection carries one request line and one reply line, parsed against a closed versioned grammar with strict percent-decoding; anything outside it is refused and runs nothing, and a refusal or broken connection changes nothing.
decision/single-validation-gate · Single command validation gate — Every call, from the socket, the command line or MCP, is checked against a closed static table of verbs and bounded argument kinds, the first failure in a fixed order, and nothing public runs a command that did not come from that validation.
decision/url-search-params-bootstrap · URLSearchParams through a bootstrap script — URLSearchParams and a URL's live searchParams, href and search reach page script through a bootstrap script, bad calls and receivers throwing TypeError and values passing USV conversion. · why: A realization on the registered script crossing, no boundary widening; ratified. · guard: url_search_params
contract/cli-roles/serve · escher-session roles · serve — serve <app> --session <dir> in exactly that shape, optionally with one --record <dir> after it, else usage.
contract/diff-node/clickable · Diff node keys · clickable — A boolean always written beside `focused`, true where the element's listeners hold `click` and it is no disabled form control; a changed node's `differs` can name it.
contract/diff-node/differs · Diff node keys · differs — On a changed node only, after `bounds`: a list of the node-key names that changed; an added node is written without it.
contract/diff-node/invalid · Diff node keys · invalid — A boolean, always written.
contract/diff-node/pressed · Diff node keys · pressed — A boolean, written only where the element states it.
contract/diff-node/selected · Diff node keys · selected — A boolean, written only where the element states it.
contract/driver-command-span/cause · The driver command span · cause — The name of the returned refusal's cause, unrecorded until it applies.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The call was accepted: one uncoloured JSON line on stdout holds the answer, or for the coverage role its one coverage line.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The call was refused: one uncoloured JSON line on stdout holds the refusal, or for the coverage role a malformed refusal.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — A usage error: the usage line goes to stderr and stdout stays empty.
contract/mcp-server/transport · MCP server · transport — Newline-delimited JSON-RPC 2.0 on the invoking user's stdin and stdout, with no authentication and no environment read.
contract/mcp-tool-arguments/value-kinds · MCP tool arguments · value kinds — A string becomes text, an i64 a number, a bool a flag; any other value is refused malformed before anything runs, and a repeated key collapses to its last value.
contract/mcp-tool-call-span/ending · The MCP tool_call span · ending — One of accepted, refused or error.
contract/mcp-tool-result/size-bound · MCP tool results · size bound — A result's text over 1,048,576 bytes is replaced by the answer-too-large result; a result holding an image is bounded by its whole written form.
contract/recorded-check/arguments · Recorded check file · arguments — Argument names drawn from that verb's schema.
contract/recorded-check/start-record · Recorded check file · start record — The array opens with `{"tool":"start","arguments":{"app":…}}`.
contract/session-socket-reply/ok-refused · Session socket reply · ok v3 refused — The call was refused; the refusal JSON follows on the line.
contract/session-socket-reply/refused-malformed · Session socket reply · refused malformed — A request line the grammar does not read or over 16,384 bytes; nothing runs.
contract/session-socket-request/call · Session socket request · call — `call v3 <verb> [<name>=<kind>:<value>]…` runs a validated verb, kinds `t` text, `n` whole number, `f` flag, verb, names and text percent-escaped.
contract/snapshot-text/clickable · Snapshot text state words · clickable — Written after `focused` only when the element's current listeners hold `click` and it is no disabled form control.
contract/snapshot-text/id-field · Snapshot text · id field — Each node's id is written as a quoted `id=` field in the string's Debug form, so the field never holds a line break or a bare `"`; the snapshot has one text form and no JSON form, Display form or parser of the text.
contract/snapshot-text/invalid · Snapshot text state words · invalid — A bare word written for an element that states it is invalid.
contract/snapshot-text/name-field · Snapshot text · name field — Each line's accessible name is written as a quoted string in the string's Debug form, so a name holding a line break or a `"` stays inside its one field.
contract/snapshot-text/pressed · Snapshot text state words · pressed — A bare word written for an element that states it is pressed.
contract/snapshot-text/selected · Snapshot text state words · selected — A bare word written for an element that states it is selected.
contract/snapshot-text/state-words · Snapshot text · state words — Control states are written as the bare words `selected` / `unselected`, `pressed` / `unpressed` and `invalid`; in JSON answers they are booleans.
contract/snapshot-text/unpressed · Snapshot text state words · unpressed — A bare word written for an element that states it is not pressed.
contract/snapshot-text/unselected · Snapshot text state words · unselected — A bare word written for an element that states it is not selected.
contract/stable-element-id/anchored-path · Stable element id · anchored path — An element with no author key reads `{key}//{segment}` (then `/{segment}` per further DOM level) under the nearest keyed element of its own component instance, else its component path, else its document path; an anchor acts inside one component only.
contract/stable-element-id/author-key · Stable element id · author key — An element's HTML `id` is its author key only when it is non-empty, holds no `/` and is the first element in document pre-order to claim it; an empty, `/`-bearing or later-duplicate value gives no key and anchors nothing.
contract/stable-element-id/derivation · The stable element id · derivation — The HTML `id` is the author key when non-empty, `/`-free and first in document order; otherwise an anchored, component or document path is used, so no `id` can make two ids equal.
contract/stable-element-id/stability · Stable element id · stability — A stable id is never a list index, pointer, hash, clock or process-local value, so a row reads the same id across a re-render, a remount and a second process.
decision/aria-label-trimmed-once · One trimmed aria-label read — An `aria-label` non-empty after trimming becomes the node's label and a whitespace-only one names nothing; there is one trimmed, non-empty `aria-label` read. · why: Follows accname-1.2; upstream's unfiltered read was dropped in the merge. · guard: accessibility_label
decision/aria-labelledby-outranks-label · aria-labelledby outranks label and aria-label — An element's `aria-labelledby` targets' text, gathered cycle-guarded, is its computed label, outranking a `<label>` and an `aria-label`; an id naming no element is ignored and `aria-label` is the fallback when the targets hold no text. · why: Upstream's behaviour, ratified at the upstream sync. · guard: accessibility_label
decision/aria-state-lowercase-exact-match · Control-state values matched exactly in lower case — `aria-selected`, `aria-pressed`, `aria-checked` and `aria-invalid` values are trimmed of ASCII whitespace and compared exactly, in lower case, against a closed set; an unrecognized value sets nothing, but for `aria-invalid`, which reads invalid. · why: A choice: the specification as read states no case rule and no browser was measured; the `aria-invalid` fallback is WAI-ARIA 1.2's rule. · except: A select's option reads its chosen state as selected, never `aria-selected`.
decision/control-states-closed-values-only · Control states cross as closed values — Control states leave the process only as closed enums or booleans, on the accessibility adapter and in the snapshot's words and JSON booleans, never as the attribute's text, and no unrecognized or long value is written anywhere as typed. · why: A ratified widening of the adapter crossing by a new author-derived class.
decision/diff-counts-only-in-logs · Only diff sizes reach a log — A diff's content never reaches a log or event, while its three sizes (counts of added, removed and changed nodes) are recorded on the driver's command span, printed at `info` or below. · why: Ratified: a diff's content never reaches a log while its three sizes may.
decision/id-record-bounded-and-session-fed · Bounded id record fed by the session — The session keeps no node handle between calls, only a record of at most 4096 id texts fed from its own snapshots and never from a call's argument.
decision/no-process-handle-crosses · No process-local handle crosses with an id — Neither the stable id, the snapshot, the diff nor any driver session, answer or refusal carries a `NodeId`, `ElementId`, `ScopeId` or pointer; only the stable id text crosses.
decision/recording-dir-never-named · Recording directory is never exposed — No tool call, verb, argument or result field names the recording directory, and nothing of it is logged. · why: Ratified.
decision/recording-directory-never-named · Recording directory never named or logged — No tool call, verb, argument or result field names the recording directory, and nothing of it is logged.
decision/session-label-only-caller-string · Session label is the one echoed caller string — The session label is never text read from the document and is the one caller string a reply carries.
decision/socket-carries-screen-reading · Session socket carries what the screen reads — An answer carries what the screen reads, ids, accessible names, control values with password and file input masked, snapshot text, diff, lint findings, UI graph, never a NodeId or an attribute value as typed. · why: The crossing was ratified.
decision/stable-id-exits · Ratified exits of the stable id — The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files. · why: Each carrier ratified as a boundary widening.
contract/cli-exit-codes/0 · CLI exit codes · 0 — Accepted: the answer as one line of JSON on stdout.
contract/cli-flags/id · escher-session flags · --id — Followed by one value, whatever that word is.
contract/cli-flags/key · escher-session flags · --key — Followed by one value, whatever that word is.
contract/cli-flags/ms · escher-session flags · --ms — Followed by one value, whatever that word is.
contract/cli-flags/part · escher-session flags · --part — Followed by one value, whatever that word is.
contract/cli-flags/record · escher-session flags · --record — Followed by a directory, taken at most once and only on start, the host role and the mcp role; anywhere else, twice or with no value is usage.
contract/cli-flags/session · escher-session flags · --session — Followed by a directory, given exactly once, else usage.
contract/cli-flags/shift · escher-session flags · --shift — Bare, taking no value.
contract/cli-flags/text · escher-session flags · --text — Followed by one value, whatever that word is.
contract/driver-argument-kinds/flag · Driver argument kinds · flag — True or false.
contract/driver-argument-kinds/id · Driver argument kinds · id — Text of 1 to 1024 bytes.
contract/driver-argument-kinds/key · Driver argument kinds · key — One name of a closed twelve-name list.
contract/driver-argument-kinds/milliseconds · Driver argument kinds · milliseconds — A whole number from 1 to 60000.
contract/driver-argument-kinds/name · Driver argument kinds · name — Text of 1 to 32 bytes of a-z, 0-9 and -.
contract/driver-argument-kinds/part · Driver argument kinds · part — A whole number from 1 to 1000.
contract/driver-argument-kinds/text · Driver argument kinds · text — Text of 0 to 4096 bytes.
contract/driver-command-span/added · The driver command span · added — The length of the returned diff's added list.
contract/driver-command-span/busy · The driver command span · busy — The schema's class word, unrecorded until it applies.
contract/driver-command-span/changed · The driver command span · changed — The length of the returned diff's changed list.
contract/driver-command-span/passes · The driver command span · passes — A count, unrecorded until it applies.
contract/driver-command-span/removed · The driver command span · removed — The length of the returned diff's removed list.
contract/driver-command-span/settled · The driver command span · settled — A boolean, unrecorded until it applies.
contract/driver-command-span/verb · The driver command span · verb — The verb table's own word, never the caller's verb text.
contract/driver-session-reply/no-attribute-text · driver session socket replies · attribute text — A reply never carries an attribute's text.
contract/driver-session-reply/state-readings · driver session socket replies · state readings — State readings are carried as closed words and booleans.
contract/mcp-tool-call-span/tool · The MCP tool_call span · tool — The verb table's word for the called tool; the span parents the call's command span.
contract/session-socket-reply/ok-accepted · Session socket reply · ok v3 accepted — The call ran and was accepted; its answer JSON follows on the line.
contract/stable-element-id/carriers · The stable element id · carriers — The stable element id leaves by the platform adapter, the driver's answers, MCP results and kept checks, and is written by escher's code to no log.
contract/stable-element-id/keyed-list-segment · Stable element id · keyed list row segment — A keyed list row's segment carries its Dioxus key as `{tag}[{key}]`; a Dioxus key that is empty or holds `/` is refused and the row reads the positional `{tag}:{n}` segment instead.
contract/stable-element-id/no-collision · Stable element id · uniqueness — An author key holds no `/`, an anchored path is the only id holding `//`, and a component or document path has no empty segment, so no `id` attribute can make two ids equal, a key spelled like a component name included.
contract/stable-element-id/no-node-id · The stable element id · NodeId — No `NodeId` crosses any carrier of the stable element id.
contract/stable-element-id/virtual-node-form · Stable element id · virtual-node form — The id grammar reserves `{owner}///{key}` for a virtual node, spelled for a non-empty key with no `/`; no served id holds `///`, and the anchored path stays the only served id holding `//`.

## data-protection · ## Data Protection
decision/committed-counter-checks · Committed stand-Counter checks — The only committed kept checks are the two of the repository's scripted stand-Counter sessions, holding no typed text, host path, name or secret and no `screenshot` step, read only through the strict `Check::from_bytes`. · why: The one exception to a kept check's owner-only medium.
decision/kept-check-call-content · Kept check call-content class — A kept check holds call content: target ids, press keys, advance ms and text typed into unmasked controls, a masked or unresolved type's text kept as MASKED_STAND_IN. · why: Ratified.
contract/kept-check-file/contents · Kept check files · contents — A kept check holds per call its verb, arguments, acceptance, digest and masked answer, never a pixel.
contract/kept-check-file/entry · Kept check file · entry — A check is a JSON array: the app label, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer and the answer itself as answer.

## api-security · ## API Security
decision/mcp-server-over-stdio · MCP server over process stdio — The MCP server role serves the driver's verbs as tools over its own stdin and stdout only, binding no listener, port or socket, and answers only the process that spawned it, with no handshake, token, secret or peer check. · why: The crossing was ratified as is.
decision/no-wait-verb-fields · No wait argument or result on verbs — No driver verb has an argument or a result field of a wait.
decision/session-socket-sole-listener · Session socket as the one listener — The driver session's Unix-domain socket is the project's one listener, serving the session lifecycle and driver calls and their answers.
decision/time-moves-only-on-advance · Time moves only on advance — Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph and actions wait on no load.
contract/mcp-server/help · MCP server · help — The help tool is answered by the server itself from the schema's fixed tables, running nothing on a session.
contract/mcp-server/tool-result · MCP server · tool result — A tool result is bounded at 1,048,576 bytes, one holding an image measured by its whole written form and a text-only one by its text.
contract/session-socket/request-line · Driver session socket · request line — A request line is at most 16,384 bytes.
contract/session-wire/recording-failed · Driver session wire · ok v3 recording-failed — Wire v3 replies include the fixed reply ok v3 recording-failed.
decision/session-socket-one-bounded-connection · One bounded connection at a time — The local session socket serves one connection at a time under fixed read, write and size bounds.
contract/driver-answer/settled · Driver command answer · settled — A step that ran and did not go quiet is a result, settled false with its busy class, not a refusal; the outcome names a class only, never a URL, request, node, id, name or value.
contract/driver-command-line/answer-bounds · Driver command line · answer bounds — A started host has 30 s to answer and each read of a call's answer is bounded at 30 s; these bound a transport and wait on no load.
contract/mcp-server/request-line · MCP server · request line — A request line is bounded at 65,536 bytes.
contract/session-socket/connections · Driver session socket · connections — One connection is served at a time, with a 2 s read and write bound.
contract/session-socket/lifecycle-reply · Driver session socket · lifecycle reply — A lifecycle reply is at most 128 bytes on its JSON line.

## dependency-security · ## Dependency Security
decision/cargo-deny-audit-tool · cargo-deny as the audit tool — cargo-deny 0.20.2, with the root deny.toml, is the dependency audit tool.
decision/dependency-audit-cargo-deny · Dependency audit with cargo-deny — cargo-deny checks advisories and licenses, locked, over the whole workspace's resolved graph with all features for the six platforms CI builds, configured by the root `deny.toml`, ignores per ID only, each with a written reason, re-read on every push. · why: Ratified: audit the whole workspace and check licences. · except: `unsound` stays at its default, which keeps memmap2's unsound advisory out of the gate.
decision/licence-gate · Licence gate on every push — The audit leg runs upstream's licences table, held byte for byte in the root deny.toml, on every push over the whole workspace's graph, so the fork gates on licences. · why: An open question, decided as the fork's chosen option. · guard: the audit leg · except: upstream's own licenses job stays reverted
decision/locked-builds · Locked builds — Every build, the flake, the bundle and every CI cargo leg, passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock
decision/no-git-dependencies · No git dependencies in the root manifest — The root manifest holds no git dependency; taffy and parley are crates.io registry versions with checksums in the lockfile.
decision/no-git-dependency · No git dependency — The workspace takes no git dependency.
decision/serde-json-read-only · serde_json, read only — serde_json, with no features and no preserve_order, reads the MCP server's untrusted stdin, a kept check's bytes and the driver's own answer line for replay, and writes nothing. · why: Chosen as read-only use.
decision/serde-json-untrusted-input-reader · serde_json as the untrusted input reader — serde_json, a workspace dependency with no preserve_order feature, is the reader of the MCP server's untrusted stdin, of a kept check's bytes and of the driver's answer line the replay compares.
decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
decision/workspace-inherited-dependency-versions · Workspace-inherited dependency versions — Dependency versions are inherited from the workspace with workspace = true rather than declared inline in each manifest.
decision/advisory-ignores-by-id · Advisory ignores by ID only — An advisory is ignored only by its ID, each with a written reason, with no blanket allow and unmaintained and unsound at their defaults.
decision/audit-ignores-per-id-with-reason · Audit ignores per ID with a written reason — Audit ignores are per advisory ID only, each with a written reason, no blanket allow, and each ignore is re-read on every push.
decision/dependency-audit-ci-gate · Dependency audit CI gate — The CI audit job runs the dependency audit on every pull request and push, in the slow tier behind the fast jobs, uncached, its log uploaded on failure.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project)
decision/telemetry-sink-target-allowlist · Telemetry sink target allowlist — escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level. · except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed

## secret-management · ## Secret Management
decision/driver-reads-only-rust-log · Driver reads only RUST_LOG — The escher-driver library and the escher-session binary read no environment variable beyond RUST_LOG.
contract/environment-variables/rust-log · Environment variables · RUST_LOG — Holds the tracing filter directives escher-telemetry reads, defaulting to warn when unset or unparsable.

## error-handling · ## Error Handling
decision/closed-refusal-cause-set · Closed refusal cause set — A driver command refusal is a typed value whose cause comes from a closed set of eight, with a fault only beside `malformed`; a new verb or argument shape is refused within the set, never by adding a cause. · why: The closed set of eight was ratified.
decision/mcp-errors-fixed-no-data · MCP protocol errors fixed, no data — MCP protocol errors are fixed codes and fixed messages and carry no data member.
decision/refusal-closed-cause-set · Command refusal as a closed set of causes — An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field. · why: So a refusal cannot hold anything a call supplied. · except: A Fault is carried only beside the malformed cause.
decision/refusal-holds-no-call-input · A refusal holds nothing a call supplied — Every refusal's name, meaning and remedy is a fixed static text holding no path, and a fault carries at most the schema's own argument name, so a refusal cannot hold anything a call supplied.
decision/refusal-log-cause-name-only · Only a refusal's cause name is logged — Exactly one thing of a refusal reaches a log: its cause's fixed name, as the `cause` field of the call's driver span; no fault, meaning, remedy or display text is logged.
decision/refusal-logged-by-cause-name · Refusal logged by cause name only — Of an escher-driver command refusal, only its cause's fixed name reaches a log.
contract/cli-exit-codes/1 · escher-session CLI exit codes · 1 — The call was refused; the refusal JSON is written to stdout.
contract/cli-exit-codes/2 · escher-session CLI exit codes · 2 — Usage: the argv shape is not one the command takes; one fixed usage line on stderr, stdout empty, nothing booted and no state created.
contract/mcp-protocol-errors/invalid-params · MCP protocol errors · invalid params — A fixed code and fixed message with no `data`; only the admitted request id is written back.
contract/mcp-tool-result/is-error · MCP tool result · isError — A refusal or session error is a tool result `{content:[{type:"text",text}], isError: true}`, its text the same JSON as on the command line, a refused `screenshot` included.
contract/refusal-causes/malformed · Driver refusal causes · malformed — The one cause that carries a fault; a call breaking the schema, such as a `graph` or `actions` call with an argument, is refused with it and nothing runs.
contract/refusal-json/fault · Refusal JSON · fault — Present only on a `malformed` refusal, naming the fault.
decision/coverage-refuses-bad-checks-malformed · Coverage refuses an unusable check as malformed — The `coverage` role refuses a check file it cannot read, one past the check bound, one that is no check or a check of another app as `malformed`, holding nothing of the path or the file, and adds no cause.
decision/session-error-fixed-table · Session errors from a fixed table — An escher-driver session error is written from a fixed table, holding nothing a call supplied.
decision/session-errors-fixed-table · Session errors as one fixed table — Each session error's kind, meaning and remedy is one row of a fixed table served by `help`, every text a fixed string holding no path, carried on stdout and in MCP results as the program's own text. · why: Ratified as the program's own fixed text inside the ratified carriers.
contract/refusal-json/cause · Refusal JSON · cause — The refusal's cause name, inside `{"refused":{...}}`, built from the cause's fixed strings.
contract/refusal-json/meaning · Refusal JSON · meaning — The cause's fixed meaning text.
contract/refusal-json/remedy · Refusal JSON · remedy — The cause's fixed remedy text.

## logging-monitoring · ## Logging & Monitoring
decision/session-output-holds-no-caller-input · Session output holds no caller input — No escher-session output line holds anything the caller typed. · except: The mcp role's echoed request id.
contract/command-span/cause · The command span line · cause — Recorded when the call is refused, as a closed cause word such as fonts-unavailable.
contract/command-span/verb · The command span line · verb — Records the instance-level verb; a graph or actions call records the verb alone, plus cause when refused.
contract/escher-session-output/caller-text · escher-session output · caller-typed text — No output line echoes a caller's argument but the admitted request id the mcp role echoes; an answer carries each node's current value, so text typed into an unmasked control returns, a masked input reading only the mask.
contract/tool-call-span/ending · The tool_call span line · ending — One of the closed words accepted, refused or error.
contract/tool-call-span/line · The tool_call span line · line — One closed line per MCP tools/call, the parent of that call's command span, holding tool and ending only.
decision/closed-span-lines · One sink line per closed span — The sink prints one line per closed span whose target the allowlist admits and nothing on creation, record, enter or exit; each span field is judged by the event rule, an outside-target span writes no byte, and a content-named span field's value is never stored. · why: A boundary widening of the sink, ratified as built.
decision/escher-target-content-fields-withheld · escher_ targets withhold content-named fields — In escher's telemetry sink an escher_ target keeps all fields but eleven content-named ones.
decision/escher-target-content-scrub · Escher targets scrub content-named fields — An event whose target starts with escher_ prints every field except url, href, src, html, text, value, attrs, path, request, error and panic.payload, which print as [redacted].
decision/no-screen-content-in-logs · Screen content stays out of logs — No driver call puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer and a coverage check reach no log line at any level. · guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds
decision/stderr-only-telemetry-sink · Escher's own stderr telemetry sink — Both seven_guis binaries, the native app and the escher-session driver host, install escher's own telemetry sink first thing: a stderr-only subscriber whose formatter applies an allowlist of targets. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed, and the chained std panic hook still prints the raw panic message to stderr.
contract/command-span/line · The command span line · line — A sink-installing host writes one closed command span line per call it runs.
contract/escher-session-mcp-stdout/protocol-lines · The escher-session mcp role's stdout · protocol lines — The mcp role writes JSON-RPC protocol lines only on stdout, and nothing at its end.
contract/escher-session-stderr/usage-messages · escher-session stderr · usage messages — escher-session writes usage messages on stderr.
contract/escher-session-stdout/mcp-role · escher-session stdout · mcp role — The mcp role writes JSON-RPC lines only on stdout.
contract/tool-call-span/tool · The tool_call span line · tool — The tool table's word for the call, a closed word outside the content-named fields; absent for an unknown tool.

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
