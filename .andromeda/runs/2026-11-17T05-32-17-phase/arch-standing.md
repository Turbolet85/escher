# arch · standing conventions

- S1 · unit tests sit at the end of their source file, or in a child test module of it, with descriptive names · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
  decision/split-test-module-as-child · Split test modules stay children of their module — A unit-test module in a file of its own is declared through `#[path]` as a `#[cfg(test)]` child of the module under test, moved out whole with its test paths unchanged. · why: As a child it reaches the module's private fields with no constructor, `pub` or `cfg(test)` shim added.
  decision/unit-test-placement · Unit-test placement — Unit tests sit in a test module at the end of the source file, or in a child file of their own.
  decision/unit-tests-at-file-end · Unit tests at the end of the source file — Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file. · except: Unit-test modules in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.
- S2 · a test asserts its fixture's own condition first, and fixture cases are numbered and stated once · conventions
  decision/feature-fixture-numbered-cases · Numbered cases in feature fixtures — Feature HTML fixtures group numbered cases under h2/h3 headings with a CSS comment per case.
  decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test.
  decision/fixtures-test-module · Shared fixtures test module — A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file.
  decision/numbered-fixture-cases · Numbered fixture cases — Feature fixtures group numbered cases under h2/h3 headings, with a CSS comment per case.
  decision/shared-fixtures-stated-once · Shared blitz-tests fixtures stated once — The blitz-tests fixture module states the fixture boot and hold, the shared fixture pages and the held network stand-in once.
  decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test.
- S3 · expect and unwrap in new code carry a justification; fallible new APIs return typed results with error enums · conventions · cross-cutting-patterns
  decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification.
  decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
- S4 · new public items and modules carry docs, and rustdoc warnings stay errors · conventions
  contract/ci-leg-legs/doc · ci-leg.sh legs · doc — The `doc` leg runs `cargo doc --workspace --no-deps`.
  decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
  decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
  decision/public-item-docs · Public item docs — Public items carry `///` docs. · except: Some upstream public items carry no `///`, a node's tree accessor among them.
  decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate.
  decision/rustdoc-warnings-denied · Rustdoc warnings are errors — Rustdoc warnings are errors over every workspace library crate.
  decision/rustdoc-warnings-gate · Workspace rustdoc warnings gate — Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision. · except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
- S5 · stand checks keep shared helpers in non-target modules and integration test files hold one behavior each · conventions
  decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
  decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate.
  decision/shared-test-modules · Shared test modules in subdirectories — The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`. · why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior.
  decision/stand-check-shared-helpers · Stand-check helpers in non-target modules — Stand checks share helpers through modules under the tests directory that are no test target.
- S6 · CI helper scripts and their tests are stdlib Python under unittest, and the CI workflow contract stays pinned by tests · conventions · standard-contracts
  contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
  decision/actions-pinned-to-commit-sha · Actions pinned to commit SHAs — Every action a CI workflow uses is pinned to a 40-hex commit SHA, its ref kept as a trailing comment. · guard: test_ci_workflows.py
  decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py
  decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function.
  decision/ci-scripts-stdlib-python · CI helper scripts on the Python stdlib — The CI helper scripts, the coverage floor among them, use the Python stdlib and PyYAML, which the ci-scripts leg installs when absent; that leg needs Python 3.11 or later for tomllib. · why: no package added
  decision/cold-agent-judge-stdlib-python · Cold-agent judge on stdlib Python — The cold-agent pipe's judge is python3 on its standard library, with no package added. · why: ratified
  decision/free-disk-space-only-in-stand-repeat · Disk freed only in the stand-repeat job — The Stand checks repeated job is the workflow's one run of the free-disk-space action, pinned by sha and run after the toolchain and before the rust cache; the matrix test job holds no such step. · guard: test_only_stand_repeat_frees_disk
  decision/pinned-actions · SHA-pinned actions — Every uses: of every workflow is pinned to a 40-hex commit SHA, its ref kept as a trailing comment, and every rust-toolchain step names its toolchain. · guard: test_ci_workflows.py
- S7 · new code holds each crate's line coverage floor and the cognitive complexity ceiling · standard-contracts · stack-and-technologies
  contract/ci-leg-legs/coverage · ci-leg.sh legs · coverage — The `coverage` leg runs `cargo llvm-cov` to an lcov file and its per-file report, then the coverage-floor script, and exits with the floor's status.
  contract/coverage-floor-exit-codes/0 · coverage_floor.py exit codes · 0 — Every crate is at or above its floor, printed as `coverage floor: N of N crates at or above their floors`.
  contract/coverage-floor-exit-codes/1 · coverage_floor.py exit codes · 1 — Some crates are under their floors, printed as `coverage floor: K of N crates under their floors`.
  contract/coverage-floor-exit-codes/2 · coverage_floor.py exit codes · 2 — A crate has no lcov record (its directory named on stderr) or an argument is malformed (a usage line).
  decision/cognitive-complexity-ceiling · Cognitive complexity ceiling — A cognitive-complexity reading on the dev host alone, never in CI, reads each function against a ceiling of 15 with rust-code-analysis-cli.
  decision/coverage-excludes-keyboard-compat · Coverage excludes the keyboard compat bridge — Both coverage measurement lines exclude dioxus-native-dom's crate-private keyboard compat module, the floors unchanged. · why: The module is upstream's temporary keyboard-types 0.7/0.8 bridge, excluded until escher takes upstream's retirement of it. · guard: test_coverage_leg_reaches_the_floor_on_each_crate
  decision/coverage-floors · Per-crate coverage floors — A cargo-llvm-cov coverage report is gated by per-crate floors: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness. · guard: CoverageFloorTest
  decision/per-crate-coverage-floor · Per-crate coverage floor — CI holds a coverage floor per crate. · guard: coverage_floor.py
  decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg gates line coverage by per-crate floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %. · guard: coverage_floor.py
