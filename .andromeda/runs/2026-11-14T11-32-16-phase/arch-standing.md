# arch · standing conventions

- S1 · a new blitz-tests integration test file is its own test target, states one behaviour, has a descriptive name, and joins the target pin · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
  decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
  decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate.
- S2 · a new stand check runs its scenarios in both layout modes, asserts the fixture's condition first and shares its helpers through non-target modules · conventions
  decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test.
  decision/stand-check-shared-helpers · Stand-check helpers in non-target modules — Stand checks share helpers through modules under the tests directory that are no test target.
  decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test.
- S3 · shared blitz-tests fixtures are stated once, in the shared fixtures module, and a stand check builds on them · conventions
  decision/single-reader-page-stays-local · Single-reader fixture page stays local — A fixture page with one reader stays in its check's file.
- S4 · new driver code is documented at module and item level, keeps crate internals restricted, returns typed errors as enums and justifies every expect or unwrap · conventions
  decision/constant-rationale-docs · Rationale for constants — Doc comments state the rationale for constants.
  decision/crate-internal-visibility · Crate internals restricted — Crate internals are `pub(crate)` / `pub(super)`.
  decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification. · except: The browser app's test module allows `clippy::expect_used` with no justification beside it.
  decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
  decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
  decision/public-item-docs · Public item docs — Public items carry `///` docs. · except: Some upstream public items carry no `///`, a node's tree accessor among them.
  decision/restricted-crate-internals · Restricted crate internals — Crate internals are `pub(crate)` or `pub(super)`.
  decision/typed-results · Typed results for fallible APIs — Fallible APIs return typed results.
  decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
- S5 · new driver unit tests live in path children of the module they test, at the file's end · conventions · existing-scopes
  decision/driver-unit-tests-in-path-children · Driver unit tests in path children — An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.
  decision/moved-unit-tests-keep-paths · Moved unit-test modules keep their test paths — Unit-test modules moved to files of their own keep their test paths.
  decision/split-test-module-as-child · Split test modules stay children of their module — A unit-test module in a file of its own is declared through `#[path]` as a `#[cfg(test)]` child of the module under test, moved out whole with its test paths unchanged. · why: As a child it reaches the module's private fields with no constructor, `pub` or `cfg(test)` shim added.
  decision/unit-test-placement · Unit-test placement — Unit tests sit in a test module at the end of the source file, or in a child file of their own.
  decision/unit-tests-at-file-end · Unit tests at the end of the source file — Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file. · except: Eight unit-test modules sit in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.
- S6 · stand readings are identical on every CI OS, the stand boots offline with a pinned environment, and the stand-repeat leg runs with a zero-failure budget · standard-contracts · inherited-defaults
  decision/stand-repeat-zero-failure · Stand-repeat zero-failure budget — The `stand-repeat` leg has a zero-failure budget.
  decision/stand-repeat-zero-failure-budget · Stand-repeat zero-failure budget — The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed. · guard: test_stand_repeat_leg_runs_three_rounds
- S7 · every escher crate keeps its per-crate coverage floor, and fmt, clippy and rustdoc warnings gate the change · inherited-defaults
  contract/ci-leg-legs/clippy · ci-leg.sh legs · clippy — The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`.
  contract/ci-leg-legs/coverage · ci-leg.sh legs · coverage — The `coverage` leg runs `cargo llvm-cov` to an lcov file and its per-file report, then the coverage-floor script, and exits with the floor's status.
  contract/ci-leg-legs/doc · ci-leg.sh legs · doc — The `doc` leg runs `cargo doc --workspace --no-deps`.
  contract/ci-leg-legs/fast · ci-leg.sh legs · fast — The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate.
  contract/ci-leg-legs/fmt · ci-leg.sh legs · fmt — The `fmt` leg runs `cargo fmt --all --check`.
  contract/coverage-floor-exit-codes/0 · coverage_floor.py exit codes · 0 — Every crate is at or above its floor, printed as `coverage floor: N of N crates at or above their floors`.
  contract/coverage-floor-exit-codes/1 · coverage_floor.py exit codes · 1 — Some crates are under their floors, printed as `coverage floor: K of N crates under their floors`.
  contract/coverage-floor-exit-codes/2 · coverage_floor.py exit codes · 2 — A crate has no lcov record (its directory named on stderr) or an argument is malformed (a usage line).
  contract/coverage-floor-output/crate-line · coverage_floor.py output · crate line — The coverage-floor script prints one line per crate, `{dir} {hit}/{lines} = {pct:.2f} % (floor {f} %)`, naming no source path but the crate directory.
  decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
  decision/coverage-excludes-keyboard-compat · Coverage excludes the keyboard compat bridge — Both coverage measurement lines exclude dioxus-native-dom's crate-private keyboard compat module, the floors unchanged. · why: The module is upstream's temporary keyboard-types 0.7/0.8 bridge, excluded until escher takes upstream's retirement of it. · guard: test_coverage_leg_reaches_the_floor_on_each_crate
  decision/coverage-floors · Per-crate coverage floors — A cargo-llvm-cov coverage report is gated by per-crate floors: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness. · guard: CoverageFloorTest
  decision/fmt-and-clippy-gate · Formatting and clippy gate — `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green.
  decision/fmt-and-clippy-gates · Format and clippy gates — `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
  decision/fmt-clippy-gates · fmt and clippy gates — CI gates every change on rustfmt and clippy.
  decision/per-crate-coverage-floor · Per-crate coverage floor — CI holds a coverage floor per crate. · guard: coverage_floor.py
  decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg gates line coverage by per-crate floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %. · guard: coverage_floor.py
  decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate.
  decision/rustdoc-warnings-denied · Rustdoc warnings are errors — Rustdoc warnings are errors over every workspace library crate.
  decision/rustdoc-warnings-gate · Workspace rustdoc warnings gate — Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision. · except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
