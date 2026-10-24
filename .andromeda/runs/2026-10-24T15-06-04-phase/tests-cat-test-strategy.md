# tests · Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/test-strategy.md:14) A check this chunk adds or widens that holds a session process (the new verbs' stderr measurement through the host) must start and stop its own host with bounded waits, a kill-and-reap guard, and the child's streams nulled unless read, so no child libtest line reaches `agent-run.sh`'s count.
  > a check that needs a session process starts and stops its own host — its own test binary re-run on one `#[ignore]` child (`--ignored --exact {child} --nocapture`), or the package's own binary through `CARGO_BIN_EXE_*` — with every wait bounded, a guard that kills and reaps a host a failing check still holds, its state directory under `CARGO_TARGET_TMPDIR` removed at `stop`, and the child's streams sent to null unless the check reads them, so the child's own libtest lines never reach the parent's output, where `agent-run.sh` would count them as tests
- (.andromeda/test-plan-detail/test-strategy.md:15) The `lint` / `part` / `--record` stderr measurement on a sink-installing host must drain the host's pipes while it runs (or capture to a file), never wait for exit before reading.
  > and a check that captures a host which installs escher's sink drains its pipes while the host runs, or captures to a file — the seven_guis checks read each stream on a thread from the spawn on:
- (.andromeda/test-plan-detail/test-strategy.md:15) A check capturing the `escher-session mcp` child under the new verbs must count its lines by span and target.
  > and the `escher-session mcp` child, since 2026-10-10-mcp-surface, one `tool_call` span line per `tools/call` it answers, the parent of that call's `command` line, so a check capturing that child counts its lines by span and target
- (.andromeda/test-plan-detail/test-strategy.md:16) A command-line measurement of `start --record` (or any new verb run as a client process) must go through the shared runner, which bounds the run and drains both streams; flow scripts stay assets of a `cargo test` target and no test framework is added.
  > seven_guis' checks run the package's own `escher-session` binary once per command as a client process and read its stdout, its stderr and its exit status — `0` accepted · `1` refused · `2` usage · `3` session error — through the shared runner of `tests/common/mod.rs`, which bounds the run and drains both streams
- (.andromeda/test-plan-detail/test-strategy.md:22) The recording/replay requirement's witnesses (`stand_replay`, seven_guis `recording`) are named in the sweep as they stand: inside `stand-repeat` with no script change, red at the first differing step.
  > read by the strict `Check::from_bytes` and run as an ordinary `cargo test` target (`stand_replay`) through `Check::replay` — every step re-validated through `Session::run`, red at the first step whose acceptance or kept answer differs, the answers compared as JSON values — and kept fresh by a byte-for-byte comparison with what the binary records (seven_guis `recording`); picked up by `agent-run.sh run stand`, stand-repeat and the CI `test` leg on three OSes with no script change
- (.andromeda/test-plan-detail/test-strategy.md:34) C1's fixture checks (the lint's `truncated` reading, `text_extent`'s RTL / vertical / transformed limits) must assert unconditionally under the bundled font with system fonts off — no skip path.
  > the headless-stand checks have no skip path — they assert unconditionally under the bundled DejaVu Sans `seven_guis::stand::font_ctx()` registers with system fonts off, passed through `HarnessOptions.font_ctx`
- (.andromeda/test-plan-detail/test-strategy.md:46) Each new check is one test-target file per behaviour; adding a `mod common;` or `mod session_common;` reader moves these stated counts, which the wrap must re-state.
  > `tests/blitz-tests` holds integration tests, one test-target file per behavior under `tests/`, and three shared modules that are no target
