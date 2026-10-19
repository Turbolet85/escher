group: architecture+test-plan-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «`cli.rs` 5»
  source: .andromeda/test-plan.md:22 «cli_tests.rs 5 (`cli::tests`, in `cli.rs` until 2026-10-19-test-splits»
  replace-with: «`cli_tests.rs` 5»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «`recording.rs` 9»
  source: .andromeda/test-plan.md:22 «recording_tests.rs 9 (`recording::tests`, in `recording.rs` until 2026-10-19-test-splits»
  replace-with: «`recording_tests.rs` 9»

- leaf: .claude/docs/services/escher-driver.md:60   class: omission
  leaf-text: «(`schema::tests` and `mcp::tests` each a `#[cfg(test)]` child in a file of its own, declared through `#[path]` at its parent, since 2026-10-16-file-splits, moved out whole with test paths unchanged)»
  source: .andromeda/test-plan.md:139 «and, since 2026-10-19-test-splits, moved out whole with their test paths unchanged, `cli::tests` in `cli_tests.rs` (packages/escher-driver/src/cli.rs:476-478), `recording::off_unix_tests` in `recordi»
  replace-with: «(`schema::tests` and `mcp::tests` since 2026-10-16-file-splits and, since 2026-10-19-test-splits, `cli::tests` and `recording::tests` (`#[cfg(all(test, unix))]`), each a `#[cfg(test)]` child in a file of its own, declared through `#[path]` at its parent, moved out whole with test paths unchanged — 58 tests inline and 34 in those four files, 72 and 20 in two from 2026-10-16-file-splits until 2026-10-19-test-splits)»

- leaf: .claude/docs/services/escher-driver.md:60   class: omission
  leaf-text: «beside three off-unix tests the windows job alone runs;»
  source: .andromeda/test-plan.md:22 «beside three off-unix tests the windows job alone runs, recording's one in a fifth `#[path]` file, `recording_off_unix_tests.rs`, since 2026-10-19-test-splits»
  replace-with: «beside three off-unix tests the windows job alone runs, recording's one (`recording::off_unix_tests`, `#[cfg(all(test, not(unix)))]`) in a fifth `#[path]` file, `recording_off_unix_tests.rs`, since 2026-10-19-test-splits;»

- leaf: .claude/docs/services/escher-driver.md:63   class: stale
  leaf-text: «`mcp_log`'s recorded test, `recorded_needles`, `the_check`»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:10 «the in-process and `mcp` stdio ones included (seven_guis `mcp_log`'s recorded test, the shared `common::recorded_needles`, `the_check`)»
  replace-with: «`mcp_log`'s recorded test, the shared `common::recorded_needles` (its own until 2026-10-19-test-splits), `the_check`»
