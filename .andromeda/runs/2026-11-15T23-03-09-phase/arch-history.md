@history_item n=1 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — the agent-run contract and its state area registered — registered `bash scripts/agent-run.sh {boot | run {stand|all|name} | status | cleanup | logs}` driving `cargo test -p blitz-tests --locked`, exit grammar `0` · `1` failed (empty run included) · `2` usage, checked first · `3` precondition, python3-`json` JSON-line events, `scripts/agent-run.ps1` a pass-through, pinned by `test_agent_run.py` in the `ci-scripts` leg; `target/agent-run/` (`status.json`, `events.jsonl`, `run.log`, raw and unscrubbed) recreated by `boot`, removed by `cleanup`, local only, never uploaded. This is the exact contract the smoke must be reachable through (rows contract/agent-run-events/*, contract/agent-run-exit-codes/*, contract/agent-run-state/state-directory, decision/windows-entries-pass-through, decision/run-state-areas-local-only): a new verb or event joins a closed set and its exit grammar, and its pin is `test_agent_run.py`.

@history_item n=2 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — the cold-agent pipe registered beside agent-run — the second project-authored pipe on the same pattern (`bash scripts/cold-agent.sh {run | status | cleanup | logs}`, exits `0`/`1`/`2` usage before any precondition/`3`, events `run.start` · `run.end` · `status` · `cleanup`, pinned by `test_cold_agent.py` in `ci-scripts`, `target/cold-agent/` local only and uploaded by no CI job). Precedent for how a pipe that spawns processes is registered (Process-wide state, Filesystem) with no port or listener; the cold-agent pipe is out of this chunk's scope and must not be touched.

@history_item n=3 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe on the escher server (contract and resources) — the pipe exits 3 when `target/debug/escher-session` is absent or not executable, building nothing; `test_cold_agent.py` runs under a `claude` shim and a fake `escher-session`; its live child is `escher-session mcp` with `RUST_LOG=info` set on that child only and stderr to a state-area log never printed. The closest precedent for a script that drives the session binary: how a missing binary maps to precondition exit 3, and how a CI-scripts test pins it with a fake binary/shim rather than anything live — the shape this chunk's contract test must take (scope: «a `cargo` shim or equivalent»). Note the smoke is asked to build the binary, unlike cold-agent.

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the driver session registered: `escher-driver`, its socket, the `escher-session` host — introduced the one local listener (`{state_dir}/session.sock`, `0600` in a `0700` caller-named state directory, unix only, `Unsupported` elsewhere), the socket then directory removed non-recursively at `stop`, the checks' state directories under `target/tmp/`, `host_binary` spawning the real binary, and `start` not checking the answering pid is its child. Grounds rows decision/session-over-unix-socket, decision/one-local-unix-listener, decision/session-state-dir-caller-named-private, decision/session-state-dir-removal, contract/session-client/start: the smoke's session must use exactly this socket and a caller-named owner-only dir, adding no listener.

@history_item n=5 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the socket carries a call; a hosted session expires; the `escher-session` CLI — widened the socket to `hello` · `stop` · `call` (wire then v2), added `IDLE_EXPIRY` 1,800 s for a command-line host, `start` spawning the running binary through `current_exe()` with streams closed, neither killing nor reaping it; the CLI became `escher-session <verb> [arguments] --session <dir>`, one line of JSON on stdout, exits `0` accepted · `1` refused · `2` usage · `3` session error, the two-argument form reading refused `unknown-verb`; seven_guis gained `cli_flow` with `tests/flows/` POSIX `sh` scripts run under plain `sh` against the built binary (no dependency or test framework added). This is the CLI grammar the smoke drives and the `cli_flow`/`counter.sh` precedent P3 must weigh for reuse; a smoke that skips `stop` leaves a host running until idle expiry.

@history_item n=6 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — wire v3, fifteen session errors, `--record`, the eighth seven_guis target — the wire is now `v3`, `SessionError` fifteen kinds (`RecordDirNotPrivate`, `RecordingFailed` added), `start <task> [--record <dir>]` spawns the host with `--record`, `recording` joins `stand-repeat`'s seven_guis list, and the checks' scratch lives under `target/tmp` via compile-time `CARGO_MANIFEST_DIR` (test-only, no crossing). Rows contract/session-wire/*, contract/session-errors/*, contract/command-line/start read this form; a relative `target/tmp/…` state dir for the smoke follows the same scratch convention.

@history_item n=7 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the committed checks and the third flow script — `tests/flows/` holds three scripts (`counter.sh`, `flight.sh`, `record_checks.sh`), `record_checks.sh` run by seven_guis `recording` into `rc-fresh`; the committed checks under `tests/blitz-tests/checks/`. The scope's P3 premise names these flows as reuse candidates; any edit to them moves `cli_flow`, `recording` and the committed checks together.

@history_item n=8 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the headless stand serves Cells and a load page, the command line six tasks — `escher-session` serves exactly six slugs `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` (`TASKS`, from which the usage line's `apps:` is written); every boot offline but the load page's, answered in process by `LoadNet` after 200 ms on a `.test` URL, its one thread per fetch. Grounds contract/escher-session/invocation, contract/headless-stand/*, decision/stand-offline, decision/stand-load-page-in-process-url: the smoke's «named stand task» must be one of these six, and a `load` smoke would exercise the bounded load wait with no network.

@history_item n=9 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — the `ci-scripts` leg's Python 3.11 floor; blitz-tests one target per file — `test_blitz_tests_targets.py` uses stdlib `tomllib`, so the `ci-scripts` leg needs Python 3.11 or later, no package added; blitz-tests keeps one test target per file with upstream's `all` at `test = false` (`BlitzTestsTargetsTest`). Grounds decision/ci-scripts-python-floor, decision/ci-scripts-stdlib-python and decision/blitz-tests-single-test-target: the smoke's contract test and any embedded python stay stdlib (plus PyYAML) on 3.11+, and `agent-run.sh run <name>` names a per-file target.

@history_item n=10 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — fork CI through one leg script; one dev-profile debuginfo level — `ci-leg.sh {leg}` with legs incl. `test`, `ci-scripts`, `fast` (the local pre-push gate), every cargo leg `--locked`, logs tee'd to `target/ci-logs/{leg}.log`, unknown leg exit 2; `[profile.dev] debug = "line-tables-only"`, inherited by `test`, one level for host and CI (the P4 fork chose a workspace stanza over a CI-only env var). Grounds decision/cargo-legs-locked, decision/locked-cargo-legs, decision/ci-contract-pinned, decision/dev-line-tables-debuginfo, decision/one-debuginfo-level, decision/linux-tested-by-fast-leg: a smoke that builds `escher-session` should build `--locked` under the dev profile and adds no profile env var.

@history_item n=11 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the `stand-repeat` leg; the test-only listener and `TMPDIR` — `stand-repeat` runs every `stand_*.rs` target three rounds then seven_guis' process checks, the first red ending the leg with no retry (zero-failure budget, pinned by `test_stand_repeat_leg_runs_three_rounds`); escher-driver's test-only `UnixListener` bind under the system temp dir registered as no crossing, `TMPDIR` read in test code only. Grounds decision/stand-repeat-zero-failure-budget and decision/stand-repeat-zero-failure: the scope says `stand-repeat` and `test` already run `cli_flow`, so the smoke is not a CI leg unless a leg must run it — and any leg it joins inherits the zero-failure, no-retry rule.

@history_item n=12 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat`'s named list and two state directories — `stand-repeat` names blitz-tests targets as literal `--test` words after the `stand_*` glob, pinned by `test_stand_repeat_leg_runs_three_rounds` and read by `stand_requirement_sweep`; `host_log` runs the binary as a client (`start crud --record` through `stop`, `RUST_LOG=trace`) in `hl-client`/`hl-client-rec` under `target/tmp/`, registered as test-only state dirs. Precedent for driving the binary end to end as a client and for registering each new state directory the smoke creates.

@history_item n=13 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — `stand-repeat`'s membership held jointly — the sweep test reads `ci-leg.sh` whole by `contains`, so it cannot see a target dropped from one leg while another names it; membership is held jointly with `test_stand_repeat_leg_runs_three_rounds`, the gap CARRY'd on «Stand requirement sweep». Trap if the plan touches `ci-leg.sh` or `stand-repeat` membership: a single-site pin can read green over a dropped target.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — the legs that do not run locked — corrected the CI contract to «every cargo leg but `fmt` and `examples/wasm_hello`» runs `--locked` (an older reading found false against `ci-leg.sh`). The except cell of decision/cargo-legs-locked and decision/locked-cargo-legs comes from here; a new cargo invocation in a script follows `--locked`.

@history_item n=15 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — the `embolden` step of `fast`; the test-only binds and no served port — `fast` is now fmt → clippy → test → ci-scripts → embolden, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`; «no network port» restated as no served TCP/UDP port, test-only binds three, each registered as test-only. Any change to the leg list or `fast` order has a pin in `test_ci_workflows.py`; the smoke adds no listener (absence/served-network-port, absence/network-listener).

@history_item n=16 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the `ci-scripts` leg discovers eight test files; the capability-statement pin — the leg's discovered test-file count is a stated fact (was seven, six), and the capability pin reads README's «the {word} driver commands as tools». A new test file in `.github/scripts/` (if the smoke's contract test is not folded into `test_agent_run.py`) moves that count, and README/harness-doc edits must keep the pinned phrase.

@history_item n=17 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — the front page's command-line clause and Try it line — README names the six served task slugs and the Try it line `target/debug/escher-session lint --session demo-session`; its pinned phrase moved lines once (citations re-pointed). The harness documents this chunk updates to name the smoke sit beside this front-page text and its pin.

@history_item n=18 marker=2026-10-12-self-description
**2026-10-12-self-description** — fixed program text on the ratified carriers, ratified; `help` and exit statuses from tables — the command line's stdout and the `mcp` role carry the program's own `'static` table text (help, a session error's remedy); text from a page, call or screen, a new carrier or a new exit still widens; `EXIT_STATUSES` drives the command line's statuses. Grounds decision/fixed-program-text-on-carriers and decision/new-carrier-widens-boundary: the smoke reads the binary's answers and must add no carrier or exit of the binary's own.

@history_item n=19 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the `escher-session` coverage role, its shape, endings and process — `escher-session` has three roles (`serve`, `mcp`, `coverage <app> <check>...`), the usage line `; coverage role: …;` before `; apps:`; the host is one process, one session, one listener (rows decision/session-host-process-model, decision/session-host-single-process); ratified at 2026-11-15T05-21-42-wrap. Defines the full usage line and role set the smoke's usage/exit-2 path meets.

@history_item n=20 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified — the escher-session coverage role and its argv files — the coverage role's PROVISIONAL marks discharged by the founder's word; the binary's role set and argv reading stand ratified, unchanged by this chunk.

@history_item n=21 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — fourteen verbs, `actions` the tenth instance verb — `VERBS` fourteen, `actions` takes no argument (one refused `malformed`, nothing run), the command line and cold-agent tool set fourteen. Grounds contract/actions-verb/arguments and contract/driver-verbs/actions: a read-only verb the smoke's «at least one call» may use safely; any verb-count text in docs reads fourteen.

@history_item n=22 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the `graph` verb — `graph` takes no argument, refused `malformed` with an argument, dispatches nothing; `VERBS` was thirteen then. Grounds contract/graph-verb/arguments, contract/graph-verb/argument and contract/driver-verbs/graph.

@history_item n=23 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot` takes one optional `part` — `part` 1 to `MAX_PART` 1000, the answer `text` plus `part`/`parts` when paged, a part past the last the empty text; the command line reads `--part`. Grounds contract/snapshot-verb/part, contract/argument-kinds/part and contract/command-line/verb-flags: a `snapshot` call is the simplest no-step smoke call.

@history_item n=24 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the acting step's bounded load wait — `LOAD_WAIT` 5 s wall time for acting verbs, `snapshot`/`screenshot` waiting on nothing. Grounds contract/load-wait/bound and contract/load-wait/verbs: if the smoke drives an acting verb on `load`, its timing bound comes from here.

@history_item n=25 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the gated part: `lint` the twelfth verb, on every carrier — `lint` takes no argument, refused `fonts-unavailable` before reading, runs no step; the command line's verbs listed. Grounds contract/driver-verbs/lint; the README Try it line uses it.

@history_item n=26 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — no seed crosses the driver's carriers — recorded the absence of a seed on every verb, argument, flag, MCP field, kept check and span field (ratified at 2026-11-14-remote-pages-local-files). Grounds decision/no-seed-on-carriers: the smoke must pass no seed flag or env and reads the default seed.

@history_item n=27 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — `no-seed-on-carriers` ratified — the row moved provisional → standing on the founder's own answer; the absence of a seed carrier is a standing constraint the smoke must not widen.

@history_item n=28 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — a fixture state on no carrier; `start_in` — `absence/fixture-state-on-carriers` (command line, MCP, socket, kept check) and the stand's in-process `start_in` table. The smoke boots a stand task only through `start <task>`; it cannot name a fixture state on the command line.

@history_item n=29 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — exploration on no carrier — `absence/exploration-on-carriers` (no verb, role, argument, MCP tool, socket call or kept check reaches exploration); the smoke adds no role or argument to the binary, consistent with the scope's boundary.

@history_item n=30 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — `CARGO_TARGET_TMPDIR` registered — a blitz-tests target's test-only fixture directory name added to Occupied Resources (row name/other/cargo-target-tmpdir). Precedent: a test-only directory or env name the smoke uses must already be registered or be registered at the wrap.

@history_item n=31 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink drops outside targets; `host_log` spawns `escher-session` — `escher-session` installs `escher_telemetry::init` (stderr only, `RUST_LOG` its one env read), and seven_guis tests spawning the binary drain each stream on a thread. Grounds decision/escher-telemetry-sink and decision/runtime-inputs: the smoke reads stdout JSON only, must not depend on stderr content, and adds no env read beyond the registered `RUST_LOG`.

@history_item n=32 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — the `dioxus_runtime_probe` target and agent-run's `run stand`/`run all` reach — records that a non-`stand_` target is in neither `run stand`, `stand-repeat` nor `a11y`, run by `test` and `run all`. Shows how agent-run's `run` selects blitz-tests targets today — the mechanism the scope says cannot reach the session binary (L1).
