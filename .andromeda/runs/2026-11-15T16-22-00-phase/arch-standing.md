# arch · standing conventions

- S1 · each new test carries a descriptive name saying the behaviour it proves · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
- S2 · each new unit test sits in the test module at the end of the source file it tests, placed as the unit-test placement rule says · conventions
  decision/driver-unit-tests-in-path-children · Driver unit tests in path children — An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.
  decision/split-test-module-as-child · Split test modules stay children of their module — A unit-test module in a file of its own is declared through `#[path]` as a `#[cfg(test)]` child of the module under test, moved out whole with its test paths unchanged. · why: As a child it reaches the module's private fields with no constructor, `pub` or `cfg(test)` shim added.
  decision/unit-test-placement · Unit-test placement — Unit tests sit in a test module at the end of the source file, or in a child file of their own.
  decision/unit-tests-at-file-end · Unit tests at the end of the source file — Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file. · except: Unit-test modules in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.
- S3 · each new test asserts its fixture's precondition first, before the behaviour it proves · conventions
  decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test.
  decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test.
- S4 · each new test module and test passes the formatting and clippy gate and the rustdoc warnings gate, and carries the module docs a new module owes · conventions
  contract/ci-leg-legs/clippy · ci-leg.sh legs · clippy — The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`.
  contract/ci-leg-legs/doc · ci-leg.sh legs · doc — The `doc` leg runs `cargo doc --workspace --no-deps`.
  contract/ci-leg-legs/fast · ci-leg.sh legs · fast — The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate.
  contract/ci-leg-legs/fmt · ci-leg.sh legs · fmt — The `fmt` leg runs `cargo fmt --all --check`.
  decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
  decision/fmt-and-clippy-gate · Formatting and clippy gate — `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green.
  decision/fmt-and-clippy-gates · Format and clippy gates — `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
  decision/fmt-clippy-gates · fmt and clippy gates — CI gates every change on rustfmt and clippy.
  decision/local-pre-push-gate · Local pre-push gate — The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate.
  decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
  decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
  decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate.
  decision/rustdoc-warnings-denied · Rustdoc warnings are errors — Rustdoc warnings are errors over every workspace library crate.
  decision/rustdoc-warnings-gate · Workspace rustdoc warnings gate — Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision. · except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
