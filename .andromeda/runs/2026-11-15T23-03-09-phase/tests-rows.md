# test-plan · the rows that bind 2026-11-15-session-binary-smoke — picked 128 of 605 (92 by every draw) · rule any · pin 1209

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/ci-scripts-unit-tested · CI Python scripts unit-tested — The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg, contract tests of the CI install, coverage floor and blitz-tests manifest scripts among them. · guard: ci-scripts leg
decision/native-platform-ci-matrix · Native platform CI matrix — CI tests on windows and macos and only builds on ios and android; linux is tested by the fast test leg, not the matrix.
decision/workspace-tests-in-fast-ci-leg · Workspace tested in the fast CI leg — Every CI run tests the whole workspace with a locked lockfile on ubuntu with default features, as the fast test leg.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/owner-only-session-files · Owner-only session socket and recordings — A session's state directory is 0700 and its socket 0600, and a recording directory open to its group is refused with nothing written. · guard: stand_session_lifecycle, stand_session_recording
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — A host that served its session and stopped exits 0, with nothing on stdout and no state directory left.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — An argument given to actions is refused with status 1.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — An argv that is no command ends as a usage error, its usage line holding nothing the caller typed, nothing booted and no state created.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — A session error: a host that cannot serve, no session, or a state directory too long, the message holding no path.
contract/session-state-directory/length-bound · session state directory · length bound — A state directory path of 94 bytes holds a session on Linux and of 90 bytes on other targets; one byte more is refused state-dir-too-long with its bound.
decision/snapshot-text-budget · Snapshot text within a byte budget — A screen's snapshot text is read whole when inside SNAPSHOT_TEXT_BUDGET (10,000 bytes); a larger screen is read in parts, each within the budget, the parts concatenating to the whole text. · guard: stand_snapshot_text, stand_snapshot_parts
contract/escher-session-exit-codes/closed-stdout · escher-session exit codes · closed stdout — A run whose stdout is closed before it starts keeps its ending's status, printing no panic and creating no state directory.
contract/session-recording/off-unix · session recording · off unix — Off unix, --record is refused Unsupported.
contract/session-recording/same-check-on-both-surfaces · session recording · same check on both surfaces — The command line and the mcp role keep the same check byte for byte, the state directory gone after stop.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/check-child-streams-to-null · Check child streams sent to null — A check's child process streams are sent to null unless the check reads them. · why: so the child's libtest lines never reach the agent run script's count
decision/check-host-state-dir-removed · Check host state directory removed — A check's host state directory is removed at stop.
decision/cli-checks-run-own-binary · Command-line checks run the package's own binary — seven_guis' command-line checks run the package's own escher-session binary once per command as a client process, reading its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
decision/failed-check-host-reaped · Host of a failing check reaped — A guard kills and reaps a host that a failing check still holds.
decision/session-check-owns-its-host · A session check owns its host — A check that needs a session process starts and stops its own host, with every wait bounded.
decision/seven-guis-cli-targets-unix-only · seven_guis command-line targets gated to unix — seven_guis' command-line and shell-flow test targets are gated to unix.
decision/shell-flows-under-cargo-test · Shell flows run under cargo test — The POSIX sh flow scripts are assets of a cargo test target, not a runner of their own, and no test framework is added for them.
decision/sink-host-output-drained · Output of a sink-installing host drained — A check capturing a host that installs escher's sink drains its pipes while the host runs, or captures to a file, and does not wait for exit before it reads.
decision/socket-session-checks-unix-only · Socket session checks gated to unix — A check holding a session over the socket is gated to unix.
decision/stand-checks-bundled-font-no-skip · Stand checks on the bundled font, no skip — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/check-owns-its-host-process · A check owns its host process — A check that needs a session process starts and stops its own host, every wait bounded, with a guard that kills and reaps a host a failing check still holds and its state directory removed at stop.
decision/child-streams-to-null · Child streams sent to null — A host child's streams are sent to null unless the check reads them. · why: So the child's own libtest lines never reach the parent's output, where the agent run script would count them as tests.
decision/cli-and-flow-targets-unix-gated · Command-line and flow targets unix-gated — The command-line check target and the shell flow target are gated to unix; the recording target is gated to unix item by item, beside an off-unix check that runs elsewhere.
decision/cli-checks-run-the-binary · Command-line checks run the binary — The seven_guis command-line checks run the escher-session binary once per command as a client process and read its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
decision/drain-host-pipes-while-running · Host pipes drained while the host runs — A check that captures a host installing escher's sink drains its pipes on threads from the spawn on, or captures to a file, and never waits for exit before reading. · why: A sink that leaks again can write far more than a pipe holds, so a check that must fail red cannot wait for exit.
decision/shell-flows-are-cargo-test-assets · Shell flows are cargo test assets — POSIX sh flow scripts run under plain sh from a Rust cargo test target; the scripts are assets of that target, not a runner of their own, and no test framework is added.
decision/socket-checks-unix-gated · Socket session checks are unix-gated — A check holding a session over the socket is gated to unix. · except: The MCP checks speak to the mcp child over piped stdin and are not unix-gated, bar the MCP server's command-line comparisons, so the windows CI job runs them.
decision/stand-checks-bundled-font · Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · guard: stand_bundled_font_families · except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
decision/started-host-held-behind-guard · Started hosts held behind a guard — The seven_guis checks hold a session spawned by the binary's start command behind a guard that stops it. · why: The start command closes the host's streams and neither kills nor reaps it.
decision/shared-test-modules-not-targets · Shared test modules are no target — Shared test modules, the flow scripts directory and the committed checks data directory are no test target; checks read shared modules through mod declarations.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/agent-run-events-are-harness-metadata · Agent-run events are harness metadata — Agent-run's events are harness metadata, not an escher-telemetry sink.
decision/agent-run-events-json-by-python · Agent-run events encoded by python3 json — Agent-run's JSON-lines events are encoded by python3's json module embedded in the script.
decision/agent-run-holds-no-process · Agent-run holds no process — The agent-run script starts, polls and listens on nothing (no daemon, PID file, status endpoint, port or socket) and reads no environment variable of its own; the stand checks it runs boot in process. · except: stand_session_lifecycle and stand_session_quiet start and stop a session host process and its Unix-domain socket, a surface of the session library, not of this contract
decision/empty-run-is-never-a-pass · An empty run is never a pass — A run's outcome is passed only when cargo exited 0, no test failed and at least one test line parsed; otherwise it is failed, exit 1.
decision/one-agent-run-driver-at-a-time · One agent-run driver at a time — One agent-run driver runs at a time: a killed run leaves running behind, which the next run overwrites.
decision/test-output-never-reaches-events · Test output never reaches agent-run events — Nothing between a failures: line and the next test result: line is read, so captured stdout and panic text never reach an agent-run event.
contract/agent-run-events/boot · agent-run events · boot — boot {ts, outcome: ready|failed, cargo_exit, head}, printed to stdout and appended to events.jsonl.
contract/agent-run-events/run-end · agent-run events · run.end — run.end {ts, selection, passed, failed, ignored, cargo_exit, outcome: passed|failed}, printed to stdout and appended to events.jsonl.
contract/agent-run-events/run-start · agent-run events · run.start — run.start {ts, selection, files}, files the --test stems passed ([] under all), printed to stdout and appended to events.jsonl.
contract/agent-run-events/status · agent-run events · status — status {booted, boot_ts, head, run: {selection, state: none|running|passed|failed, passed, failed, ignored}}, printed only.
contract/agent-run-exit-codes/0 · agent-run exit codes · 0 — Success.
contract/agent-run-exit-codes/1 · agent-run exit codes · 1 — The verb ran and failed: a cargo build failure (a failing boot, leaving status not-ready), a failing run, an empty run.
contract/agent-run-exit-codes/2 · agent-run exit codes · 2 — Usage: a missing or unknown verb, a wrong argument count, an unknown or pattern-failing selection; a usage error is checked before not-booted.
contract/agent-run-exit-codes/3 · agent-run exit codes · 3 — A precondition is unmet: not booted (status, run and logs before boot), or logs with no events file.
contract/agent-run-selection/name · agent-run run selection · {name} — {name} runs one file, matching ^[a-z0-9_]+$ with tests/blitz-tests/tests/{name}.rs present; any other selection exits 2.
contract/agent-run-state/files · agent-run state · files — Agent-run's state is target/agent-run/{status.json, events.jsonl, run.log} only, gitignored under target/.
contract/agent-run-verbs/boot · agent-run verbs · boot — boot recreates target/agent-run/ with a fresh events.jsonl, builds every blitz-tests test binary (--locked --no-run, output to run.log), writes status.json (booted, boot_ts, head) and emits the boot event; exit 0 ready, 1 failed.
contract/agent-run-verbs/cleanup · agent-run verbs · cleanup — cleanup removes target/agent-run/ if present and prints the cleanup event; idempotent, exit 0 on a clean tree, touching nothing else under target/.
contract/agent-run-verbs/logs · agent-run verbs · logs — logs prints events.jsonl verbatim, one JSON object per line, exit 3 when it is absent; raw cargo output stays in run.log, never printed.
contract/agent-run-verbs/run · agent-run verbs · run — run {selection} runs cargo test -p blitz-tests --locked with merged output to run.log, truncated at the run's start, sets status.json's run state running then passed or failed, and parses libtest's pretty lines into events.
contract/agent-run-verbs/status · agent-run verbs · status — status prints one status object from status.json and builds or runs nothing; exit 0 booted, 3 not (no status.json, or booted false).
contract/agent-run/invocation · agent-run · invocation — bash scripts/agent-run.sh {boot | run {selection} | status | cleanup | logs}, run from the repository root.
contract/agent-run/windows-entry · agent-run · agent-run.ps1 — The Windows entry passes every argument unchanged to the bash script and exits with its code, holding no contract logic.
contract/cli-command/stdout · escher-session commands · stdout — A command's stdout is its answer and nothing else: one line of JSON, with no ESC byte.
contract/host-role/stdout · escher-session host role · stdout — The host role writes nothing to stdout.
contract/stand-boot/boot · seven_guis headless stand · stand::boot — stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.
decision/python-script-tests-under-unittest · Python script tests under unittest — The Python tests of the CI scripts run under the standard library's unittest, by discovery over the CI scripts directory.
contract/agent-run-events/cleanup · agent-run events · cleanup — cleanup {outcome: done}, printed only.
contract/agent-run-events/test · agent-run events · test — test {file, test, outcome: ok|failed|ignored}, one per libtest line, file the target binary's stem with its hash stripped or doc:{crate}; printed to stdout and appended to events.jsonl.
contract/agent-run-selection/all · agent-run run selection · all — all runs the whole package, with no --test.
contract/agent-run-selection/stand · agent-run run selection · stand — stand passes one --test per tests/blitz-tests/tests/stand_*.rs present; with none present it is an empty run, no cargo call, exit 1.
contract/agent-run-selections/all · agent-run selections · run all — Runs every reader of the blitz-tests crate, the stand readers included; there is no `run fixtures` selection.
contract/agent-run-selections/file · agent-run selections · run {file} — Runs one named non-stand reader.
contract/agent-run-selections/stand · agent-run selections · run stand — Runs the `stand_*` readers of the shared module.
contract/cli-command/stderr · escher-session commands · stderr — An accepted command's stderr is empty with RUST_LOG unset.
contract/stand-boot/boot-cells · seven_guis headless stand · stand::boot_cells — stand::boot_cells(HarnessOptions) boots Cells as Task::Cells in TaskShell, with no LeanTask.
contract/stand-boot/boot-load · seven_guis headless stand · stand::boot_load — stand::boot_load(HarnessOptions) boots the load page, TaskShell titled Load, its options' net_provider replaced by the stand's LoadNet and every other field kept.
contract/stand-boot/boot-timer · seven_guis headless stand · boot_timer — boot_timer(HarnessOptions) boots the Timer as a fresh VirtualDom over from_vdom rendering the task in TaskShell, and returns the harness with its TimerTicks.
contract/stand-log/stream · Stand log · stream — The two seven_guis binaries, seven_guis_native and escher-session, write their log to stderr only, with no ANSI.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/agent-run-events-content-free · Agent-run output carries no panic text — A failing agent run puts no captured panic text in stdout or the events file, and no event carries a scrub-set key; the raw cargo output goes only to the run log. · guard: test_agent_run.py

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
contract/driver-argument-kinds/name · Driver argument kinds · ArgKind::Name — A name argument is 1 to 32 bytes of `a-z`, `0-9` and `-`.
contract/driver-cli/refused-line · Driver command line · refused line — A command line that is no command, or that the schema refuses, runs nothing.
contract/driver-verbs/level-check · Driver verbs · level check — A verb is checked at its own level only: a session-level verb given as an instance call is refused `unknown-verb` before its arguments are read; the session commands and `help` are admitted at session level, an argument to `help` refused `malformed`.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
contract/session-host/non-unix · session host · non-unix arms — Off unix the session host's arms return `Unsupported`.
contract/driver-session/held-state · driver session · held state — One session booted through the stand holds state across commands and reads as a fresh boot, in both layout modes.
contract/driver-session/run · driver session · run — A call run through the session returns after settle with the diff of the snapshots taken before and after it.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/session-state-owner-only · Owner-only session state — The session state directory is created with mode 0700 and its socket with mode 0600. · guard: stand_session_lifecycle
contract/session-errors/alreadyrunning · session errors · AlreadyRunning — Returned by start while a session is already up.
contract/session-errors/dead · Session errors · Dead — A socket file that nothing answers on, as after the session host is killed.
contract/session-errors/nosession · session errors · NoSession — Returned when a session is reached and none is running.
contract/session-errors/unsupported · Session errors · Unsupported — Returned by the session arms on a non-unix target.
contract/session-state-directory/session-sock · the session state directory · session.sock — The state directory holds session.sock alone; a kept recording lives in the launcher's recording directory.
contract/session-run/diff · escher_driver Session · run — A call run through `Session::run` on a held session returns after settle with the diff of the snapshots taken before and after it.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/cli-checks-through-own-binary · Command-line checks through the package's own binary — Each escher-session driver command is run as a process of the package's own binary against a host its start command spawned.
decision/ui-driven-through-session-binary · App UI driven through the session binary — No test in the apps launches a window; the UI is driven through the escher-session binary.
decision/checks-hold-no-screenshot-step · No screenshot step in committed checks — The scripts behind the committed checks hold no screenshot step. · why: a shot's PNG bytes differ by OS

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
contract/cli-exit-codes/1 · escher-session CLI exit codes · 1 — A call a target cannot take is refused with its cause.
contract/cli-exit-codes/2 · escher-session CLI exit codes · 2 — A line that is no command, a misplaced `--record` or a malformed `coverage` role call is a usage error.
contract/cli-exit-codes/3 · escher-session CLI exit codes · 3 — A session error: a command that finds no session, a state directory too long, or a group-open recording directory.
contract/cli-snapshot/budget · escher-session CLI snapshot · size budget — A snapshot answers the in-process text as one line inside its 10,000-byte budget.
contract/cli-state-dir/max-length · escher-session CLI state directory · length bound — A state directory of up to 94 bytes on Linux, 90 bytes off Linux, starts and stops; a longer one ends `state-dir-too-long`, status 3, its message naming the bound and no path.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/stand-boots-offline · Stand boots offline — Every headless stand boot keeps no network provider, and no real network is used by the stand. · guard: the_stand_options_stay_offline · except: The stand load page's boot, which supplies the self-answering LoadNet stand-in, answering one fixed sheet from its own thread after a delay.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/hand-written-stand-ins · Hand-written stand-ins, no mocking library — Test doubles are hand-written stand-ins; no mocking library is used in CI, the build configuration or the apps.
decision/stand-pinned-real-font · Stand checks keep a real pinned font — The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello, which has no Cargo.lock
decision/ci-legs-host-reproducible · Host-reproducible CI legs — Every linux CI job runs one leg of a shared leg script that the dev host also runs, so every linux leg is reproducible on the dev host.
decision/font-assertions-on-system-fonts · Font-dependent assertions on system fonts — Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested. · except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.

