
## 2026-10-19-test-splits — eight `#[path]` test children; the seven_guis log checks' shared fragment
**Section:** §Conventions → Tests (`conv-tests` group) · §Occupied Resources → Process-wide state and threads (`res-process-wide-state-and-threads` group) · §Existing Scopes (`scopes` group, escher-driver)
**Change:**
- §Conventions: unit-test modules in files of their own were five, now eight; the three new are escher-driver's `cli::tests` in `cli_tests.rs`, `recording::tests` in `recording_tests.rs` (`#[cfg(all(test, unix))]`) and `recording::off_unix_tests` in `recording_off_unix_tests.rs` (`#[cfg(all(test, not(unix)))]`, the windows job alone), each moved whole, test paths unchanged.
- §Occupied Resources: seven_guis' shared test module also holds the log checks' readers — `Needles`, `stand_needles`, `stamped_lines`, `found`, `occurrences`, the unix-only `recorded_needles` — for `host_log` and `mcp_log` alone.
- §Existing Scopes: escher-driver's `cli` and `recording` hold their tests in those `#[path]` children, as `schema` and `mcp` do; no module added to the public tree, the row unchanged.
**Why:** this chunk's own test-only moves (the playbook's «Accurate this-chunk addition», the plan's reviewed expected amendment); written as detail groups below their standing records, the module count replaced in place.
**Ref:** .andromeda/runs/2026-10-19T21-24-54-wrap/
