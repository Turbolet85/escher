group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 58 · «- 2026-10-15: In a contract test whose expected total follow»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`escher-driver`: 74 unit tests in thirteen files, 56 inline and, since 2026-10-16-file-splits, 18 in two `#[path]` test files (73 until 2026-10-16-fast-failing-mutants»
  source: .andromeda/test-plan.md:22 «- **escher-driver:** 87 unit tests in fourteen files, 67 inline and, since 2026-10-16-file-splits, 20 in two `#[path]` test files, beside three off-unix tests the windows job alone runs»
  replace-with: «`escher-driver`: 87 unit tests in fourteen files, 67 inline and, since 2026-10-16-file-splits, 20 in two `#[path]` test files, beside three off-unix tests the windows job alone runs (74 in thirteen until 2026-10-19-session-recording, 73 until 2026-10-16-fast-failing-mutants»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`error.rs` 5 (thirteen `SessionError` messages»
  source: .andromeda/test-plan.md:22 «fifteen since 2026-10-19-session-recording added `RecordDirNotPrivate` and `RecordingFailed`»
  replace-with: «`error.rs` 5 (fifteen `SessionError` messages, thirteen until 2026-10-19-session-recording added `RecordDirNotPrivate` and `RecordingFailed`»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`session.rs` 4 (the label rule;»
  source: .andromeda/test-plan.md:22 «session.rs 6 (4 until 2026-10-19-session-recording, which added that a recording directory open to others is refused with nothing written and that a recording that ends reads failed once»
  replace-with: «`session.rs` 6 (4 until 2026-10-19-session-recording, which added a recording directory open to others refused with nothing written and a recording that ends reading failed once; the label rule;»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`wire.rs` 12 (the closed `v2` grammar: `hello`, `stop` and `call`, the lifecycle replies and `accepted` · `refused` · `oversize`»
  source: .andromeda/test-plan.md:22 «of the `v3` grammar — `v2` until 2026-10-19-session-recording — round-trips: `hello`, `stop` and `call`, the lifecycle replies and `accepted` · `refused` · `oversize` · `recording-failed`»
  replace-with: «`wire.rs` 12 (the closed `v3` grammar, `v2` until 2026-10-19-session-recording: `hello`, `stop` and `call`, the lifecycle replies and `accepted` · `refused` · `oversize` · `recording-failed`»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`mcp_tests.rs` 12 (`mcp::tests`, in `mcp.rs` until 2026-10-16-file-splits; 11 until»
  source: .andromeda/test-plan.md:22 «mcp_tests.rs 14 (12 until 2026-10-19-session-recording, which added that each session a recording boot starts is kept as one check and that a call that ends the recording is answered»
  replace-with: «`mcp_tests.rs` 14 (12 until 2026-10-19-session-recording, which added each session a recording boot starts kept as one check and a call that ends the recording answered as its session error; `mcp::tests`, in `mcp.rs` until 2026-10-16-file-splits; 11 until»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «`describe.rs` 2 (the document holds `help`'s seven fields in order; every text in it is its tables')»
  source: .andromeda/test-plan.md:22 «since 2026-10-19-session-recording recording.rs 9 (the digest is FNV-1a 64 of the written answer; a recording reads back as the calls it kept; a masked `type` keeps the stand-in»
  replace-with: «`describe.rs` 2 (the document holds `help`'s seven fields in order; every text in it is its tables'), since 2026-10-19-session-recording `recording.rs` 9 (the FNV-1a 64 digest of the written answer; a recording read back as the calls it kept; a masked `type` keeping the stand-in and nothing of the text; a call no record can spell not kept; the check and its directory owner-only, no file replaced; a recording past its bound or on a failed write ending with its check removed, never a temporary it did not create; a check in the grammar read, one outside it refused `malformed`, 35 rows; a check at its bounds read and one past them refused)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «and `mcp_log` (1) and in process by `stand_mcp_tools`»
  source: .andromeda/test-plan.md:10 «`tests/mcp_log.rs` (2 tests — 1 until 2026-10-19-session-recording added a recording `mcp` role that logs nothing of its checks at `trace`, unix-gated with its `recorded_needles`»
  replace-with: «and `mcp_log` (2, 1 until 2026-10-19-session-recording added a recording `mcp` role that logs nothing of its checks at `trace`, unix-gated) and in process by `stand_mcp_tools`»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «— the two seven_guis targets not unix-gated but `mcp_server`'s two command-line comparisons, run by the windows job too —»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:10 «since 2026-10-19-session-recording a check that records is `cfg(unix)`-gated item by item, the in-process and `mcp` stdio ones included (seven_guis `mcp_log`'s recorded test, `recorded_needles`»
  replace-with: «— the two seven_guis targets not unix-gated but `mcp_server`'s two command-line comparisons and, since 2026-10-19-session-recording, `mcp_log`'s recorded test, run by the windows job too —»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «and a host ending at its idle expiry as on `stop`); seven_guis' `host_binary` (2)»
  source: .andromeda/test-plan.md:27 «a recorded session — `stand_session_recording` 3, declaring `mod session_common;`: a recorded session replays on each lean task in both layout modes with every step's acceptance and digest equal»
  replace-with: «and a host ending at its idle expiry as on `stop`) and, since 2026-10-19-session-recording, `stand_session_recording` (3: a recorded session replaying on each lean task in both layout modes with every step's acceptance and digest equal and keeping no screen reading at rest, a password typed in a recorded session kept as the stand-in — 0 sentinel occurrences, a secret of another length leaving byte-identical files — and a recording directory open to its group refused with nothing written); seven_guis' `host_binary` (2)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «and `host_log` (1) reads a host at `RUST_LOG=trace` while three calls run»
  source: .andromeda/test-plan.md:10 «`tests/host_log.rs` (2 tests, unix only; 1 until 2026-10-19-session-recording added a recording host that logs nothing of its check at `trace`)»
  replace-with: «and `host_log` (2, 1 until 2026-10-19-session-recording added a recording host that logs nothing of its check at `trace`) reads a host at `RUST_LOG=trace` while three calls run»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «`host_timer` (1) the timer host by a hosted `advance`,»
  source: .andromeda/test-plan.md:10 «since 2026-10-19-session-recording `tests/recording.rs` (3 tests, unix only, and 1 off-unix) — the command line and the `mcp` role keep the same check byte for byte»
  replace-with: «`host_timer` (1) the timer host by a hosted `advance`, since 2026-10-19-session-recording `recording` (3, unix only, and 1 off-unix: the command line and the `mcp` role keeping the same check byte for byte, the state directory gone after `stop`; a recording directory open to its group refused before anything runs; the `mcp` role's listing and `help` the same with and without `--record`; off unix `--record` refused `Unsupported`),»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «`host_log` and `mcp_log` also drive shots and read 0 occurrences of a shot's first 64 base64 characters on stderr at `trace`, under both builds.»
  source: .andromeda/test-plan.md:185 «since 2026-10-19-session-recording `host_log` and `mcp_log` each also run a recording host and a recording `mcp` role at `RUST_LOG=trace` and read 0 of the recording needles»
  replace-with: «`host_log` and `mcp_log` also drive shots and read 0 occurrences of a shot's first 64 base64 characters on stderr at `trace`, under both builds; since 2026-10-19-session-recording each also runs a recording host or a recording `mcp` role at `RUST_LOG=trace` and reads 0 of the recording needles (the kept check's and its directory's name) on stderr, the span counts unchanged.»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`virtual_clock_engine` (4: an engine check on a parsed page, both layout modes — the scrollbar fade moving only with the clock,»
  source: .andromeda/test-plan.md:27 «`virtual_clock_engine` 5 (4 until 2026-10-19-session-recording added a finger pan and its fling moving only with the clock, 3 cases in both layout modes»
  replace-with: «`virtual_clock_engine` (5, 4 until 2026-10-19-session-recording added a finger pan and its fling moving only with the clock: an engine check on a parsed page, both layout modes — the scrollbar fade moving only with the clock,»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «three rounds of the 42 `stand_*.rs` files and seven_guis' seven process checks»
  source: .andromeda/test-plan.md:318 «43 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording»
  replace-with: «three rounds of the 43 `stand_*.rs` files and seven_guis' eight process checks (42 and seven until 2026-10-19-session-recording added `stand_session_recording` and `recording`)»

- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «the 42 `stand_*.rs` files and seven_guis' seven process checks run three times with no retry»
  source: .andromeda/test-plan.md:347 «run the 43 `stand_*.rs` files and seven_guis' eight process checks three times with no retry»
  replace-with: «the 43 `stand_*.rs` files and seven_guis' eight process checks run three times with no retry (42 and seven until 2026-10-19-session-recording)»

- leaf: .claude/docs/tests-summary.md:17   class: omission
  leaf-text: «the `escher-session` host writes one `command` span line per call it runs»
  source: .andromeda/test-plan.md:104 «writes one closed `command` span line under `escher_driver` per call run, the same with a recording held»
  replace-with: «the `escher-session` host writes one `command` span line per call it runs, the same with a recording held»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «`stand_requirement_sweep` and `stand_virtual_clock` (23 readers;»
  source: .andromeda/test-plan.md:102 «(24 readers; 23 before `stand_session_recording`, 22 before `stand_virtual_clock`»
  replace-with: «`stand_requirement_sweep`, `stand_virtual_clock` and, since 2026-10-19-session-recording, `stand_session_recording` (24 readers;»

- leaf: .claude/rules/testing.md:15   class: omission
  leaf-text: «the refusal readers `refused` and `refused_unchanged`)»
  source: .andromeda/test-plan.md:102 «the module's new `replay(session, check)` (each kept step run again, its acceptance and digest), `kept(check)` (each kept step's acceptance and digest) and the unix-only `mode(path)`»
  replace-with: «the refusal readers `refused` and `refused_unchanged`, and since 2026-10-19-session-recording the recording readers `replay` · `kept` and the unix-only `mode`)»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «and is not unix-gated but `mcp_server`'s two command-line comparisons;»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:10 «since 2026-10-19-session-recording a check that records is `cfg(unix)`-gated item by item, the in-process and `mcp` stdio ones included (seven_guis `mcp_log`'s recorded test, `recorded_needles`»
  replace-with: «and is not unix-gated but `mcp_server`'s two command-line comparisons; since 2026-10-19-session-recording a check that records is `cfg(unix)`-gated item by item, in-process and `mcp` stdio ones included (`mcp_log`'s recorded test), beside an off-unix check that a recording is refused `Unsupported`, which the windows job alone runs;»
