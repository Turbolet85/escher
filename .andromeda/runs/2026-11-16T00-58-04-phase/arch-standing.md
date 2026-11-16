# arch · standing conventions

- S1 · every new scenario test runs in both layout modes, and both modes must give identical layouts · conventions
  decision/one-incremental-pipeline · One pipeline for incremental and full layout — Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations. · guard: incremental_oracle
- S2 · a new test asserts its fixture's precondition first, with numbered cases in a feature fixture and relative URLs between fixtures · conventions
  decision/feature-fixture-numbered-cases · Numbered cases in feature fixtures — Feature HTML fixtures group numbered cases under h2/h3 headings with a CSS comment per case.
  decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test.
  decision/fixture-relative-asset-urls · Relative asset URLs in fixtures — HTML fixtures reference sibling assets with ./-relative URLs.
  decision/numbered-fixture-cases · Numbered fixture cases — Feature fixtures group numbered cases under h2/h3 headings, with a CSS comment per case.
  decision/relative-fixture-urls · Relative URLs between fixtures — Fixtures reference sibling assets with `./`-relative URLs.
  decision/standalone-example-fixtures · Standalone HTML example fixtures — Example fixtures are standalone HTML documents with no build manifest, source module or config, referencing external assets.
  decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test.
- S3 · new tests carry descriptive names, one behaviour per integration test file, and unit tests sit at the end of their source file or in path children of their driver module · conventions · existing-scopes
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
  decision/driver-unit-tests-in-path-children · Driver unit tests in path children — An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.
  decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
  decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate.
  decision/split-test-module-as-child · Split test modules stay children of their module — A unit-test module in a file of its own is declared through `#[path]` as a `#[cfg(test)]` child of the module under test, moved out whole with its test paths unchanged. · why: As a child it reaches the module's private fields with no constructor, `pub` or `cfg(test)` shim added.
  decision/unit-test-placement · Unit-test placement — Unit tests sit in a test module at the end of the source file, or in a child file of their own.
  decision/unit-tests-at-file-end · Unit tests at the end of the source file — Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file. · except: Unit-test modules in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.
- S4 · every new expect or unwrap carries its justification, clippy allows stay local, and rustdoc warnings are errors · conventions
  decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
  decision/clippy-allow-scoping · Clippy allows kept local — Clippy allows are local to the code they cover. · except: `clippy::collapsible_if` is allowed crate-wide in five crates (browser, blitz-dom, blitz-html, blitz-paint, blitz-vibey-script) and `clippy::module_inception` in blitz-dom's node module.
  decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification.
  decision/local-lint-allows · Lints allowed locally — Clippy lints are allowed locally, some with a reason, not crate-wide. · except: `clippy::collapsible_if` crate-wide in browser, blitz-dom, blitz-html, blitz-paint and blitz-vibey-script; `clippy::module_inception` in the node module.
  decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate.
  decision/rustdoc-warnings-denied · Rustdoc warnings are errors — Rustdoc warnings are errors over every workspace library crate.
  decision/rustdoc-warnings-gate · Workspace rustdoc warnings gate — Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision. · except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
  decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
- S5 · every new module and public item is documented, with spec URLs cited where the code implements a CSS layout rule · conventions
  decision/cite-the-spec · Behave like a browser, and cite the spec — Web semantics are written against named specifications and named browser engines.
  decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
  decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
  decision/public-item-docs · Public item docs — Public items carry `///` docs. · except: Some upstream public items carry no `///`, a node's tree accessor among them.
  decision/spec-url-citations · Spec URLs cited at implementations — Spec URLs are cited above or next to implementations.
- S6 · a new stand check keeps its shared helpers in non-target modules and its readings identical on every CI OS · standard-contracts
  decision/fixtures-test-module · Shared fixtures test module — A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file.
  decision/shared-test-modules · Shared test modules in subdirectories — The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`. · why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior.
  decision/stand-check-shared-helpers · Stand-check helpers in non-target modules — Stand checks share helpers through modules under the tests directory that are no test target.
