@history_item n=1 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (amd-lrluctc2) — created the §3 → Session lifecycle key (session-start · session-host · session-wire · session-state · session-binary · session-checks · session-proof), the `stand_session_*` checks and `session_common`; the contract this chunk extends with a state on `Session::start`/`start`. Traps it recorded: an `#[ignore]` host child blocks a bare `--ignored` run; a child's inherited libtest lines are counted by `agent-run.sh`; a sink host at `trace` overruns an undrained pipe.

@history_item n=2 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** (amd-s4wow4l3) — `host_log` reads the real `escher-session` at `RUST_LOG=trace` for 0 ids and names; guard of `decision/no-ids-or-names-in-stand-logs`. A state name is a call-supplied argument the scope keeps off every log line, so a started-in-state run is the needle case. Trap: a check that must read red on a leaking sink cannot use `start` on a piped host.

@history_item n=3 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** (amd-jdiryrjq) — the schema's unit pins (closed causes, verb table, `validate`'s malformed witness per rule per kind, first-failure order), every table stated in its test with its row count asserted (`decision/table-tests-count-their-rows`). A new `start` argument kind needs its admitted and malformed rows here. Trap: a count stated in two word orders.

@history_item n=4 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** (amd-t5mve4e6) — rules for driver checks: a failure message names a layout mode, a task and an index, never an id or what a screen reads; no deleting key outside `stand_act_keys`. Binds `stand_start_in_state`'s by-hand-from-boot comparison.

@history_item n=5 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (amd-s53lzeiq) — the command span line and `stand_act_spans`; a sink capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green. The scope requires the command span to record no state name.

@history_item n=6 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** (amd-myqerkgx) — trap: a headless key press differs by platform (Backspace arm compiled out on macOS, green on Linux, red on macOS CI); prove a state change with clicks and typed characters. Bears on reaching each named state by hand from boot through the driver's verbs.

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (amd-itmrnscn · amd-keodlnj3 · amd-ieh35jjn) — the command line as a driven surface: `cli_commands`, `cli_flow`, `host_binary`'s argv rows ending as usage errors, `start`/`status`/`stop`, three ignored host children, the session-wire `call` grammar, `hello` with `idle_expiry_s`; session-binary's endings. `start {task} --state {name}` extends these rows (exit codes 1/2/3, usage line, no echo of caller input).

@history_item n=8 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (amd-u7vpyh4t) — `BlitzTestsTargetsTest` (`test_blitz_tests_targets.py`) pins one auto-discovered target per file and the unbuilt `all` table; the scope names this pin for the new `stand_start_in_state` target (rows `decision/one-test-target-per-file`, `decision/blitz-tests-manifest-difference`).

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (amd-u7jdserc) — MCP checks (`mcp_server`, `mcp_log`, `stand_mcp_tools`), the `tool_call` line parenting `command`; MCP stdio-child checks run on windows while socket checks are `cfg(unix)`. The MCP `start` tool is a carrier the state crosses. Trap: `tracing` callsite interest is process-global, a `with_default` check needs its own recorder per test.

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** (amd-q3tlwlgs) — `describe()` and `help` (same bytes over MCP and CLI), describe's count row, each cause's `remedy` in `cli_commands`; an argument on `start` moves `help`'s text and possibly describe's row and the MCP schema.

@history_item n=11 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (amd-n5xqwtgz) — per-crate coverage floors (escher-driver 97 %) and the zero-failure stand-repeat budget over `stand_*.rs` (rows `decision/per-crate-coverage-floors`, `decision/stand-checks-zero-failure-budget`): new driver code for the state argument counts against the floor; `stand_start_in_state` joins stand-repeat by prefix.

@history_item n=12 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (amd-ybvlv3ad) — `advance` accepted on every session, moving the clock by exactly `ms`; `timer_step` returns nothing and carries its remainder; eight causes in order. Rows `decision/advance-accepted-on-every-session`, `decision/timer-step-carries-remainder`; a Timer state reached by hand passes through these.

