@history_item n=1 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — 3. Test Harness Contract: `~ contract/driver-session/acting-verbs` gained `caused` (last among the acting verbs' fields), and `~ contract/session-recording/exploration-checks` now says a path whose accepted answer carries a panic item ends panicked with no payload and its check holds the panicked step. This chunk adds text to that same `caused` and to the panic item, so both rows' promises are what it extends. Panicked exploration paths must keep replaying green once the item carries a payload and location.

@history_item n=2 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — 5. Integration Test Strategy: `+ decision/root-boundary-err-on-own-session`. A handler Err the root boundary takes is checked on its own session, its sentinel check reading `caused` alone, because the boundary replaces the app on screen. The new cause-chain acceptance on the root boundary's Err must follow this row, and the narrowed sentinel check has to be widened to admit the message text without dropping the session split.

@history_item n=3 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — set out the stand log line format and the scrub (allowlist, bound to obs-plan §3). It added escher-telemetry's scrub-branch unit tests and the one-process-per-behaviour files `telemetry_scrub` and `telemetry_panic_hook`. This chunk reuses the scrub in `packages/escher-telemetry/src/format.rs` and the panic hook in `packages/escher-telemetry/src/panic.rs`. Those checks are where field redaction and panic payload/location reading are already pinned, and they must stay green when the readers are reused for `caused`.

@history_item n=4 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `host_log` covers a sink-installing host at `RUST_LOG=trace` and reads 0 ids and names. It drains both streams on a thread each, uses `attach` and never `start`, and prints counts only on failure. `telemetry_drop` proves a third-party target is dropped. Traps: a check that must fail red on a leaking sink cannot use `start` on a piped host. Read needles from the stand rather than keeping a hand list. This bears on `decision/telemetry-target-allowlist`, `decision/sink-drops-third-party-targets` and `decision/drain-host-pipes-while-running`. The chunk admits engine and escher targets into the capture while the sink must still drop them from log lines.

@history_item n=5 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — recorded the limit of `stand_session_quiet`: a host with no sink proves nothing about a sink-installing host, which is now `decision/log-silence-read-on-a-sink-host`. Traps: an inherited child's libtest lines get counted as tests (now `decision/child-streams-to-null`), and a sink host at `trace` overruns an undrained pipe. Any new no-message-on-stderr check this chunk adds must run on a sink host and drain the pipes.

@history_item n=6 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — added `stand_act_spans` and the closed-span line. It found that the same check read green per package and red in the workspace leg, because the workspace build compiles blitz-dom's `tracing` call sites in. This is the source of `decision/sink-capture-counted-by-target` and `decision/per-package-green-not-workspace-green`. The chunk's capture of engine and framework records will see different engine records under each runner, so it must count by target and be read green under both builds.

@history_item n=7 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the stand log's second record class (closed span lines) is ratified, which is now `decision/closed-spans-logged-as-lines`. The chunk's boundary keeps the `command` and `tool_call` spans to their fixed fields, and no message reaches a span field. The span-line checks are the guard on that.

@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role writes one `tool_call` line per call (`tool` and `ending` only), parent of `command`, and `mcp_log` covers the role at `trace` under both builds. Standing trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test. This matters for any in-process check of the capture that reads the app, framework and engine records.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the command-line test level and measured typed text on the session host (3 calls, 3 command-span lines, 0 sentinel occurrences, both builds). A command's stdout is its answer and nothing else. This is the precedent for `contract/cli-command/stdout`, `contract/cli-command/stderr` and `decision/session-logs-no-screen-content`. When the error text crosses on the CLI JSON line, it must cross there alone.

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `describe()` and its tests: describe.rs's count row, schema_tests pinning field kinds and help, `cli_commands` `help`, and `mcp_server` `help` returning the same bytes on both surfaces. The chunk's change to the help (the new fields of `caused`, the cut's and the count's meanings) moves exactly these pins and `contract/mcp-server/help`, `contract/cli-help/output` and `contract/mcp-help/parity`.

@history_item n=11 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the latest verb-table pin set: schema_tests field kinds 13 with `ActionNodes` last, `describe`'s count row `[14, 14, 12, 3, 8, 4, 15]`, and the `amended` help rows at 12. If `caused` gains new field kinds or help rows, the count row and these schema_tests tables move; the precedent is to key field kinds per verb. The boundary forbids a new verb, cause or session error, so those cells of the row must stay unmoved.

@history_item n=12 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `CapabilityStatementTest` pins `docs/capabilities.md`, whose verbs, causes and counts are read in words from the schema's tables (`decision/capability-statement-from-schema`). A claim's check is named in its `## Where each claim is proven` map. A new stated reach of `caused`, or an unread app boundary stated by the help, must be mapped to its check there.

@history_item n=13 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the recording records each answered instance call: `tool`, `arguments`, `masked`, `accepted`, `digest` (FNV-1a over the whole written line). It ends past 400 records or 1,048,576 bytes and does not roll back the call (`contract/session-recording/bounds`, `contract/session-recording/record-fields`). With per-record error text, a kept check's lines grow and every digest of an acting step that caused something changes, so the per-answer cut and count must keep the recording inside its bounds.

@history_item n=14 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each record keeps `answer` whole, and `Check::replay` stops at the first differing acceptance or kept answer (`contract/check-replay/stop`). The committed checks under `tests/blitz-tests/checks/` are replayed by `stand_replay` and kept fresh by seven_guis `recording`. Error text in `caused` must replay deterministically, so a kept step with `caused` text must read equal on replay. If a committed check's steps cause anything, it must be re-recorded byte-equal.

@history_item n=15 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (the shot's same-host limit) — PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no `screenshot` step. This is now `decision/no-screenshot-in-cross-os-checks`. It is the precedent for any answer content that can differ by OS: a panic location's file path or an engine message in `caused` may differ by host. It must not enter a cross-OS byte-compared check or digest.

@history_item n=16 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — `+ decision/cross-os-readings-by-answer-digest`: `stand_cross_os_readings` compares the six served tasks' answer digests at boot and after one acting step in both modes against one committed table. The acting step's answer now carries `caused` text. If that step causes any record (an engine or framework log line, now captured), the digest table moves, and it must be identical on every CI OS.

@history_item n=17 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the precedents for answer size and value checks. The Cells `actions` answer was measured through the binary (106,088 B), and `the_whole_cells_answer_is_within_the_answer_bound` asserts within 1,048,576 in both modes. The actions answer holds no value: a typed sentinel and `MASKED_VALUE` occur 0 times. These are the pattern for the chunk's bound acceptance and its mask acceptance (`decision/typed-password-never-returned`).

@history_item n=18 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the Cells screen and its graph answer, measured) — the plan forecast the graph under 200 KB and measurement disproved it (547,685 B, then 293,869 B after fixes), leaving a stated limit with a residual row. This is the cautionary precedent for the chunk's "bounds set by measurement": take the message-length distribution before fixing the cut and count, and assert the bound measured, not assumed.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (a graph call logs its verb alone and no graph needle) — a new answer content class got its needle helper (`graph_needles()` in seven_guis' shared test module), and `host_log` and `mcp_log` each drive one call, reading 0 needles at `trace` under both builds. `stand_act_spans` reads the verb's `command` line as recording `verb` alone. This is the precedent for proving the error text crosses only as the answer: add error-message needles and read them absent from stderr.

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions call in the span and log checks) — the latest form of that pattern: `actions_needles()`, with `host_log` and `mcp_log` each driving one more call. The command-span and `tool_call` counts are restated (host_log 9 calls / 9 lines, mcp_log 12 / 10), and `stand_act_spans` reaches 38 calls per mode. A new needle class for `caused` text extends these counts, `contract/driver-command-span/one-line-per-call` and `contract/escher-session-stderr/no-ids-names-or-typed-text`.

