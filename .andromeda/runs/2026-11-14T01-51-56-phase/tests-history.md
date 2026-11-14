@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — HarnessOptions went from six fields to eight (`font_ctx`, `incremental`), with `stand::options(incremental)` building the pinned literal, the stand checks asserting unconditionally under the bundled DejaVu Sans, and `TimerTicks` standing in for the timer's real delay. This is the precedent for adding a HarnessOptions field (a seed). The stand's pinned options literal and its boots (`stand::boot`, `boot_timer`) are what a seeded run extends. Rows: stand-checks-bundled-font, stand-timer-ticks.

@history_item n=2 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — The bridge tests sit in a `cfg(test)` child module of `dioxus_document`, and no function under test gains a constructor or `pub`. Tables and helpers shared by stand checks moved into one shared module that is no target, stated as expectation. This is the constraint (bridge-tests-without-widening, shared-stand-tables-in-one-module) for any dioxus-native-dom randomness source test, and for where the proof's tables go.

@history_item n=3 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — Added `settle`: no clock read, no time advanced, no load waited on. Also `Session::act`, and a fetching `net_provider` wrapped in the load counter (§7 Builders and options). Rows: settle-never-advances-time, harness-counts-fetching-providers-only. A seed added to HarnessOptions/`into_config` must keep these construction defaults and settle's side-effect-free contract.

@history_item n=4 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — Set the driver unit-test rules a schema change follows: every table is stated in its test with its row count asserted, an assertion over a call carries a row index only, and mutation controls are recorded. Trap: the crate's count appears in two word orders. Rows: table-tests-count-their-rows, call-assertions-carry-row-index, assertions-print-no-argument-value. These apply if `start` gains a seed argument kind.

@history_item n=5 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — session-start states `run` and `with_time`, the time-step builder on `Session`. It set the rule that a driver check's failure message names a layout mode, a task and an index, never an id or screen text. The scope names a seed builder "beside `with_time`", so this is the surface and the message rule the seed's builder and proof follow.

@history_item n=6 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — Trap: a headless key press is not the same on every platform. The editor's Backspace arm is compiled out on macOS, so a check read green on Linux and red on macOS. Prove a state change with clicks and typed characters. This bears on the proof's scripted sequence, which must read the same state at every step on every host.

@history_item n=7 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `host_log` covers a sink-installing host at `trace`, and `telemetry_drop` drops other targets. Trap: a check that must fail red on a leaking sink cannot `start` a piped host, so it drains while the host runs. Rows: telemetry-target-allowlist, drain-host-pipes-while-running, host-stderr-no-screen-content. The scope keeps a seed out of every log line, so a seed-bearing start's log check follows this pattern.

@history_item n=8 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — A check reading a sink's capture counts lines by target, never in total. A green per-package run is not the workspace leg's green (engine `tracing` is compiled in under the workspace build only). Rows: sink-capture-counted-by-target, per-package-green-not-workspace-green, driver-log-fixed-words-only. A seeded run's span or log assertion must hold under both builds.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — Introduced the `tool_call` line (`tool` and `ending` only) parenting `command`, the MCP checks, and `stand_mcp_tools`. Standing trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test. This applies if the seed's absence from span fields is asserted in process. Rows: mcp-capture-counts-by-span, command-span/verb.

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** — Added the `help` self-description (`describe.rs`, the count row) and its pins across the CLI and MCP: `help` reads the same bytes on both surfaces and is answered before any start. The scope lists `help`'s self-description among the surfaces a seed may touch, so a seed argument moves describe's row and these pins.

@history_item n=11 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — The direct determinism precedent. The harness's time became a `VirtualClock` its documents read, `advance` moves it by exactly `ms` on every session, and `timer_step` carries its remainder. `stand_virtual_clock`'s 800 ms sleep is the one recorded sleep. Rows: one-virtual-clock, harness-virtual-clock, advance-moves-clock-by-exactly-ms. The scope's premise that `BoaClockAdapter` anchors `Date` at the host's `SystemTime::now()` rests on this clock.

