proposals:
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Pipeline facts (Legs)"
    change: "Thirteen linux ci.yml jobs each run `bash .github/scripts/ci-leg.sh {leg}`. The list gains `stand-repeat`, which is not one of the fast legs: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` (40 files), then `cargo test -p seven_guis --locked --test cli_commands --test cli_flow --test host_binary --test host_log --test host_timer --test mcp_server --test mcp_log`. The first red command ends the leg with its status (no retry, no `--ignored`, no env var), and three green rounds end with `stand-repeat: 3 of 3 rounds green`. Its job, `Stand checks repeated`, runs on `ubuntu-latest`, needs the fast four, uses `apt-install.sh libfontconfig1-dev` and has a failure-only `ci-log-stand-repeat` upload. The audit leg reads `cargo deny --locked --workspace --all-features check advisories licenses`, where it was `cargo deny check advisories`. The clippy leg runs the workspace line and then, only when that line is green, `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings`. The coverage leg ends with the floor script's status (see §9 Coverage). The leg script finds the floor script and the `stand_*.rs` set beside itself (`SCRIPTS=\"$(dirname \"$0\")\"`)."
    sidecar: "test-plan §9 Legs: thirteen linux jobs. The new stand-repeat leg and job (three rounds, zero-failure budget). The audit runs over the whole workspace with --all-features and checks advisories and licences. The clippy leg gains its blitz-dom line. The coverage leg ends with its floor (2026-10-14-quality-gates)."
    rationale: "Report Changes → Symbols / APIs (`ci-leg.sh`: `LEGS` gains `stand-repeat`, the clippy, audit and coverage arms; `ci.yml`: new job `stand-repeat`) and Harness / gate surface. The test-plan §9 Legs bullet still states twelve jobs and the audit as `cargo deny check advisories`, so it no longer describes the runner the chunk uses. Expected amendments: `auditcmd` test-plan.md:315 changed."
    basis: ".github/scripts/ci-leg.sh:43-57; .github/scripts/ci-leg.sh:21-25; .github/workflows/ci.yml:300-323; test-plan.md:315"
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Pipeline facts (Cache)"
    change: "The test job saves under `shared-key: workspace-test`, which two jobs restore with `save-if: false`: the `a11y` job and, since 2026-10-14-quality-gates, the `stand-repeat` job (`Stand checks repeated`). Keep CI run 37386253475's exact hit and the past wall-time readings as they read; each is a reading of a named past run."
    sidecar: "test-plan §9 Cache: the stand-repeat job is the second restore-only job of the workspace-test cache (2026-10-14-quality-gates)."
    rationale: "Report Changes → Symbols / APIs (`ci.yml` new job with `shared-key: workspace-test` and `save-if: false`). Counts → restore-only cache jobs `{a11y}` → `{a11y, stand-repeat}`; `a11yleg` changed test-plan.md:317 (\"which the a11y job restores\"). `checks16` lists :317 as no-change."
    basis: ".github/workflows/ci.yml:300-323; test-plan.md:317"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Pipeline facts (Package installs)"
    change: "Ten steps call `bash .github/scripts/apt-install.sh <package>...`: `libfontconfig1-dev` in nine jobs (the eight named plus `stand-repeat`) and `python3-yaml` in `ci-scripts`, behind its guard. Every attempt after the first now starts with a bounded repair, `sudo timeout --kill-after=GRACE BOUND dpkg --configure -a`, in phase `repair`. A failed repair fails that attempt and is named by the existing diagnostic: `apt-install: attempt {n} of {N} failed in repair (exit {s})`. The first attempt runs no `dpkg`, and the usage gate still exits 2 before `sudo`, `timeout`, `apt-get` or `dpkg`. Retire \"a bound that fires inside `apt-get install` can leave dpkg interrupted, which the script does not handle\": the next attempt repairs it first. No run has yet shown the repair on a real interrupted dpkg; the stand-in tests are its witness. CI run 37985678276 (eight jobs ran the script) stays as that run's reading."
    sidecar: "test-plan §9 Package installs: ten calls and nine libfontconfig1-dev jobs. A bounded `dpkg --configure -a` repair runs before each retried attempt, and the unhandled-dpkg limit is retired (2026-10-14-quality-gates)."
    rationale: "Report Changes → Symbols / APIs (`apt-install.sh`: `repair()`, `attempt_once` taking the attempt number, phase `repair`) and Counts (`joblist` changed test-plan.md:319; `dpkg` changed test-plan.md:319 for the nine steps, the eight jobs and the unhandled-dpkg limit). Outcome: `apt-install.sh` runs `dpkg --configure -a` bounded before each retried attempt and never on a healthy install."
    basis: ".github/scripts/apt-install.sh:52-54; .github/scripts/apt-install.sh:57-60; test-plan.md:319"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Pipeline facts (new bullet: per-OS tallies)"
    change: "Add a bullet with the per-OS tallies of CI#38649787488 on `eb5ba90c` (green 16/16), taken from its six test-bearing job logs (escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/per-os-tallies.md): linux 185 · 951 · 0 · 12; macos and coverage 166 · 948 · 0 · 12; windows 166 · 926 · 0 · 9; ios and android 0 (build only). linux minus macos and linux minus coverage are both the 19 `Doc-tests` result lines (3 passed). macos minus windows is 22 passed · 3 ignored, all unix-gated, across the nine binaries the evidence names."
    sidecar: "test-plan §9: the per-OS test tallies of CI#38649787488 are attributed: the linux, macos/coverage and windows differences are Doc-tests and unix-gated binaries (2026-10-14-quality-gates, CARRY 8)."
    rationale: "Report Changes → Cross-project / external claims (CI#38649787488 per-OS tallies) and Expected amendments, test-plan §1 / §9: CARRY 8's attribution is new text for §9, and no master states the per-OS tallies (`perostally` 0 changed)."
    basis: "escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/per-os-tallies.md"
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets"
    change: "Replace the NOT YET MEASURED line (342) with the two gates now set on fork CI. (1) Coverage floors on the three driver crates, on the founder's «3 крейта, на 1 пункт ниже (Рекомендую)»: packages/escher-driver 97 %, packages/dioxus-native-dom 83 %, packages/blitz-test-harness 90 %. They are enforced by the `coverage` leg through `.github/scripts/coverage_floor.py`. At this tree they read 98.34 % (4784/4865), 84.25 % and 91.69 % (evidence/coverage-floor.md). The leg's verdict is `coverage floor: 3 of 3 crates at or above their floors`, on the host and in CI#38692629940. A floor of 99 exits 1 naming escher-driver. (2) A zero-failure flakiness budget for the stand checks, on the founder's «Повторное задание, ноль сбоев (Рекомендую)»: the `stand-repeat` leg and its job `Stand checks repeated` run the 40 `stand_*.rs` files and seven_guis' seven targets three times with no retry. 20 local rounds read 0 failures, each 168 · 0 · 7 and 24 · 0 · 0 (evidence/flake-repeats.md), and the first CI run, CI#38692629940, was green 17/17. Keep one NOT YET MEASURED for a performance budget alone."
    sidecar: "test-plan §10: the coverage floors (97 / 83 / 90 on the three driver crates) and the stand checks' zero-failure flakiness budget are set and enforced on fork CI. The performance budget stays not measured (2026-10-14-quality-gates)."
    rationale: "Report Changes → Counts (`flake` changed test-plan.md:342; `covfloor` changed test-plan.md:340 and :334; `cov5323` changed test-plan.md:340), Symbols / APIs (`coverage_floor.py`, the stand-repeat leg), Outcome (Coverage met, stand-repeat met, 20 local rounds met) and Expected amendments, test-plan §10. Founder labels as given in Cross-project → inputs#I2."
    basis: "test-plan.md:342; .github/scripts/coverage_floor.py:70-101; .github/scripts/ci-leg.sh:43-57"
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets"
    change: "Keep 53.23 % (2026-10-05) as the first line-coverage reading, taken before any threshold existed, not as the current state. Add the reading at this tree: workspace TOTAL 67.06 % lines (46 686 lines, 15 377 missed), with the three floored crates as in the gate line (evidence/coverage-floor.md)."
    sidecar: "test-plan §10: 53.23 % is restated as the first reading. The workspace now reads 67.06 % lines, and the driver crates are under floors (2026-10-14-quality-gates)."
    rationale: "Report Changes → Counts → Coverage: TOTAL 67.06 % lines against 53.23 % stated as the one reading; `cov5323` 1 changed (test-plan.md:340); `covfloor` changed :340 (the \"no threshold\" qualifier)."
    basis: "test-plan.md:340"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Coverage"
    change: "The `coverage` leg runs `cargo llvm-cov --workspace --locked --lcov --output-path target/coverage/lcov.info` and then `cargo llvm-cov report --workspace --locked`. Chained after them, it runs `python3 .github/scripts/coverage_floor.py target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`, and the leg exits with the floor script's status. The script (stdlib) sums `LF`/`LH` over the `SF` records under each crate directory and prints one line per crate, `{dir} {hit}/{lines} = {pct} % (floor {f} %)`. Its verdict is `coverage floor: N of N crates at or above their floors` (exit 0) or `K of N crates under their floors` (exit 1). A crate with no record or a malformed argument exits 2. It prints no source path but the crate directory. Retire \"with no threshold\"."
    sidecar: "test-plan §9 Coverage: the leg ends with coverage_floor.py over three crate floors and exits with its status (2026-10-14-quality-gates)."
    rationale: "Report Changes → Symbols / APIs (`ci-leg.sh` arm `coverage`, `coverage_floor.py`) and Counts (`covfloor` changed test-plan.md:334, the \"no threshold\" qualifier)."
    basis: "test-plan.md:334; .github/scripts/coverage_floor.py:70-101"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope (CI Python scripts)"
    change: "The `ci-scripts` leg runs 94 tests in seven test files. It was 79 in six until 2026-10-14-quality-gates, which added test_coverage_floor.py (7), 5 leg cases and 3 install cases. Among them are now the eight contract tests of `.github/scripts/apt-install.sh` (`InstallScriptTest`, five until that chunk) and test_coverage_floor.py's `CoverageFloorTest`, which covers the coverage floor script."
    sidecar: "test-plan §1 CI Python scripts: 94 tests in seven files. InstallScriptTest has eight tests, and test_coverage_floor.py joins (2026-10-14-quality-gates)."
    rationale: "Report Changes → Counts → CI scripts: `Ran 79` → `Ran 94` (+7 floor · +5 leg · +3 install) in six → seven test files; `ciscripts` changed test-plan.md:8; `dpkg` changed test-plan.md:8 (\"five contract tests\"). Outcome: `ci-scripts` passes, `Ran 94`. Each new CI-script path carries its unit tests (Coverage of new surfaces)."
    basis: "test-plan.md:8; .github/scripts/test_coverage_floor.py:17-88"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover (CI workflows and leg script)"
    change: "`CiWorkflowTest`'s tables gain `stand-repeat` (`LINUX_JOB_LEGS`, `RESTORE_ONLY_JOBS` `{a11y, stand-repeat}`, `INSTALL_JOBS`). The restore-only jobs are `a11y` and `stand-repeat`, and the jobs that call the install script are exactly the ten of `INSTALL_JOBS` (was nine). `LegScriptTest` changes as follows. The audit row asserts `--workspace`, `--all-features`, `advisories` and `licenses`. The coverage row becomes two cases over a seeded lcov: the crates at their floors exit 0 with `3 of 3`, and escher-driver one line under exits 1 with `1 of 3 … under`. A clippy case covers the workspace line and then the blitz-dom line. Two `stand-repeat` cases cover the three rounds and the stop at the first red. `InstallScriptTest` has eight tests (five until this chunk), against a stand-in `dpkg` recorded in the same call list: a healthy install runs no repair, an install failed by its bound is repaired before the next attempt, and a failed repair fails its attempt in phase `repair`. Three existing call lists now include the repair. Add test_coverage_floor.py's `CoverageFloorTest`, 7 cases: a crate above its floor passes; one at its floor passes; one under fails; two crates are read apart; two crates both holding pass; an absent crate exits 2 naming it; a malformed call exits 2 with usage. The CI-scripts leg runs 94 tests over seven files, test_ci_workflows.py 33 and test_coverage_floor.py 7 with the others unchanged, green at the fast leg (`Ran 94`, OK) and at CI#38692629940 on `d44090eb`. The counts of 79 and 78 stay as past readings."
    sidecar: "test-plan §4 CI workflows: the CiWorkflowTest tables include stand-repeat; new LegScriptTest cases (audit, coverage floor, clippy, stand-repeat); InstallScriptTest eight with the repair; CoverageFloorTest 7; 94 tests over seven files (2026-10-14-quality-gates)."
    rationale: "Report Changes → Symbols / APIs (`test_ci_workflows.py`, `test_coverage_floor.py`), Counts (`ciscripts` changed test-plan.md:145; `dpkg` changed :145, \"nine of `INSTALL_JOBS`\" and \"five tests\"; `INSTALL_JOBS` nine → ten; `RESTORE_ONLY_JOBS` `{a11y}` → `{a11y, stand-repeat}`; `joblist` changed :145) and Deviations 1 and 6."
    basis: "test-plan.md:145; .github/scripts/test_ci_workflows.py:323-367; .github/scripts/test_ci_workflows.py:418-452; .github/scripts/test_coverage_floor.py:35-88"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope (blitz-dom covered behaviours)"
    change: "Extend the `@font-face` coverage. Since 2026-10-14-quality-gates the hint-less source path (upstream #1109; an `@font-face` `data:` source with no format hint and no URL extension, its format sniffed from its bytes) is read by blitz-tests' `font_face_sniffed_source` 2. It runs offline, in both layout modes, under a font context that holds no family, with every request held by a hand `HeldNetProvider`. The source is requested exactly once, for its `data:` URL, and the text width is 0 before the answer. Answered with the bundled `wOF2` body, it settles with a width above 0 and no further request. An unsniffable body (`abcd`) registers nothing and the width stays 0. It installs no subscriber and prints nothing."
    sidecar: "test-plan §1 blitz-dom: the sniffed @font-face source (upstream #1109) is witnessed by font_face_sniffed_source (2026-10-14-quality-gates)."
    rationale: "Report Changes → Symbols / APIs (`font_face_sniffed_source.rs`, new target), Crates / modules (new target) and Expected amendments → security-plan §Input Validation: `fontface` extended test-plan.md:12 (`@font-face` registration covered). Outcome: `font_face_sniffed_source` passes."
    basis: "test-plan.md:12; tests/blitz-tests/tests/font_face_sniffed_source.rs:86-109; tests/blitz-tests/tests/font_face_sniffed_source.rs:111-135"
  - detector: D-tests-coverage
    severity: warning
    section: "§8 Mocking & Stubbing Discipline → Real dependencies kept"
    change: "Retire the claim that tests needing real font metrics skip with an `eprintln!` when text measures 0x0 without `system-fonts` (document.rs:2972-2990). The two `hover_state_tests` (`hovering_text_in_anonymous_block_reports_text_cursor`, `hovering_anonymous_block_whitespace_reports_default_cursor`) now assert their precondition through `#[track_caller] fn assert_text_has_size`, which needs the container's height over 50 px, and have no skip path. Their fixture builds `html`, `body` and both `div`s with the two-argument `qual_name!(…, html)`. With the one-argument form, which gives a no-namespace element the default sheet misses, both tests skipped under `--features system-fonts` too. Measured by `cargo test -p blitz-dom --locked --features system-fonts --lib hover_state_tests -- --nocapture`: exit 0, `2 passed`, no `skipping`."
    sidecar: "test-plan §8: the hover_state_tests font skip is gone. Both tests assert their precondition, and the skip was a one-argument qual_name! fixture, not missing system fonts (2026-10-14-quality-gates)."
    rationale: "Report Changes → Spec claims disproved by measurement (test-plan.md:293) and Symbols / APIs (`blitz-dom` `document.rs` `hover_state_tests`); Counts → `fontskip` changed test-plan.md:293 (the skip it cites is gone); Outcome → Hover met."
    basis: "test-plan.md:293; packages/blitz-dom/src/document.rs:2972-2981"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope (escher-driver)"
    change: "escher-driver holds 73 inline unit tests in thirteen files (72 until 2026-10-14-quality-gates). client.rs holds 3. It gains a unix-only `client::imp::tests` with `an_answer_read_past_its_bound_is_a_timeout`, which asserts `ANSWER.0 == wire::ANSWER_BOUND` and `wire::ANSWER_BOUND == Duration::from_secs(30)`. A `UnixListener` bound under `std::env::temp_dir()/eab-{pid}` accepts and never answers, and `exchange` under (200 ms, `wire::MAX_CALL_REPLY_BYTES`) returns `SessionError::Timeout` with elapsed at least 200 ms and under 10 s. The directory is removed afterwards."
    sidecar: "test-plan §1 escher-driver: 73 unit tests. client.rs pins the 30 s per-read answer bound, a read past it a Timeout (2026-10-14-quality-gates)."
    rationale: "Report Changes → Symbols / APIs (`escher-driver` `client.rs` new `client::imp::tests`), Crates / modules (new test module) and Counts (`escher_driver` lib 72 → 73). Outcome → Answer bound met."
    basis: "packages/escher-driver/src/client.rs:261-306"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover (escher-driver)"
    change: "client.rs also pins, since 2026-10-14-quality-gates, the per-read answer bound: `ANSWER` is `wire::ANSWER_BOUND`, 30 s, and an answer that is never sent ends as `SessionError::Timeout` at a shortened bound. seven_guis' `cli_commands` count reads 12 (11 until that chunk)."
    sidecar: "test-plan §4 escher-driver: client.rs pins the 30 s answer bound, and cli_commands reads 12 (2026-10-14-quality-gates)."
    rationale: "Report Changes → Symbols / APIs (`client.rs` test; `cli_commands.rs` new check) and Counts (escher_driver lib 72 → 73; seven_guis `cli_commands` 11 → 12). The §4 escher-driver bullet restates the client.rs coverage and the `cli_commands` count."
    basis: "packages/escher-driver/src/client.rs:261-306; examples/seven_guis/tests/cli_commands.rs:585-610"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope (apps, rdme, bump, examples — examples/seven_guis targets)"
    change: "`tests/cli_commands.rs` holds 12 tests (11 until 2026-10-14-quality-gates). The new check is `a_closed_stdout_drops_the_answer_and_keeps_the_ending`, through a local `with_stdout_closed`. That helper spawns the binary with `RUST_LOG` unset and stdin null, its stdout the write end of a pipe whose read end is dropped first and its stderr drained through `common::drain`, bounded by `common::RUN_BOUND`. `help` exits 0 with no `panicked` and no ESC byte on stderr. `snapshot --session {dir}` with no session exits 3, its stderr exactly `SessionError::NoSession`'s fixed message and line break, and no state directory is created. `common/mod.rs` is unchanged."
    sidecar: "test-plan §1 seven_guis: cli_commands has 12 tests; a closed stdout drops the answer and keeps the ending's status (2026-10-14-quality-gates)."
    rationale: "Report Changes → Symbols / APIs (`cli_commands.rs`: `with_stdout_closed`, the new check), Counts (seven_guis `cli_commands` 11 → 12) and Deviation 4. Outcome → Closed stdout met."
    basis: "examples/seven_guis/tests/cli_commands.rs:548-583; examples/seven_guis/tests/cli_commands.rs:585-610"
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Command line (`escher-session`, unix only)"
    change: "`cli_commands` holds 12 tests (11 until 2026-10-14-quality-gates, 10 until 2026-10-12-headless-screenshot, 8 until 2026-10-12-self-description). Since 2026-10-14-quality-gates, a run with its stdout closed exits with its ending's status (`help` 0; `snapshot` with no session 3, with exactly `NoSession`'s message). It prints no panic and writes nothing typed back."
    sidecar: "test-plan §6 Command line: cli_commands has 12 tests, with the closed-stdout check (2026-10-14-quality-gates)."
    rationale: "Report Changes → Counts (seven_guis `cli_commands` 11 → 12) and Symbols / APIs (`cli_commands.rs`). This is a second statement of the `cli_commands` count."
    basis: "examples/seven_guis/tests/cli_commands.rs:585-610"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope (tests/blitz-tests — `stand_act_spans`)"
    change: "Retire \"no stand step reads not settled, so `busy` is covered by its unit test only\". Since 2026-10-14-quality-gates `stand_act_spans` has an in-file `restless_fixture` that mounts a freshly keyed element on every render. In both layout modes, its info capture child runs `press tab` on it through `Session::run`. The outcome reads not settled with `Busy::Render`, and its diff equals the file's own snapshots' diff. The captured `command` line carries `settled=false` and `busy=\"render\"` (`BUSY_CLASSES[0]`), with no `passes` and no `cause`. Only `verb · settled · busy · added · removed · changed` sit beside the sink's `span · message · time.busy · time.idle`, the three counts equal to the diff's lists. `Run::finish` asserts that exactly one line carried a busy class. The run has 32 calls per mode (31 before)."
    sidecar: "test-plan §1 stand_act_spans: busy is read on a real not-settled command line (settled=false, busy=render); 32 calls per mode (2026-10-14-quality-gates)."
    rationale: "Report Changes → Symbols / APIs (`stand_act_spans.rs`: `Expect::Restless`, `restless_fixture`, the not-settled assertions, 31 → 32 calls) and Expected amendments → obs-plan §4: `busyline` changed test-plan.md:26. Deviation 2: six of the span's eight fields on that line. Outcome → `busy` met."
    basis: "test-plan.md:26; tests/blitz-tests/tests/stand_act_spans.rs:550-562"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Local baseline (`cargo test --workspace`)"
    change: "Append a re-count at 2026-10-14-quality-gates: 186 result lines, 955 passed · 0 failed · 12 ignored (from 185 · 951 · 0 · 12). The changes are `font_face_sniffed_source` +1 line and +2, the `escher_driver` lib 72 → 73, and seven_guis `cli_commands` 11 → 12. Timings were not re-measured. Source: that chunk's `ci-leg.sh fast` run (evidence/counts.md), equal in CI#38692629940's linux job."
    sidecar: "test-plan §9 Local baseline: the workspace re-counts to 186 · 955 · 0 · 12 (2026-10-14-quality-gates)."
    rationale: "Report Changes → Counts → Workspace test counts: 185 · 951 · 0 · 12 → 186 · 955 · 0 · 12 with the deltas attributed; `wscount` changed test-plan.md:330."
    basis: "test-plan.md:330"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts (Engine features by runner)"
    change: "Retire \"Not measured: `seven_guis_native`'s stderr by level as the workspace build makes it\". Since 2026-10-14-quality-gates three readings are recorded by build and level with 0 needles over 28 runs: the windowed binary, taken on the host's display (`wayland-1`); a client's own stderr; and flows beyond the three CRUD commands (escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/stderr-by-level.md, its instrument `stderr-by-level.py`)."
    sidecar: "test-plan §9 Engine features by runner: seven_guis_native's stderr is now measured by build and level, 0 needles over 28 runs (2026-10-14-quality-gates)."
    rationale: "Report Changes → Expected amendments → obs-plan §8: `stderrlvl` changed test-plan.md:324. Outcome → Stderr readings: the three readings are recorded by build and level with 0 needles, 28 runs, and the windowed reading was taken on `wayland-1`, so nothing is \"not measured\"."
    basis: "test-plan.md:324"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Session host ↔ what it writes"
    change: "Extend the by-level sentence (\"By level, both sink-installing binaries read 0 ids and 0 names …, the windowed stand by the evidence instrument on the dev host, not by a standing check\"). Since 2026-10-14-quality-gates, the windowed binary, a client's own stderr and flows beyond the three CRUD commands are also read by build and level with 0 needles over 28 runs, on the host's display (`wayland-1`) for the windowed one (evidence/stderr-by-level.md). This is still an evidence instrument, not a standing check."
    sidecar: "test-plan §5 Session host ↔ what it writes: the by-level readings extend to both builds, the windowed binary and further flows (2026-10-14-quality-gates)."
    rationale: "Report Changes → Expected amendments → obs-plan §8: `stderrlvl` extended test-plan.md:183 (by level). Outcome → Stderr readings met."
    basis: "test-plan.md:183"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "In `session-wire`, after \"a call's answer is read under 30 s per read\", add: pinned since 2026-10-14-quality-gates by escher-driver's unit test `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout`. It checks that `ANSWER.0 == wire::ANSWER_BOUND == 30 s`, and that a listener which accepts and never answers ends the read as `SessionError::Timeout`."
    sidecar: "test-plan §3 → Session lifecycle (session-wire): a unit test now pins the 30 s per-read answer bound (2026-10-14-quality-gates)."
    rationale: "Report Changes → Expected amendments → security-plan §Input Validation: `clipaths` extended registries/contracts/test-plan/session-lifecycle.md:6 · :7 (\"30 s per read\"), a check now pins each true claim; Symbols / APIs (`client.rs` test)."
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:7; packages/escher-driver/src/client.rs:261-306"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "In `session-host`, after `call(…)` \"reads the answer under 30 s per read\", add: pinned since 2026-10-14-quality-gates by `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout`, a read past the bound being `Timeout`."
    sidecar: "test-plan §3 → Session lifecycle (session-host): the call's 30 s per-read bound is pinned by a unit test (2026-10-14-quality-gates)."
    rationale: "Report Changes → Expected amendments: `clipaths` extended registries/contracts/test-plan/session-lifecycle.md:6 (\"30 s per read\"); the same claim as session-wire's, stated a second time."
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:6"
    dependent-of: D-tests-coverage
note: "D-tests-obs-harness has no violation. The report's Harness / gate surface says the agent-run harness is unchanged (scripts/agent-run.sh not in the diff), and Schema / config says no scrub, redaction, log field or span changed, so test-plan §3 and obs-plan §3 still agree. The stand-repeat job's `ci-log-stand-repeat` follows the generic `ci-log-{job id}` form (`faillog` no-change at test-plan.md:318). Several D-tests-coverage proposals are primaries for distinct claims, so `dependent-of: D-tests-coverage` binds to the nearest preceding primary of the same claim. Sites the report gives no line for (§1 escher-driver, §1 seven_guis, §4 escher-driver, §6 Command line) are named by section only."
