# arch · standing conventions

- S1 · each new integration test lives in blitz-tests, one behaviour per test file and one test target per file · design-philosophy · conventions
  decision/blitz-tests-one-target-per-file · One test target per file in blitz-tests — blitz-tests keeps one test target per file. · except: upstream's tests/all.rs, declared and held at test = false.
  decision/blitz-tests-single-test-target · blitz-tests manifest declares one test table — The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false. · guard: BlitzTestsTargetsTest
  decision/blitz-tests-target-per-file · One blitz-tests target per file — blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`. · why: Ratified; a standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest
  decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
  decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate.
- S2 · each new test has a descriptive name, asserts its fixture's condition first, and states shared fixtures once with numbered cases · conventions
  decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names.
  decision/feature-fixture-numbered-cases · Numbered cases in feature fixtures — Feature HTML fixtures group numbered cases under h2/h3 headings with a CSS comment per case.
  decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test.
  decision/fixtures-test-module · Shared fixtures test module — A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file.
  decision/numbered-fixture-cases · Numbered fixture cases — Feature fixtures group numbered cases under h2/h3 headings, with a CSS comment per case.
  decision/select-tests-name-element · Select tests name the element — The select tests the driver reads name the element and never the role.
  decision/shared-fixtures-stated-once · Shared blitz-tests fixtures stated once — The blitz-tests fixture module states the fixture boot and hold, the shared fixture pages and the held network stand-in once.
  decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test.
- S3 · each new engine scenario runs in both layout modes · conventions
- S4 · unit tests go at the end of the source file, and driver unit tests go in path children; stand-check helpers go in non-target modules · conventions · existing-scopes
  decision/driver-unit-tests-in-path-children · Driver unit tests in path children — An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.
  decision/shared-test-modules · Shared test modules in subdirectories — The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`. · why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior.
  decision/split-test-module-as-child · Split test modules stay children of their module — A unit-test module in a file of its own is declared through `#[path]` as a `#[cfg(test)]` child of the module under test, moved out whole with its test paths unchanged. · why: As a child it reaches the module's private fields with no constructor, `pub` or `cfg(test)` shim added.
  decision/stand-check-shared-helpers · Stand-check helpers in non-target modules — Stand checks share helpers through modules under the tests directory that are no test target.
  decision/unit-test-placement · Unit-test placement — Unit tests sit in a test module at the end of the source file, or in a child file of their own.
  decision/unit-tests-at-file-end · Unit tests at the end of the source file — Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file. · except: Eight unit-test modules sit in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.