@history_item n=12 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (counts entry) — `into_config` puts the harness's clock in `DocumentConfig.clock`, and a wrapped document keeps its own. `virtual_clock_engine` and `stand_virtual_clock` run in both layout modes. This is the pattern a run's seed would follow from HarnessOptions into the document config. Row: wrapped-document-keeps-own-clock.

@history_item n=13 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (Epoch 2 close ratification) — The founder ratified the one-virtual-clock PROVISIONAL marks, including `advanced_ms`, the eight causes in order, the clock moved by exactly `ms`, and the harness clock never moved by `screenshot`/`settle`. The time-determinism rows this chunk sits beside are therefore standing, not provisional.

@history_item n=14 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `start <task> [--record <dir>]` became a ratified boundary widening. The kept check's record fields are `tool`, `arguments` in schema order, `masked`, `accepted` and `digest`. The wire went to `v3`, and checks that record are `cfg(unix)`-gated item by item beside an off-unix `Unsupported` check. This is the precedent for `start` taking an argument and for recording a seed in the kept check. Rows: session-recording/record-fields, recording-checks-unix-gated.

@history_item n=15 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — Added the kept `answer`, `Check::replay` (nothing on another app, stops at the first differing acceptance or answer), the committed checks under `tests/blitz-tests/checks/` replayed by `stand_replay`, and kept fresh by seven_guis `recording`. The scope's "a kept check replays the run it recorded" rests on this. A seed in the kept check changes the committed checks and their freshness test. Rows: recorded-checks-replayed-as-tests, recorded-checks-kept-fresh, check-replay/stop.

@history_item n=16 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (counts entry) — `stand_screenshot_repeat`'s byte-identity holds on one host only, because a shot's PNG bytes differ between linux x86_64 and macOS aarch64. A check compared byte for byte across OSes holds no `screenshot` step. The cross-host trap for "two runs of one seed read the same state": the proof compares snapshot text and diffs, not shots. Rows: no-screenshot-in-cross-os-checks, no-screenshot-step-in-committed-checks.

@history_item n=17 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — Added the third shared module `tests/fixtures/mod.rs` (`boot`, `hold`, shared pages, `HeldNet`) with no print, tracing, env read, file or socket. The test-data bootstrap was measured: "Nothing is seeded", the fixtures are code. Rows: shared-fixtures-in-one-module, shared-fixture-pages, test-data-is-code. A randomness-reading fixture page for the proof goes here if more than one check reads it. Otherwise it stays in its own check file.

@history_item n=18 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — The wall-time exceptions in session checks: `stand_load_wait`'s clock reads and `LateNet`'s 200 ms sleep join `stand_virtual_clock`'s sleep, and a check whose subject is wall time may read it. Rows: no-wall-time-in-session-checks, no-sleep-or-wall-clock-in-checks. The seeded-runs proof must take no sleep or wall-clock read.

@history_item n=19 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — A new argument kind (`ArgKind::Part`, 1 to `MAX_PART`) was added across the ratified carriers (CLI `--part`, MCP, socket, recording), with the counts it moved (`cli_commands`, `stand_act_spans`, `host_log`/`mcp_log` needles, `session_common` readers). This is the precedent the scope's premise names for a seed as "a new argument kind on a ratified carrier".

@history_item n=20 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `test_capability_statement.py` (`CapabilityStatementTest`) pins `docs/capabilities.md`. Every verb and cause is a code span, counts are in words from the schema's tables, the README's tool count is checked, and the claim section maps each claim to its check. Rows: capability-statement-pinned, capability-claims-name-their-checks, capability-check-names-held-to-schema. The scope lists `docs/capabilities.md` and its pin.

@history_item n=21 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (served pages' boots) — `stand::boot_cells` and `stand::boot_load` were added. `boot_load` replaces the options' `net_provider` with `LoadNet` and keeps every other field. Every other stand boot keeps no network provider. Rows: stand-boots-offline, stand-boot/boot-load, stand-load-page-fixed-answer. The stand reads no randomness today, so the proof's randomness-reading app or stand surface is a new boot or fixture beside these.

@history_item n=22 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six served slugs) — `main` hands six slugs (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), only the Timer gets a time step, and `cli_commands` starts each task. A seed entering through `start` reaches all six served tasks and the binary's start path.

