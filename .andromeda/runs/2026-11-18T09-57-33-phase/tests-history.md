@history_item n=1 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** (the pipe contract; CI-scripts 37 → 64) — set up the cold-agent pipe's §3 block (exit grammar, verdict fields, positive-evidence `passed`, `reasons` tokens, events, state in `target/cold-agent/` only) and its proof: `test_cold_agent.py` plus one live run, never in CI. This is where the verdict-and-reasons register a new `caused` count joins starts, and where the live run is kept off CI (`cold-agent-live-run-off-ci`).

@history_item n=2 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (the pipe on the escher server; contract tests 33) — moved the pipe onto `escher-session mcp`: `STAND_PROMPT` names no id or slug, the check is judged and replayed full and ablated, verdict fields and reasons `no-check` / `check-failed` / `check-cannot-fail` were added, `counts_agree` and `wrong_calls` were defined, and no check text reaches an event (four planted markers). `ColdAgentPipeTest` runs under a `claude` shim and a fake `escher-session` with no live model. A verdict `caused` count and a prompt goal extend exactly this judge, the fake and the scrub markers.

@history_item n=3 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six apps by the 5 % bar, the recorded live child) — the one task `stand` became six apps (counter, flight booker, timer, CRUD, cells, load page), still naming no element id or app slug. `passed` gained all six met from accepted answers, the counted capabilities (`lint_calls` · `screenshot_calls` · `advance_calls` · `part_reads` > 0), `recording_kept` and the 5 % wrong-call bar, with reasons `capability-unused` · `recording-not-kept` · `wrong-calls-over-bar`. `server.log` is read only to count span lines, and `record/` is owner-only and never printed. This is the direct precedent for a seventh app, a new prompt goal and a new verdict count: it decides whether the count is required, and how `tasks_met`, the reasons list and `server.log` counting grow.

@history_item n=4 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (served pages' counts, the pipe's contract tests) — `stand_cells_served` and `stand_load_page` joined `run stand` and the `test` / `stand-repeat` legs by their `stand_` prefix with no script change (`run stand` 205 over 50 files, workspace 208 · 1117). `test_cold_agent.py` went to 37, its fake answering Cells paged and the load page, keeping one check per accepted start under `--record`, with 34 of 37 red against the base pipe as the control. `session_common` readers went 29 → 31. A new served app moves these same counts and the fake's per-app answers.

@history_item n=5 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages' boots, `LoadNet`, the host-sink part read) — served pages boot through their own `stand::boot_*` (`boot_cells`, `boot_load` in TaskShell). `net_provider: None` holds for every caller but `boot_load`, and `LoadNet` is a hand-written stand-in. Served-page checks declare `mod session_common;` and no `mod fixtures;`, run in both layout modes, are picked up by prefix and read no clock. `host_log` serves a served page at `RUST_LOG=trace` with needles from `common`. This is the template for the seventh app's boot, its stand file and its `host_log` reading, and it bounds `stand-boots-offline`.

@history_item n=6 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six served slugs through the binary) — `cli_commands` starts each of the six served tasks. `host_binary` and the lifecycle run on the six slugs (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), `main` handing the six slugs and only the timer a time step. The hosted snapshot line sizes were re-measured. The served-task table `TASKS: [(&str, Served); 6]` growing to seven moves these binary checks and the slug list.

@history_item n=7 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the stand's tables re-pinned; `WITNESSES_020` `Subject::Served`) — the shared `controls(task)`, `ENABLED_TAB_ORDER`, `STATE_ROWS` and `stand_contrast` / `stand_keyboard` rows were re-pinned. `stand_requirement_sweep` gained `Subject::Served` witnesses naming a check on a page served beside the lean four. `stand_lint`'s boot findings were re-pinned. A new served page that joins these per-task tables, or the requirement sweep, follows this re-pin pattern.

