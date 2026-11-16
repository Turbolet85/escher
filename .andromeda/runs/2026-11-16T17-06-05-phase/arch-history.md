@history_item n=1 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — (Standard Contracts: 6 rows) added `contract/acting-verb-results/caused` (closed words, ids and counts only), `decision/capture-dispatcher-pair`, `contract/reached-record/take-step-reached` and `contract/dioxus-native-dom-api/root-error-mark` (`RootErrorMark` compares by identity, its Debug prints its name alone), and changed `contract/answer-json/outcome` (caused only when non-empty) and `contract/busy-classes/render`. This is the register this chunk extends: `caused` gains text, `RootErrorMark` must start reading the error and its source chain, and the dispatcher-pair design (its rejected per-thread alternative) constrains how the capture can admit more targets.

@history_item n=2 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — (Conventions; Established Decisions; Cross-cutting Patterns; Standard Contracts) added `decision/capture-before-own-recorder` (an operator CI fix: a stand check that installs its own recorder opens an empty capture first). Narrowed `decision/driver-library-side-effect-free`, `decision/driver-spans-only` and `decision/escher-driver-spans-ungated` to "no global subscriber", with only escher-telemetry's scoped capture default around an acting step. Added `boundary` to the ungated modules of `decision/accessibility-feature-gating`, and reworded `contract/telemetry/target-filter` so the drop is the sink's alone, since the capture counts outside targets. Widening the capture to engine and escher targets and to their text extends each of these rows. The sink's drop and scrub rules must stay the sink's.

@history_item n=3 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — registered escher-telemetry: `init`, the stderr line shape, the allowlist scrub and the panic event (target `escher_telemetry::panic`, `panic.file` / `line` / `column`, a redacted `panic.payload`, then the previous hook). This is the origin of `contract/telemetry-panic/panic-event` and `contract/telemetry/panic-hook`. The chunk reads the panic's payload and location from the same hook, and the payload is redacted on the log line, so carrying it in the answer must not change what the sink prints.

@history_item n=4 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — set the two public prefix sets `ENGINE_TARGET_PREFIXES` and `ESCHER_TARGET_PREFIXES`. An engine target prints only `SAFE_FIELDS`, an escher target prints every field but `CONTENT_FIELDS`, and any other target is dropped whole at every level, with the drop in the formatter. These are the scrub-by-field-name rules in `packages/escher-telemetry/src/format.rs` that the chunk reuses for answer text, and the origin of the telemetry-scrub rows. The standing rule: a third-party record never reaches the sink, and the chunk's widened capture must not change that.

@history_item n=5 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (the command span; `tracing` the third dependency) set the `command` span's eight fields, recorded only where they apply. It is never fielded with screen text, label, ids, arguments, `Outcome` or `Refusal`, and is built by hand with no `#[instrument]`. Its PROVISIONAL sibling entry added the sink's closed-span line, judged by the event rule. The chunk's boundary keeps these fixed fields: the error text crosses only as the answer, never as a span field (`contract/command-span/never-fielded`, `decision/driver-hand-built-spans`).

