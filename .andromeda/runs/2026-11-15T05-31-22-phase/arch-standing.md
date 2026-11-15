# arch · standing conventions

- S1 · each upstream test file taken lives in blitz-tests as one behaviour per file with its own test target, and states shared fixtures once · conventions
  decision/fixtures-test-module · Shared fixtures test module — A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file.
  decision/shared-fixtures-stated-once · Shared blitz-tests fixtures stated once — The blitz-tests fixture module states the fixture boot and hold, the shared fixture pages and the held network stand-in once.
  decision/shared-test-modules · Shared test modules in subdirectories — The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`. · why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior.
  decision/single-reader-page-stays-local · Single-reader fixture page stays local — A fixture page with one reader stays in its check's file.
  decision/stand-check-shared-helpers · Stand-check helpers in non-target modules — Stand checks share helpers through modules under the tests directory that are no test target.
- S2 · a new check (the inline-root hit coordinate, if owed) gets a descriptive test name, cites the upstream commit it regresses against, asserts its fixture first and runs in both layout modes · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
  decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test.
  decision/regression-tests-cite-upstream · Regression tests cite upstream — Regression tests cite upstream issue or PR URLs.
  decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test.
- S3 · any expect or unwrap in new or resolved code carries a justification, and clippy allows stay local and justified · conventions
  decision/clippy-allow-scoping · Clippy allows kept local — Clippy allows are local to the code they cover. · except: `clippy::collapsible_if` is allowed crate-wide in five crates (browser, blitz-dom, blitz-html, blitz-paint, blitz-vibey-script) and `clippy::module_inception` in blitz-dom's node module.
  decision/local-lint-allows · Lints allowed locally — Clippy lints are allowed locally, some with a reason, not crate-wide. · except: `clippy::collapsible_if` crate-wide in browser, blitz-dom, blitz-html, blitz-paint and blitz-vibey-script; `clippy::module_inception` in the node module.
- S4 · every new log line or tracing call site is gated per call site and passes the telemetry scrub and target filter · cross-cutting-patterns
  decision/wpt-runner-logging · WPT runner logging — The WPT runner logs through the `log` facade with `env_logger`.
- S5 · every CI leg the take re-runs goes through the one leg runner, locked · standard-contracts