@history_item n=8 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's rows asserted) — the load page is also booted by `stand_keyboard`'s Tab-sequence test and `stand_contrast`'s ratio test, and `stand_load_page` asserts its rows. `common` states `LOAD_TAB_ORDER` / `CELLS_TAB_ORDER`. This is the precedent for a new served screen's a11y readings (keyboard, contrast) in the a11y-leg files.

@history_item n=9 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint skips a closed select's options) — `stand_lint`'s `at_boot` became `[0, 0, 0, 0]`. The scope requires the new app's boot to read 0 lint findings "as the lean stand screens do", against this pin.

@history_item n=10 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — `stand_snapshot_text` holds each screen under `SNAPSHOT_TEXT_BUDGET` (10,000 bytes) and under an in-file ceiling, with the same text across calls, modes and boots. This is the origin of `snapshot-text-budget`, which the scope's "snapshot of the new app stays under `SNAPSHOT_TEXT_BUDGET`" rests on.

@history_item n=11 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — `contract/driver-session/acting-verbs` gained `caused` where the step caused something, and a panicked exploration path's kept check now holds the panicked step (`contract/session-recording/exploration-checks`). `decision/root-boundary-err-on-own-session` was added: an Err the root boundary takes is checked on a session of its own, because the boundary replaces the app. The new app's Err control must follow this: the root error boundary takes the app, so the Err control's check cannot share a session with the panic and log controls.

@history_item n=12 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (3. Test Harness Contract) — `decision/socket-carries-commands` now carries a step's `caused` with its error text out, ruled by the operator. `contract/driver-session/payload-unread` was added: a panic the hook missed answers the fixed `<payload unread>` text. This is the crossing the new app's authored error text rides, and the fixed-text panic item its panic checks may meet.

@history_item n=13 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (5. Integration Test Strategy) — `root-boundary-err-on-own-session` was restated so that on its own session `caused` carries the error text and its chain. `decision/mcp-result-decoded-whole` was added after the Windows red CI#43940101161: an MCP result compared to the in-process answer is decoded whole through the one shared helper. The new app's MCP checks of `caused` text must use that helper.

@history_item n=14 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (2. Test Strategy; 4. Unit Test Strategy) — added `decision/engine-items-asserted-on-tracing-build`: engine-sourced caused items are asserted only on a tracing build, read through the engine's constant, with the check running on both builds. Also added `decision/expected-value-pinned-as-literal`. A `caused` check on the new app that meets engine warnings must gate them this way, and its controls must pin measured values.

@history_item n=15 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `decision/stand-boots-offline` and `stand-load-page-fixed-answer` gained a second exception, the error-text check's failing page with its `FailingNet` stand-in. Also added `decision/doc-leg-gates-public-doc-edits`: a public doc comment linking a crate-private item went red only in CI's Documentation job. A new served app that keeps a network provider would need another named exception. New public doc comments must pass the doc leg.

@history_item n=16 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the same check read green per package and red in the workspace leg (engine `tracing` compiled in under the workspace build only), which gave `per-package-green-not-workspace-green`. A sink capture counts lines by target, never in total. This is a trap for a `caused` or log check whose engine items depend on the tracing feature.

@history_item n=17 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `host_log` covers the real binary at `RUST_LOG=trace`, needles read from the stand in-process, both streams drained. Trap: a check that must fail red on a leaking sink cannot use `start` on a piped host, and a needle list read from the stand can outgrow a hand-kept one. A logging control on the new app must keep `no-ids-or-names-in-stand-logs` / `escher-session-stderr` true at `trace`.

@history_item n=18 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions call in the span and log checks) — `host_log` and `mcp_log` each drive one call per new read verb with 0 needles at `trace`, and `stand_act_spans` covers every instance verb. These are the latest counts (host_log 9 calls, mcp_log 12, 38 calls per mode) that a new served app's log readings, or authored log messages, would move.

