@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — the CI matrix tests windows and macos and only builds ios and android, linux being the fast `test` leg's; this is the origin of decision/native-platform-ci-matrix, so an exploration check runs as a test on three OSes and has to hold off linux too.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — stand checks assert with no skip under the bundled DejaVu Sans with system fonts off, `stand::options(incremental)` pinned, and `TimerTicks` stands in for the Timer's delay. These are the origins of decision/stand-checks-bundled-font(-no-skip), decision/stand-pinned-real-font and decision/stand-timer-ticks, and an exploration check over the served tasks inherits all three.

@history_item n=3 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — the agent-run 5-command contract: `run stand` picks up files by their `stand_*` prefix, `run {name}` matches `^[a-z0-9_]+$`, and a stand-less run exits 1. A new `stand_*` exploration target joins `run stand` with no script change (contract/agent-run-selection/*).

@history_item n=4 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — trap: a headless key press is not the same on every OS. The editor's Backspace arm is compiled out on macOS, so a deleting key read green on Linux and red on macOS. Breadth-first exploration that drives `press` over every offered key will hit this, so prove state changes with clicks and typed characters, or keep expected paths OS-neutral.

@history_item n=5 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — `tests/blitz-tests` keeps one test-target file per behaviour beside shared modules that are no target (`tests/common/mod.rs`). This is the root of decision/one-test-target-per-behaviour. Trap: a grep address that ends at a function name also selects every item whose name starts with it.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — opened the §3 → Session lifecycle key (session-start … session-proof) that exploration extends, plus the process-lifecycle checks. Traps: an `#[ignore]` host child blocks a bare `--ignored` run; `agent-run.sh` counts a child's inherited libtest lines as tests; a sink-installing host at `trace` overruns an undrained pipe.

@history_item n=7 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `host_log` and `telemetry_drop` cover the sink-installing host at `trace`, the basis of decision/no-ids-or-names-in-stand-logs, decision/telemetry-target-allowlist and decision/host-stderr-no-screen-content. Traps: a check that must go red on a leaking sink cannot use `start` on a piped host, and a needle list read from the stand can outgrow a hand-kept one. Any exploration span or carrier is read the same way.

@history_item n=8 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — `settle` and `Session::act`: `NotSettled{Busy}` is a result and is not rolled back, and settle never advances time (decision/settle-never-advances-time). Trap: the changed set is non-empty on a freshly booted harness, so a check that reads the flag drains it first. Every exploration step is one settled step and must report a not-settled step without stopping.

@history_item n=9 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — escher-driver's schema pins: the closed cause set and a refusal's printed form, the verb table, its lookup and levels, and `validate`'s first-failure order. Every table is stated in its test with its row count asserted. These are the origins of contract/driver-verbs/* and contract/refusal-causes/*. Exploration reads `VERBS`, so these pins bound what it may assume.

@history_item n=10 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` executes the schema, covered by `stand_act_*` files in both layout modes. The rule it set: a driver check's failure message names a layout mode, a task and an index, never an id or what a screen reads. An exploration check's failure and report messages follow this.

@history_item n=11 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the refusal order (`disabled` · `off-screen` · `covered`), `stale` vs `not-found` from the record of ids (at most 4096), and any of the eight causes refused with the instance unchanged. These are the rows contract/driver-session/refusal-order, stale-or-not-found and id-record. Exploration catches and reports these closed causes and relies on the refused-call-changes-nothing guarantee.

@history_item n=12 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — one `escher_driver` command span line per call at INFO, fixed words and counts only. A check that reads a sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green. This is the origin of decision/sink-capture-counted-by-target and decision/per-package-green-not-workspace-green, and binds any exploration span ("fixed words and counts only").

@history_item n=13 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the stand log's second record class, a closed span line, was ratified as a boundary widening (decision/closed-spans-logged-as-lines). A new exploration span rides that class with no new field carrying content.

@history_item n=14 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — the fork keeps one target per file, with upstream's `all` target held `test = false` and pinned by `BlitzTestsTargetsTest` (decision/blitz-tests-one-target-per-file). A new exploration target is auto-discovered with no manifest line.

@history_item n=15 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (test strategy and harness contract) — adds the command-line test level (`cli_commands`, `cli_flow`, unix-gated) and three ignored host children. A check that waits for an idle expiry watches the state directory, because `attach`/`status` restart the count. This is the precedent if exploration reaches a binary carrier (decision/cli-and-flow-targets-unix-gated, decision/seven-guis-cli-targets-unix-only).

@history_item n=16 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — a raw harness click on an element outside the padding box of a box that clips it no longer lands (contract/harness-input/clipped-click). A check that leaned on the old hit carries none of the words a sweep for it would search, and is found only by running the stand gates.

@history_item n=17 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the MCP role's `tool_call` line is the parent of `command`. Standing trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test. This matters if exploration's span is asserted in process.

@history_item n=18 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` is answered before any start and is the same bytes on the command line and MCP. Remedies, exit statuses and describe's count row are pinned in schema/describe tests. A new verb or role from exploration moves these pins (contract/mcp-server/help).

@history_item n=19 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `stand_requirement_sweep` with `WITNESSES`, the requirement-to-check record checked through `Session::run` alone on each lean task. A new capability's witness (v030-09) belongs in this kind of record.

@history_item n=20 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure flakiness budget, with stand checks run three rounds and no retry. These are the origins of decision/stand-checks-zero-failure-budget and decision/zero-failure-stand-repeat. Exploration code in escher-driver must keep the 97 % floor, and its stand check must pass three rounds clean.

@history_item n=21 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold agent's check: `valid_step` over the driver verbs with closed keys, at most 400 steps, and a malformed check never replayed. The replay runs whole and then ablated, each step validated again (contract/cold-agent-check/step, contract/cold-agent-tasks/stand). Explored paths are checks of the same format and bound.

@history_item n=22 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — `advance` is accepted on every session and moves the clock by exactly `ms`. There are eight causes in a fixed order, `fonts-unavailable` last. The harness's time is a `VirtualClock` that `screenshot`/`settle` never move, and `stand_virtual_clock`'s 800 ms sleep is the one recorded sleep. These are the origins of decision/one-virtual-clock, decision/advance-moves-exactly-the-ms-asked, decision/eight-refusal-causes-in-order and decision/harness-virtual-clock, and of the no-wall-time exceptions exploration must not widen.

@history_item n=23 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (the clock checks counted) — `into_config` puts the harness clock in `DocumentConfig.clock` while a wrapped document keeps its own (decision/wrapped-document-keeps-own-clock). A fixture booted for exploration reads the harness clock.

@history_item n=24 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified) — the founder ratified the one-virtual-clock marks: `advanced_ms` exact, the eight causes in order and the timer step's remainder carry (decision/timer-step-carries-remainder). These are standing rules, not provisional ones, when exploration drives `advance`.

@history_item n=25 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (lifecycle contract, wire v3) — `Session::recording_into` (unix only) creates the directory `0700` or refuses `RecordDirNotPrivate`, then writes `check-{n}.json` as `0600`. Each answered instance call is recorded, a masked `type` as `MASKED_STAND_IN`. Past 400 records or 1,048,576 bytes, or on a failed write, the recording ends and `recording_failed()` reads true once. `run`'s signature and span stay unchanged. This is the owner-only, bounded format every explored path must be left in (contract/session-recording/*, decision/owner-only-session-files).

@history_item n=26 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the recording's tests) — a check that records is `cfg(unix)`-gated item by item beside an off-unix `Unsupported` check that only the windows job runs. `stand_session_recording` and the seven_guis `recording` checks, with the stand-repeat list. An exploration check that keeps paths must follow the same per-item unix gating.

@history_item n=27 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (the kept answer and the replay) — each record now keeps `answer`, and `Check::from_bytes` is strict. `Check::replay` runs nothing on another app's session, runs each step through `Session::run`, and stops at the first differing acceptance or kept answer; `Replay`/`Change` hold no reading or `serde_json` type. Committed checks are replayed as tests. "Replays green through `Check::replay`" is this contract (contract/check-replay/*, decision/replay-through-the-call-path, decision/recorded-checks-replayed-as-tests).

@history_item n=28 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (the shot's same-host limit) — a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step. `record_checks.sh` keeps the committed checks byte-equal. This is why an explored path holds no screenshot (decision/no-screenshot-in-cross-os-checks, decision/checks-byte-equal-to-recording).

@history_item n=29 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — a test module moved out of line into a `*_tests.rs` `#[path]` file is no record of the crate's coverage reading, so the reading moves (escher-driver 98.33 → 98.07 %). A later split should forecast a reading change. Exploration's unit tests in escher-driver bear on the 97 % floor.

@history_item n=30 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — `tests/fixtures/mod.rs` holds `boot`, `hold` and the shared pages, with no print, tracing, env read, file, socket, listener or `blitz_net`. A page two checks share goes in `fixtures`; a page with one reader stays in its file. There is no `run fixtures` selection. This governs where exploration's panicking-handler and refusing-control fixtures live (decision/shared-fixtures-in-one-module, decision/shared-fixture-pages).

@history_item n=31 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — acting steps wait out a load in flight up to `LOAD_WAIT` (5 s wall) and end `NotSettled(Loads)` at the bound. A check whose subject is wall time may read it, the plan's second exception (decision/load-wait-bound, decision/no-sleep-or-wall-clock-in-checks). Exploring the load page goes through this wait. Exploration's check itself takes no sleep.

@history_item n=32 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the pipe over six apps) — the live server records into an owner-only `record/` (`mkdir -m 700`) that is never printed, copied or committed (decision/cold-agent-record-private). This is the precedent for exploration's kept paths: owner-only, outside the tree.

@history_item n=33 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages' boots) — `stand::boot_cells` and `stand::boot_load` with `LoadNet`, the one networked boot (fixed answer after `LOAD_ANSWER_DELAY`). Every other stand boot is offline (contract/stand-boot/*, decision/stand-boots-offline, decision/stand-load-page-fixed-answer). Exploration over the six served tasks boots through these.

@history_item n=34 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six served slugs, the recording bound) — a full recorded Cells session keeps 312,431 bytes, measured by a probe, against the 1,048,576-byte recording bound. Breadth-first exploration of Cells' paths is sized against this bound.

@history_item n=35 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the dev-host complexity ceiling) — a cognitive-complexity ceiling of 15, read on the dev host with rust-code-analysis. `cognitive.sum` counts a closure's or nested `fn`'s branches in its parent, so only module-level sibling items lower a reading. A breadth-first driver loop is the kind of function this gate reads.

@history_item n=36 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat` runs the `stand_*.rs` glob plus a fixed list of named `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds`, and `run stand` keeps its prefix rule. An exploration target without a `stand_` prefix would need a named word in `ci-leg.sh` and in its pin.

@history_item n=37 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `CapabilityStatementTest` pins `docs/capabilities.md`. Every `VERBS` verb and cause appears as a code span, five counts are written in words, and the claims map names each claim's check (decision/capability-claims-name-their-checks). The exploration capability's claim and limits must name their checks there.

@history_item n=38 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause) — an option of a closed select stays `covered`, and the fixed meaning says to click the select first (contract/refusal-cause/covered). Exploration over the flight booker meets these refusals until the list opens. Each one is a reported cause, not a failure.

@history_item n=39 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired) — a key press on a focused control whose anonymous block was dropped panicked in the engine, and a remote page could reach it. It is now fixed and pinned by `key_press_after_container_lost_its_box`. This is a precedent for a panic inside a step, the class exploration must catch and report.

@history_item n=40 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load ends the wait) — the load counter forwards `NetHandler::error`, so a failed load ends the driver's wait instead of reading `NotSettled(Loads)` for good. Trap: a take that extends a trait must sweep the trait's implementers.

@history_item n=41 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — the `RefCell already borrowed` panic of a `MountedData` call in a task spawned by a key handler is closed by the buffered render. The stated limit is that a task polled during render sees the previous render's document. This is a precedent for handler-raised panics and for which panics a fixture can still provoke.

@history_item n=42 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the session contract takes graph) — adding the instance verb `graph` moved `VERBS`, the session-binary verb count, the recording's instance-verb set, `cli_commands`, `mcp_server`, the cold-agent pipe's tool list and `isolated`, and describe's count row. This is the full fan-out a new verb on a carrier would cost. The socket crossing was recorded as a PROVISIONAL widening.

@history_item n=43 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (`stand_actions`) — `Session::actions` lists each served screen's actionable nodes with the table's verbs and the causes the call would return, changes nothing, and holds no value. Its proof is `stand_actions` in both layout modes. This is the reader exploration enumerates from (contract/driver-verbs/actions, decision/actions-answer-holds-no-value, contract/actions-command/argument).

@history_item n=44 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the measured actionable counts) — `ACTIONABLE_AT_BOOT` reads counter 2, flight-booker 7, timer 3, crud 11, cells 678 and load 2. The Cells `actions` answer is 106,088 bytes, with 461 cells `off-screen` for `click`/`type`/`screenshot`. Breadth-first exploration's frontier on Cells is large and must be bounded by the step budget.

@history_item n=45 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the verb table at fourteen) — `VERBS` 14 with 10 instance verbs, describe's row `[14, 14, 12, 3, 8, 4, 15]`, the cold-agent pipe at fourteen tools, and the listing test renamed. Any new exploration verb or role moves all of these pins again.

@history_item n=46 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (`stand_handler_coverage`) — `Check::replay` runs through the crate-private `Check::replay_each` with a no-op hook. `handler_coverage` is its second caller: it replays each kept check on a fresh session and reads the UI graph after each step. It is covered by a stand check, not a unit test. This is the nearest precedent for exploration's fresh-session-per-path replay and its hook.

@history_item n=47 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the session binary's coverage role) — the `coverage <app> <check>...` role on the command line, a boundary widening confirmed PROVISIONAL by the delegate. It reads checks bounded and refuses `malformed` with nothing of the path. This is the precedent for the scope's open carrier question (in process, a binary role like `coverage`, or a verb).

@history_item n=48 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the kept actions step's replay, measured) — `stand_replay` replays a kept `actions` step, and a changed copy stops at step 3 keyed `["nodes"]`, discharging the CARRY. Explored paths that hold `actions` steps replay under this proof.

@history_item n=49 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stated limits) — handler coverage does not count a handler inside a canvas (deferred to 0.4.0), and a bubbling dispatch counts every ancestor listener. Its limits are written into `docs/capabilities.md`'s claim map. This is the pattern for exploration's stated limits (canvas, navigation).

@history_item n=50 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — decision/handler-coverage-reads-every-replayed-screen moved from provisional to standing on the founder's word. This is the ratified reading of screens across replays that exploration's per-path replays rest on.

@history_item n=51 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (5. Integration Test Strategy) — decision/layout-tests-in-both-layout-modes gains the named exception for upstream targets taken as written. Exploration's own checks get no such exception and run in both layout modes.

@history_item n=52 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `Harness::from_vdom` provides `SeededRandom::from_seed(DEFAULT_SEED)` as a root context (contract/harness-construction/from-vdom). Exploration reads `DEFAULT_SEED` alone with no seed argument, per scope.

@history_item n=53 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — adds decision/cross-os-readings-by-answer-digest: the six served tasks are read on every CI OS by answer digests against one committed table, with no shot and no kept check file. The bundled-font decision gains the guard `stand_bundled_font_families`. Exploration's cross-OS reading of the served tasks must stay shot-free and digest-comparable.

@history_item n=54 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (5. Integration Test Strategy) — adds decision/start-in-state-reads-as-by-hand-path: a session started in a named fixture state reads as that state reached by hand, in both layout modes. Exploration re-reaches each path from a fresh session as `Session::reach` does, so this equivalence is the standing contract it builds on.

@history_item n=55 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (2. Test Strategy) — decision/inline-cfg-test-modules' exception dropped its count, so a further `#[path]` cfg(test) module in the driver needs no row edit.
