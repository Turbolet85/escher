group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «72 inline and, since 2026-10-16-file-splits, 20 in two `#[path]` test files, beside three off-unix tests the windows job alone runs»
  source: .andromeda/test-plan.md:22 «58 inline and 34 in four `#[path]` test files (72 inline and 20 in two from 2026-10-16-file-splits until 2026-10-19-test-splits), beside three off-unix tests the windows job alone runs, recording's one»
  replace-with: «58 inline and 34 in four `#[path]` test files (72 inline and 20 in two from 2026-10-16-file-splits until 2026-10-19-test-splits), beside three off-unix tests the windows job alone runs, recording's one in a fifth `#[path]` file, `recording_off_unix_tests.rs` (`recording::off_unix_tests`), since 2026-10-19-test-splits»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`cli.rs` 5 (a line read by its rules in order»
  source: .andromeda/test-plan.md:22 «cli_tests.rs 5 (`cli::tests`, in `cli.rs` until 2026-10-19-test-splits — a line is read by its rules in order»
  replace-with: «`cli_tests.rs` 5 (`cli::tests`, in `cli.rs` until 2026-10-19-test-splits; a line read by its rules in order»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «since 2026-10-19-session-recording `recording.rs` 9 (the FNV-1a 64 digest»
  source: .andromeda/test-plan.md:22 «since 2026-10-19-session-recording recording_tests.rs 9 (`recording::tests`, in `recording.rs` until 2026-10-19-test-splits — the digest is FNV-1a 64 of the written answer»
  replace-with: «since 2026-10-19-session-recording `recording_tests.rs` 9 (`recording::tests`, in `recording.rs` until 2026-10-19-test-splits; the FNV-1a 64 digest»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «reads 0 of the recording needles (the kept check's and its directory's name) on stderr, the span counts unchanged.»
  source: .andromeda/test-plan.md:10 «since 2026-10-19-test-splits the log checks' fragment, stated once — `Needles` and `stand_needles`»
  replace-with: «reads 0 of the recording needles (the kept check's and its directory's name) on stderr, the span counts unchanged; since 2026-10-19-test-splits the two log checks' fragment is stated once in seven_guis' `tests/common/mod.rs` — `Needles` and `stand_needles` (the CRUD stand's id and name needles), `stamped_lines`, `found`, `occurrences` and the unix-only `recorded_needles` (`mcp_log`'s own until then) — called by `host_log` and `mcp_log` alone.»

- leaf: .claude/docs/tests-summary.md:42   class: omission
  leaf-text: «the merged tree reading 98.19 %, 84.25 %, 91.69 % —»
  source: .andromeda/test-plan.md:348 «since 2026-10-19-test-splits escher-driver reads 4266/4350 = 98.07 %, 3 of 3 crates still at or above their floors»
  replace-with: «the merged tree reading 98.19 %, 84.25 %, 91.69 %; a `*_tests.rs` `#[path]` test file is no record of the per-crate reading, so a test module moved out of line moves its crate's reading — escher-driver 98.07 % since 2026-10-19-test-splits, 3 of 3 crates at or above their floors —»