@history_item n=19 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — MCP stdio-child checks through `Speaker` are not unix-gated, bar `mcp_server`'s command-line comparison, while socket checks are. Standing trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test. This bears on the new app's MCP `caused` checks and on `socket-checks-unix-gated`.

@history_item n=20 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — added the command-line test level: the binary run once per command as a client, streams and exit status read through a shared runner, and shell flows under `tests/flows/`, unix only. The new app reached "through the host and the `mcp` role as the other served tasks are" uses this level (`cli-checks-run-own-binary`, `seven-guis-cli-targets-unix-only`).

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (stand checks' shared modules gain the graph's drives) — `GRAPH_DRIVES` in `tests/common/mod.rs` holds each stand task's drives, every conditional write driven once so no stand write reads a flag. A new served app with registered state must add its drive row (`graph-drives-confirm-every-write`), or `stand_graph` / `stand_handler_coverage` read a flag.

@history_item n=22 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the unconfirmed flag's stated limit) — a declared write whose handler writes only on a condition reads `unconfirmed` after runs that skipped it. This is why `GRAPH_DRIVES` drives each conditional write (`unrun-declared-write-reads-unconfirmed`). It binds handlers on the error page that panic or Err before writing.

@history_item n=23 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb held in session; the stand's state through the registry) — `stand_graph` checks every served screen in both modes: no served id holds `{owner}///{key}`, the answer holds no value, and the stand registers task signals through `use_graph_signal`. The new served screen joins these sweeps (`reserved-graph-id-form`, `graph-answer-holds-no-value`, `graph-signal-registration`).

@history_item n=24 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading and the `CLICKABLE` table) — `stand_non_lean_tasks` counts elements reading `clickable` with no role against a 7-row `CLICKABLE` table (`contract/snapshot-node/clickable`). A new app may need a row.

@history_item n=25 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (one actionable rule; `stand_actions`; `ACTIONABLE_AT_BOOT`) — added `decision/one-actionable-rule`. `stand_actions` lists each of the six served screens' actionable nodes, equal to `Session::run` causes, in both modes. `ACTIONABLE_AT_BOOT` counts the six served tasks. A seventh served app extends both tables (`contract/driver-verbs/actions`, `actions-answer-holds-no-value`).

@history_item n=26 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (`stand_handler_coverage` and its tables) — `stand_handler_coverage` runs over the six served tasks with `HANDLERS_AT_BOOT` (six tasks) and `UNEXERCISED_AFTER_DRIVES` (seven), stated by hand and held to the graph's reading. The new app's panic, Err and log handlers join these tables. A handler whose step panics, or whose element is removed by the root boundary, has precedent: a removed element reads exercised.

@history_item n=27 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the stand reading) — the latest stand counts: `run stand` 262 · 0 · 7 over 54 files, `stand-repeat` one `--test` per `stand_*.rs` over 54, joined by prefix with no script change, and the flakiness budget's 54 files. A new `stand_*` file moves these.

@history_item n=28 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — added `decision/cross-os-readings-by-answer-digest`: the six served tasks are read on every CI OS by `stand_cross_os_readings` against one committed digest table. The scope holds the six apps' answers byte for byte under this check, and a seventh served app either joins the table or is named outside it.

@history_item n=29 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — committed checks under `tests/blitz-tests/checks/` are replayed by `stand_replay` and kept byte-equal to what the binary records (`checks-byte-equal-to-recording`). Their scripts hold no `screenshot` step, because a shot's PNG differs by OS (`no-screenshot-in-recorded-check-scripts`). "Kept checks stay byte for byte" rests here.

@history_item n=30 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — added `contract/stand-boot/boot-alone`, `harness-construction/from-component-alone` and `vdom-alone`, plus `decision/component-alone-root-context-before-build` (TimerTicks provided before the build), `pinned-viewport-read-from-document` and `harness-graph-tests-enable-accessibility`. The cold-agent contracts `cold-agent-check/step` and `cold-agent-verdict/isolated` moved to fifteen verbs. These are the latest boot surfaces a new app's headless boot can build on, and the latest verb set for the pipe's fake.

