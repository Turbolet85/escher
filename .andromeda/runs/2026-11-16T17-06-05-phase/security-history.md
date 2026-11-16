@history_item n=1 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Threat Model Summary: 2 rows) — added `+ decision/caused-answer-crossing`, which keeps caused to closed kind and level words, the first handler's id, owner and event and a count, "never a message, payload, target or field value", and says "the text is left to a follow-up". It also added `+ decision/root-boundary-err-as-screen-content`: a root-boundary Err crosses only as Dioxus's screen rendering, and caused holds no byte of its text. This chunk is that follow-up. It reverses both rows' text clauses, so both rows must be re-amended, and the widening needs the founder's ratification. The operator ratified the first crossing as written, «Применить как написано».

@history_item n=2 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Input Validation: 2 rows) — gave `~ decision/driver-installs-no-subscriber` and `~ decision/driver-no-subscriber-no-env` the exception that an acting step runs under escher-telemetry's scoped log capture, "which prints nothing". This chunk widens what the capture keeps: messages, targets, levels, log.file/log.line, fields, and engine and escher targets it does not admit today. The capture must still print nothing and install no subscriber, so the exception's wording may need restating.

@history_item n=3 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (API Security: 1 row) — scoped `~ contract/driver-answer/settled` so that only the busy reading "names a class only, never a URL, request, node, id, name or value", because caused already held handler ids. Message text in caused (for example a failed load's URL) must not leak back into the busy reading. The promise stays scoped to that reading.

@history_item n=4 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help` and the program's fixed text) — ratified the rule that the program's own fixed `'static` table text inside a ratified carrier is no widening. Text from a page, a call or a screen, or a new carrier or exit, still widens; this is now a playbook rule. Behind `decision/mcp-listing-fixed-text`. The scope's `describe()` additions (the new caused fields, the cut and count meanings) are fixed text under this rule. The error text itself is authored or engine text, so it is a widening owed to the founder.

@history_item n=5 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the closed-field rule) — set the rule behind `decision/boundary-widening-rule` and `decision/closed-value-field-no-widening`. A closed-value field in a ratified carrier is no widening, but "a field holding authored text, an open value set, a new carrier or a new exit still does". Message, payload, cause chain, target and field values are authored or open-set text, so every new caused text field is a widening. The scope's Boundaries already say so.

@history_item n=6 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — wired escher's sink scrub, which this chunk reuses. Engine targets print only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`. Eleven names are redacted at any target: `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error`, `panic.payload`. The sink's reach was named as "the std panic hook's raw message and `log.file` host paths stay as-is". Traps for this chunk:
- `panic.payload` and `error` are content-named fields that the sink redacts, yet the chunk's purpose is to carry a panic payload and an error message.
- `log.file` is an allowlisted field that can carry a host path. `decision/engine-target-field-allowlist` excepts it for that reason.

@history_item n=7 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — made the sink's three outcomes: engine targets keep the safe fields, `escher_` targets lose the eleven content-named fields, and any other target's record is dropped whole at every level, WARN and ERROR included. It also recorded that a host path can ride `log.file` only on a bridged engine-target record. These are `decision/outside-target-dropped`, `decision/other-log-targets-dropped`, `decision/log-sink-redacts-and-allowlists` and `decision/telemetry-sink-target-allowlist`.
- The scope says "the app, the UI framework and the engine". Records from dioxus_core, Stylo and the app's own crate fall outside the allowlist.
- The scope's "scrub by field name that already stands on the log sink" has no rule for an outside target. The sink drops such a record whole, so the chunk must decide that target's field rule rather than inherit one.

@history_item n=8 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (what a sink-installing host logs) — measured accessible names printing at `trace` through `dioxus_core::diff::node` and author-key ids through Stylo's records, before the allowlist dropped them. Behind `decision/accessible-names-not-logged` and `decision/stable-id-not-logged`. If caused admits framework and engine records, those records carry ids and names into the answer. That is a crossing of ids and names in a new field on the ratified carriers, and the mask must hold on it too. Trap kept: a "nothing in the logs" check over a host with no sink passes vacuously.

@history_item n=9 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (where a failed load's error text stops; a failed load reaches the handler) — measured a failed load's error text stopping in `ResourceLoadResponse` and the `Resource load failed` line, with "no carrier takes it". The sink prints that line's message `[redacted]`, and `url` and `error` are in its scrub set. `NetHandler::error` holds no tracing site. Under this chunk, an engine record of a failed load (its URL and error text) would first cross onto a carrier through caused. That reverses the "no carrier" row and is the clearest instance of URL-bearing engine text the scrub and cut must handle.

@history_item n=10 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — set that a settle's outcome names a busy class only, never a URL, request, node, id, name or value. This is the root of `contract/driver-answer/settled`. Engine load records that caused carries must not get around that by putting a URL in the same answer. Any URL in caused text crosses as the new scrubbed text and must be stated so.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (one span, eight fixed fields; the sink's second record class) — fixed the `command` span's fields as fixed words, booleans and counts. No argument, Refusal, Fault or screen text is fielded, and only a refusal's cause name reaches a log. It also made the sink print closed spans, where every span field is judged by the field rule. Behind `decision/one-command-span-per-call`, `decision/command-span-content-free`, `decision/refusal-log-cause-name-only` and `decision/refusal-logged-by-cause-name`. The scope keeps the `command` and `tool_call` spans' fields fixed. caused's text must reach no span field or log line.