@history_item n=6 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the closed-span line and the rule that a diff's counts may reach a log while its content never may. This is the precedent the chunk's "no message, payload or field value reaches a log line" boundary follows: counts in logs are allowed, content is not.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (driver session registered) added escher-driver, the Unix socket and the `escher-session` host, and a second `escher_telemetry::init` install site. It set the standing rule that the first request carrying screen content is a new crossing question, and returns Unsupported off unix. This is the origin of `decision/session-over-unix-socket`, `decision/cross-process-session-unsupported-on-windows` and `decision/telemetry-one-install-site`.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the socket carries a call and its answer) made the socket carry ids, names, masked values, snapshot text and diffs out, a ratified widening, and stated that no log carries any of it. Its contract sibling set the 1,048,576-byte answer bound, the `oversize` reply and `AnswerTooLarge`, and the hand-written JSON forms. Error text is new page- and app-authored content on this carrier, and must fit the same bound (`contract/session-wire/ok-oversize`, `contract/session-errors/answer-too-large`).

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the MCP server over the `mcp` role's stdio (a ratified crossing; the next surface carrying screen content asks again), with the result bound of 1,048,576 bytes. It took `serde_json` as a reader only and added the `tool_call` span with `tool` and `ending` only. Standing rule: no span field holds what a call supplied or a screen reads. The chunk's text rides the MCP tool result under that bound and adds no span field.

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `help` / `describe()` from the tables, and ratified that the program's own fixed `'static` text rides the carriers. "Text from a page, a call or a screen, a new carrier or a new exit still widens." This is the trap for this chunk: `describe()` naming the new `caused` fields is fixed text inside the ratified carriers, while the error text itself is page- and app-authored content, a widening owed its ratification (`decision/fixed-program-text-on-carriers`).

@history_item n=11 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — sent a shot's pixels in band (ratified) and bounded an MCP result holding an image by its whole written form (`ToolResult::bounded`), with the bounds unchanged. This is the precedent for a larger answer under the same 1,048,576-byte bound: a `screenshot` step's `caused` text adds to that bound (`decision/screenshot-in-band`, `contract/mcp-server/result-bound`).

@history_item n=12 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — extended the snapshot `value` mask to a password and a file input: `MASKED_VALUE`, never the text, its length or a host path. Later the mask applied on every carrier and in kept checks (`decision/masked-input-values`, `contract/node-keys/value`). The scope keeps the mask unmoved, and error and log text must not reintroduce a masked value or a file path through a message.

@history_item n=13 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added the kept check: owner-only, never logged, a masked `type` as one stand-in, wire `v3`, and fifteen session errors. This is the origin of `decision/recorded-session-checks`, `decision/recorded-session-kept-check` and `decision/kept-check-never-logged`. A `caused` with text is kept at rest in every recorded acting step.

@history_item n=14 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — kept each answer whole at rest beside its digest, and committed the two stand-Counter checks (no typed text, host path, name or secret). `Check::replay` compares the live answer line to the kept one and stops at the first difference, and `serde_json` reads that line. Trap: error text carrying OS-, path- or line-specific content makes a kept answer differ on replay or across hosts. The committed checks must stay free of host paths (`contract/check-replay/first-difference`, `decision/committed-stand-checks`).

@history_item n=15 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — added the acting step's bounded load wait (`settle_waiting`, `LOAD_WAIT` 5 s). It reads `Instant` and sleeps only, with no time move, no print and no `tracing` call. This is the origin of the load-wait rows. The capture that `caused` is read from wraps this step, so new text capture must keep the wait's "no tracing call" and "no time moves" promises (`contract/load-wait/no-time-moves`).

@history_item n=16 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (Driver session) retired `time-unavailable`, closing `Cause` at eight, and made `advance` move the session's clock on every session. This is the origin of `decision/refusal-causes-closed-at-eight` and `decision/advance-moves-session-clock`. The chunk adds no refusal cause, and `advance` is one of the acting verbs whose `caused` gains text. The harness-time sibling entry is the origin of `decision/input-helpers-do-not-settle`.

@history_item n=17 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — closed the MountedData hazard as a ratified direct engine change for every Dioxus document. The vdom renders into `MutationBuffer` with the document unborrowed, then replays it under one borrow, a port of upstream `286874cc1`, not upstreamable. This is the origin of `decision/bridge-buffered-render` and `decision/render-with-document-unborrowed`. Reading the root error boundary's error happens on this bridge, so any read must take no borrow across the render.

@history_item n=18 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the UI graph crosses the ratified carriers) a founder-ratified widening that sends names, closed flag words and booleans over the socket, the command line, MCP and kept checks. It added no carrier, exit, cause, session error, span field or dependency, and measured that a typed sentinel occurs 0 times. Its registry sibling made the bridge's dispatch inert without a registry. This is the precedent for the chunk's widening and for `decision/dioxus-event-path-records-opt-in`.

@history_item n=19 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actions answer crosses the ratified carriers; the actions call's span and needles) a ratified widening with the same no-new-carrier shape, measured against the answer bound. Its log checks `host_log` / `mcp_log` read 0 id, name and answer needles at `trace` under both builds. This is the pattern this chunk's text needs: needles proving the message never reaches a log line.

@history_item n=20 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the bridge's listener attributes over-report) the bridge's listener mark over-reports, while the vdom walk's current listeners are exact. This is the origin of `decision/listeners-read-from-vdom`. `caused` names the first reached handler, so where the text says a failure happened rests on this reading.

@history_item n=21 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the reached record on the bridge's dispatch) added the ungated `reached` module, `record_reached` / `reached_handlers`, and a record written before `runtime.handle_event` with no borrow across the render. Its sibling entries added the `coverage` role, which reads named kept checks (ratified on 2026-11-15T05-21-42-wrap). These are the origins of `decision/reached-record-before-dispatch`, the `reached-record/*` contracts and the coverage-role rows. A coverage replay of a check whose answers now hold text compares and reads them unchanged.

@history_item n=22 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the coverage role's kept-check files on argv and the `escher-session` coverage role (they were PROVISIONAL). These are the standing rows `decision/coverage-reads-named-checks` / `decision/coverage-role-reads-named-checks` and `contract/cli-coverage-role/*` that the picked rows hold. Kept checks with text in `caused` pass through this reader.

@history_item n=23 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — narrowed `decision/driver-spans-only` to one span per call on the run path, with an in-process read outside the run path opening none. Added `decision/layout-read-in-process-only` as PROVISIONAL, for the founder's word at the epoch boundary. This is the precedent for how an answer-text widening is held: in process only versus on every carrier. This chunk takes every carrier, so its widening is owed the founder's ratification.

@history_item n=24 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added `explore` with a per-path panic catch (`contract/driver-explore/path-ending`). Changed `contract/kept-check/opening-record` to carry `format` (`CHECK_FORMAT` 1), with a reader that refuses any other value. A change to the kept answer's shape bears on that format number. Panics caught per path are a second reader of panic text beside `caused`.

@history_item n=25 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — added `Session::reach` and `contract/headless-stand/start-in`. A reach runs acting steps through the one run path, stopping at the first refused or unsettled step, and its steps are kept as ordinary calls. Reached steps produce `caused` like any acting step, and their kept answers carry the new text.

@history_item n=26 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — added `decision/stand-readings-same-on-every-ci-os`: the six served tasks read the same snapshot, lint and actions digests on linux, windows and macOS, in both layout modes. Trap: error text holding a source path or a platform-specific message must not make acting answers or digests differ across OSes on the stand.

@history_item n=27 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — added `decision/one-run-seed` and the `seed` module to the ungated list of `decision/accessibility-feature-gating`, a list `boundary` later joined. Any new module this chunk adds to dioxus-native-dom (an error reader) must be placed in or out of that closed list.

@history_item n=28 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — made the stand serve six tasks, Cells and the load page (`LoadNet`, offline but for the load page). The cold-agent run sends the served screens to the model provider, a ratified content widening. These are the origins of `contract/headless-stand/boot`, `contract/headless-stand/offline-boots` and `decision/cold-agent-run-carries-stand-screen`. Error text in `caused` now also reaches the model provider through the MCP server.

@history_item n=29 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — moved the cold-agent pipe onto the escher MCP server, where the run carries the stand's screen, a ratified widening. Its `server.log` is never printed or copied. This bears on `decision/cold-agent-run-adds-no-listener` and `decision/raw-run-output-never-printed`, both in the picked rows.

@history_item n=30 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — gathered the seven_guis log checks' readers (`Needles`, `stand_needles`, `found`, `occurrences`, `recorded_needles`) in the shared test module for `host_log` and `mcp_log`. This is where the needles proving that no error text reaches a log line belong.

@history_item n=31 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — added the `stand-repeat` leg: three rounds, the first red command ending it, no retry. This is the origin of `decision/stand-repeat-zero-failure-budget` and `contract/ci-leg-legs/stand-repeat`. New stand checks of the error text join it by their `stand_` prefix, or by name.

@history_item n=32 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Conventions) gave `decision/both-layout-modes` its exception: upstream-taken targets keep their one mode, while escher's own targets run both. The acceptance requires both layout modes for the new checks.

@history_item n=33 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (`Parallel` style traversals serialised) stated that no headless document sets `style_threading` (`decision/headless-documents-default-threading`), and recovered `PARALLEL_TRAVERSAL_LOCK`'s poison with no panic path. This is a precedent for `decision/mutex-poison-recovery`, relevant to any lock the capture holds while it records text.

@history_item n=34 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (one dev-profile debuginfo level) set `[profile.dev] debug = "line-tables-only"`, inherited by `test`, for one level on the dev host and CI (`decision/one-debuginfo-level`). Panic locations come from `Location`, not debuginfo, but any backtrace or source-location text the chunk measures reads under this profile.
