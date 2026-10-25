# Task — obs-plan · 2026-10-25-cold-agent-bar

## The session
The stand grew to six served tasks: `seven_guis::stand` adds `boot_cells` and `boot_load`; the load page boots with its own in-process network stand-in `LoadNet`, and `escher-session` serves `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`.
The flight booker's two `aria-pressed` mode buttons are retired. One `select#flight-type` replaces them, so the stand no longer states `aria-pressed` anywhere.
The cold-agent pipe now runs six apps. Its verdict gains seven fields and three reasons, `wrong_calls` decides by the founder's 5 % bar, and the live server child runs `mcp --record` into `target/cold-agent/record/`.
Measured: `host_log`'s new fourth test serves `cells` at `RUST_LOG=trace` and reads a paged `snapshot --part 2` through the host's own sink. It finds 0 id and 0 name needles under both builds, which discharges the residual for the host-sink paged part read.
The founder's one live run of 2026-10-25 passed: 6 tasks, 66 calls, 0 wrong, $0.8244. It carried the new screens to the model provider.
No driver schema, verb, span, span field, log line, dependency or `packages/` file moved.

## Reached files
- tests/blitz-tests/tests/stand_cells_served.rs
- tests/blitz-tests/tests/stand_load_page.rs
- examples/seven_guis/src/tasks/flight_booker.rs
- examples/seven_guis/src/stand.rs
- examples/seven_guis/src/session_host.rs
- examples/seven_guis/tests/cli_commands.rs
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/common/mod.rs
- tests/blitz-tests/tests/common/mod.rs
- tests/blitz-tests/tests/session_common/mod.rs
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/stand_act_states.rs
- tests/blitz-tests/tests/stand_snapshot_state.rs
- tests/blitz-tests/tests/stand_act_disabled.rs
- tests/blitz-tests/tests/stand_act_ids.rs
- tests/blitz-tests/tests/stand_act_diff.rs
- tests/blitz-tests/tests/stand_element_ids.rs
- tests/blitz-tests/tests/stand_id_persistence.rs
- tests/blitz-tests/tests/stand_snapshot.rs
- tests/blitz-tests/tests/stand_session_recording.rs
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/stand_accessibility_ids.rs
- scripts/cold-agent.sh
- .github/scripts/test_cold_agent.py
- escher-0.2.0/verification-matrix.json
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/scope-record.md
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-verdict.json
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-run.md
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/operator-pass.md

## Touches
- §1 examples row: its citation `examples/seven_guis/src/session_host.rs:30-32` moved, because the file gained lines 1-5 and 12-27 before `main`.
- §3 Logging stack, the `escher-session` row: it says `main` hands `command_line` the stand's four slugs. The binary now serves six tasks through the private `Served` enum and `TASKS` table. The same row's `session_host.rs:30-32` citation also moved.
- §3 Logging stack, the headless stand row: it lists `seven_guis::stand` as `boot` / `boot_timer` / `options`. It must now carry `boot_cells`, `boot_load` and `LOAD_ANSWER_DELAY`, with `options` still keeping `net_provider: None` for every other caller.
- §3 citation `examples/seven_guis/src/stand.rs:1-244` moved: the file grew by 174 lines, including 131-245 and 283-330.
- §3 counts of the shared-module readers: these are the 21 readers of `tests/blitz-tests/tests/common/mod.rs` and the twenty-nine readers of `tests/blitz-tests/tests/session_common/mod.rs`. The new `stand_cells_served` imports from `session_common`, and the new `stand_load_page` starts sessions through it, so re-read both counts.
- §3 seven_guis spawn row: it says `host_log` spawns the binary on `crud` at `trace` over three hosted calls. `host_log` now has four tests, and the new one serves `cells` and reads a paged part.
- §3 citation `examples/seven_guis/tests/host_log.rs:303-322` moved, because the body shifted by lines 134-278. The citations `examples/seven_guis/tests/common/mod.rs:203-211` and `:1-9` moved too, because lines 6-7 were added.
- Silence detail, the seven_guis shared-module row: its citation `examples/seven_guis/tests/common/mod.rs:371-514` moved. The readers it lists for `host_log` and `mcp_log` now include `cells_needles`.
- Silence detail, the `stand_load_wait` row: it presents `LateNet` beside `HeldNet` as the hand-written network stand-ins. `LoadNet` in the stand's own boot is now a third.
- Silence detail, the visual-lint row: `stand_lint` was re-pinned to the flight booker's measured `overlap` at boot. The lint defect itself is unresolved, and no driver lint code changed.
- Silence detail, the rows for the re-pinned stand checks (`stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`, `stand_requirement_sweep` and its `WITNESSES_020`): the tables and tests behind them changed, though their silence claims stand. `WITNESSES_020` gained three rows and a `Served` subject.
- §3 Cold-agent pipe log: it describes the run as four apps. The per-run `mcp.json` spawn line for the live server child now adds `--record` into `target/cold-agent/record`, and replays still run without it.
- §3 Cold-agent pipe log state area: it lists `verdict.json` through `check.json`. It must now add `target/cold-agent/record/`, which is owner-only and removed by `cleanup`.
- §3 Cold-agent pipe log citations `scripts/cold-agent.sh:42-536` and `:161-175` moved: the file grew by 127 lines in 25 ranges.
- §6 Log format of the cold-agent pipe: its citation `scripts/cold-agent.sh:501-533` moved, and the `run.end {every verdict field}` event now carries the seven new fields.
- §6 verdict-fields row: it says `wrong_calls` counts refused, error and foreign uses. `wrong_calls` now also decides the verdict against the 5 % bar.
- §6 server-log row: `span_counts` now also counts accepted `tool_call` lines per counted verb. Its citation `scripts/cold-agent.sh:148-157` moved.
- §6 client-stream row: its citation `scripts/cold-agent.sh:580-584` moved. The kept checks under `target/cold-agent/record/` are added to the files the pipe never prints or copies.
- §6 Logged events, examples row: `escher-session` now serves six tasks. Its usage line's `apps:` and the MCP `start` tool's `enum` follow; both are the program's own fixed text, and no new diagnostic was added.
- log-coverage detail: the stale pipe row at log-coverage.md:14 and the `session_host.rs:N` citation at log-coverage.md:34 both need updating.
- §8 Past escher's scrub: the 0-needle reading in the `escher-session` host now extends to a paged part read on a served Cells screen, under both builds.
- pii-scrubbing-compliance detail line 11: it holds that the host boots only the four lean tasks and none pages. The report measured that false since `serve cells`.
- §8 cold-agent pipe log row: the screens that `transcript.jsonl` carries to the model provider now also hold Cells' contents, the select's options and value, a lint's findings and a shot.
- §9 local-state row: it lists the agent-run and cold-agent state under `target/`. That now includes `target/cold-agent/record/` with the live run's kept checks, which are uncommitted and uploaded by no leg.
- obs ci-integration detail line 15: its `host_log.rs:N` citation moved.
- Retired: the flight booker's buttons `flight-one-way` and `flight-return`, with their CSS. Any obs text naming them as stand controls is stale.
- Indirect, the stand-offline claim: the stand is offline except for the load page's in-process `LoadNet`. It is reached through `boot_load`, and it reads no env var and uses no `blitz_net` and no socket. Any obs row that calls every stand boot offline moves.

