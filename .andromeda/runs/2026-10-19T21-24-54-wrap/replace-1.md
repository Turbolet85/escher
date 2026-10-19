- line: .andromeda/test-plan.md:22
old: «92 unit tests in fifteen files, 72 inline and, since 2026-10-16-file-splits, 20 in two `#[path]` test files, beside three off-unix tests the windows job alone runs (87 in fourteen»
new: «92 unit tests in fifteen files, 58 inline and 34 in four `#[path]` test files (72 inline and 20 in two from 2026-10-16-file-splits until 2026-10-19-test-splits), beside three off-unix tests the windows job alone runs, recording's one in a fifth `#[path]` file, `recording_off_unix_tests.rs`, since 2026-10-19-test-splits (87 in fourteen»

- line: .andromeda/test-plan.md:22
old: «), cli.rs 5 (a line is read»
new: «), cli_tests.rs 5 (`cli::tests`, in `cli.rs` until 2026-10-19-test-splits — a line is read»

- line: .andromeda/test-plan.md:22
old: «since 2026-10-19-session-recording recording.rs 9 (the digest»
new: «since 2026-10-19-session-recording recording_tests.rs 9 (`recording::tests`, in `recording.rs` until 2026-10-19-test-splits — the digest»

- line: .andromeda/test-plan.md:22
old: «packages/escher-driver/src/cli.rs:476-1032;»
new: «packages/escher-driver/src/cli_tests.rs:1-559;»

- line: .andromeda/test-plan.md:145
old: «; cli.rs pins the command line»
new: «; cli_tests.rs (`cli::tests`, in `cli.rs` until 2026-10-19-test-splits) pins the command line»

- line: .andromeda/test-plan.md:145
old: «since 2026-10-19-session-recording recording.rs pins the FNV-1a»
new: «since 2026-10-19-session-recording recording_tests.rs (`recording::tests`, in `recording.rs` until 2026-10-19-test-splits) pins the FNV-1a»

- line: .andromeda/test-plan.md:145
old: «off unix, a recording refused `Unsupported` (packages/escher-driver/src/recording.rs:469-981;»
new: «off unix, a recording refused `Unsupported` — `recording::off_unix_tests`, in `recording_off_unix_tests.rs` since 2026-10-19-test-splits (packages/escher-driver/src/recording_tests.rs:1-517; packages/escher-driver/src/recording_off_unix_tests.rs:1-15;»

- line: .andromeda/test-plan.md:10
old: «unix-gated with its `recorded_needles`:»
new: «unix-gated, reading the shared `common::recorded_needles` (its own until 2026-10-19-test-splits):»

- line: .andromeda/test-plan.md:10
old: «the child killed and reaped in its `Drop`; read through `mod common;` by all eight)»
new: «the child killed and reaped in its `Drop`; since 2026-10-19-test-splits the log checks' fragment, stated once — `Needles` and `stand_needles` (the CRUD stand's id and name needles, asserted non-empty and no part of the sentinel), `stamped_lines` (every stderr line stamped `service.name=seven_guis`), `found`, `occurrences` and the unix-only `recorded_needles` — called by `host_log` and `mcp_log` alone (examples/seven_guis/tests/common/mod.rs:370-482); read through `mod common;` by all eight)»

- line: .andromeda/test-plan.md:68
old: «bar five `cfg(test)` modules held in files of their own through `#[path]`»
new: «bar eight `cfg(test)` modules held in files of their own through `#[path]`»

- line: .andromeda/test-plan.md:68
old: «`mcp::tests` (packages/escher-driver/src/mcp.rs:291-293) and `schema::tests` (packages/escher-driver/src/schema.rs:482-484);»
new: «`mcp::tests` (packages/escher-driver/src/mcp.rs:291-293), `schema::tests` (packages/escher-driver/src/schema.rs:482-484) and, since 2026-10-19-test-splits, `cli::tests`, `recording::off_unix_tests` and `recording::tests` (packages/escher-driver/src/cli.rs:476-478; packages/escher-driver/src/recording.rs:453-455; packages/escher-driver/src/recording.rs:457-459);»

- line: .andromeda/test-plan.md:139
old: «five unit-test modules sit out of line, each declared `#[cfg(test)]` with `#[path]` at its parent»
new: «eight unit-test modules sit out of line, each declared `#[cfg(test)]` with `#[path]` at its parent (`recording::off_unix_tests` `#[cfg(all(test, not(unix)))]`, `recording::tests` `#[cfg(all(test, unix))]`)»

- line: .andromeda/test-plan.md:139
old: «`mcp::tests` in `mcp_tests.rs` (packages/escher-driver/src/mcp.rs:291-293) and `schema::tests` in `schema_tests.rs` (packages/escher-driver/src/schema.rs:482-484)»
new: «`mcp::tests` in `mcp_tests.rs` (packages/escher-driver/src/mcp.rs:291-293), `schema::tests` in `schema_tests.rs` (packages/escher-driver/src/schema.rs:482-484) and, since 2026-10-19-test-splits, moved out whole with their test paths unchanged, `cli::tests` in `cli_tests.rs` (packages/escher-driver/src/cli.rs:476-478), `recording::off_unix_tests` in `recording_off_unix_tests.rs` (packages/escher-driver/src/recording.rs:453-455) and `recording::tests` in `recording_tests.rs` (packages/escher-driver/src/recording.rs:457-459)»

- line: .andromeda/test-plan.md:348
old: «.github/scripts/test_ci_workflows.py:327 — as measured at escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md)»
new: «.github/scripts/test_ci_workflows.py:327 — as measured at escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md). A `*_tests.rs` `#[path]` test-child file is no record of the per-crate reading — the coverage job's per-file report lists none, `mcp_tests.rs` and `schema_tests.rs` included — so a test module moved out of line moves its crate's reading with no line of code changed: since 2026-10-19-test-splits escher-driver reads 4266/4350 = 98.07 %, 3 of 3 crates still at or above their floors (as measured at escher-0.2.0/chunks/2026-10-19-test-splits/evidence/ci.md)»

- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:10
old: «(seven_guis `mcp_log`'s recorded test, `recorded_needles`, `the_check`)»
new: «(seven_guis `mcp_log`'s recorded test, the shared `common::recorded_needles`, `the_check`)»

- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:10
old: «(`recording.rs`'s `off_unix_tests`, `session.rs`,»
new: «(`recording::off_unix_tests` in `recording_off_unix_tests.rs`, `session.rs`,»

- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «(cli.rs 5 · client.rs 3»
new: «(cli_tests.rs 5 · client.rs 3»

- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «mcp_tests.rs 14 · recording.rs 9 · refusal.rs 3»
new: «mcp_tests.rs 14 · recording_tests.rs 9 · refusal.rs 3»

- line: .andromeda/architecture.md:137
old: «five unit-test modules sit in files of their own»
new: «eight unit-test modules sit in files of their own»
