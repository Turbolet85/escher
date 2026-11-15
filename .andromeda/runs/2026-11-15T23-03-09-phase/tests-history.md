@history_item n=1 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — (the agent-run 5-command contract measured) set §3's Agent-run contract the smoke must enter: `bash scripts/agent-run.sh {boot | run {selection} | status | cleanup | logs}` from the repo root, `.ps1` a logic-free pass-through, exit grammar `0` · `1` (build failure, failing run, empty run) · `2` usage checked before · `3` precondition, selections `stand` · `all` · `{name}`, JSON-line events (`boot` · `run.start` · `test` · `run.end` appended; `status` · `cleanup` printed only), state in `target/agent-run/{status.json, events.jsonl, run.log}` only, no daemon/PID/port/env var; `test_agent_run.py` (14 cases under a `cargo` shim) its proof. The three open points (`files` under `all`, the stand-less empty run, usage before not-booted) stand on the operator's ruling — a new smoke verb or entry must keep this grammar, its event schema harness metadata bound to obs-plan §3, and extend the shim-tested contract the scope names as its model.

@history_item n=2 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — scoped the agent-run premise: was «nothing is started, polled or listened on»; now the script itself starts and listens on nothing, while two `run stand` checks start their own session host and socket under `target/tmp/`; added the §3 → Session lifecycle key. Traps recorded: an `#[ignore]` host child blocks a bare `--ignored` run; a child's inherited libtest lines are counted by `agent-run.sh` as tests; a sink-installing host at `trace` overruns an undrained pipe — exactly the hazards a smoke that boots `escher-session` from the agent-run surface meets (rows `decision/agent-run-holds-no-process`, `decision/child-streams-to-null`, `decision/drain-host-pipes-while-running`).

@history_item n=3 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `host_log` runs the real `escher-session` with both streams drained by a thread each, `attach` under a bound and never `start`; trap: a check that must fail red on a leaking sink cannot use `start` on a piped host (the unfixed host filled the pipe before answering). Origin of `decision/drain-host-pipes-while-running` / `decision/sink-host-output-drained` the smoke's runner must honour.

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (three entries: coverage scope; test strategy and harness contract; unit/integration/e2e/CI) — created the command-line level the smoke would reuse: seven_guis `cli_commands`, `cli_flow` (two `sh` flows, assets of a cargo target, no framework added) and `host_timer`, unix only, `tests/flows/` no target, the shared runner (`Ran`, `run`, `client`) and `Started` guard; a host the `start` command spawns is neither killed nor reaped by it; `host_binary`'s nine usage-error argv rows and host role `serve <task> --session <dir>`; `ss-idle` joins agent-run's state directories; the macOS CI leg the only witness of the two shell flows. Origin of rows `decision/cli-checks-run-the-binary`, `decision/shell-flows-are-cargo-test-assets`, `decision/started-host-held-behind-guard`, `decision/cli-and-flow-targets-unix-gated` — the scope's P3 question (reuse `counter.sh` or carry its own) rests on them.

@history_item n=5 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — recorded the gap «no standing check boots `escher-session` on `timer` … booted by hand only» (closed by `host_timer` at 2026-10-10-driver-cli): the precedent of a by-hand boot recorded as a gap and discharged by a standing check, the same shape as this chunk's L1 (the boot-path smoke driven by hand).

@history_item n=6 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — the first «boot smoke» named in test-plan: `RUST_LOG=info timeout 10 target/debug/seven_guis_native` (exit 124) as the stand log format's witness, and the §3 marker narrowed to boot/status/cleanup/logs commands still unmeasured — the precedent of a hand-run boot smoke the plan cites, and the stand log row (`contract/stand-log/stream`) the smoke's stderr must keep.

@history_item n=7 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — a sibling script beside agent-run with the same exit grammar (`2` usage before any precondition, `3` precondition), per-run `mktemp -d` session, `timeout`, state under `target/cold-agent/` only, proven by `test_cold_agent.py` under a `claude` shim, never live in CI — the nearest precedent for a second harness entry point and its contract-test shape.

@history_item n=8 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (§3 pipe and §1/§4 contract tests) — the cold-agent pipe exits 3 for an absent or non-executable `target/debug/escher-session` and builds nothing; `test_cold_agent.py` runs against a fake `target/debug/escher-session` that writes span lines and records its spawns — no live model, credential or built binary; CI-scripts leg 94 → 100. The direct precedent for shimming the session binary in a CI-scripts contract test, and a decided alternative (build vs. require-built) the smoke's shape must choose between.