@history_item n=21 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `host_log` gained a standing check of a recording session's clients at `trace` reading 0 needles. `stand_act_spans` added a paged part read with its ids and names joining the needles. This shows that every carrier (recording included) gets a needle check, which bears on the chunk's "every carrier" scope with the kept check among them. The same entry fixed `stand-repeat` to run named non-`stand_` targets by a literal list (`decision/stand-checks-zero-failure-budget`), so a new non-prefixed check must be named there to join the zero-failure budget.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list through the process checks) — added `option_needles()` with one test each in `cli_commands`, `host_log` and `mcp_log`. The `mcp_log` test records nothing, since recording there needs a `cfg!(unix)` site (`decision/recording-checks-unix-gated`). This is the precedent for adding a `caused`-text needle test to the process checks while keeping the unix gating rules.

@history_item n=23 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the rules it set: a driver check's failure message names a layout mode, a task and an index, never an id or what a screen reads. With `2026-10-07-command-and-refusal-schema`'s "an assertion over a call carries a row index only", this is the origin of `decision/call-assertions-carry-row-index` and `decision/assertions-print-no-argument-value`. Assertions on error messages, payloads and cause chains must not print the asserted text.

@history_item n=24 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the schema's unit pins: the closed cause set and a refusal's printed form (cause name, then remedy), with every table stated in its test and its row count asserted. An assertion over a call carries a row index only. This bears on `contract/refusal-causes/printed-form`, and the boundary forbids a new refusal cause. Table-driven tests of the cut, count and scrub should state their tables with asserted row counts.

