proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary"
    change: "escher-driver row: '92 unit tests in fifteen files, 58 inline and, since 2026-10-16-file-splits, 34 in four `#[path]` test files (mcp_tests.rs 14 · schema_tests.rs 6 · since 2026-10-19-test-splits cli_tests.rs 5 · recording_tests.rs 9; 72 inline and 20 in two until 2026-10-19-test-splits), beside three off-unix tests the windows job alone runs, recording's one since 2026-10-19-test-splits in a fifth `#[path]` file, recording_off_unix_tests.rs' — count history kept."
    sidecar: "test-plan §1 escher-driver: the inline / #[path] split 72 / 20 in two → 58 / 34 in four (cli_tests.rs 5, recording_tests.rs 9), the off-unix recording test in a fifth #[path] file (2026-10-19-test-splits)."
    rationale: "Report Changes → Counts F1 (split-count): 92 unit tests unchanged, the split moves from 72 inline / 20 in two #[path] files to 58 inline / 34 in four, and the off-unix recording test now sits in recording_off_unix_tests.rs; the pre-pass sweep names test-plan.md:22 @c53,109 stale."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:63-73; .andromeda/test-plan.md:22"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary"
    change: "escher-driver row's per-file entry 'cli.rs 5 (…)' reads 'cli_tests.rs 5 (`cli::tests`, in cli.rs until 2026-10-19-test-splits; …)', its five test descriptions unchanged."
    sidecar: "test-plan §1 escher-driver: cli.rs 5 → cli_tests.rs 5 (`cli::tests`, moved whole, test paths unchanged)."
    rationale: "Report Changes → Symbols: `cli::tests` moved whole into cli_tests.rs (five tests at cli_tests.rs:15-559), declared at cli.rs:476-478; F1 names the per-file list change cli.rs 5 → cli_tests.rs 5."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:24-26; .andromeda/test-plan.md:22"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary"
    change: "escher-driver row's per-file entry 'since 2026-10-19-session-recording recording.rs 9 (…)' reads '… recording_tests.rs 9 (`recording::tests`, in recording.rs until 2026-10-19-test-splits; …)', its nine test descriptions unchanged."
    sidecar: "test-plan §1 escher-driver: recording.rs 9 → recording_tests.rs 9 (`recording::tests`, moved whole, test paths unchanged)."
    rationale: "Report Changes → Symbols: `recording::tests` moved whole into recording_tests.rs (nine tests at recording_tests.rs:14-517), declared at recording.rs:457-459; F1 names recording.rs 9 → recording_tests.rs 9."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:30-32; .andromeda/test-plan.md:22"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary"
    change: "escher-driver row's citation list: `packages/escher-driver/src/cli.rs:476-1032` re-pointed to `packages/escher-driver/src/cli_tests.rs:1-559`."
    sidecar: "test-plan §1 escher-driver: the cli tests' citation re-pointed cli.rs:476-1032 → cli_tests.rs:1-559."
    rationale: "Report F1 / cli-moved: test-plan.md:22's `cli.rs:476-1032` points into a moved span, now cli_tests.rs:1-559 (cli.rs now 478 lines)."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:70-71; .andromeda/test-plan.md:22"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy"
    change: "escher-driver pins sentence: 'cli.rs pins the command line — …' reads 'cli_tests.rs (`cli::tests`, in cli.rs until 2026-10-19-test-splits) pins the command line — …'."
    sidecar: "test-plan §4: the command-line pins named in cli_tests.rs (`cli::tests`), moved out of cli.rs at 2026-10-19-test-splits."
    rationale: "Report F1: test-plan.md:145's `cli.rs pins …` entry — the file renamed; `cli::tests` now lives in cli_tests.rs, test paths unchanged."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:79-80; .andromeda/test-plan.md:145"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy"
    change: "escher-driver pins sentence: 'since 2026-10-19-session-recording recording.rs pins the FNV-1a 64 digest …' reads '… recording_tests.rs (`recording::tests`, in recording.rs until 2026-10-19-test-splits) pins …', and 'off unix, a recording refused `Unsupported`' names `recording::off_unix_tests` in recording_off_unix_tests.rs; its citation `packages/escher-driver/src/recording.rs:469-981` re-pointed to `packages/escher-driver/src/recording_tests.rs:1-517; packages/escher-driver/src/recording_off_unix_tests.rs:1-15`."
    sidecar: "test-plan §4: the recording pins named in recording_tests.rs (`recording::tests`) and the off-unix refusal in recording_off_unix_tests.rs; citation recording.rs:469-981 → recording_tests.rs:1-517 + recording_off_unix_tests.rs:1-15."
    rationale: "Report F1 / rec-moved: test-plan.md:145's `recording.rs:469-981` points into a moved span (now recording_tests.rs:1-517; the off-unix refusal recording_off_unix_tests.rs:1-15), and the line's recording.rs entry names a renamed file."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:72-73; .andromeda/test-plan.md:145"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "session-proof: escher-driver's 92 unit tests list reads 'cli_tests.rs 5 · client.rs 3 · … · mcp_tests.rs 14 · recording_tests.rs 9 · refusal.rs 3 · …' (cli.rs 5 and recording.rs 9 renamed since 2026-10-19-test-splits), the other entries and the three off-unix tests unchanged."
    sidecar: "test-plan §3 Session lifecycle key file: the escher-driver per-file list names cli_tests.rs 5 and recording_tests.rs 9."
    rationale: "Report F1: session-lifecycle.md:11 @c5676 the per-file list `cli.rs 5 … recording.rs 9` — stale; the modules moved whole into cli_tests.rs and recording_tests.rs."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:83; .andromeda/registries/contracts/test-plan/session-lifecycle.md:11"
    dependent-of: D-tests-coverage
  - detector: D-tests-framework
    severity: warning
    section: "§2 Test Strategy"
    change: "Directory pattern: 'bar eight `cfg(test)` modules held in files of their own through `#[path]` — … `mcp::tests` …, `schema::tests` … and, since 2026-10-19-test-splits, `cli::tests` in cli_tests.rs (packages/escher-driver/src/cli.rs:476-478), `recording::tests` in recording_tests.rs (packages/escher-driver/src/recording.rs:457-459) and `recording::off_unix_tests` in recording_off_unix_tests.rs (packages/escher-driver/src/recording.rs:453-455), each moved out whole with its test paths unchanged' (five until then)."
    sidecar: "test-plan §2 Directory pattern: out-of-line #[path] test modules five → eight (escher-driver cli::tests, recording::tests, recording::off_unix_tests, 2026-10-19-test-splits)."
    rationale: "Report F2 (path-children, split-count): out-of-line unit-test modules held in files of their own through #[path] go five → eight, test paths unchanged; test-plan.md:68 @c117,148 / @c177,297,331 named stale. Declarations at cli.rs:476-478, recording.rs:453-455 and :457-459 per Changes → Symbols."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:86-93; .andromeda/test-plan.md:68"
  - detector: D-tests-framework
    severity: warning
    section: "§4 Unit Test Strategy"
    change: "Test file location: 'eight unit-test modules sit out of line, each declared `#[cfg(test)]` (the off-unix one `#[cfg(all(test, not(unix)))]`, `recording::tests` `#[cfg(all(test, unix))]`) with `#[path]` at its parent — … `mcp::tests` in `mcp_tests.rs` …, `schema::tests` in `schema_tests.rs` … and, since 2026-10-19-test-splits, moved out whole with their test paths unchanged, `cli::tests` in `cli_tests.rs` (packages/escher-driver/src/cli.rs:476-478), `recording::off_unix_tests` in `recording_off_unix_tests.rs` (packages/escher-driver/src/recording.rs:453-455) and `recording::tests` in `recording_tests.rs` (packages/escher-driver/src/recording.rs:457-459)' (five until then)."
    sidecar: "test-plan §4 Test file location: five → eight out-of-line #[path] unit-test modules, the three new escher-driver ones named with their declarations."
    rationale: "Report F2: test-plan.md:139 stale (five → eight); Changes → Symbols gives each declaration's cfg and its lines (cli.rs:476-478; recording.rs:453-455 all(test, not(unix)); recording.rs:457-459 all(test, unix))."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:24-32; .andromeda/test-plan.md:139"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§1 Test Scope Summary"
    change: "apps row, seven_guis `tests/mcp_log.rs`: 'unix-gated with its `recorded_needles`' reads 'unix-gated, reading the shared `common::recorded_needles` (its own until 2026-10-19-test-splits)'."
    sidecar: "test-plan §1: mcp_log's recorded test reads the shared common::recorded_needles, no longer its own (2026-10-19-test-splits)."
    rationale: "Report F6 (common-mod): recorded_needles is no longer host_log's and mcp_log's own — it is `#[cfg(unix)] pub fn recorded_needles` in examples/seven_guis/tests/common/mod.rs:457-482; test-plan.md:10 @c5582 named content-stale."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:115-119; .andromeda/test-plan.md:10"
  - detector: D-tests-framework
    severity: warning
    section: "§1 Test Scope Summary"
    change: "apps row, seven_guis' shared module `tests/common/mod.rs` (no target): its contents list adds 'and, since 2026-10-19-test-splits, the log checks' fragment stated once — `Needles` and `stand_needles` (the CRUD stand's id and name needles, asserted non-empty and no part of the sentinel), `stamped_lines`, `found`, `occurrences` and the unix-only `recorded_needles` — called by `host_log` and `mcp_log` alone'."
    sidecar: "test-plan §1: seven_guis tests/common/mod.rs now holds the log checks' needle readers, stamp check and counts (Needles, stand_needles, stamped_lines, found, occurrences, recorded_needles)."
    rationale: "Report Changes → Symbols and F6: common/mod.rs gains Needles, stand_needles, stamped_lines, found, occurrences and recorded_needles (common/mod.rs:370-482), callers host_log and mcp_log only; test-plan.md:10's enumeration of what the shared module holds stops at Speaker / Spoke."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:37-47; .andromeda/test-plan.md:10"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§3 → Session lifecycle"
    change: "session-checks: '(seven_guis `mcp_log`'s recorded test, the shared `common::recorded_needles`, `the_check`)' and the off-unix check named as '`recording::off_unix_tests` (in recording_off_unix_tests.rs since 2026-10-19-test-splits), `session.rs`, `mcp_tests.rs` and seven_guis `recording`'."
    sidecar: "test-plan §3 Session lifecycle key file: recorded_needles is common's; the off-unix recording module sits in recording_off_unix_tests.rs."
    rationale: "Report F6: session-lifecycle.md:10 ('seven_guis `mcp_log`'s recorded test, `recorded_needles`') content-stale; F1: the same line's `recording.rs`'s `off_unix_tests` — the module now in recording_off_unix_tests.rs, path unchanged."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:81-82; escher-0.2.0/chunks/2026-10-19-test-splits/report.md:121; .andromeda/registries/contracts/test-plan/session-lifecycle.md:10"
    dependent-of: D-tests-framework
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets"
    change: "Coverage floors: add 'the per-crate reading counts no `*_tests.rs` `#[path]` test-child file — cargo-llvm-cov leaves each out of the lcov before coverage_floor.py reads it — so moving an inline test module into one moves the reading: escher-driver 5180/5268 = 98.33 % at CI#39319403188 → 4266/4350 = 98.07 % at CI#39324662409 (2026-10-19-test-splits, LF −918, LH −914), the floors unchanged and 3 of 3 crates at or above them'."
    sidecar: "test-plan §10 Coverage floors: measured — a *_tests.rs #[path] file is absent from the lcov, so a test split moves a crate's reading (escher-driver 98.33 % → 98.07 %, 2026-10-19-test-splits)."
    rationale: "Report Spec claims disproved + F7 + Decisions: 'moved lines stay counted' measured false — the coverage job's per-file report lists no *_tests.rs file in either run; the report leaves to P2 whether test-plan §10 should state it; the floor this section mandates now reads a test split differently than its forecast assumed."
    basis: "escher-0.2.0/chunks/2026-10-19-test-splits/report.md:171-184; .andromeda/test-plan.md:348"
note: "D-tests-obs-harness: no drift — the report's Harness / gate surface reads none. Not proposed: the F5 log-cite citation digits (test-plan.md:10, :50, :51, :68 @c7821, :105, :186) are left to P2's citation sweep as the report states; test-plan.md:68 @c7433 (the shared module 'read through `mod common;` by all eight') names no contents and stays true; '92 unit tests in fifteen files' stays (each move is one file for one)."