## New facts
- F1 · pii-scrubbing-compliance · `host_log::a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace` measures it. The test runs `serve cells` at `RUST_LOG=trace`, commits a sentinel into a part-2 cell and calls `snapshot --part 2`. It reads 0 id and 0 name needles under both builds, and its failures name a kind and a count only.
- F2 · log-coverage · The load page's stand-in `LoadNet` (`impl NetProvider` in the stand's boot) answers each fetch with fixed CSS from a thread of its own after 200 ms. It has no `blitz_net`, print, env read, socket or listener, and the silence census reads 0.
- F3 · log-coverage · The cold-agent verdict adds `wrong_call_bar_pct` (5), `lint_calls`, `screenshot_calls`, `advance_calls`, `part_reads`, `recorded_sessions` and `recording_kept`. Each is a count, a boolean or a number.
- F4 · log-coverage · The verdict adds the reasons `capability-unused`, `recording-not-kept` and `wrong-calls-over-bar` (`wrong_calls × 100 > 5 × tool_calls`), after `task-unmet` and before the check's reasons.
- F5 · log-coverage · `span_counts` returns accepted `tool_call` span lines per counted verb (`lint` · `screenshot` · `advance`) from `server.log`, which is still never printed or copied.
- F6 · log-coverage · The live server child keeps one check per accepted `start` under `target/cold-agent/record/`. The directory is created with `mkdir -m 700` and removed by `cleanup`. The pipe never prints or copies these checks, and the replay children run without `--record`.
- F7 · log-coverage · `tasks_met` reads accepted snapshot answers only, resets an app on its accepted `start`, and joins the latest answer of each part of a paged screen. The replay's expect match also decodes a paged answer's JSON-escaped text (`paged`, `screen_text`).
- F8 · log-coverage · The seven_guis tests shared module holds `cells_needles(part)`, which returns the in-process Cells screen's author keys for one part and that part's middle column-A cell. It feeds `host_log`'s part read, and its messages name no needle.
- F9 · ci-integration · `target/cold-agent/record/` is local state that no CI leg uploads. No kept check of the live run is committed, and the two committed Counter checks stay the only committed ones.
- F10 · pii-scrubbing-compliance · The live run of 2026-10-25 carried Cells' contents, the select's options and value, a lint's findings and a shot to the model provider, on the founder's word. It ran 6 tasks and 66 calls with 0 wrong; `transcript.jsonl` is still never printed, and the committed transcript is host-path-masked.

## Untouched
- escher-telemetry's subscriber, line format, closed-span record class, target allowlist and scrub sets: nothing under `packages/` changed.
- escher-driver's two spans, `command` with its eight fields and `tool_call` with `tool` · `ending`: no span, field or log line moved.
- escher-driver's silent modules (`lint`, `image`, `recording`, `replay`, `describe`) and their `Debug` and `to_json` rules: no driver source changed. The lint defect is only surfaced.
- §4 span and trace coverage and its absence searches: no span was added, and the new boots run under the host's existing `command` spans.
- §5 metric coverage: no metric, counter or exporter was added.
- §7 error capture and panic hooks: no hook, error class or reporter changed.
- §2 telemetry strategy and feature wiring: no manifest, lockfile or cargo feature changed.
- The upstream apps' `fmt::init()` stdout subscribers, the WPT runner's `env_logger`, and the engine values logged as-is: no engine call site was touched.
- The agent-run harness log's event shape and its state area under `target/agent-run/`: only test counts moved, not the events.
- seven_guis `mcp_log`, `host_binary`, `host_timer`, `cli_flow` and `recording` process checks: none changed.
- The fixtures shared module, its `HeldNet` stand-in and its 24 readers: the two new stand checks declare no fixtures module.
- The CI artifact uploads, the package-install step and the publish builds' logging: no CI step changed.
- Service identity (`service.name=seven_guis` for both binaries): unchanged.
