# tests · standing conventions

- S1 · a test split out of a source file stays an inline cfg(test) child module of the source it tests, unit tests sitting in their source file · test-strategy · 2-test-strategy
- S2 · a split test module keeps its tests' paths, and moved table-driven tests keep asserting their row count, their failures printing no argument value · unit-test-strategy · 4-unit-test-strategy
  decision/assertions-print-no-argument-value · Test failures print no argument value — An assertion over a driver call names only its row and index and never an argument value. · why: A failing test log then prints no argument value.
  decision/call-assertions-carry-row-index · Call assertions carry a row index only — The driver's assertions over a call carry a row index only, never an argument value. · why: so a failing log prints no argument value
  decision/layout-reason-words-each-read · Every layout reader word read on a fixture — Every placing word and every size cause of the layout reader is read on a fixture node in both layout modes, from tables that count their rows. · guard: layout_reason
  decision/table-tests-count-their-rows · Table-driven tests assert their row count — Each table a driver test reads is stated in the test itself and its row count asserted.
  decision/test-module-split-keeps-paths · Split test modules keep their paths — A unit-test module moved to a file of its own is moved out whole with its test paths unchanged.
- S3 · Rust tests stay on the built-in harness with shared fixtures and stand tables in one shared module · 3-test-harness-contract
  decision/dedicated-headless-test-harness · Dedicated headless test harness — Blitz documents are tested headlessly through a dedicated, unpublished harness crate.
  decision/rust-tests-on-built-in-harness · Rust tests on the built-in harness — Rust tests run under the standard built-in harness: #[cfg(test)] modules of #[test] functions asserting with assert! and assert_eq!, and #[tokio::test] for async worker tests. · except: upstream's merged checks that return TestResult<()> and assert with test-that's verify_that!, under the same harness
  decision/shared-fixture-pages · Shared fixture pages stated once — A fixture page that more than one check reads is stated once as an rsx component and booted with the stand's options. · except: a page with one reader stays in its check's file