@history_item n=23 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — Set the coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure stand-repeat budget, with three rounds and no retry. Rows: stand-checks-zero-failure-budget, zero-failure-stand-repeat, stand-repeat-three-rounds. New seed code in escher-driver, the harness and dioxus-native-dom counts against these floors. The proof joins the repeat leg.

@history_item n=24 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat` runs the `stand_*.rs` glob plus named `--test` words (then ten) pinned by `test_stand_repeat_leg_runs_three_rounds`, chosen over renaming. A proof target with no `stand_` prefix enters the repeat leg only by a named word and its pin.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (stand-repeat) — The newest leg shape: 51 `stand_*.rs` files at the time (54 now) and seventeen named `--test` words, `test_stand_repeat_leg_runs_three_rounds` naming the glob and the seventeen. This is the register the chunk's proof target extends if it carries no `stand_` prefix.

@history_item n=26 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`zucqymgd`, blitz-vibey-script inline-test absence) — blitz-vibey-script's 13 `runtime/` and `dom/element/` modules (`runtime/bootstrap.rs` among them) hold no test module, and the crate's script tests stand in `tests/`. The page-facing `Math.random` replacement lands in those bootstrap scripts, so its test follows the crate's `tests/dom.rs` convention.

@history_item n=27 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`cp6hvbjs`, script tests' files on disk) — `temp_dir_with_files` reads `TMPDIR` through `std::env::temp_dir()` in a test only. The script tests' seeds are inline raw strings and one shared page. Bears on the scope's boundary of no new env read: a seeded script test states its page inline.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`wb5mdgi4`) — `pseudo_element_flow_classification` was brought to `for incremental in [false, true]`, ending its exception. With 2026-11-13's named exception for upstream targets, the both-layout-modes rule binds every check escher writes. The scope's proof must run both modes. Rows: layout-tests-in-both-layout-modes, stand-checks-in-both-layout-modes.

@history_item n=29 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (`gkbc5lsj`) — `decision/layout-tests-in-both-layout-modes` gained its except clause: upstream targets taken in a sync are kept as written, and those that run one mode are named exceptions. This is the current form of the rule the chunk's proof must meet. The exception covers no new escher check.

@history_item n=30 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (`jiyzcs5d`) — The stand registers task state through `tasks::use_graph_signal`, a dioxus-native-dom hook (`use_graph_state`), with no dioxus-core change and no manifest moved. This is the precedent for the scope's app-facing source as "one engine-provided source" (a root context or hook) that a Dioxus app reads. Row: graph-signal-registration.

@history_item n=31 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (`xep3xm3z`) — dioxus-native-dom's unit scope grew, and the bridge tests' one `cfg(test)` shim `DioxusDocument::dispatch_to_vdom` became the named exception to bridge-tests-without-widening. A dioxus-native-dom seed source's unit test may need no further shim beyond this rule.

@history_item n=32 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (`d7nbibtj`) — Both committed checks were re-recorded through `record_checks.sh` when the kept answer changed, and their byte sizes are no longer stated. `stand_replay` reads them green. If a seed is recorded in the kept check, the committed checks are re-recorded the same way and stay byte-equal to the binary's recording.

@history_item n=33 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (`i7b3rmrc`) — `Check::replay` runs through `Check::replay_each`, and `handler_coverage` is its second caller, replaying kept checks each on a fresh session. The `coverage` role reads kept checks through `Check::from_bytes`. A seed field in the kept check reaches the replay, the coverage role and their strict reader.

@history_item n=34 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (`gh4iy36p`) — `stand_replay` (4) replays a recorded Counter session, and a changed copy stops at the differing step, naming its keys. This is the replay check a seeded kept check's "replays the run it recorded" would extend.

@history_item n=35 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (`7hxia3by`) — The session binary's `coverage` role (`coverage <app> <check>...`) is a ratified boundary widening with its own endings. With `start [--record]` it is a carrier a seed argument or recorded seed would cross. The session-binary contract rows (command form, usage line) move with any seed argument.