@history_item n=12 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the span record class and the standing rule: "a count of a diff's nodes may reach a log, no node, id, name or value of it may — a field that holds any of those is a new crossing question". caused's cut-by count and its records-past-the-count by source are counts and could reach a log under this rule. Any message text in a span field would be a new crossing question.

@history_item n=13 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the socket carries calls and answers; the command line as an external surface; ids, names and values leave in an answer) — ratified the socket's crossing and its bounds: a 16,384-byte request, a 1,048,576-byte answer, `oversize`, and a 30 s answer read. It also set that no command-line ending writes back anything the caller typed, and that a Refusal and a SessionError are written from fixed strings. Behind `decision/session-socket-carries-calls`, `decision/session-socket-bounds`, `decision/no-echo-of-caller-input`, `decision/cli-echoes-nothing-typed` and `contract/session-socket-reply/ok-oversize`.
- Error text can hold what the caller typed, for example an app panic or log message that quotes the typed value. The scrub has to keep this to the ratified "current value" class, not an echo of input.
- The answer bound must hold with caused's text added.

@history_item n=14 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP crossing ratified; serde_json read-only and the `tool_call` span) — ratified the MCP result crossing: results are bounded at 1,048,576 bytes, protocol errors are fixed with no data, and the `tool_call` span holds `tool` and `ending` only. `serde_json` reads only and writes nothing. Behind `decision/mcp-result-crossing`, `decision/mcp-bounded-lines`, `decision/mcp-errors-fixed-no-data`, `decision/serde-json-read-only`, `decision/serde-json-untrusted-input-reader` and `contract/mcp-tool-result/size-bound`.
- caused text in an MCP result goes to the client and onward, a model provider included, and the result bound must hold over it.
- caused's JSON must stay hand-written, not written by serde_json.

@history_item n=15 marker=2026-10-12-self-description
**2026-10-12-self-description** (a session error's remedy and its table) — made each session error one row of a fixed table, holding no path. Behind `decision/session-errors-fixed-table` and `decision/session-error-fixed-table`. The scope adds no session error. caused's app-, engine- and panic-sourced text is not table text and must stay out of `SessionError` and `Refusal` texts, which keep holding nothing a call supplied.

@history_item n=16 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — set Refusal as a closed set of causes with fixed texts and no String field. The standing rule is to keep every refusal text fixed and free of call content. Behind `decision/closed-refusal-cause-set`, `decision/refusal-closed-cause-set` and `decision/no-panic-on-call-input`. The scope adds no refusal cause. The error text rides an accepted answer's caused, never a refusal.

@history_item n=17 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — closed the cause set at eight (`time-unavailable` retired). Every later chunk keeps that count, including the coverage role, the `actions` arguments and `graph`. This chunk's "no new refusal cause" boundary keeps it at eight.

@history_item n=18 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — extended the mask to a file input. The DOM still holds the chosen file's host path in the `value` attribute, and form submission still carries it. Behind `decision/masked-input-values`, `decision/password-file-value-masked` and `decision/mask-holds-on-every-carrier`. The scope says the mask is unmoved. An engine or framework record that prints an attribute value (the `value` of a password or file input) would cross unmasked in caused unless the scrub withholds it. The `value` and `attrs` field names cover fields, not a message's text.

@history_item n=19 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the render-path recorder) — recorded that the buffered render's replay re-fires `MutationWriter`'s own `trace!` calls, and that the recorder holds attribute values and text in process only. With engine and framework records admitted to the capture, those trace records carry DOM content: attribute values, text, possibly a masked control's value. An acting step's render can put them in caused. This is a concrete source for the scrub and mask check.

@history_item n=20 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the kept check at rest and `--record`; wire v3) — set the recording bounds: 400 records or 1,048,576 bytes, or a failed write, ends the recording with `recording-failed`. Kept checks are owner-only (0600 in 0700) and never logged, and a recorded masked `type` is kept as `MASKED_STAND_IN`. Behind `decision/recording-bounds-end-recording`, `decision/recording-ends-on-failure`, `decision/kept-check-owner-only`, `decision/recorded-type-masked`, `contract/session-wire/recording-failed` and `contract/kept-check-file/medium`. caused text grows each kept answer, so a recording ends sooner at the byte bound, and the error text is now at rest in kept checks.