@history_item n=25 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the kept actions step's replay, measured) — a new answer content in a kept check was ruled owed a replay test, not accepted by construction (`2026-11-08-what-can-i-do-here`'s CARRY). It was discharged by `stand_replay`'s kept-step replay plus a flipped-copy stop naming the differing key. This is the precedent for `caused` text in a kept check: replay a recorded step that caused a record, and show that an altered copy stops naming `caused`.

@history_item n=26 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — set up `tests/fixtures/mod.rs` with no print, `tracing` call, env read, file, socket or listener. A page two checks share goes in `fixtures`, and a page with one reader stays in its file (`decision/shared-fixtures-in-one-module`, `decision/shared-fixture-pages`). The chunk's real-failure fixture (panic in a code path, Err with a cause chain, framework or engine record) must place itself by that rule. A fixture emitting log records cannot live in the shared module, which holds no `tracing` call.

@history_item n=27 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — moved the stand checks' tables and helpers into `tests/common/mod.rs`, stated once and never as a second role or name mapping (`decision/shared-stand-tables-in-one-module`). Trap: a sed or grep address ending at a function's name also selects every item whose name opens with it. Any shared `caused` reader or needle table goes there under that rule.

@history_item n=28 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — `~ decision/layout-tests-in-both-layout-modes` except: upstream targets that run one mode stand as named exceptions. The both-modes rule (`decision/stand-checks-in-both-layout-modes`) otherwise binds, and the chunk's acceptance names both layout modes in process, through the CLI and over MCP.

@history_item n=29 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `+ decision/layout-reason-words-each-read` (each of a reader's closed words held to a fixture reading) and `+ decision/no-change-read-control-needs-pending-work`. These are the latest precedents for coverage of closed words: the cut marker, the per-source overflow counts and any source word in `caused` should each be read on a fixture. A control proving a read changes nothing needs pending work.

@history_item n=30 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — `+ contract/session-recording/exploration-checks` (one kept check per explored path, replaying green) and `~ decision/mutation-controlled-tests`: a control that removes a bound loosens it to twice the budget rather than removing it. This applies directly to controls of the message cut and record count, and the exploration contract is the one 2026-11-16-errors-with-the-action extended for panic items.

@history_item n=31 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the per-crate line floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure `stand-repeat` budget. These are now `decision/per-crate-coverage-floors` and `decision/stand-checks-zero-failure-budget`. The chunk's new `caused`-text code in escher-driver (and the boundary in dioxus-native-dom) must keep those floors.

@history_item n=32 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — a `*_tests.rs` `#[path]` file is no record of the per-crate reading, so moving a test module out of line moved escher-driver's coverage (98.33 % to 98.07 %). A later split should forecast a reading change. This matters if `caused.rs`'s tests grow into a `#[path]` file near the 97 % floor.

@history_item n=33 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (`stand-repeat` runs named targets) — the leg's named `--test` words are a literal list pinned by `test_stand_repeat_leg_runs_three_rounds`. A new non-`stand_` check of the chunk joins the zero-failure budget only by being added there, with the pin updated. A `stand_*` file joins by its prefix.

@history_item n=34 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `~ overview/session-lifecycle`: the session is no verb of the agent-run contract, and its run session selection runs the binary's `cli_flow` flows. This is the overview the picked `session-lifecycle` rows sit under, and the context for the chunk's command-line and MCP acceptance legs.
