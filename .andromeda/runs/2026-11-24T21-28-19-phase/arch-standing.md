# arch · standing conventions

- S1 · every new scenario and stand check runs in both layout modes · conventions
- S2 · a test asserts its fixture's precondition first, then the behaviour; fixtures use numbered cases and relative asset URLs · conventions
  decision/feature-fixture-numbered-cases · Numbered cases in feature fixtures — Feature HTML fixtures group numbered cases under h2/h3 headings with a CSS comment per case.
  decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test.
  decision/fixture-relative-asset-urls · Relative asset URLs in fixtures — HTML fixtures reference sibling assets with ./-relative URLs.
  decision/numbered-fixture-cases · Numbered fixture cases — Feature fixtures group numbered cases under h2/h3 headings, with a CSS comment per case.
  decision/relative-fixture-urls · Relative URLs between fixtures — Fixtures reference sibling assets with `./`-relative URLs.
  decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test.
- S3 · descriptive test names, one behaviour per integration test file, one test target per file in blitz-tests · conventions
  decision/blitz-tests-one-target-per-file · One test target per file in blitz-tests — blitz-tests keeps one test target per file. · except: upstream's tests/all.rs, declared and held at test = false.
  decision/blitz-tests-single-test-target · blitz-tests manifest declares one test table — The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false. · guard: BlitzTestsTargetsTest
  decision/blitz-tests-target-per-file · One blitz-tests target per file — blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`. · why: Ratified; a standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
  decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
  decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate. · except: blitz-vibey-script keeps its own integration tests in its tests directory
  decision/select-tests-name-element · Select tests name the element — The select tests the driver reads name the element and never the role.
- S4 · unit tests at the end of the source file, driver unit tests as path children, split test modules stay children of their module · conventions · existing-scopes
  decision/driver-unit-tests-in-path-children · Driver unit tests in path children — An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.
  decision/moved-unit-tests-keep-paths · Moved unit-test modules keep their test paths — Unit-test modules moved to files of their own keep their test paths.
  decision/split-test-module-as-child · Split test modules stay children of their module — A unit-test module in a file of its own is declared through `#[path]` as a `#[cfg(test)]` child of the module under test, moved out whole with its test paths unchanged. · why: As a child it reaches the module's private fields with no constructor, `pub` or `cfg(test)` shim added.
  decision/unit-test-placement · Unit-test placement — Unit tests sit in a test module at the end of the source file, or in a child file of their own.
  decision/unit-tests-at-file-end · Unit tests at the end of the source file — Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file. · except: Unit-test modules in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.
- S5 · new public items and modules carry docs, expect and unwrap carry a justification, clippy allows kept local, rustdoc warnings are errors · conventions
  contract/ci-leg-legs/clippy · ci-leg.sh legs · clippy — The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`.
  contract/ci-leg-legs/doc · ci-leg.sh legs · doc — The `doc` leg runs `cargo doc --workspace --no-deps`.
  decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
  decision/clippy-allow-scoping · Clippy allows kept local — Clippy allows are local to the code they cover. · except: `clippy::collapsible_if` is allowed crate-wide in five crates (browser, blitz-dom, blitz-html, blitz-paint, blitz-vibey-script) and `clippy::module_inception` in blitz-dom's node module.
  decision/constant-rationale-docs · Rationale for constants — Doc comments state the rationale for constants.
  decision/fmt-and-clippy-gates · Format and clippy gates — `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
  decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification.
  decision/local-lint-allows · Lints allowed locally — Clippy lints are allowed locally, some with a reason, not crate-wide. · except: `clippy::collapsible_if` crate-wide in browser, blitz-dom, blitz-html, blitz-paint and blitz-vibey-script; `clippy::module_inception` in the node module.
  decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
  decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
  decision/public-item-docs · Public item docs — Public items carry `///` docs. · except: Some upstream public items carry no `///`, a node's tree accessor among them.
  decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate.
  decision/rustdoc-warnings-denied · Rustdoc warnings are errors — Rustdoc warnings are errors over every workspace library crate.
  decision/rustdoc-warnings-gate · Workspace rustdoc warnings gate — Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision. · except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
  decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
- S6 · stand-check helpers live in non-target modules and stand readings stay identical on every CI OS under the stand-repeat zero-failure budget · standard-contracts
  contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
  decision/shared-test-modules · Shared test modules in subdirectories — The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`. · why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior.
  decision/stand-check-shared-helpers · Stand-check helpers in non-target modules — Stand checks share helpers through modules under the tests directory that are no test target.
  decision/stand-repeat-zero-failure · Stand-repeat zero-failure budget — The `stand-repeat` leg has a zero-failure budget.
  decision/stand-repeat-zero-failure-budget · Stand-repeat zero-failure budget — The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed. · guard: test_stand_repeat_leg_runs_three_rounds