## session-lifecycle · key of ## 3. Test Harness Contract
decision/session-checks-own-their-host · Session checks own their host — A check that needs a session process starts and stops its own host, bounds every wait, holds the host behind a guard that kills and reaps it, and sends the child's streams to null unless it reads them.
decision/socket-session-checks-unix-gated · Socket session checks unix-gated — A check holding a session over the socket is `cfg(unix)`-gated; a check driving the `mcp` stdio child bounds every read, kills and reaps the child in its guard's drop and is not unix-gated, so the windows job runs it. · except: The `mcp_server` command-line comparison is unix-gated.
contract/escher-session-command-line/command-form · escher-session command line · command form — `escher-session <verb> … --session <dir>` is the whole command line, fourteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare).
contract/escher-session-command-line/start · escher-session command line · start — `start <task> [--record <dir>]`, the task one of `counter` · `flight-booker` · `timer` · `crud`; with `--record` the recording directory is checked before anything is spawned, a refusal status `3` with nothing spawned.
contract/escher-session-exit-status/0 · escher-session exit status · 0 — Accepted: the answer as one uncoloured line of JSON on stdout.
contract/escher-session-exit-status/1 · escher-session exit status · 1 — Refused: the refusal on stdout.
contract/escher-session-exit-status/2 · escher-session exit status · 2 — Usage: one fixed line on stderr with stdout empty, nothing booted and no state created.
contract/escher-session-exit-status/3 · escher-session exit status · 3 — Session error: the error object on stdout and its fixed message on stderr; `RecordDirNotPrivate` and `RecordingFailed` end as session errors.
contract/session-errors/already-running · Session errors · AlreadyRunning — A start where a session already answers.
contract/session-errors/error-set · Session errors · error set — Fifteen session errors, each stating a kind, a fixed meaning and a remedy; the state directory's message states its bound.
contract/session-errors/no-session · Session errors · NoSession — No socket file in the state directory.
contract/session-errors/timeout · Session errors · Timeout — An answer read past its 30 s per-read bound.
contract/session-host/start · Session host · start — Spawns the host command and returns its first answer.
contract/session-host/stop · Session host · stop — Returns when the state directory is gone, bounded at 5 s.
contract/session-state/directory · Session state directory · directory — A directory the caller names, mode `0700`, holding `session.sock` alone while a session is up and absent after `stop` or the host's idle expiry.
contract/session-state/path-bound · Session state directory · path bound — A socket address leaves about a hundred bytes for the whole path: 94 bytes on Linux, 90 elsewhere, a longer one refused with that bound stated.
contract/session-state/socket · Session state directory · session.sock — The session's socket, mode `0600`.
contract/driver-session/reading-verbs · Driver session · reading verbs — `snapshot`, `screenshot`, `lint` and `graph` run no step and wait on nothing.
contract/driver-session/run-validation · Driver session · run — A call is validated first, and a refused call has run nothing.
contract/driver-session/snapshot · Driver session · snapshot — Returns the screen's text, or, for a screen over the snapshot budget or a `part` asked, one part's text with the counts `part` and `parts`.
contract/escher-session-command-line/no-echo · escher-session command line · caller input — No ending writes back anything the caller typed.
contract/escher-session-command-line/serve · escher-session command line · serve — The host role is `serve <task> --session <dir> [--record <dir>]`; it ends with nothing written and status `0` after a stop or an expiry, and `3` on a session error.
contract/escher-session-command-line/snapshot · escher-session command line · snapshot — `--part` is a whole number 1 to 1000; a paged answer is one line `{"text":…,"part":n,"parts":m}`, status `0`; a whole screen still answers `{"text":…}` alone.
contract/escher-session-command-line/two-argument-form · escher-session command line · two-argument form — The two-argument form `<task> <state-dir>` reads as an unknown verb, refused, status `1`.
contract/escher-session-command-line/usage-line · escher-session command line · usage line — The usage line reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]` and `coverage role: coverage <app> <check>...`, listing `graph` after `lint` and `help` after `stop`.
contract/escher-session-error-object/fields · escher-session error object · fields — A session error object carries `kind`, `message` and `remedy`.
contract/session-host/call · Session host · call — Sends one call and returns `Answer { accepted, json }`; the call is validated first and a schema refusal is answered without reaching a session.

# test-plan · its absences, names and overviews, whole

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
> NOT YET MEASURED — the reading gathered no tier justification, critical-path list or coverage triggers for this summary

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
> NOT YET MEASURED — windowed or browser-driven end-to-end runs lay outside every slice read

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
absence/windowed-end-to-end-tests — An automated test that launches a window

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
absence/mocking-library — A mocking library

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
> NOT YET MEASURED — a performance budget: none is set

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## 12-test-decisions-log · ## 12. Test Decisions Log
> NO RECORDED INTENT

## session-lifecycle · key of ## 3. Test Harness Contract
overview/session-lifecycle — The session checks drive one headless app instance held by one process, and a lifecycle another process drives, with the calls it sends, over a Unix-domain socket. The session is not part of the five-command agent-run contract, which starts and binds nothing.