@history_item n=9 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the counts entry) — `ci-scripts` 108 tests in eight files, `test_cold_agent.py` 37, its fake answering per served task, 34 of 37 red against the base pipe (red-before-green control); `run stand` re-count picked up new files by prefix with no script change. The tally and control discipline a new smoke contract test extends.

@history_item n=10 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the cold-agent pipe's tools at fourteen; the verb table at fourteen) — the pipe's fake `escher-session` speaks fourteen tools; `VERBS` 14, ten instance-level, session-lifecycle's command line fourteen verbs — the current verb set behind `contract/escher-session-command-line/command-form` that a smoke's call must use.

@history_item n=11 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the session contract takes graph as its ninth instance verb) — session-binary's usage line lists `graph` after `lint`; `snapshot`, `screenshot`, `lint`, `graph` run no step and wait on nothing — backs `contract/escher-session-command-line/usage-line` and `contract/driver-session/reading-verbs`, the read a smoke would make after boot.

@history_item n=12 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (cli_commands and the coverage role; the session binary's coverage role; escher-driver's unit tests) — `cli_commands` 18; the usage line gains `coverage role: coverage <app> <check>...`, a misplaced `--record` and `coverage nowhere <dir>` usage errors status 2; the coverage role's endings `0`/`1`/`2`/`3`; its unit tests' scratch files under the workspace `target/tmp`. Latest state of the exit-code and usage-line rows the smoke's verdict maps onto.

@history_item n=13 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the session binary's coverage role (was PROVISIONAL): the `contract/escher-session-command-line/usage-line` and exit-status rows stand as ratified.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** — `cli_commands` gained the 94/95-byte state-directory bound and each cause's `remedy`; statuses from `EXIT_STATUSES` — origin of `contract/session-state/path-bound` / `contract/cli-state-dir/max-length` the scope's relative `target/tmp/…` state dir is chosen against.

@history_item n=15 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `cli_commands`' off-Linux 90/91 bound test with its macOS witness (CI#38642493451) — the second half of the length-bound row a smoke's state dir must fit on every OS.

@history_item n=16 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — added the `stand-repeat` leg, the zero-failure flakiness budget, `cli_commands`' closed-stdout check (`contract/escher-session-exit-codes/closed-stdout`) and `client.rs`'s per-read bound; CI-scripts 79 → 94. A smoke that joins any repeated leg falls under the budget.

@history_item n=17 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part`, paged answer line, usage line opening `snapshot [--part <part>]`, `stand_snapshot_parts`; with **2026-10-06-compact-snapshot-serialization**'s `SNAPSHOT_TEXT_BUDGET` (10,000 bytes) it backs `decision/snapshot-text-budget` and `contract/escher-session-command-line/snapshot`.

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (both entries) — wire `v3`, `start <task> [--record <dir>]`, refused directory status 3 with nothing spawned, fifteen session errors, recording checks `cfg(unix)`-gated item by item beside an off-unix `Unsupported` check the windows job alone runs — origin of `contract/session-recording/off-unix`, `contract/escher-session-command-line/start` and `decision/cli-and-flow-targets-unix-gated`'s recording clause; a smoke script that boots the binary must gate or refuse off unix the same way.

@history_item n=19 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (both entries) — `tests/flows/` gains its third script `record_checks.sh`, run through the binary by seven_guis `recording`; a shot's PNG bytes differ between linux and macOS, so a check compared across OSes holds no `screenshot` step (`decision/checks-hold-no-screenshot-step`) — constrains which call the smoke runs if its verdict is compared across hosts.

@history_item n=20 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — socket checks `cfg(unix)`-gated, the MCP stdio-child checks not (run on windows) — origin of `decision/socket-checks-unix-gated` / `decision/socket-session-checks-unix-gated` with its `except`; the smoke holds a session over the socket.

@history_item n=21 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six served slugs through the binary) — `cli_commands` starts each of the six served tasks; `host_binary` and the lifecycle run on `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`; the hosted snapshot line sizes on the four lean tasks — the named stand tasks a smoke may boot.

@history_item n=22 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — measured the test-data bootstrap (fixtures compiled into each target by `agent-run.sh boot`) and decided no `run fixtures` selection on the operator's «Existing selections (Recommended)» — origin of `contract/agent-run-selections/all`; a precedent that new selections are refused in favour of existing ones.

@history_item n=23 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (`stand-repeat` runs ten named targets) — `run stand` keeps its prefix rule on the founder's «Список в скрипте», the leg's named words pinned by `test_stand_repeat_leg_runs_three_rounds` — a precedent for how a selection's membership is changed and pinned.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`run stand` 237 and `run all` 727; the open list through the process checks) — `run {file}` the non-stand readers; host-driving checks name state dirs like `cc-popup` / `hl-popup` under `CARGO_TARGET_TMPDIR`, `#[cfg(unix)]` as every host-driving test — state-dir naming and gating precedent.