- (.andromeda/test-plan-detail/test-strategy.md:46) The shared-module reader counts move with every new `stand_*` reader the sweep adds.
  > `tests/common/mod.rs`, read through `mod common;` by 21 `stand_*.rs` checks (19 until 2026-10-14-stand-requirement-sweep added `stand_requirement_sweep.rs` and `stand_non_lean_tasks.rs`, 18 until 2026-10-13-stand-keyboard-harness added `stand_keyboard.rs`, 17 until 2026-10-13-stand-contrast-harness added `stand_contrast.rs`), and `tests/session_common/mod.rs`, read through `mod session_common;` by 29 checks
- (.andromeda/test-plan-detail/test-strategy.md:47) A new C1 fixture check that reads a shared page declares `mod fixtures;` and moves this count.
  > and, since 2026-10-19-shared-fixtures, `tests/fixtures/mod.rs`, read through `mod fixtures;` by 24 checks — 22 `stand_*.rs`
- (.andromeda/test-plan-detail/test-strategy.md:48) Each new `.rs` file under `tests/blitz-tests/tests/` is its own auto-discovered target with no manifest line, moving the 145 / 144 count.
  > every other `.rs` file directly under `tests/` stays its own auto-discovered target: 145 `.rs` files, 144 test targets and the unbuilt `all.rs`
- (.andromeda/test-plan-detail/test-strategy.md:50) The manifest stays untouched by new targets: one target per file, the two-line difference pinned by `BlitzTestsTargetsTest`.
  > the fork keeps one target per file by the operator's decision of 2026-10-10, the two-line difference pinned by `BlitzTestsTargetsTest` (§4)