@history_item n=13 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (amd-xliprnr6 · amd-3unj565m) — `--record` and the kept check (record fields, `Check::from_bytes` strict, closed keys), wire `v3`, `RecordDirNotPrivate`/`RecordingFailed`; a recording test is `cfg(unix)` item by item beside an off-unix `Unsupported` check. The scope's P4 fork (a kept check of a session started in a state takes the state name or is refused) changes this at-rest form.

@history_item n=14 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (amd-7qex4fkt · amd-siqngma2) — `Check::replay` (nothing on another app's session, stop at the first differing acceptance or kept answer), committed checks under `tests/blitz-tests/checks/` replayed by `stand_replay` and kept byte-equal by seven_guis `recording`; a cross-OS byte-compared check holds no screenshot step. A replay must now start from the recorded state.

@history_item n=15 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** (amd-nlavi4cc) — `tests/fixtures/mod.rs` (`boot`, `hold`, shared pages, `HeldNet`), holding no print, tracing, env read, file, socket or listener; a page with one reader stays in its file; nothing seeded — fixtures are code (`decision/test-data-is-code`), no `run fixtures` selection. The scope builds on this boot and hold rather than a second boot path.

@history_item n=16 marker=2026-10-20-load-wait
**2026-10-20-load-wait** (amd-tem72y7f) — acting verbs wait on a load in flight up to `LOAD_WAIT`; `LateNet`; a check whose subject is wall time may read it. Row `decision/load-wait-bound`; a load-page state reached by hand or started in rides this wait.

@history_item n=17 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (amd-pgaqvksr) — precedent for a new bounded argument on ratified carriers: `ArgKind::Part` (1 to `MAX_PART`), `snapshot --part` in `cli_commands` with 0, 1001 and `x` refused exit 1, the usage line `snapshot [--part <part>]`, MCP `part`, session-binary restated; founder ratified the carriage. Template for `start --state`.

@history_item n=18 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** (amd-472linzq) — `test_capability_statement.py` pins verb and cause names and counts in words and README's tool count; claims map to checks, a renamed or moved check changes the claim section by hand. The scope touches it where verb or argument counts move.

@history_item n=19 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (amd-sam3o45d · amd-o3j7n4hl · amd-u7t47llq) — the six served tasks' boots (`stand::boot_cells`, `stand::boot_load` with `LoadNet`, offline otherwise), `cli_commands` and `host_binary` on six slugs, the stand-boot contract rows; the cold-agent pipe's closed-key `valid_step` over `start`. The states table and boot extend these served tasks.

@history_item n=20 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-7efxs2hi · amd-5zqg4udh · amd-afxaahjb · amd-z23dex63) — `GRAPH_DRIVES` in `tests/common/mod.rs` (each task's drives from boot), session-binary/session-start restated for a new verb, schema pins and describe's row moved; the socket's crossing a boundary widening ratified by the founder (provisional delegate confirm). Precedent for a carrier change and for drive tables a state could reuse.

@history_item n=21 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-dkb4oe4y · amd-ldelf34f · amd-sid6zlb7 · amd-q5lvjmtv) — current verb table: `VERBS` 14, describe's row `[14, 14, 12, 3, 8, 4, 15]`, cli_tests' rule rows, `tools/list` order, `cli_commands` 17, `mcp_server` 8; shared-module reader counts; the cold-agent pipe's fourteen tools. Counts a `start` argument moves.

@history_item n=22 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-i7b3rmrc · amd-7hxia3by · amd-eiwetouu · amd-gh4iy36p) — `handler_coverage` replays kept checks each on a fresh session via `Check::replay_each`; the `coverage` role and its refusals (cli_tests rule table 109 rows); `stand_replay` 4. `coverage.rs`'s replay boot is in scope: a check started in a state must boot there.

@history_item n=23 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** (amd-efljpsvp) — `Harness::from_vdom` provides `SeededRandom::from_seed(DEFAULT_SEED)` as a root context where none is given, a builder's root context standing; the precedent the scope names for an app-supplied initial state riding a Dioxus root context.

@history_item n=24 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** (amd-uimtzpsg) — `decision/cross-os-readings-by-answer-digest`: the six served tasks read on every CI OS by `stand_cross_os_readings` against one committed digest table, at boot and after one acting step, both modes, no shot; the proof by answer digest and any states row in that table extend it.