@history_item n=25 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the stand reading with stand_handler_coverage) — newest Proof: `run stand` 262 · 0 · 7 over 54 files, picked up by prefix with no script change; stand-repeat over 54 files — the readings an agent-run script change must re-measure; with **2026-11-04-dioxus-runtime-probe**'s `run all` 743 · 0 · 12 the newest `run all`.

@history_item n=26 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — trap: the §3 Proof series stands as a master site for `run all`, and a sweep pattern missed it; a chunk that edits `agent-run.sh` must find and extend that series.

@history_item n=27 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — each linux leg runs `bash .github/scripts/ci-leg.sh {leg}`, the same command the host runs; the `ci-scripts` leg and `LegScriptTest` with a cargo shim — origin of `decision/ci-legs-host-reproducible`, `decision/ci-scripts-unit-tested`, `decision/workspace-tests-in-fast-ci-leg` and `decision/native-platform-ci-matrix`; the scope's «not a CI leg of its own» is read against these.

@history_item n=28 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — `LegScriptTest`'s shim checks each leg; trap: `cargo llvm-cov report` without `--workspace` exits 0 with an empty table — a shim-tested leg can read green over nothing, the empty-run rule's cousin.

@history_item n=29 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (every cargo leg `--locked` but `fmt` too) — `decision/cargo-legs-locked`'s exceptions are `fmt` and `examples/wasm_hello`; a smoke that builds the binary must run `--locked`.

@history_item n=30 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`; the CI scripts' count) — precedent for adding a step to `fast` with no CI job of its own, pinned last in `FAST_LEGS` by `test_fast_leg_ends_on_the_font_embolden_witness`; `ci-scripts` 110 tests in eight files, test_ci_workflows.py 35 — the latest tally a new smoke contract test moves.

@history_item n=31 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (the CI-scripts leg runs 78 tests over six files) — a CI-scripts test reading TOML via `tomllib` needs Python ≥ 3.11, the runner's version unread; the per-file tally convention the leg keeps.

@history_item n=32 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** (the install script's tests) — `InstallScriptTest` runs a script in a session of its own against stand-in `sudo`/`apt-get` with the real `timeout`, bad calls exiting 2 before anything runs, four mutation controls — a shim pattern for a script whose bounded external calls must be proven without the real tool.

@history_item n=33 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — trap: a green per-package run is not the workspace leg's green (blitz-dom's `tracing` compiled in only under the workspace build) — matters if the smoke builds `escher-session` per package while CI's legs build the workspace.

@history_item n=34 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — `decision/mutation-controlled-tests` now: a new check is controlled (a named mutation turns it red, the restored tree green), a control removing a bound loosens it rather than removing it — binds the smoke's contract test; also `+ contract/session-recording/exploration-checks` in the session-recording area the rows touch.

@history_item n=35 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — `+ decision/cross-os-readings-by-answer-digest` and `~ decision/stand-checks-bundled-font` guard `stand_bundled_font_families` in §2 Test Strategy — the cross-OS reading strategy beside the stand rows this chunk's area holds.

@history_item n=36 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows (601 rows), each verified against the code; later amendments name rows by handle — the form this chunk's amendment (to test-plan §3's agent-run contract, at the wrap) will take.

@history_item n=37 marker=2026-10-24-u55-door
**2026-10-24-u55-door** — test-plan split into its body and 9 category files (map `.andromeda/test-plan.map.tsv`) — where the §3 agent-run contract and its leaves live for the wrap's cascade the scope names.
