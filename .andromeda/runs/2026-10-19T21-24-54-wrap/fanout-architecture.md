proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → escher-driver row
    change: 'the escher-driver row names its three new private `cfg(test)` children beside `schema_tests.rs` and `mcp_tests.rs` — cli (… and, since 2026-10-19-test-splits, its tests in `cli_tests.rs`: packages/escher-driver/src/cli.rs:476-478) and recording (… and, since 2026-10-19-test-splits, its unix tests in `recording_tests.rs` and its one off-unix test in `recording_off_unix_tests.rs`, both through `#[path]`: packages/escher-driver/src/recording.rs:453-455; packages/escher-driver/src/recording.rs:457-459) — each moved out whole, test paths unchanged, no module added to the public tree'
    sidecar: 'arch §Existing Scopes escher-driver row — cli::tests, recording::tests and recording::off_unix_tests now named as `#[path]` children in cli_tests.rs, recording_tests.rs and recording_off_unix_tests.rs (2026-10-19-test-splits)'
    rationale: 'report Changes → Crates / modules: "escher-driver gains three private `cfg(test)` child modules in files of their own (`cli::tests`, `recording::tests`, `recording::off_unix_tests`)"; Symbols / APIs gives their declarations at cli.rs:476-478, recording.rs:453-455 and recording.rs:457-459; F2 lists architecture.md:331 stale ("the row names neither new file"); Expected amendments: "the escher-driver row names the new files"'
    basis: .andromeda/architecture.md:331
  - detector: D-arch-resources
    severity: warning
    section: §Conventions → Tests
    change: 'the out-of-line unit-test sentence reads eight modules, not five: after escher-driver''s `mcp::tests` and `schema::tests` add, since 2026-10-19-test-splits, escher-driver''s `cli::tests`, `recording::tests` and `recording::off_unix_tests` in `cli_tests.rs`, `recording_tests.rs` and `recording_off_unix_tests.rs` (packages/escher-driver/src/cli.rs:476-478; packages/escher-driver/src/recording.rs:453-455; packages/escher-driver/src/recording.rs:457-459), each moved out whole with its test paths unchanged'
    sidecar: 'arch §Conventions Tests — out-of-line `#[path]` unit-test modules five → eight, escher-driver''s cli, recording and recording off-unix tests added (2026-10-19-test-splits)'
    rationale: 'report F2 · path-children / split-count: "out-of-line unit-test modules held in files of their own through `#[path]`: five → eight", architecture.md:137 stale (@c884,953,1644,1663 and @c1114,1368,1402,1435,1471 — the count and the module list); Expected amendments: "architecture §Conventions → Tests (five → eight `#[path]` children)"'
    basis: .andromeda/architecture.md:137
    dependent-of: D-arch-resources
note: 'architecture.md:309 (the blitz-dom row, flagged stale by F2) states only blitz-dom''s own two `#[path]` children, which this chunk left untouched — read true, no proposal. D-arch-decisions: no dependency, runtime or locked-decision change (Dependencies: none). Not drift under these detectors but for the orchestrator: architecture.md:223''s list of what seven_guis'' tests/common/mod.rs shares does not name the log checks'' new needle readers (F6, report marks the row citation-only), and the F5 citation digits at :214, :223, :337 are P2''s citation sweep; the coverage-leg omission of `*_tests.rs` (Spec claims disproved) is left to P2 as the report says, :206 true as written.'