@history_item n=21 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — kept each answer whole at rest and compared by an FNV-1a digest on replay. It committed two stand-Counter checks publicly, holding no typed text, host path, name or secret. Behind `decision/kept-check-answer-whole`, `decision/committed-counter-checks`, `decision/replay-through-session-run`, `contract/kept-check-file/entry` and `contract/recorded-check/digest`.
- Error text holding a host path (a panic's file location, `log.file`) or anything run-dependent would put a host path into a committed check.
- It would also make a replay's digest differ across machines. The scope's "kept check (recorded, replayed, its answer kept whole)" must answer both.

@history_item n=22 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (Data Protection: 3 rows; Input Validation: 1 row) — added `+ decision/exploration-report-closed-content`: an exploration's report holds closed content only, "never a panic payload, NodeId, screen text or screen digest". It also changed `~ contract/kept-check-file/medium` (each explored path its own check) and `~ contract/recorded-check/start-record` (the strict `format` key). Each explored path is a kept check, so panic payloads in caused would reach those checks, while the report must still hold none. A format change for the new caused text would go through the strict `format` key.

@history_item n=23 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer on the socket and in kept checks) — a recent precedent for a new answer content class ratified onto every carrier, kept checks at rest included. It is `decision/kept-check-actions-answer`, never a control value, typed text or NodeId. The kept-step replay test was owed until handler-coverage. Planning should follow the same per-carrier walk, with the at-rest and replay witnesses planned up front.

@history_item n=24 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role, a kept check named on the command line), ratified at 2026-11-15T05-21-42-wrap — made the coverage role a second reader of caller-named kept checks: bounded, strict, refused `malformed` holding nothing of the path or file. It also defined the coverage line's closed content. Behind `decision/coverage-check-files-untrusted`, `decision/recorded-check-strict-reader`, `decision/recorded-check-untrusted` and `contract/cli-roles/coverage`. Kept checks carrying caused text must still pass `Check::from_bytes` within its byte bound. The coverage line must not pick up any of the error text.

@history_item n=25 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — set the committed statement `docs/capabilities.md` as stating the MCP crossing's carried set, with each carried-set site reconciled to the ratified crossings. Its secret search covers that file and its pin. Behind `decision/capability-statement-mcp-crossing`. Error text in caused grows the carried set, so the statement's carried-onward list and every carried-set site in the threat model and input validation rows must follow.

@history_item n=26 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's size under the answer bound) — the precedent for an unpaged answer's size. Cells' graph reads 293,869 bytes under the 1,048,576-byte bound, a stated limit owned by residual `res-7cc5bi4k`, with no part argument added because one would widen the crossing. The scope's fixed per-message cut and per-answer record count serve the same bound. They must be set from measured lengths so the MCP and socket bounds hold with no new argument.

@history_item n=27 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — made `host_log` the standing check of a sink-installing host's stderr at `RUST_LOG=trace`, with 0 id, name, sentinel and recording-path needles under both builds. `mcp_log` was extended to each new verb's answer since then. Guard behind `decision/no-screen-content-in-logs` and `decision/accessible-names-not-logged`. The scope says no message, payload or field value reaches a log line, so the same needle check should drive a failing step and read caused's text 0 times in the logs.

@history_item n=28 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (Logging & Monitoring: 1 row) — changed `~ decision/no-screen-content-in-logs` to "no driver call or in-process read puts screen content in a log", the list gaining a layout answer. This chunk's error text is answer content, so the row's list should gain it in the same way.

@history_item n=29 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the merged ARIA reads) — set descriptions' one exit as the platform adapter. No driver or MCP carrier, verb, field, log field or span carries one (`decision/descriptions-adapter-only`). An engine or framework record that prints description text and crosses in caused would break that row. The scrub has to cover it, or the row has to be re-read.

@history_item n=30 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the control states row) — set that control states cross only as closed enums or booleans, and that "no unrecognized or long value is written anywhere as typed" (`decision/control-states-closed-values-only`). A record that prints an attribute's raw value would write it as typed in caused, so the error text must not carry it out.

@history_item n=31 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Error Handling: 1 row) — added `+ contract/blitz-net-provider-error/file-forbidden`, a blitz-net error whose text is promised fixed and path-free. This is a precedent that provider error text holds no path. Error and cause-chain text that crosses in caused should keep that promise rather than add paths.

@history_item n=32 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (Input Validation: 1 row; Error Handling: 1 row) — added `+ decision/fixture-name-closed-table` and `+ decision/reach-error-closed-set`, with closed, fixed error texts so that a refused name or step echoes nothing supplied. caused text must not reach these errors' texts. The pattern holds for every driver error that this chunk leaves unchanged.
