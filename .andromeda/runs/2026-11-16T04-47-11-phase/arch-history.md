@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — registered escher-telemetry: `init` / `init_with_writer`, the stderr line, the allowlist scrub (`CONTENT_FIELDS` redacted), the panic event at `escher_telemetry::panic` with a redacted `panic.payload` and a chaining panic hook; matters because this chunk must capture a step's records and a handler panic without changing what the sink prints (scope «Scrub held»), and the panic hook already redacts the payload this chunk would carry into an answer.

@history_item n=2 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink admits only `ENGINE_TARGET_PREFIXES` and `ESCHER_TARGET_PREFIXES` (`escher_`), an engine target printing `SAFE_FIELDS` only, an escher target every field but `CONTENT_FIELDS`, any other target dropped whole at every level in the formatter; standing rule that a new escher log target must start with `escher_`. A per-step capture layer must not move this drop/scrub (rows decision/telemetry-target-prefix-drop, contract/telemetry-scrub/*), and an app's handler log under its own target is dropped by the sink today, a fact the capture design must reckon with.

@history_item n=3 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span; `tracing` its third dependency) — every call through `Session::run` opens one hand-built `command` span (target `escher_driver`, INFO, eight fields, never an argument, `Outcome`, `Refusal`, screen text); the crate installs no subscriber and reads no env var; `stand_act_spans` re-executes children installing the sink over an in-memory capture. Matters: the scope holds that `command` / `tool_call` gain no field carrying the new answer content (decision/driver-spans-only, contract/command-span/never-fielded, decision/driver-library-side-effect-free), and the re-exec + in-memory capture is the precedent for proving a capture without a global subscriber in the driver.

@history_item n=4 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the sink prints a closed span; the diff's lengths reach a log — both PROVISIONAL) — added the sink's closed-span line and the diff's three counts on the command span; these are the founder's «log rulings of 2026-10-07/09» the scope says bound what of a log may enter an answer (decision/telemetry-closed-span-lines, decision/diff-content-never-logged).

@history_item n=5 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the closed-span line and the diff counts in a log, with the standing rule «a count of a diff's nodes may reach a log, nothing of its content may», and the ruling that an in-process call returning screen content is the returned value his rule allows. This is the 2026-10-09 log ruling the chunk's boundary fork is bounded by: the reverse direction (log content into an answer) is the new question for the founder.

@history_item n=6 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — the snapshot text left the process through the returned value only, with the standing rule that each later surface asks the crossing question again when content first leaves the process; precedent for treating authored log/panic text entering an answer as a crossing to put to the founder (decision/snapshot-text-never-logged, decision/no-snapshot-in-logs).

@history_item n=7 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — `Harness::settle`, `Session::act` and `NotSettled(Busy)`: a not-settled step is a result, not rolled back, its outcome naming a class only; settle logs nothing. Matters: a caught handler panic or captured error is likewise a result field on the acting step, not a refusal or session error (contract/acting-verb-results/settled), and the settle path must stay silent.

@history_item n=8 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — one `VERBS` table every surface reads, the acting result fields `settled` · `busy` · `added` · `removed` · `changed`, causes closed at eight, rules that a later entry adds to the table never a second one and that `Command`/`Call`/`ArgValue` print ids under `Debug` so nothing fields them. The new answer field joins this table's result fields (contract/driver-session/command-schema, decision/refusal-causes-closed-at-eight).

@history_item n=9 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` the one entry, `validate` first, `Outcome` per verb, rules: nothing public runs an unvalidated `Command`, nothing resolved outlives a call, nothing fields an `Outcome`. The panic catch belongs on this run path (decision/validated-commands-only), and the new field rides `Outcome` under these rules.

@history_item n=10 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (socket carries a call and its answer; Driver session contract wire v2 / written forms; nine verbs) — the socket's crossing ratified as a boundary widening with «the next surface that carries them asks again», `Outcome::to_json` / `Refusal::to_json` / `SessionError::to_json` hand-written, `SessionError` with `AnswerTooLarge` (the call ran, oversize), the command line's four endings; «the crate prints nothing» narrowed to its answer line. Matters: the new field must ride these hand-written forms and bounds (decision/hand-written-answers, contract/session-wire/*, contract/session-errors/answer-too-large) and no new session error or exit is admitted.

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the MCP role and its crossing ratified; `serde_json` a reader only; the `tool_call` span with `tool` and `ending` only, standing rule «no span field holds what a call supplied or a screen reads». The MCP result and the tool_call span must carry the new field (or not) under these rules (decision/serde-json-reader-only, contract/tool-call-span/*, contract/mcp-server/tool-result).

@history_item n=12 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` / `describe()` written from the tables alone, `FieldKind` and `NODE_FIELDS` with help, the MCP listing's descriptions carrying result fields; the program's fixed `'static` text ratified on the carriers, while «text from a page, a call or a screen … still widens» and `differs` crosses as a closed set under the rule of 2026-10-10. This is the rule the scope cites (closed value set not a widening; authored text is) and the place the new field's help and listing text must come from (decision/fixed-program-text-on-carriers, decision/measured-limits-stated-in-the-tool).

@history_item n=13 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — precedent of adding answer content across every carrier: a new `FieldKind`, an `Outcome` variant, the MCP image block, the result bound by whole written form, the `command` span keeping its eight fields and never fielding the image (decision/screenshot-in-band). Same pattern for an errors field.

@history_item n=14 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Driver session and CLIs; ratified by 2026-10-18-engine-debts) — `advance` runs the app's own time step inside one settled step on every session and `Cause` closed at eight; `advance` is one of the five acting verbs whose step can run app code (a handler or task panic), and no cause is admitted for it.

@history_item n=15 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the kept check; wire v3, fifteen session errors) — a kept check keeps each answered call owner-only at rest, never logged, with a 64-bit digest; `RecordingFailed`, `ok v3 recording-failed`, `SESSION_ERRORS: [ErrorSpec; 15]`. Matters: the new field rides the kept check and the digest (the P4 design point whether it is held out of the digest), and no session error may be added (decision/recorded-session-checks, contract/kept-check/*).

@history_item n=16 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer held whole as the driver wrote it, `Check::replay` stopping at the first step whose acceptance or answer differs, a `Change` naming differing result fields in table order; same-host limit for byte-identity. An errors field joining the answer changes replay equality and the committed Counter checks (decision/kept-check-replays-through-run, contract/check-replay/*, decision/committed-stand-checks).

@history_item n=17 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the acting step settles through `settle_waiting` on every carrier, with no print or `tracing` call; `snapshot` and `screenshot` wait on nothing. The capture window of «records emitted during the step» is this settled step, and it must stay log-free (contract/load-wait/*).

@history_item n=18 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `lint` the twelfth verb) — precedent for a new answer content on every carrier and recorded: closed defect words and numbers, an accepted call recording `verb` alone, findings never fielded, no new carrier, exit, cause or span field (contract/lint-verb/wire-form).

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the MountedData hazard of a spawned task) — measured that a task spawned from a key handler panicked (`RefCell already borrowed`) inside `render_immediate`; the first recorded handler-path panic, the kind of panic this chunk must catch on the run path.

@history_item n=20 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (hazard re-measured; component through public API; listener attributes over-report) — a node's owning component reads through Dioxus 0.7.10 public API; hook-local state unreachable; the bridge's listener mark over-reports, the vdom walk reading current listeners exactly. Matters for tying an error to the handler: read listeners from the vdom walk, never the mark (decision/owning-component-via-public-api, decision/listeners-read-from-vdom, decision/listener-mark-outlives-listener).

@history_item n=21 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (buffered render as contract; hazard closed; first-poll paths) — the bridge renders into `MutationBuffer` unborrowed then replays; a task spawned during render is first polled by the next `poll` or inside `render_immediate`'s `Work::PollTask`; its limit on nodes created in the same render. Matters: where a handler's or spawned task's panic/log fires relative to the step's dispatch decides attribution (decision/bridge-buffered-render, guard `dioxus_runtime_probe`).

@history_item n=22 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph crosses the ratified carriers; the graph reader; the registry and the bridge's dispatch; async handler writes; log checks) — the graph's node/handler identity (`GraphNode`, `GraphHandler`, owner ids, event names) crosses as names, closed words and booleans, never a value or `NodeId`; `handle_event` sets the registry's dispatch around `runtime.handle_event`; an async handler's spawned task runs inside its dispatch; `host_log` / `mcp_log` read 0 needles at `trace`. This chunk ties each error «through the UI graph to the handler», may reuse the dispatch bracket for attribution, and must keep the log checks at 0 needles (decision/ui-graph-crossing, contract/ui-graph/*, decision/signal-write-recorded-against-dispatch).

@history_item n=23 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — `actions` the tenth instance verb, ratified widening of closed words and booleans; the shared `check_target`; an `actions` call's span recording `verb` alone and the log checks' needles at 0. Latest precedent for a field crossing every carrier with no span field (decision/actions-answer-crossing, decision/validated-verb-set-widened-by-ratification).

@history_item n=24 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached record on the bridge's dispatch; `handler_coverage`; the coverage role) — the ungated reached record writes each chain node's stable id whose current listeners hold the event before the handlers run, so a handler removing its element still counts; bubbling counts every ancestor listener, `stopPropagation` unread (an open residual). The nearest existing mechanism for tying an error to node id and handler, with its bubbling limit (decision/reached-record-before-dispatch, decision/reached-recorded-before-handlers, contract/reached-record/*).

@history_item n=25 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the coverage role's kept-check files on argv and the coverage role itself (was PROVISIONAL delegate); the coverage line crossing is ratified closed content, a carrier this chunk must leave unwidened (decision/coverage-role-reads-named-checks, contract/coverage-role/answer).

@history_item n=26 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — `explore` catches a handler panic per path (`PathEnding` panicked), exploration absent on every carrier, the kept check's opening record gaining `format` / `CHECK_FORMAT`. Matters: the run-path catch must not widen `explore`'s closed path ending (security-plan exploration-report-closed-content), and adding a field to kept answers may touch the check format number (contract/driver-explore/path-ending).

@history_item n=27 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — `Session::reach` runs fixture steps through the one run path, stopping at the first refused or unsettled step; a recorded reach keeps ordinary calls; fixture states absent on carriers. The scope requires `reach` to keep its contract when the run path gains a panic catch and an errors field (contract/driver-session/reach).

@history_item n=28 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `decision/driver-spans-only` reworded to one span per call its run path runs, an in-process read opening none; `Session::layout` and `decision/layout-read-in-process-only` (PROVISIONAL). The scope keeps the layout read's contract, and the spans rule is the one the capture must not break.

@history_item n=29 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — seven_guis' shared test module holds the log checks' `Needles`, `stand_needles`, `stamped_lines`, `recorded_needles` for `host_log` and `mcp_log`; the needle readers this chunk extends to prove no log line carries the new answer content.

@history_item n=30 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — `tests/blitz-tests/tests/fixtures/mod.rs` states once the fixture boot and the pages more than one check reads; the home for the minimal panicking/logging fixture app the scope expects beside the stand.