- (.andromeda/test-plan-detail/test-strategy.md:50) The eight seven_guis targets are the ones `stand-repeat` runs; a new process check there joins this list (and the leg's selection) or is carried inside `host_log` / `mcp_log`.
  > `examples/seven_guis` holds a `tests/` directory with eight integration-test targets
  >  — `host_binary.rs`, `host_log.rs`, since 2026-10-10-driver-cli `cli_commands.rs`, `cli_flow.rs` and `host_timer.rs`, since 2026-10-10-mcp-surface `mcp_server.rs` and `mcp_log.rs`, and since 2026-10-19-session-recording `recording.rs`
- (.andromeda/test-plan.md:19) C1 is this owed item: the text extent's stated limits get a check.
  > the text extent's and the select's stated limits are asserted by no check
- (.andromeda/test-plan.md:38) The lint's `truncated` reading (C1) must be asserted by an integration check, not an escher-driver unit test.
  > the executor's verbs, its refusals and `Session::lint` are read by integration checks, not unit tests — the crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance
- (.andromeda/test-plan.md:147) The sweep's record table is stated expectation, never a second mapping derived from the code.
  > the tables are stated expectation, never a second role or name mapping
- (.andromeda/test-plan.md:151) A C1 fixture page with one reader stays in its check's file; the shared fixtures module gains no print, log, env read, file, socket or net.
  > a page with one reader, or whose ids differ, stays in its file (`stand_act_obstructed`'s covered page, `stand_act_filled`'s and `stand_act_screenshot`'s password pages, both `restless_fixture`s); the module holds no print, `tracing` call, env read, file, socket, listener or `blitz_net`
- (.andromeda/test-plan.md:155) The `screenshot` precedent the new verbs' stderr measurement follows: closed words in the span only, 0 needles on host and `mcp` stderr at `trace` under both builds, read by `host_log` and `mcp_log`.
  > a shot's `command` line records `verb` (and `cause` when refused), never the image, its size or its dimensions, and the host's and the `mcp` role's stderr at `trace` hold 0 occurrences of a shot's first 64 base64 characters under both builds (`host_log`, `mcp_log`; examples/seven_guis/tests/host_log.rs:155-168; examples/seven_guis/tests/mcp_log.rs:141-154)
- (.andromeda/test-plan.md:164) A witness counts as inside `run stand` only by the `stand_` prefix; a non-`stand_` fixture check is not selected.
  > `stand` = one `--test {stem}` per `tests/blitz-tests/tests/stand_*.rs` present
- (.andromeda/test-plan.md:176) The newest `run stand` and `run all` tallies; the chunk re-counts them at its end.
  > `run stand`'s `run.end` unmoved at passed 195 · failed 0 · ignored 7 over 48 files, and `run all` reads passed 617 · failed 0 · ignored 12
- (.andromeda/test-plan.md:189) The fixture readers' selections move if a sweep check reads the shared fixtures.
  > `run stand` runs the 22 `stand_*` readers of the shared module, `run {file}` the two non-stand readers (`run font_face_sniffed_source`, which holds a load, and `run select_interaction`) and `run all` the others; no `run fixtures` selection
- (.andromeda/test-plan.md:259) New session checks take no sleep and no wall-clock read.
  > no sleep and no wall-clock read in the checks, except the two whose subject is wall time
- (.andromeda/test-plan.md:265) The "new verbs' stderr id-free" requirement, as the plan states it: host and `mcp` role, `trace`, no id, name or typed text.
  > a host that installs escher's sink and the `mcp` role write no id, name or typed text on stderr at `trace`
- (.andromeda/test-plan.md:374) Any widening of `stand-repeat`'s selection keeps three rounds, first-red-ends, no retry, no `--ignored`, no env var.
  > the `stand-repeat` leg runs the stand checks and seven_guis' process checks for three rounds, the first red command ending the leg with its status — no retry, no `--ignored`, no env var
- (.andromeda/test-plan.md:389) The stderr measurement counts lines by target under both builds, and a `text_extent` vertical-text check must hold in the workspace build where `writing-mode` is on; a per-package green is not the leg's green.
  > `cargo test --workspace --locked` and `cargo test -p blitz-tests --locked` build the same test file against different engine features — `tracing` and `writing-mode` on in the workspace build only — so a check that reads a sink's capture counts lines by target, never in total, a green per-package run is not the workspace leg's green, and each body of the engine's bounds reader holds a check
- (.andromeda/test-plan.md:408) The floors the chunk must read at or over at its end ("coverage floors held").
  > `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %
- (.andromeda/test-plan.md:410) "Inside stand-repeat" means under this zero-failure budget.
  > the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files and seven_guis' process checks three times with no retry (§9 Legs)
- (.andromeda/test-plan.md:412) v020-02's witness (mutants) stays a dev-host run; the sweep never places it in CI.
  > it runs on the dev host alone, never in CI

## Bindings
- **Process-lifecycle and capture rules ↔ obs-plan §6 log format / §8**: the new verbs' 0-needle reading on host and `mcp` stderr is the obs sink's guarantee measured by tests' capture discipline — (.andromeda/test-plan-detail/test-strategy.md:15), (.andromeda/test-plan.md:155), (.andromeda/test-plan.md:265)
- **Witness selection ↔ §9 CI Integration / §10 Quality Gates (`stand-repeat`, coverage floors)**: a witness outside `stand_*` is outside `run stand` and `stand-repeat`; widening the leg's selection is a CI gate change pinned by `test_ci_workflows.py` — (.andromeda/test-plan.md:164), (.andromeda/test-plan.md:374), (.andromeda/test-plan.md:410), (.andromeda/test-plan.md:408)
- **Target-per-file rule ↔ §4 `BlitzTestsTargetsTest` and `.github/scripts/test_blitz_tests_targets.py`**: new fixture checks add no manifest line — (.andromeda/test-plan-detail/test-strategy.md:50)
- **C1 checks ↔ arch / playbook engine-change rule**: a check that pins `text_extent`'s limits or the lint's `truncated` asserts target behaviour; judging either a defect is a founder fork, not a test change — (.andromeda/test-plan.md:19), (.andromeda/test-plan.md:38)
- **Mutation tier ↔ §10 / CLAUDE.md key commands**: v020-02's witness is a dev-host cargo-mutants run, never CI — (.andromeda/test-plan.md:412)
