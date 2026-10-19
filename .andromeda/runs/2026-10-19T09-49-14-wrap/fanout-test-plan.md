proposals:
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → session-start'
    change: 'After the recording clause, state the record''s sixth key and the replay: each record now writes `tool` · `arguments` · `masked` (a `type` only) · `accepted` · `digest` (`answer_digest`, unchanged, FNV-1a 64 over the whole written line, a shot''s PNG included) · `answer` — the kept line as its own JSON object, masked as on every carrier and a shot''s written without `png` (`{"width":…,"height":…}`) — and `Check::from_bytes` requires `answer` as a JSON object (missing, a string, a number, an array or `null` refused `malformed`; bounds unchanged); since 2026-10-19-recorded-check-replay `Check::replay(&mut Session) -> Replay` runs nothing on a session whose label is not the check''s app, otherwise runs each step''s call through `Session::run` (validated again) and stops at the first step whose acceptance differs or whose live kept line, parsed as a JSON value, is not equal to the kept `answer`; `Replay` (`holds` · `another_app` · `ran` · `change`) and `Change` (`step` · `accepted_then` · `accepted_now` · `keys`, the differing keys taken from the verb''s result fields in table order, then `refused`) hold no reading and no `serde_json` type (packages/escher-driver/src/recording.rs:134-136; packages/escher-driver/src/replay.rs:21-38; packages/escher-driver/src/replay.rs:40-70)'
    sidecar: 'session-start: the kept record''s `answer` key (required on read) and `Check::replay` · `Replay` · `Change` stated (2026-10-19-recorded-check-replay)'
    rationale: 'Report Changes → Symbols / APIs (the record''s new `answer` key; the reader; the replay, new public API in replay.rs) and Schema / config (the kept-check record grammar, F3 B); Expected amendments names test-plan §3 session-start for the `answer` key and `Check::replay`.'
    basis: 'packages/escher-driver/src/replay.rs:40-70'
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → session-proof'
    change: 'Restate `stand_session_recording` 3 (session-lifecycle.md:11 @c287 / @c390): its first test, renamed `a_recorded_session_keeps_each_answer_but_a_shots_image_and_replays`, now asserts that each record ends with `,"answer":{line}}`, the line rebuilt in the test, that the check holds no `png` and no shot''s first 64 base64 characters, and that the recorded session replays through `Check::replay` on the four lean tasks in both layout modes — retiring «every step''s acceptance and digest equal, keeping no screen reading».'
    sidecar: 'session-proof: `stand_session_recording`''s first test restated — each answer kept but a shot''s image, replayed through `Check::replay`; «keeping no screen reading» retired (2026-10-19-recorded-check-replay)'
    rationale: 'Report Counts / qualifiers moved: the qualifier «no screen reading» is now false, pattern `reading` site `session-lifecycle.md:11` @c390 stale, and pattern `replay` site `session-lifecycle.md:11` @c287 (recorded-session replay, now through the API) stale; Deviations 5 (the renamed test checks each kept answer byte-wise); Symbols / APIs → Removed (`session_common::replay`, its two call sites now `Check::replay`).'
    basis: 'registries/contracts/test-plan/session-lifecycle.md:11'
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary'
    change: 'In the tests/blitz-tests bullet (test-plan.md:27 @c17674 / @c17777 / @c17795), restate `stand_session_recording` 3 the same way: each kept answer whole but a shot''s image (every record ending with its kept line, no `png`, no shot''s first 64 base64 characters), replayed through `Check::replay` on each lean task in both layout modes — retiring «with every step''s acceptance and digest equal and keeps no screen reading at rest».'
    sidecar: '§1 tests/blitz-tests: `stand_session_recording` restated — answers kept but a shot''s image, replay through `Check::replay`; «keeps no screen reading at rest» retired (2026-10-19-recorded-check-replay)'
    rationale: 'Report Counts / qualifiers moved: pattern `reading` site `test-plan.md:27` @c17777 stale, pattern `at-rest` site `test-plan.md:27` @c17795 stale, pattern `replay` site `test-plan.md:27` @c17674 stale; the same claim as the session-proof row.'
    basis: 'test-plan.md:27'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → session-proof'
    change: 'Add, after `stand_session_recording`, since 2026-10-19-recorded-check-replay `stand_replay` 3, not unix-gated, declaring `mod session_common;`: each committed check (`tests/blitz-tests/checks/counter-cli.json`, `counter-mcp.json`) holds on a fresh stand Counter in both layout modes; against a test-local `Counter` variant (a component of the task''s own name, so its ids do not move) each turns red at its first activation step — step 1 (CLI) and step 3 (MCP), read from the steps and asserted equal to the table — every earlier step holding and `changed` among the keys; a check on a session of another app runs nothing; neither check holds a `png` key or a shot''s first 64 base64 characters; green on linux, windows and macOS (3 each) and in `Stand checks repeated` (9) on CI#39227884248 (tests/blitz-tests/tests/stand_replay.rs:81-110; tests/blitz-tests/tests/stand_replay.rs:112-137; tests/blitz-tests/tests/stand_replay.rs:206-237; tests/blitz-tests/tests/stand_replay.rs:239-261).'
    sidecar: 'session-proof: `stand_replay` 3 added — the two committed checks hold on the stand and turn red at their first activation step on a changed Counter (2026-10-19-recorded-check-replay)'
    rationale: 'Report Harness / gate surface (new blitz-tests target `stand_replay`, not unix-gated); Coverage of new surfaces (`Check::replay` tests integ `stand_replay` 3; the committed checks tests integ `stand_replay`); Cross-project claims (CI#39227884248 per-job counts); Deviations 1 and 2. Expected amendments: session-proof gains the replay target.'
    basis: 'tests/blitz-tests/tests/stand_replay.rs:206-237'
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary'
    change: 'In the tests/blitz-tests bullet (test-plan.md:27), after the `stand_session_recording` clause, add `stand_replay` 3 as in session-proof: the two committed Counter checks hold in both layout modes, turn red at step 1 / step 3 on the test-local `Counter` variant with every earlier step held and `changed` among the keys, a check on another app runs nothing, no `png` or shot prefix in either check; not unix-gated (tests/blitz-tests/tests/stand_replay.rs:81-110; tests/blitz-tests/tests/stand_replay.rs:206-237; tests/blitz-tests/tests/stand_replay.rs:239-261).'
    sidecar: '§1 tests/blitz-tests: `stand_replay` 3 added (2026-10-19-recorded-check-replay)'
    rationale: 'Report Files → New (`tests/blitz-tests/tests/stand_replay.rs`), Harness / gate surface; the §1 bullet lists each session check and omits the new one.'
    basis: 'test-plan.md:27'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → session-proof'
    change: 'seven_guis `recording` 3 (+1 off-unix) → 4 (+1 off-unix) (session-lifecycle.md:11 @c640), adding `the_committed_checks_are_what_the_binary_records` (unix-gated): `record_checks.sh` run through `CARGO_BIN_EXE_escher-session` into `CARGO_TARGET_TMPDIR/rc-fresh` writes both checks byte-equal to the committed copies (`include_bytes!`), under `run`''s 90 s bound with both streams drained and a `Started` guard, a failure naming only `step N`; green on macOS and linux and in `Stand checks repeated` on CI#39227884248, its first run CI#39225375577 red on macOS while the scripts held a `screenshot` step (examples/seven_guis/tests/recording.rs:205-262).'
    sidecar: 'session-proof: seven_guis `recording` 3 → 4 (+1 off-unix), the committed checks'' freshness test (2026-10-19-recorded-check-replay)'
    rationale: 'Report Counts / qualifiers moved: seven_guis `recording` tests 3 (+1 off-unix) → 4 (+1 off-unix), pattern `rec-tests` site `session-lifecycle.md:11` @c640 stale; Harness / gate surface (the new seven_guis test); Cross-project claims (CI#39225375577, CI#39227884248).'
    basis: 'examples/seven_guis/tests/recording.rs:205-262'
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary'
    change: 'In the apps bullet''s seven_guis targets, `tests/recording.rs` (3 tests, unix only, and 1 off-unix) → (4 tests, unix only, and 1 off-unix; 3 until 2026-10-19-recorded-check-replay), adding that the committed checks are what the binary records — `record_checks.sh` run through the binary, both files compared byte for byte with the committed copies (examples/seven_guis/tests/recording.rs:205-262).'
    sidecar: '§1 apps: seven_guis `tests/recording.rs` 3 → 4 tests (+1 off-unix) (2026-10-19-recorded-check-replay)'
    rationale: 'Report Counts / qualifiers moved (seven_guis `recording` tests 3 → 4, adding `the_committed_checks_are_what_the_binary_records`); the §1 apps bullet restates the old count «`tests/recording.rs` (3 tests, unix only, and 1 off-unix)».'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → session-proof'
    change: 'escher-driver''s 87 unit tests → 92 (session-lifecycle.md:11 @c4266), the per-file list gaining `replay.rs 5` (recording.rs staying 9), its history adding «87 until 2026-10-19-recorded-check-replay».'
    sidecar: 'session-proof: escher-driver unit tests 87 → 92, `replay.rs 5` in the per-file list (2026-10-19-recorded-check-replay)'
    rationale: 'Report Counts / qualifiers moved: escher-driver unit tests 87 → 92, fourteen → fifteen files, `replay.rs` +5, `recording.rs` stays 9; pattern `unit-count` site `session-lifecycle.md:11` @c4266 stale (its per-file list gains `replay.rs 5`); gate `cargo test -p escher-driver --locked` green (92).'
    basis: 'packages/escher-driver/src/replay.rs:141-318'
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary'
    change: 'escher-driver bullet (test-plan.md:22 @c21): 87 unit tests in fourteen files, 67 inline → 92 in fifteen files, 72 inline, the two `#[path]` test files'' 20 unchanged and the three off-unix tests beside (history «87 in fourteen until 2026-10-19-recorded-check-replay»), and add to the per-file list since 2026-10-19-recorded-check-replay replay.rs 5 (a check replayed on a fresh session holds; an acceptance that flips is a change naming the keys that differ; a changed answer names its keys and stops the replay; a shot holds at its size whatever its pixels; a check of another app runs nothing — packages/escher-driver/src/replay.rs:141-318).'
    sidecar: '§1 escher-driver: 87 → 92 unit tests in fifteen files, 72 inline, replay.rs 5 listed (2026-10-19-recorded-check-replay)'
    rationale: 'Report Counts / qualifiers moved: pattern `unit-count` site `test-plan.md:22` @c21 stale; Symbols / APIs (unit tests in replay.rs:141-318: holds · acceptance flip · changed answer naming its keys and stopping the replay · a shot holding at its size · another app running nothing).'
    basis: 'test-plan.md:22'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary'
    change: 'escher-driver bullet''s recording.rs 9 description (test-plan.md:22): its rows reworked — each record keeps its answer after the digest, a shot''s without `png`; one outside the grammar is refused `malformed`, 40 rows (35 until 2026-10-19-recorded-check-replay added `answer` missing, a string, a number, an array, `null` and an unknown key beside it), and a check in the grammar is read, 8 rows (6 until then: an object `answer`, a refusal''s `answer`, keys out of order).'
    sidecar: '§1 escher-driver: recording.rs rows restated for the `answer` key, refused 35 → 40, read 6 → 8 (2026-10-19-recorded-check-replay)'
    rationale: 'Report Deviations 4 (the escher-driver grammar rows now carry `answer`; refused table 35 → 40, read table 6 → 8); Counts (recording.rs stays 9 unix plus 1 off-unix, its rows reworked); Coverage of new surfaces (the `answer` key: 40 refused and 8 read rows).'
    basis: 'test-plan.md:22'
  - detector: D-tests-coverage
    severity: warning
    section: '§4 Unit Test Strategy'
    change: 'In What unit tests cover → escher-driver, `Check::from_bytes`''s strict grammar «35 refusal rows» → 40 refusal rows and 8 read rows, the record keeping its answer (a shot''s without `png`), and add since 2026-10-19-recorded-check-replay replay.rs pinning that a check replayed on a fresh session holds, an acceptance flip and a changed answer each stop the replay naming their keys, a shot holds at its size whatever its pixels and a check of another app runs nothing (packages/escher-driver/src/replay.rs:141-318).'
    sidecar: '§4 escher-driver: recording.rs refusal rows 35 → 40 (8 read), replay.rs pins added (2026-10-19-recorded-check-replay)'
    rationale: 'Report Deviations 4 (refused 35 → 40, read 6 → 8) and Symbols / APIs (replay.rs unit tests); the §4 escher-driver bullet restates «`Check::from_bytes`''s strict grammar (35 refusal rows, edge rows at both bounds)» and names no replay.rs test.'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§3 Test Harness Contract'
    change: 'Crate-local test helpers → blitz-tests (session checks) (test-plan.md:102): `stand_session_recording` no longer reads the module''s `replay(session, check)` (removed, its two call sites now `Check::replay`) but `kept(check)`, the unix-only `mode(path)` and, since 2026-10-19-recorded-check-replay, `shot_prefix(session)` (the first 64 base64 characters of a whole-screen shot as the driver writes it, tests/blitz-tests/tests/session_common/mod.rs:240-250), which `stand_replay` reads too; 24 readers → 25 (24 before `stand_replay`).'
    sidecar: '§3 session_common: `replay` removed, `shot_prefix` added, 24 → 25 readers (+`stand_replay`) (2026-10-19-recorded-check-replay)'
    rationale: 'Report Symbols / APIs → Removed (`session_common::replay`) and Added (`session_common::shot_prefix`, session_common/mod.rs:240-250, read by `stand_replay` and `stand_session_recording`); Counts: `session_common` readers 24 → 25, pattern `sc-readers` site `test-plan.md:102` @c2738 stale, the helper list naming `replay(session, check)` stale; pattern `replay` site `test-plan.md:102` @c2489 stale.'
    basis: 'tests/blitz-tests/tests/session_common/mod.rs:240-250'
  - detector: D-tests-coverage
    severity: warning
    section: '§2 Test Strategy'
    change: 'Directory pattern (test-plan.md:67): `tests/session_common/mod.rs` read through `mod session_common;` by 24 checks → 25 (24 before `stand_replay.rs`); and 135 `.rs` files, 134 test targets → 136 and 135 (135 and 134 until 2026-10-19-recorded-check-replay added `stand_replay.rs`, its own auto-discovered target).'
    sidecar: '§2 directory pattern: session_common readers 24 → 25, blitz-tests 135/134 → 136/135 (+`stand_replay.rs`) (2026-10-19-recorded-check-replay)'
    rationale: 'Report Counts: `session_common` readers 24 → 25 (pattern `sc-readers` site `test-plan.md:67` @c1090 stale); blitz-tests `.rs` files and targets 135 / 134 → 136 / 135 (pattern `bt-files` site `test-plan.md:67` @c2485 stale; @c3447 history, no-change).'
    basis: 'test-plan.md:67'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy'
    change: 'Command line (`escher-session`, unix only): add since 2026-10-19-recorded-check-replay that seven_guis `recording`''s `the_committed_checks_are_what_the_binary_records` runs `record_checks.sh` — a CLI session (`start counter --record` · `snapshot` · `click counter-increment` · `advance 600` · `click counter-increment` · `snapshot` · `stop`) and an MCP session (`initialize` … `scroll counter-increment` · `snapshot` · `stop`) — through the binary and reads both kept checks byte-equal to `tests/blitz-tests/checks/counter-{cli,mcp}.json`; the scripts hold no `screenshot` step since a shot''s PNG bytes differ by OS (CI#39225375577) (examples/seven_guis/tests/recording.rs:205-262).'
    sidecar: '§6 command line: the committed checks'' freshness test through the binary (2026-10-19-recorded-check-replay)'
    rationale: 'Report Harness / gate surface (new seven_guis test running record_checks.sh through `CARGO_BIN_EXE_escher-session`), Schema / config (the recording script''s two sessions), Reverted / negative API facts (the screenshot step removed); Coverage of new surfaces (`record_checks.sh` → tests integ, the freshness check runs it).'
    basis: 'examples/seven_guis/tests/recording.rs:205-262'
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → session-proof'
    change: '`stand_screenshot_repeat` 1 (session-lifecycle.md:11 @c2632): «a shot of one state byte-identical in two child runs and the parent» restated as same-host only — a shot''s PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577, `Test (macos)`, job 117972311781), so a check compared byte for byte across OSes, a committed check''s digest included, holds no `screenshot` step; the shot path''s kept form is covered on each OS by `stand_session_recording`.'
    sidecar: 'session-proof: `stand_screenshot_repeat`''s byte-identity restated as same-host only, CI#39225375577 cited, the committed checks holding no `screenshot` step (2026-10-19-recorded-check-replay)'
    rationale: 'Report Counts → the new measured limit (pattern `shot-os`, site `session-lifecycle.md:11` @c2632); Spec claims disproved 2 and 3; Addendum inputs#I5 (a) and (b) supersede «no-change» — the row is stale, restated same-host only with CI#39225375577 and the limit beside it, and `stand_session_recording` recorded as the shot path''s coverage on each OS.'
    basis: 'registries/contracts/test-plan/session-lifecycle.md:11'
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary'
    change: 'In the tests/blitz-tests bullet''s `stand_screenshot_repeat` clause (test-plan.md:27, «two children and the parent printing byte-identical PNGs for 12 shots»), add that the byte-identity is same-host only: a shot''s PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a committed check holds no `screenshot` step.'
    sidecar: '§1 tests/blitz-tests: `stand_screenshot_repeat`''s byte-identity qualified same-host only (2026-10-19-recorded-check-replay)'
    rationale: 'Same claim as the session-proof `shot-os` row, worded «printing byte-identical PNGs»; Addendum inputs#I5 (a): the byte-identical shot claims restated as same-host only, citing CI#39225375577.'
    basis: 'test-plan.md:27'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§2 Test Strategy'
    change: 'Pixel tests: after «byte-identical across two child runs of the test binary''s own `#[ignore]` child … and the parent» (`stand_screenshot_repeat`), add «on one host — a shot''s PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes holds no shot».'
    sidecar: '§2 pixel tests: the shot byte-identity qualified same-host only (2026-10-19-recorded-check-replay)'
    rationale: 'Same claim as the session-proof `shot-os` row, restated in §2 Pixel tests; Report Decisions & corrections (a shot''s PNG bytes differ between linux x86_64 and macOS aarch64) and Addendum inputs#I5 (a).'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§9 CI Integration'
    change: 'Engine features by runner: «`stand_screenshot_repeat` reads byte-identical shots under the per-package and the workspace build» qualified as on one host — the shot''s bytes differ across OSes (CI#39225375577, macOS aarch64 against linux x86_64).'
    sidecar: '§9 engine features by runner: the shot byte-identity qualified same-host only (2026-10-19-recorded-check-replay)'
    rationale: 'Same claim as the session-proof `shot-os` row; Report Spec claims disproved 2 (byte-identity holds on one host only) and Addendum inputs#I5 (a).'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§9 CI Integration'
    change: 'Legs → stand-repeat (test-plan.md:318 @c1285): one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 43 files → 44, `stand_replay` joining by its prefix since 2026-10-19-recorded-check-replay, and `test_stand_repeat_leg_runs_three_rounds` now naming `stand_replay` (.github/scripts/test_ci_workflows.py:359).'
    sidecar: '§9 stand-repeat: 43 → 44 `stand_*.rs` files (+`stand_replay`), the leg''s test pin naming it (2026-10-19-recorded-check-replay)'
    rationale: 'Report Counts: `stand_*.rs` files 43 → 44, pattern `stand-files` site `test-plan.md:318` @c1285 stale; Harness / gate surface (`stand_replay` picked up by stand-repeat''s glob, `ci-leg.sh` unchanged; test_ci_workflows.py:359 names `stand_replay`).'
    basis: '.github/scripts/test_ci_workflows.py:359'
  - detector: D-tests-coverage
    severity: warning
    section: '§10 Quality Gates & Coverage Targets'
    change: 'Flakiness budget (test-plan.md:347): «the 43 `stand_*.rs` files» → the 44 (43 until 2026-10-19-recorded-check-replay added `stand_replay`, whose stand-repeat run read `stand-repeat: 3 of 3 rounds green` and, on CI#39227884248, `Stand checks repeated` `stand_replay` 9), seven_guis'' eight process checks unchanged.'
    sidecar: '§10 flakiness budget: 43 → 44 stand files (2026-10-19-recorded-check-replay)'
    rationale: 'Report Counts: pattern `stand-files` site `test-plan.md:347` («the 43 `stand_*.rs` files») stale; Gates (`ci-leg.sh stand-repeat` green, `3 of 3 rounds green`); Cross-project claims (CI#39227884248, `Stand checks repeated` 9).'
    basis: 'test-plan.md:347'
    dependent-of: D-tests-coverage
  - detector: D-tests-framework
    severity: warning
    section: '§2 Test Strategy'
    change: 'Command-line checks and shell flows: «two POSIX `sh` scripts under `examples/seven_guis/tests/flows/` (`counter.sh`, `flight.sh`) … run under plain `sh` by the Rust target `cli_flow`» → three, adding since 2026-10-19-recorded-check-replay `record_checks.sh` (binary and an existing output directory as its two arguments, `rec/` made `0700`, printing only `step N` on failure), run by seven_guis `recording`''s unix-gated `the_committed_checks_are_what_the_binary_records` — still an asset of a `cargo test` target, no runner of its own and no framework added.'
    sidecar: '§2 command-line checks: a third `sh` asset, `record_checks.sh`, run by seven_guis `recording` (2026-10-19-recorded-check-replay)'
    rationale: 'Report Files → New (`examples/seven_guis/tests/flows/record_checks.sh`, 65 lines), Schema / config (the recording script, POSIX `sh`), Harness / gate surface (run by the new seven_guis test); the §2 runner statement names two scripts run by `cli_flow` alone.'
    basis: 'examples/seven_guis/tests/recording.rs:205-262'
  - detector: D-tests-framework
    severity: warning
    section: '§2 Test Strategy'
    change: 'Directory pattern, seven_guis clause («one directory that is no target, `tests/flows/`, holding the two `sh` flow scripts») → holding the two `sh` flow scripts and, since 2026-10-19-recorded-check-replay, `record_checks.sh`, the committed checks'' recording script.'
    sidecar: '§2 directory pattern: `tests/flows/` holds three `sh` scripts (2026-10-19-recorded-check-replay)'
    rationale: 'Same claim as the §2 Command-line checks row (the flows directory''s two scripts), restated in the directory pattern; Report Files → New (`record_checks.sh`).'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: '§2 Test Strategy'
    change: 'Add a test level, recorded-check replay (since 2026-10-19-recorded-check-replay): a check a session recorded, committed as data under `tests/blitz-tests/checks/` (`counter-cli.json`, `counter-mcp.json`, git mode `100644`), read by the strict `Check::from_bytes` and run as an ordinary `cargo test` target (`stand_replay`) through `Check::replay` — every step re-validated through `Session::run`, red at the first step whose acceptance or kept answer differs, compared as JSON values — and kept fresh by a byte-for-byte comparison with what the binary records (seven_guis `recording`); picked up by `agent-run.sh run stand`, stand-repeat and the CI `test` leg on three OSes with no script change; the directory pattern gains `tests/blitz-tests/checks/`, a data directory that is no target.'
    sidecar: '§2 test levels: recorded-check replay — committed checks under `tests/blitz-tests/checks/` replayed by `stand_replay` (2026-10-19-recorded-check-replay)'
    rationale: 'Report Harness / gate surface (`stand_replay`, picked up by agent-run, stand-repeat and the CI test leg on three OSes), Schema / config (two committed checks), Symbols / APIs (`Check::replay`); a new test style and a new test-data directory the §2 levels and directory pattern do not state.'
    basis: 'tests/blitz-tests/tests/stand_replay.rs:24-36'
  - detector: D-tests-framework
    severity: warning
    section: '§7 Test Data & Fixtures'
    change: 'Fixture sources: add the committed checks — `tests/blitz-tests/checks/counter-cli.json` (8 lines, 2,929 B) and `counter-mcp.json` (11 lines, 3,720 B), the stand Counter''s scripted CLI and MCP sessions as `escher-session` records them through `record_checks.sh` (ids, names, values, snapshot text and diffs; no typed text, host path, name, secret or `screenshot` step), embedded by `stand_replay` and seven_guis `recording` as bytes (tests/blitz-tests/tests/stand_replay.rs:24-36; examples/seven_guis/tests/recording.rs:187-203).'
    sidecar: '§7 fixture sources: the two committed Counter checks under `tests/blitz-tests/checks/` (2026-10-19-recorded-check-replay)'
    rationale: 'Report Files → New (the two check files and their sizes), Schema / config (two committed checks, content, F1 A; no `screenshot` step since `f15c28fc`), Coverage of new surfaces (PII ✓, inputs#I3).'
    basis: 'tests/blitz-tests/tests/stand_replay.rs:24-36'
note: 'D-tests-obs-harness: no proposal — the report changes no agent-run verb, status shape or log format (Harness / gate surface: no CI workflow, leg script or agent-run script changed; no new log field). Not proposed for want of a reported number: §9 Per-OS test tallies and §3 agent-run Proof re-counts (the report gives CI#39227884248 per-job `stand_replay` counts and a `run stand_replay` smoke only, no whole-job tallies or `run stand` run.end), and the §9/§10 coverage reading (escher-driver 98.33 % is reported, but the doc records per-crate readings only at named chunks). The `shot-os` dependents (§1, §2 Pixel tests, §9 Engine features by runner) restate the same-host shot claim beyond the two rows the report swept; the primary follows Addendum inputs#I5 (a).'
