group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/testing.md:14   class: stale
  leaf-text: «(five modules sit in files of their own, each declared `#[cfg(test)]` with `#[path]` at its parent: dioxus-native-dom's `bridge_tests`, a child of `dioxus_document`, so it reaches that module's private fields, and, since 2026-10-16-file-splits, moved out whole with their test paths unchanged, blitz-dom's `document::changed_set_tests` and `document::font_reading_tests` and escher-driver's `mcp::tests` and `schema::tests`)»
  source: .andromeda/test-plan.md:139 «eight unit-test modules sit out of line, each declared `#[cfg(test)]` with `#[path]` at its parent (`recording::off_unix_tests` `#[cfg(all(test, not(unix)))]`»
  replace-with: «(eight modules sit in files of their own, each declared `#[cfg(test)]` with `#[path]` at its parent — `recording::off_unix_tests` `#[cfg(all(test, not(unix)))]`, `recording::tests` `#[cfg(all(test, unix))]` —: dioxus-native-dom's `bridge_tests`, a child of `dioxus_document`, so it reaches that module's private fields, and, moved out whole with their test paths unchanged, since 2026-10-16-file-splits blitz-dom's `document::changed_set_tests` and `document::font_reading_tests` and escher-driver's `mcp::tests` and `schema::tests`, and since 2026-10-19-test-splits escher-driver's `cli::tests` in `cli_tests.rs`, `recording::off_unix_tests` in `recording_off_unix_tests.rs` and `recording::tests` in `recording_tests.rs`)»

- leaf: .claude/rules/testing.md:15   class: omission
  leaf-text: «read its stdout, stderr and exit status (`0` · `1` · `2` · `3`) through `tests/common/mod.rs`;»
  source: .andromeda/test-plan.md:10 «`stamped_lines` (every stderr line stamped `service.name=seven_guis`), `found`, `occurrences` and the unix-only `recorded_needles` — called by `host_log` and `mcp_log` alone»
  replace-with: «read its stdout, stderr and exit status (`0` · `1` · `2` · `3`) through `tests/common/mod.rs`, which since 2026-10-19-test-splits also states the log checks' fragment once — `Needles` and `stand_needles` (the CRUD stand's id and name needles), `stamped_lines`, `found`, `occurrences` and the unix-only `recorded_needles` — called by `host_log` and `mcp_log` alone;»

- leaf: .claude/rules/testing.md:48   class: omission
  leaf-text: «by `.github/scripts/coverage_floor.py` —»
  source: .andromeda/test-plan.md:348 «A `*_tests.rs` `#[path]` test-child file is no record of the per-crate reading — the coverage job's per-file report lists none, `mcp_tests.rs` and `schema_tests.rs` included —»
  replace-with: «by `.github/scripts/coverage_floor.py` (a `*_tests.rs` `#[path]` test-child file is no record of the per-crate reading, so a test module moved out of line moves its crate's reading with no line of code changed: since 2026-10-19-test-splits escher-driver reads 4266/4350 = 98.07 %, 3 of 3 crates still at or above their floors) —»
