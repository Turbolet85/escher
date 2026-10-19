proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle (session-proof)"
    change: "Add since 2026-10-19-session-recording `stand_session_recording` 3 (a recorded session replays on the four lean tasks in both layout modes, every step's acceptance and digest equal, keeping no screen reading; a password typed in a recorded session kept as the stand-in `MASKED_STAND_IN`, 0 sentinel occurrences and a secret of another length leaving byte-identical files; a recording directory open to its group refused with nothing written) and seven_guis `recording` 3 (+1 off-unix: the command line and the `mcp` role keep the same check, the state directory gone after `stop`; a group-open recording directory refused before anything runs; the `mcp` role's listing and `help` the same with and without `--record`); `host_log` 1 → 2 and `mcp_log` 1 → 2 (a recording host and a recording `mcp` role log nothing of their checks at `trace`); escher-driver's 74 unit tests → 87 (recording.rs 9 · session.rs 6 · mcp_tests.rs 14 added to the per-file list, their three off-unix tests run on the windows leg only), as measured at escher-0.2.0/chunks/2026-10-19-session-recording/report.md, with CI#39202401256 on `3cbb0eeef` (17/17 green) the witness of the off-unix tests' windows build."
    sidecar: "§3 Session lifecycle session-proof: stand_session_recording 3, seven_guis recording 3 (+1 off-unix), host_log and mcp_log 2 each, escher-driver 87 unit tests (2026-10-19-session-recording)."
    rationale: "Report §Changes New files (`stand_session_recording.rs`, seven_guis `recording.rs`) and Coverage of new surfaces (recording.rs 9 + off-unix 1, stand_session_recording 3, seven_guis recording 3 + off-unix 1); Outcome Gates `cargo test -p escher-driver --locked` 87 unit tests; New text rows add `a_recording_host_logs_nothing_of_its_check_at_trace` (host_log.rs:30-34) and `a_recording_mcp_role_logs_nothing_of_its_checks_at_trace` (mcp_log.rs:56-64), session.rs tests 284-333, mcp_tests.rs tests 587-688. The proof list names none of them."
    basis: "packages/escher-driver/src/recording.rs:439-901; packages/escher-driver/src/session.rs:265-333; packages/escher-driver/src/mcp_tests.rs:587-689; tests/blitz-tests/tests/stand_session_recording.rs:126-343; examples/seven_guis/tests/recording.rs:107-284"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle (session-wire)"
    change: "Version `v3` on every request since 2026-10-19-session-recording (`hello v3` → `ok v3 pid=…`, `stop v3` → `ok v3 stopping`, `call v3 …` → `ok v3 accepted <json>` · `ok v3 refused <json>` · `ok v3 oversize` · and the one new fixed reply `ok v3 recording-failed` (`Reply::RecordingFailed`), which a host sends in place of the answer when `recording_failed()` reads true after a call and the client reads as `SessionError::RecordingFailed`); a `v2` request now reads `refused version`; no other request, reply or bound changed."
    sidecar: "§3 Session lifecycle session-wire: wire v2 → v3 with the `ok v3 recording-failed` reply (2026-10-19-session-recording)."
    rationale: "Report §Changes Symbols / APIs (The wire moves `v2` → `v3`) and Counts [wire] naming `session-lifecycle.md:7` changed; Spec claims disproved: `call v2 snapshot` now reads `refused version`."
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:7"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle (session-binary)"
    change: "`--record <dir>` taken at most once on `start <task> --session <dir> --record <dir>`, on the host role `serve <task> --session <dir> [--record <dir>]` and on the `mcp` role `mcp [--record <dir>]` (no longer `mcp` alone); anywhere else, twice or with no value it is usage, status `2`, once the call passed `validate` (a stray `--record` was an unnamed argument, status `1`, before 2026-10-19-session-recording); `start` checks the recording directory before it spawns and passes `--record` to the host, a refusal status `3` with nothing spawned; `mcp --record` checks it before it serves, a refusal status `3`, its fixed message on stderr and stdout empty; the usage line reads `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]` and `mcp role: mcp [--record <dir>]`; `RecordDirNotPrivate` and `RecordingFailed` end as session errors, status `3`; nothing the caller typed is written back."
    sidecar: "§3 Session lifecycle session-binary: `--record <dir>` on start · serve · mcp, its usage and exit-3 edges (2026-10-19-session-recording)."
    rationale: "Report §Changes Symbols / APIs (The command line) and Counts [usage] naming `session-lifecycle.md:9` (@c645, @c724) changed; Expected amendment 5 names session-binary `--record` and the two errors."
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:9"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle (session-start)"
    change: "Add: since 2026-10-19-session-recording `Session::recording_into(dir)` (unix only, `Unsupported` elsewhere) creates `dir` `0700` or refuses `RecordDirNotPrivate`, then `check-{n}.json` `0600` with `create_new`, never replacing a file; `run` then records each answered call of the seven instance verbs — `tool` · `arguments` (schema order) · `masked` (a `type` on a masked or unresolved target, its text the stand-in `MASKED_STAND_IN`) · `accepted` · `digest` (`answer_digest`, FNV-1a 64) — rewriting the check whole through a `0600` temporary and a rename; past 400 records or 1,048,576 bytes, or on a failed write, the recording ends, its check removed, and `recording_failed()` reads true once, the call not rolled back; `Check::from_bytes` reads a kept check strictly, anything else refused `malformed`; `run`'s signature and its `command` span unchanged."
    sidecar: "§3 Session lifecycle session-start: recording_into · recording_failed · the kept check and its reader (2026-10-19-session-recording)."
    rationale: "Report §Changes Symbols / APIs (`Session::recording_into`, `Session::recording_failed`, `Session::run` records, `Check::from_bytes`) and Schema / config (the kept-check form and its bounds); the session contract the checks drive states none of it."
    basis: "packages/escher-driver/src/session.rs:127-156; packages/escher-driver/src/recording.rs:125-176"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle (session-host)"
    change: "The edges gain `RecordDirNotPrivate` (a recording directory open to its group or others) and `RecordingFailed` (a recording that ended — read from an `ok v3 recording-failed` reply), each with a fixed meaning and remedy, fifteen session errors in all."
    sidecar: "§3 Session lifecycle session-host: two new session-error edges, RecordDirNotPrivate and RecordingFailed (2026-10-19-session-recording)."
    rationale: "Report §Changes Symbols / APIs (`SessionError::RecordDirNotPrivate`, `SessionError::RecordingFailed`, `SESSION_ERRORS` 13 → 15) — the session-host bullet's edge list names the thirteen."
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle (session-checks)"
    change: "Add: a check that records is `cfg(unix)`-gated item by item, the in-process and `mcp` stdio ones included (seven_guis `mcp_log`'s recorded test, `recorded_needles`, `the_check`), beside an off-unix check that a recording is refused `Unsupported` (recording.rs `off_unix_tests`, session.rs, mcp_tests.rs and seven_guis `recording`), which the windows job alone runs."
    sidecar: "§3 Session lifecycle session-checks: recording checks unix-gated with off-unix Unsupported checks (2026-10-19-session-recording)."
    rationale: "Report Deviations 4 and 13 (recording unix-only; the off-unix tests and the `#[cfg(unix)]` gates the operator pass added) and Insufficient fixes (two Windows CI reds, CI#39199277925 and CI#39201074261, from an ungated recorded check and a helper calling a gated item)."
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → escher-driver"
    change: "87 unit tests in fourteen files (74 in thirteen until 2026-10-19-session-recording, which added recording.rs 9 — the digest is FNV-1a 64 of the written answer, a recording reads back as the calls it kept, a masked `type` keeps the stand-in and nothing of the text, a call no record can spell is not kept, the check and its directory are owner-only and no file is replaced, a recording past its bound or unable to write ends and removes its check, a check in the grammar is read, one outside it refused `malformed` (35 rows), the bounds read and one past them refused — session.rs 2 and mcp_tests.rs 2, and three off-unix tests the windows job alone runs); describe.rs's count row reads `[11, 13, 12, 3, 8, 4, 15]`; cli.rs's line rows include `--record`; as measured at escher-0.2.0/chunks/2026-10-19-session-recording/report.md."
    sidecar: "§1 escher-driver: 74 → 87 unit tests, recording.rs's nine added (2026-10-19-session-recording)."
    rationale: "Report Outcome Gates (87 unit tests after the operator pass), Coverage of new surfaces (recording.rs 9 + off-unix 1), Spec claims disproved (describe.rs's count row `[11, 13, 12, 3, 8, 4, 15]`) and New text rows for recording.rs, session.rs and mcp_tests.rs tests."
    basis: "test-plan.md:22; packages/escher-driver/src/recording.rs:439-901"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → escher-driver"
    change: "error.rs: every `SessionError` message — all fifteen variants — is non-empty, and the fifteen errors each state a kind, a meaning and a remedy (thirteen until 2026-10-19-session-recording added `RecordDirNotPrivate` and `RecordingFailed`)."
    sidecar: "§1 escher-driver error.rs: thirteen → fifteen session errors (2026-10-19-session-recording)."
    rationale: "Report §Changes Symbols / APIs: `SESSION_ERRORS` grows 13 → 15; the [serr] sweep reached test-plan.md:144 only, while :22 restates the thirteen twice («all thirteen variants», «the thirteen errors each state»)."
    basis: "test-plan.md:22"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → escher-driver"
    change: "wire.rs: each request and each reply of the `v3` grammar round-trips — the `ok v3 recording-failed` reply among them (`v2` until 2026-10-19-session-recording); the dated «the `v2` wire's rows at … driver-cli» history stays."
    sidecar: "§1 escher-driver wire.rs: v2 → v3 grammar rows (2026-10-19-session-recording)."
    rationale: "Report Counts [wire] names test-plan.md:22 changed, its @c8570 dated offset no-change."
    basis: "test-plan.md:22"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: "Add since 2026-10-19-session-recording: recording.rs pins the FNV-1a 64 digest of the written answer, a recording's read-back, the masked `type` stand-in, the calls no record can spell left out, owner-only modes with no file replaced and no foreign temporary removed, a recording ending at its bound or on a failed write with its check removed, and `Check::from_bytes`'s strict grammar (35 refusal rows, edge rows at both bounds); session.rs that a recording directory open to others is refused with nothing written and that a recording that ends reads failed once; mcp_tests.rs that each session a recording boot starts is kept as one check and that a call that ends the recording is answered `isError` as its session error; client.rs that `ok v3 recording-failed` reads as `SessionError::RecordingFailed`; off-unix, a recording refused `Unsupported`."
    sidecar: "§4 escher-driver: the recorder's, the check reader's and the recording-failed unit tests (2026-10-19-session-recording)."
    rationale: "Report Coverage of new surfaces (the kept check, `Check::from_bytes`, wire/MCP `recording-failed` — tests unit `wire.rs`, `client.rs`, `mcp_tests.rs`) and Reverted / negative API facts (the foreign-temporary removal a unit test caught)."
    basis: "test-plan.md:144; packages/escher-driver/src/recording.rs:439-901; packages/escher-driver/src/mcp_tests.rs:587-689"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: "wire.rs pins the closed grammar at `v3` — `call v3 …`, `ok v3 accepted <json>` · `ok v3 refused <json>` · `ok v3 oversize` and `ok v3 recording-failed` round-trip (`v2` until 2026-10-19-session-recording)."
    sidecar: "§4 escher-driver wire.rs: grammar pinned at v3 (2026-10-19-session-recording)."
    rationale: "Report Counts [wire] names test-plan.md:144 changed."
    basis: "test-plan.md:144"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: "error.rs pins that each of the fifteen session errors states a kind, a meaning and a remedy (thirteen until 2026-10-19-session-recording)."
    sidecar: "§4 escher-driver error.rs: fifteen session errors (2026-10-19-session-recording)."
    rationale: "Report Counts [serr] names test-plan.md:144 «the thirteen session errors» changed."
    basis: "test-plan.md:144"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → dioxus-native-dom"
    change: "73 unit tests (72 until 2026-10-19-session-recording, which added `a_password_or_file_input_is_masked_and_nothing_else_is` — seven rows over the new public `is_masked_input`, the snapshot `value` reader's mask test, no snapshot reading changed (packages/dioxus-native-dom/src/snapshot.rs:687-724))."
    sidecar: "§1 dioxus-native-dom: 72 → 73 unit tests, is_masked_input's test (2026-10-19-session-recording)."
    rationale: "Report Deviation 10 (dioxus-native-dom gained one unit test), Coverage of new surfaces (`is_masked_input` tests unit, 7 rows) and New text row snapshot.rs:687-724."
    basis: "packages/dioxus-native-dom/src/snapshot.rs:687-724"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-dom (clock)"
    change: "Extend: since 2026-10-19-session-recording blitz-dom's `iframe::tests::a_sub_documents_config_carries_its_parents_clock` pins a sub-document's config holding its parent's clock (the same `Arc`) (packages/blitz-dom/src/iframe.rs:194-212), and `virtual_clock_engine` also reads pan and the fling."
    sidecar: "§1 blitz-dom (clock): the iframe clock test and virtual_clock_engine's pan and fling reads (2026-10-19-session-recording)."
    rationale: "Report Counts [vce] names test-plan.md:16 extend; Outcome (tests, CARRY) cites blitz-dom `iframe` `a_sub_documents_config_carries_its_parents_clock`; New text row iframe.rs:194-212."
    basis: "test-plan.md:16; packages/blitz-dom/src/iframe.rs:187-213"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "`virtual_clock_engine` 5 (4 until 2026-10-19-session-recording, which added a finger pan and its fling moving only with the clock, 3 cases in both layout modes (tests/blitz-tests/tests/virtual_clock_engine.rs:232-287))."
    sidecar: "§1 tests/blitz-tests: virtual_clock_engine 4 → 5, the pan and fling test (2026-10-19-session-recording)."
    rationale: "Report Counts [vce] names test-plan.md:27 @c46343 «`virtual_clock_engine` 4» changed; Outcome (tests, CARRY) cites `a_finger_pan_and_its_fling_move_only_with_the_clock` (3 cases × 2 modes)."
    basis: "test-plan.md:27; tests/blitz-tests/tests/virtual_clock_engine.rs:232-287"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "Add to the session checks: since 2026-10-19-session-recording `stand_session_recording` 3, declaring `mod session_common;` — a recorded session replays on each lean task in both layout modes with every step's acceptance and digest equal and keeps no screen reading at rest; a password typed in a recorded session is kept as the stand-in (0 sentinel occurrences, a secret of another length leaving byte-identical files); a recording directory open to its group is refused with nothing written (tests/blitz-tests/tests/stand_session_recording.rs:126-343)."
    sidecar: "§1 tests/blitz-tests: stand_session_recording 3 joins the session checks (2026-10-19-session-recording)."
    rationale: "Report §Changes New file `tests/blitz-tests/tests/stand_session_recording.rs`; Outcome (security, owner-only / password / no screen reading; tests, round trip); the blitz-tests session-check list (stand_session_state … stand_session_quiet) names no recording check."
    basis: "test-plan.md:27; tests/blitz-tests/tests/stand_session_recording.rs:126-343"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions (Directory pattern)"
    change: "135 `.rs` files, 134 test targets and the unbuilt `all.rs` (134 and 133 until 2026-10-19-session-recording added `stand_session_recording.rs`, its own auto-discovered target)."
    sidecar: "§2 Directory pattern: blitz-tests 134 → 135 `.rs` files, 133 → 134 targets (2026-10-19-session-recording)."
    rationale: "Report Counts [btfiles] names test-plan.md:67 @c2428 «134 `.rs` files, 133 test targets» changed."
    basis: "test-plan.md:67"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions (Directory pattern)"
    change: "`tests/session_common/mod.rs`, read through `mod session_common;` by 24 checks (23 before `stand_session_recording.rs`)."
    sidecar: "§2 Directory pattern: session_common readers 23 → 24 (2026-10-19-session-recording)."
    rationale: "Report Counts [readers] names test-plan.md:67 @c1090 «by 23 checks» changed."
    basis: "test-plan.md:67"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers (blitz-tests (session checks))"
    change: "The module's helpers gain, since 2026-10-19-session-recording, `replay(session, check)` (each kept step run again, its acceptance and digest), `kept(check)` (each kept step's acceptance and digest) and the unix-only `mode(path)` (a file's permission bits) (tests/blitz-tests/tests/session_common/mod.rs:240-273); `stand_session_recording` declares `mod session_common;` (24 readers; 23 before `stand_session_recording`)."
    sidecar: "§3 Crate-local helpers (session checks): replay · kept · mode added, readers 23 → 24 (2026-10-19-session-recording)."
    rationale: "Report Counts [readers]: test-plan.md:102 @c43 extend (the module's stated helpers gain `replay`, `kept` and the unix-only `mode`) and @c2393 «23 readers» changed; New text rows session_common/mod.rs:240-273."
    basis: "test-plan.md:102; tests/blitz-tests/tests/session_common/mod.rs:240-273"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → apps (rdme, examples, accesskit_xplat)"
    change: "`examples/seven_guis` holds eight integration-test targets (seven until 2026-10-19-session-recording added `tests/recording.rs` — 3 tests, unix only, and 1 off-unix: the command line and the `mcp` role keep the same check byte for byte, the state directory gone after `stop`; a recording directory open to its group refused before anything runs; the `mcp` role's listing and `help` the same with and without `--record`; off unix, `--record` refused `Unsupported` — run as `cargo test -p seven_guis --locked --test recording` and in the stand-repeat leg)."
    sidecar: "§1 apps: seven_guis integration targets 7 → 8, `tests/recording.rs` (2026-10-19-session-recording)."
    rationale: "Report Counts [sgtargets] names test-plan.md:10 changed (seven_guis test targets 7 → 8, the leg's command gaining `--test recording`); New file examples/seven_guis/tests/recording.rs and its New text rows; Coverage of new surfaces (seven_guis recording 3 + off-unix 1)."
    basis: "test-plan.md:10; examples/seven_guis/tests/recording.rs:107-284"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → apps (rdme, examples, accesskit_xplat)"
    change: "`tests/common/mod.rs` … read through `mod common;` by all eight (all seven until `tests/recording.rs`, which drives the `mcp` role through the shared `Speaker`)."
    sidecar: "§1 apps: seven_guis `mod common;` readers seven → eight (2026-10-19-session-recording)."
    rationale: "The seven_guis target count restated as «by all seven» on the same line; report Deviation 10 keeps `examples/seven_guis/tests/common/mod.rs` unchanged while `recording.rs` builds on its `Speaker` (New text row recording.rs:86-93 «fn mcp(record) -> Speaker»)."
    basis: "test-plan.md:10; examples/seven_guis/tests/recording.rs:86-93"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → apps (rdme, examples, accesskit_xplat)"
    change: "`tests/host_log.rs` (2 tests, unix only; 1 until 2026-10-19-session-recording added a recording host that logs nothing of its check at `trace`) and `tests/mcp_log.rs` (2 tests; 1 until that chunk added a recording `mcp` role that logs nothing of its checks at `trace`, unix-gated with its `recorded_needles`)."
    sidecar: "§1 apps: host_log and mcp_log 1 → 2 tests each, the recorded runs (2026-10-19-session-recording)."
    rationale: "Report New text rows host_log.rs:30-34 and mcp_log.rs:56-64; Outcome (obs) cites both tests; Deviation 13 the unix gate on `mcp_log`'s recorded test."
    basis: "test-plan.md:10; examples/seven_guis/tests/host_log.rs:30-61; examples/seven_guis/tests/mcp_log.rs:56-91"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → apps (rdme, examples, accesskit_xplat)"
    change: "The host role reads `serve <task> --session <dir> [--record <dir>]`."
    sidecar: "§1 apps: the host role shape gains `[--record <dir>]` (2026-10-19-session-recording)."
    rationale: "Report Counts [usage] names test-plan.md:10 @c1250 changed, @c4298 no-change."
    basis: "test-plan.md:10"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Stand log format"
    change: "`escher-session` in its host role, `serve <task> --session <dir> [--record <dir>]`, runs each `call` request through `Session::run` and writes one closed `command` span line per call run — the same with a recording held."
    sidecar: "§3 Stand log format: the host role shape gains `[--record <dir>]`, spans unchanged with recording on (2026-10-19-session-recording)."
    rationale: "Report Counts [usage] names test-plan.md:104 @c840 changed; Outcome (obs): span counts unchanged on the recorded runs."
    basis: "test-plan.md:104"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Session host ↔ what it writes"
    change: "Add: since 2026-10-19-session-recording `host_log` and `mcp_log` each also run a recording host and a recording `mcp` role at `RUST_LOG=trace` and read 0 of the recording needles (the kept check's names and its directory's name) on stderr, the span counts unchanged; the kept check lives in the launcher's recording directory, the state directory still `session.sock` alone."
    sidecar: "§5 Session host ↔ what it writes: the recorded host_log and mcp_log runs, 0 recording needles (2026-10-19-session-recording)."
    rationale: "Report Outcome (obs) `a_recording_host_logs_nothing_of_its_check_at_trace` and `a_recording_mcp_role_logs_nothing_of_its_checks_at_trace`: 0 of the recording needles, span counts unchanged; Counts [statedir]."
    basis: "examples/seven_guis/tests/host_log.rs:30-61; examples/seven_guis/tests/mcp_log.rs:56-91"
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → MCP (`escher-session mcp`)"
    change: "`mcp_log` (2, 1 until 2026-10-19-session-recording) reads its stderr at `trace`, a recording role's included; since that chunk seven_guis `recording` speaks the role with `--record`: its listing and `help` the same bytes with and without it, and its kept check byte-identical to the command line's."
    sidecar: "§6 MCP: mcp_log 2 and seven_guis recording's `mcp --record` runs (2026-10-19-session-recording)."
    rationale: "Report Outcome (tests, CLI = MCP) `the_command_line_and_the_mcp_role_keep_the_same_check` and (layouts / design) listing and `help` identical with and without `--record`; New text row mcp_log.rs:56-64."
    basis: "examples/seven_guis/tests/recording.rs:107-247"
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Command line (`escher-session`)"
    change: "Add: since 2026-10-19-session-recording `start <task> --record <dir>` run as a process keeps one `0600` check in a `0700` directory, the state directory gone after `stop`, a group-open recording directory refused status `3` with nothing spawned (seven_guis `recording`), and `cli_commands` reads the usage line naming `--record` on `start`, the host role and the `mcp` role, a misplaced `--record` a usage error, status `2`."
    sidecar: "§6 Command line: `--record` on start, its refusal and the usage line (2026-10-19-session-recording)."
    rationale: "Report Symbols / APIs (The command line) and Outcome (security, owner-only; layouts / design: the usage line names `--record` on the three (`cli_commands`)); New text rows cli_commands.rs and recording.rs:180-220."
    basis: "examples/seven_guis/tests/recording.rs:180-220; examples/seven_guis/tests/cli_commands.rs:502-512"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts (Legs)"
    change: "stand-repeat: one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 43 files, `stand_session_recording` joining by its prefix — then `cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server`, `mcp_log` and, since 2026-10-19-session-recording, `recording`; `test_ci_workflows.py`'s `test_stand_repeat_leg_runs_three_rounds` pins both words."
    sidecar: "§9 Legs: stand-repeat's seven_guis list gains `recording` (2026-10-19-session-recording)."
    rationale: "Report Harness / gate surface (ci-leg.sh stand-repeat adds `--test recording`; `stand_session_recording` joins by its `stand_` prefix; test_ci_workflows.py pins both words) and Counts [sgtargets] naming test-plan.md:318."
    basis: "test-plan.md:318; .github/scripts/ci-leg.sh:57; .github/scripts/test_ci_workflows.py:358-359"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts (Legs)"
    change: "«— 42 files —» → «— 43 files —» (42 until 2026-10-19-session-recording added `stand_session_recording.rs`)."
    sidecar: "§9 Legs: stand-repeat's stand_* file count 42 → 43 (2026-10-19-session-recording)."
    rationale: "The [standf] sweep reached test-plan.md:347 and architecture alone, yet test-plan.md:318 restates the count «with one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 42 files»; report Harness / gate surface: `stand_session_recording` joins by its prefix."
    basis: "test-plan.md:318"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets → Flakiness budget"
    change: "The `stand-repeat` leg runs the 43 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (42 and seven until 2026-10-19-session-recording); this chunk's run read `stand-repeat: 3 of 3 rounds green`."
    sidecar: "§10 Flakiness budget: 42 → 43 stand files, seven → eight seven_guis checks (2026-10-19-session-recording)."
    rationale: "Report Counts [standf] names test-plan.md:347 «the 42 `stand_*.rs` files and seven_guis' seven process …» changed (43, and eight); Outcome Gates stand-repeat `3 of 3 rounds green`."
    basis: "test-plan.md:347"
note: "D-tests-framework and D-tests-obs-harness hold: the chunk's tests run on the built-in Rust harness through cargo (the windows-gnu `cargo check --tests` is a compile check, no runner), and neither the 5-command agent-run harness, its status shape nor its event format changed (report Harness / gate surface: no agent-run or CI workflow step changed); the stand log format's host-role shape moves on both sides (test-plan.md:104 here, obs-plan.md:183 in the report's [usage] rows), so obs-plan's matching proposal must land with it. Lines cited without a report-stated number (§1 dioxus-native-dom, §5, §6, session-lifecycle session-start/-host/-checks/-proof) are named by section only."
