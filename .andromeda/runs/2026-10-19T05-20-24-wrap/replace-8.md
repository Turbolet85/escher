- line: .andromeda/test-plan.md:10
  old: «`examples/seven_guis` holds seven integration-test targets, one shared module»
  new: «`examples/seven_guis` holds eight integration-test targets (seven until 2026-10-19-session-recording added `tests/recording.rs`), one shared module»
- line: .andromeda/test-plan.md:10
  old: «read through `mod common;` by all seven) and a script directory»
  new: «read through `mod common;` by all eight) and a script directory»
- line: .andromeda/test-plan.md:10
  old: «`tests/host_binary.rs` (2 tests) — the `escher-session` binary in its host role, `serve <task> --session <dir>`, serves»
  new: «`tests/host_binary.rs` (2 tests) — the `escher-session` binary in its host role, `serve <task> --session <dir> [--record <dir>]`, serves»
- line: .andromeda/test-plan.md:10
  old: «`tests/host_log.rs` (1 test, unix only) — the `escher-session` binary on `crud`»
  new: «`tests/host_log.rs` (2 tests, unix only; 1 until 2026-10-19-session-recording added a recording host that logs nothing of its check at `trace`) — the `escher-session` binary on `crud`»
- line: .andromeda/test-plan.md:10
  old: «`tests/mcp_log.rs` (1 test: the role on `crud` at `RUST_LOG=trace`»
  new: «`tests/mcp_log.rs` (2 tests — 1 until 2026-10-19-session-recording added a recording `mcp` role that logs nothing of its checks at `trace`, unix-gated with its `recorded_needles`: the role on `crud` at `RUST_LOG=trace`»
- line: .andromeda/test-plan.md:10
  old: «`cargo test --workspace --locked --test mcp_server --test mcp_log`; and since 2026-10-07-act-by-id»
  new: «`cargo test --workspace --locked --test mcp_server --test mcp_log`; since 2026-10-19-session-recording `tests/recording.rs` (3 tests, unix only, and 1 off-unix) — the command line and the `mcp` role keep the same check byte for byte, the state directory gone after `stop`; a recording directory open to its group refused before anything runs; the `mcp` role's listing and `help` the same with and without `--record`; off unix, `--record` refused `Unsupported` — run as `cargo test -p seven_guis --locked --test recording` and in the stand-repeat leg; and since 2026-10-07-act-by-id»
- line: .andromeda/test-plan.md:16
  old: «`DocumentConfig.clock` and the engine's time reads are read by blitz-tests' `virtual_clock_engine` (tests/blitz-tests, below)»
  new: «`DocumentConfig.clock` and the engine's time reads are read by blitz-tests' `virtual_clock_engine` (tests/blitz-tests, below), pan and the fling among them since 2026-10-19-session-recording, and blitz-dom's `iframe::tests::a_sub_documents_config_carries_its_parents_clock` pins a sub-document's config holding its parent's clock, the same `Arc` (packages/blitz-dom/src/iframe.rs:194-212)»
- line: .andromeda/test-plan.md:22
  old: «74 unit tests in thirteen files, 56 inline and, since 2026-10-16-file-splits, 18 in two `#[path]` test files (73 until»
  new: «87 unit tests in fourteen files, 67 inline and, since 2026-10-16-file-splits, 20 in two `#[path]` test files, beside three off-unix tests the windows job alone runs (74 in thirteen until 2026-10-19-session-recording, 73 until»
- line: .andromeda/test-plan.md:22
  old: «all thirteen variants, `NotSettled` in each of its three classes and the thirteenth, `AnswerTooLarge`, included»
  new: «all fifteen variants (thirteen until 2026-10-19-session-recording), `NotSettled` in each of its three classes and `AnswerTooLarge` included»
- line: .andromeda/test-plan.md:22
  old: «since 2026-10-12-self-description the thirteen errors each state a kind»
  new: «since 2026-10-12-self-description the errors — fifteen since 2026-10-19-session-recording added `RecordDirNotPrivate` and `RecordingFailed` — each state a kind»
- line: .andromeda/test-plan.md:22
  old: «session.rs 4 (a label of the closed set is valid;»
  new: «session.rs 6 (4 until 2026-10-19-session-recording, which added that a recording directory open to others is refused with nothing written and that a recording that ends reads failed once; a label of the closed set is valid;»
- line: .andromeda/test-plan.md:22
  old: «wire.rs 12 (each request and each reply of the `v2` grammar round-trips — `hello`, `stop` and `call`, the lifecycle replies and `accepted` · `refused` · `oversize` —»
  new: «wire.rs 12 (each request and each reply of the `v3` grammar — `v2` until 2026-10-19-session-recording — round-trips: `hello`, `stop` and `call`, the lifecycle replies and `accepted` · `refused` · `oversize` · `recording-failed` —»
- line: .andromeda/test-plan.md:22
  old: «mcp_tests.rs 12 (`mcp::tests`,»
  new: «mcp_tests.rs 14 (12 until 2026-10-19-session-recording, which added that each session a recording boot starts is kept as one check and that a call that ends the recording is answered as its session error; `mcp::tests`,»
- line: .andromeda/test-plan.md:22
  old: «describe.rs 2 (the document holds `help`'s seven fields in order; every text in it is its tables')»
  new: «describe.rs 2 (the document holds `help`'s seven fields in order; every text in it is its tables'; its count row reads `[11, 13, 12, 3, 8, 4, 15]` since 2026-10-19-session-recording), since 2026-10-19-session-recording recording.rs 9 (the digest is FNV-1a 64 of the written answer; a recording reads back as the calls it kept; a masked `type` keeps the stand-in and nothing of the text; a call no record can spell is not kept; the check and its directory are owner-only and no file is replaced; a recording that would pass its bound or cannot write ends and removes its check, never a temporary it did not create; a check in the grammar is read; one outside it is refused `malformed`, 35 rows; a check at its bounds is read and one past them refused)»
- line: .andromeda/test-plan.md:27
  old: «(`stand_session_quiet`, 1 and its `#[ignore]` host child `session_host_with_typed_text`; its limit is in §5)»
  new: «(`stand_session_quiet`, 1 and its `#[ignore]` host child `session_host_with_typed_text`; its limit is in §5), and since 2026-10-19-session-recording a recorded session — `stand_session_recording` 3, declaring `mod session_common;`: a recorded session replays on each lean task in both layout modes with every step's acceptance and digest equal and keeps no screen reading at rest; a password typed in a recorded session is kept as the stand-in, 0 sentinel occurrences and a secret of another length leaving byte-identical files; a recording directory open to its group is refused with nothing written (tests/blitz-tests/tests/stand_session_recording.rs:126-343)»
- line: .andromeda/test-plan.md:27
  old: «the two clock checks — `virtual_clock_engine` 4, an engine check on a parsed page»
  new: «the two clock checks — `virtual_clock_engine` 5 (4 until 2026-10-19-session-recording added a finger pan and its fling moving only with the clock, 3 cases in both layout modes, tests/blitz-tests/tests/virtual_clock_engine.rs:232-287), an engine check on a parsed page»
- line: .andromeda/test-plan.md:67
  old: «read through `mod session_common;` by 23 checks (22 before»
  new: «read through `mod session_common;` by 24 checks (23 before `stand_session_recording.rs`, 22 before»
- line: .andromeda/test-plan.md:67
  old: «134 `.rs` files, 133 test targets and the unbuilt `all.rs` (the two newest»
  new: «135 `.rs` files, 134 test targets and the unbuilt `all.rs` (134 and 133 until 2026-10-19-session-recording added `stand_session_recording.rs`, its own auto-discovered target; the two newest before it»
- line: .andromeda/test-plan.md:67
  old: «holds a `tests/` directory with seven integration-test targets — `host_binary.rs`, `host_log.rs`, since 2026-10-10-driver-cli `cli_commands.rs`, `cli_flow.rs` and `host_timer.rs`, and since 2026-10-10-mcp-surface `mcp_server.rs` and `mcp_log.rs` —»
  new: «holds a `tests/` directory with eight integration-test targets — `host_binary.rs`, `host_log.rs`, since 2026-10-10-driver-cli `cli_commands.rs`, `cli_flow.rs` and `host_timer.rs`, since 2026-10-10-mcp-surface `mcp_server.rs` and `mcp_log.rs`, and since 2026-10-19-session-recording `recording.rs` —»
- line: .andromeda/test-plan.md:67
  old: «read through `mod common;` by all seven, and one directory that is no target»
  new: «read through `mod common;` by all eight, and one directory that is no target»
- line: .andromeda/test-plan.md:102
  old: «none a `stand_act_*` file (23 readers; 22 before `stand_virtual_clock`»
  new: «none a `stand_act_*` file, and since 2026-10-19-session-recording `stand_session_recording`, which reads the module's new `replay(session, check)` (each kept step run again, its acceptance and digest), `kept(check)` (each kept step's acceptance and digest) and the unix-only `mode(path)` (a file's permission bits) (tests/blitz-tests/tests/session_common/mod.rs:240-273) (24 readers; 23 before `stand_session_recording`, 22 before `stand_virtual_clock`»
- line: .andromeda/test-plan.md:104
  old: «`escher-session` in its host role, `serve <task> --session <dir>`, runs each `call` request through `Session::run` and writes one closed `command` span line under `escher_driver` per call run»
  new: «`escher-session` in its host role, `serve <task> --session <dir> [--record <dir>]`, runs each `call` request through `Session::run` and writes one closed `command` span line under `escher_driver` per call run, the same with a recording held»
- line: .andromeda/test-plan.md:144
  old: «wire.rs pins the closed grammar at `v2` — `hello`, `stop` and, since 2026-10-10-driver-cli, a third request, `call v2 <verb>»
  new: «wire.rs pins the closed grammar at `v3` (`v2` until 2026-10-19-session-recording, which added the fixed `ok v3 recording-failed` reply) — `hello`, `stop` and, since 2026-10-10-driver-cli, a third request, `call v3 <verb>»
- line: .andromeda/test-plan.md:144
  old: «error.rs pins, since that chunk, that each of the thirteen session errors states a kind»
  new: «error.rs pins, since that chunk, that each of the session errors — fifteen since 2026-10-19-session-recording — states a kind»
- line: .andromeda/test-plan.md:144
  old: «describe.rs that the document holds `help`'s seven fields in order and»
  new: «since 2026-10-19-session-recording recording.rs pins the FNV-1a 64 digest of the written answer, a recording's read-back, the masked `type` stand-in, the calls no record can spell left out, owner-only modes with no file replaced and no foreign temporary removed, a recording ending at its bound or on a failed write with its check removed, and `Check::from_bytes`'s strict grammar (35 refusal rows, edge rows at both bounds); session.rs that a recording directory open to others is refused with nothing written and that a recording that ends reads failed once; mcp_tests.rs that each session a recording boot starts is kept as one check and that a call that ends the recording is answered `isError` as its session error; client.rs that `ok v3 recording-failed` reads as `SessionError::RecordingFailed`; off unix, a recording refused `Unsupported` (packages/escher-driver/src/recording.rs:439-901; packages/escher-driver/src/mcp_tests.rs:587-689); describe.rs that the document holds `help`'s seven fields in order and»
- line: .andromeda/test-plan.md:185
  old: «0 of 8 names (tests/blitz-tests/tests/stand_session_quiet.rs:30-115; examples/seven_guis/tests/host_log.rs:24-315;»
  new: «0 of 8 names; since 2026-10-19-session-recording `host_log` and `mcp_log` each also run a recording host and a recording `mcp` role at `RUST_LOG=trace` and read 0 of the recording needles (the kept check's names and its directory's name) on stderr, the span counts unchanged, the kept check living in the launcher's recording directory and the state directory still `session.sock` alone (tests/blitz-tests/tests/stand_session_quiet.rs:30-115; examples/seven_guis/tests/host_log.rs:24-315; examples/seven_guis/tests/host_log.rs:30-61; examples/seven_guis/tests/mcp_log.rs:56-91;»
- line: .andromeda/test-plan.md:202
  old: «examples/seven_guis/tests/host_timer.rs:22-126; as measured at escher-0.1.0/chunks/2026-10-10-driver-cli/report.md, the two new tests at escher-0.1.0/chunks/2026-10-12-self-description/report.md)»
  new: «examples/seven_guis/tests/host_timer.rs:22-126; as measured at escher-0.1.0/chunks/2026-10-10-driver-cli/report.md, the two new tests at escher-0.1.0/chunks/2026-10-12-self-description/report.md); since 2026-10-19-session-recording `start <task> --record <dir>` run as a process keeps one `0600` check in a `0700` directory, the state directory gone after `stop`, a group-open recording directory refused status `3` with nothing spawned (seven_guis `recording`), and `cli_commands` reads the usage line naming `--record` on `start`, the host role and the `mcp` role, a misplaced `--record` a usage error, status `2` (examples/seven_guis/tests/recording.rs:180-220; examples/seven_guis/tests/cli_commands.rs:502-512)»
- line: .andromeda/test-plan.md:203
  old: «as measured at escher-0.1.0/chunks/2026-10-10-mcp-surface/evidence/live-trial.md, the two new tests at escher-0.1.0/chunks/2026-10-12-self-description/report.md)»
  new: «as measured at escher-0.1.0/chunks/2026-10-10-mcp-surface/evidence/live-trial.md, the two new tests at escher-0.1.0/chunks/2026-10-12-self-description/report.md); since 2026-10-19-session-recording `mcp_log` (2) reads a recording role's stderr at `trace` too, and seven_guis `recording` speaks the role with `--record`: its listing and `help` the same bytes with and without it, and its kept check byte-identical to the command line's (examples/seven_guis/tests/recording.rs:107-247)»
- line: .andromeda/test-plan.md:318
  old: «— 42 files — then `cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server` and `mcp_log`;»
  new: «— 43 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording — then `cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server`, `mcp_log` and, since that chunk, `recording`, `test_stand_repeat_leg_runs_three_rounds` pinning both words;»
- line: .andromeda/test-plan.md:347
  old: «run the 42 `stand_*.rs` files and seven_guis' seven process checks three times with no retry (§9 Legs);»
  new: «run the 43 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (§9 Legs; 42 and seven until 2026-10-19-session-recording, whose run read `stand-repeat: 3 of 3 rounds green`);»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:5
  old: «cross the `mcp` role's stdio (architecture §Standard Contracts → Driver session) (packages/escher-driver/src/session.rs:27-203;»
  new: «cross the `mcp` role's stdio (architecture §Standard Contracts → Driver session); since 2026-10-19-session-recording `Session::recording_into(dir)` (unix only, `Unsupported` elsewhere) creates `dir` `0700` or refuses `RecordDirNotPrivate`, then `check-{n}.json` `0600` with `create_new`, never replacing a file, and `run` records each answered call of the seven instance verbs — `tool` · `arguments` in schema order · `masked` (a `type` on a masked or unresolved target, its text the stand-in `MASKED_STAND_IN`) · `accepted` · `digest` (`answer_digest`, FNV-1a 64) — rewriting the check whole through a `0600` temporary and a rename; past 400 records or 1,048,576 bytes, or on a failed write, the recording ends, its check removed, and `recording_failed()` reads true once, the call not rolled back; `Check::from_bytes` reads a kept check strictly, anything else refused `malformed`; `run`'s signature and its `command` span are unchanged — ratified by the founder (2026-10-19, F1 A · F2 B) (packages/escher-driver/src/recording.rs:125-176; packages/escher-driver/src/session.rs:27-203;»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:6
  old: «`StateDirNotPrivate`, `Io`, and `Unsupported` on a non-unix target»
  new: «`StateDirNotPrivate`, `Io`, `Unsupported` on a non-unix target and, since 2026-10-19-session-recording, `RecordDirNotPrivate` (a recording directory open to its group or others) and `RecordingFailed` (a recording that ended, read from an `ok v3 recording-failed` reply) — fifteen session errors, each with a fixed meaning and remedy»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:7
  old: «version `v2` on every request since 2026-10-10-driver-cli — `hello v2` → `ok v2 pid={pid} label={label} served={n} idle_expiry_s={s}`, `stop v2` → `ok v2 stopping`, `call v2 <verb> [<name>=<kind>:<value>]…` → `ok v2 accepted <json>` · `ok v2 refused <json>` · `ok v2 oversize`,»
  new: «version `v3` on every request since 2026-10-19-session-recording (`v2` from 2026-10-10-driver-cli) — `hello v3` → `ok v3 pid={pid} label={label} served={n} idle_expiry_s={s}`, `stop v3` → `ok v3 stopping`, `call v3 <verb> [<name>=<kind>:<value>]…` → `ok v3 accepted <json>` · `ok v3 refused <json>` · `ok v3 oversize` · `ok v3 recording-failed` (sent in place of the answer when the host's recording ended after the call, read as `SessionError::RecordingFailed`),»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9
  old: «`start <task>`, `status`, `stop`, then»
  new: «`start <task> [--record <dir>]`, `status`, `stop`, then»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9
  old: «the host role is `serve <task> --session <dir>`; since 2026-10-10-mcp-surface the `mcp` role is `mcp` alone (anything after it usage)»
  new: «the host role is `serve <task> --session <dir> [--record <dir>]`; since 2026-10-10-mcp-surface the `mcp` role is `mcp` alone or, since 2026-10-19-session-recording, `mcp --record <dir>` (anything else after it usage)»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9
  old: «the two-argument form `<task> <state-dir>` now reads as an unknown verb (refused, status `1`);»
  new: «the two-argument form `<task> <state-dir>` now reads as an unknown verb (refused, status `1`); since 2026-10-19-session-recording `--record <dir>` is taken at most once on `start`, the host role and the `mcp` role, and anywhere else, twice or with no value it is usage, status `2`, once the call passed `validate` (a stray `--record` was an unnamed argument, status `1`, before); `start` checks the recording directory before it spawns and passes `--record` to the host, a refusal status `3` with nothing spawned; `mcp --record` checks it before it serves, a refusal status `3`, its fixed message on stderr and stdout empty; `RecordDirNotPrivate` and `RecordingFailed` end as session errors, status `3`; the usage line reads `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]` and `mcp role: mcp [--record <dir>]`; nothing the caller typed is written back;»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:10
  old: «so the windows job runs it;»
  new: «so the windows job runs it; since 2026-10-19-session-recording a check that records is `cfg(unix)`-gated item by item, the in-process and `mcp` stdio ones included (seven_guis `mcp_log`'s recorded test, `recorded_needles`, `the_check`), beside an off-unix check that a recording is refused `Unsupported` (`recording.rs`'s `off_unix_tests`, `session.rs`, `mcp_tests.rs` and seven_guis `recording`), which the windows job alone runs;»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
  old: «`stand_session_quiet` 1 (+1 ignored host child) · seven_guis `host_binary` 2 · seven_guis `host_log` 1 (the»
  new: «`stand_session_quiet` 1 (+1 ignored host child) · since 2026-10-19-session-recording `stand_session_recording` 3 (a recorded session replays on the four lean tasks in both layout modes, every step's acceptance and digest equal, keeping no screen reading; a password typed in a recorded session kept as the stand-in, 0 sentinel occurrences and a secret of another length leaving byte-identical files; a recording directory open to its group refused with nothing written) and seven_guis `recording` 3 (+1 off-unix: the command line and the `mcp` role keep the same check, the state directory gone after `stop`; a group-open recording directory refused before anything runs; the `mcp` role's listing and `help` the same with and without `--record`) · seven_guis `host_binary` 2 · seven_guis `host_log` 2 (1 until 2026-10-19-session-recording added a recording host that logs nothing of its check; the»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
  old: «`mcp_log` 1 and `stand_mcp_tools` 5»
  new: «`mcp_log` 2 (1 until 2026-10-19-session-recording added a recording `mcp` role that logs nothing of its checks) and `stand_mcp_tools` 5»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
  old: «escher-driver's 74 unit tests (cli.rs 5 · client.rs 3 · command.rs 7 · describe.rs 2 · error.rs 5 · execute.rs 3 · image.rs 5 · json.rs 7 · mcp_tests.rs 12 · refusal.rs 3 · schema_tests.rs 6 · session.rs 4 · wire.rs 12 —»
  new: «escher-driver's 87 unit tests (cli.rs 5 · client.rs 3 · command.rs 7 · describe.rs 2 · error.rs 5 · execute.rs 3 · image.rs 5 · json.rs 7 · mcp_tests.rs 14 · recording.rs 9 · refusal.rs 3 · schema_tests.rs 6 · session.rs 6 · wire.rs 12, and three off-unix tests the windows job alone runs — 74 until 2026-10-19-session-recording, as measured at escher-0.2.0/chunks/2026-10-19-session-recording/report.md;»