@history_item n=31 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the cold-agent pipe's tools at fourteen) — the pipe's `VERBS`, `valid_step`, `isolated` and the fake `escher-session` in `test_cold_agent.py` move together when the surface moves (test count held at 37). The new app moves the fake's per-app answers rather than the verb list, but the same lockstep applies.

@history_item n=32 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added `decision/count-and-rename-sweeps`: a missed numeric pin and a missed renamed witness set the convention that a count change sweeps every site and a rename sweeps every witness. A served-app count moving from six to seven (`TASKS`, capability statement, README, fake, tables) falls under it.

@history_item n=33 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — origin of `test_capability_statement.py` (`CapabilityStatementTest`): verbs and causes as code spans, schema counts in words, README's tool count, and the claim section naming each check. The scope moves the capability statement's pins where the served-app count or the verdict's fields are held.

@history_item n=34 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — restated `capability-statement-pinned`, `capabilities-doc-pinned-to-schema`, `capability-check-names-held-to-schema` and `capability-statement-from-schema`. The pin now also holds the a11y leg's file count and stand-repeat's named-target count against `ci-leg.sh`, and some witness counts are stated by hand. A new stand file joining the a11y leg or the named stand-repeat targets reddens ci-scripts until `docs/capabilities.md` is re-stated.

@history_item n=35 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (4. Unit Test Strategy) — added `decision/prompt-goal-control-phrase-once`: the prompt test's control did not hold on words shared with the Cells sentence and held only on a unique phrase. The scope names this row for the error-page prompt goal.

@history_item n=36 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (3. Test Harness Contract) — `decision/cold-agent-server-log-counted-only` was shortened to "read only to count its span lines", because the pipe now also counts the command span lines' causes. A `caused` count read from the server child must stay a count: bools and counts only, never text (`cold-agent-events-content-free`, `cold-agent-output-scrubbed`).

@history_item n=37 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — added `decision/requirement-witness-inside-stand-repeat`: every 0.3.0 requirement names its witness inside stand-repeat, pinned by function name in `stand_requirement_sweep`. `typed-password-never-returned` was widened to every answer carrier (guard `stand_masks_on_new_answers`). The v030-10 reach this chunk extends needs a witness row there.

@history_item n=38 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the zero-failure flakiness budget: `stand_*.rs`, the named targets and seven_guis' process checks run three rounds with no retry, and any red ends the leg (`stand-checks-zero-failure-budget`, `zero-failure-stand-repeat`). New stand checks of panics and errors enter that budget.

@history_item n=39 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat` runs the `stand_*` glob plus named `--test` words pinned by `test_stand_repeat_leg_runs_three_rounds`, then seven_guis' process checks. `run stand` keeps its prefix rule. A new non-`stand_` target would have to be named there, and a `stand_` one joins by prefix (`joining-file-keeps-its-legs`).

@history_item n=40 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `contract/agent-run-verbs/run` was restated: stand, all and a file name run `-p blitz-tests`, and session runs `-p seven_guis --test cli_flow`. `contract/agent-run-selection/session`, `agent-run-holds-no-process` and `test-data-is-code` were updated. These are the run selections the new stand checks and any new flow ride.

@history_item n=41 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — `tests/fixtures/mod.rs` was introduced. A page two checks share goes in `fixtures` as an `rsx!` component booted with the stand's options, and a page with one reader stays in its file (`shared-fixture-pages`). This governs where an error-page fixture lives if several checks read it.

@history_item n=42 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — sets the rule that a driver check's failure message names a layout mode, a task and an index, never an id or what a screen reads. The new app's stand checks inherit it, which matters doubly when the screen carries authored error text.

@history_item n=43 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — trap: a headless deleting key is not the same on every platform (the Backspace arm is compiled out on macOS), so a state change is proved with clicks and typed characters. This bears on which ordinary driver steps reach the error page's controls.
