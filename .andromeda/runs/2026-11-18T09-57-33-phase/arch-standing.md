# arch · standing conventions

- S1 · each new blitz-tests stand check is one test target per file, declared in the one test table, runs its scenario in both layout modes, asserts its fixture first, and keeps shared stand-check helpers in non-target modules · conventions
  decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test.
  decision/shared-test-modules · Shared test modules in subdirectories — The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`. · why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior.
  decision/stand-check-shared-helpers · Stand-check helpers in non-target modules — Stand checks share helpers through modules under the tests directory that are no test target.
  decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test.
- S2 · each new test is named descriptively and covers one behaviour per integration test file · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
  decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
- S3 · each new CI Python test uses unittest on the Python stdlib at the ci-scripts floor · conventions · stack-and-technologies
  decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function.
  decision/ci-scripts-python-floor · Python 3.11 floor for the ci-scripts leg — The `ci-scripts` leg needs Python 3.11 or later.
  decision/ci-scripts-stdlib-python · CI helper scripts on the Python stdlib — The CI helper scripts, the coverage floor among them, use the Python stdlib and PyYAML, which the ci-scripts leg installs when absent; that leg needs Python 3.11 or later for tomllib. · why: no package added
- S4 · each new source file carries module and item docs, its expect and unwrap justified, its lint allows kept local, and passes the fmt, clippy and rustdoc gates · conventions
  contract/ci-leg-legs/clippy · ci-leg.sh legs · clippy — The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`.
  contract/ci-leg-legs/doc · ci-leg.sh legs · doc — The `doc` leg runs `cargo doc --workspace --no-deps`.
  contract/ci-leg-legs/fmt · ci-leg.sh legs · fmt — The `fmt` leg runs `cargo fmt --all --check`.
  decision/clippy-allow-scoping · Clippy allows kept local — Clippy allows are local to the code they cover. · except: `clippy::collapsible_if` is allowed crate-wide in five crates (browser, blitz-dom, blitz-html, blitz-paint, blitz-vibey-script) and `clippy::module_inception` in blitz-dom's node module.
  decision/constant-rationale-docs · Rationale for constants — Doc comments state the rationale for constants.
  decision/example-docs · Example header docs — Examples open with a `//!` comment describing what they demonstrate. · except: Most upstream examples open with a block comment or none.
  decision/fmt-and-clippy-gate · Formatting and clippy gate — `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green.
  decision/fmt-and-clippy-gates · Format and clippy gates — `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
  decision/fmt-clippy-gates · fmt and clippy gates — CI gates every change on rustfmt and clippy.
  decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification.
  decision/local-lint-allows · Lints allowed locally — Clippy lints are allowed locally, some with a reason, not crate-wide. · except: `clippy::collapsible_if` crate-wide in browser, blitz-dom, blitz-html, blitz-paint and blitz-vibey-script; `clippy::module_inception` in the node module.
  decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
  decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
  decision/public-item-docs · Public item docs — Public items carry `///` docs. · except: Some upstream public items carry no `///`, a node's tree accessor among them.
  decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate.
  decision/rustdoc-warnings-denied · Rustdoc warnings are errors — Rustdoc warnings are errors over every workspace library crate.
  decision/rustdoc-warnings-gate · Workspace rustdoc warnings gate — Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision. · except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
  decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
- S5 · each new stand target runs in the stand-repeat leg under its zero-failure budget · stack-and-technologies
