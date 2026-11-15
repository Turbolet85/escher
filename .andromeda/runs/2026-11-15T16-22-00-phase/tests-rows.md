# test-plan · the rows that bind 2026-11-15-browser-corrective — picked 28 of 605 (14 by every draw) · rule any · pin 1111

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/workspace-tests-in-fast-ci-leg · Workspace tested in the fast CI leg — Every CI run tests the whole workspace with a locked lockfile on ubuntu with default features, as the fast test leg.
decision/test-tier · Test tier — The project's test tier is 0.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/inline-unit-test-modules · Inline unit test modules — Unit tests are Rust test functions in cfg(test) modules beside the code under test. · except: modules held in files of their own through a path attribute
decision/regression-tests-document-bug · Regression tests document their bug — A regression test documents the bug and the real-world page that exposed it.
decision/regression-tests-named-for-bug · Regression tests named for their bug — A regression test is named for the bug it guards.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/inline-cfg-test-modules · Unit tests in inline test modules — Unit tests sit in cfg(test) modules inside their source files. · except: cfg(test) modules held in files of their own through a path attribute at their parent.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/rust-tests-on-built-in-harness · Rust tests on the built-in harness — Rust tests run under the standard built-in harness: #[cfg(test)] modules of #[test] functions asserting with assert! and assert_eq!, and #[tokio::test] for async worker tests. · except: upstream's merged accessibility_label and accessibility_description checks return TestResult<()> and assert with test-that's verify_that!, under the same harness
decision/test-data-is-code · Test data is code, nothing seeded — Nothing is seeded for the agent-run contract: the fixtures are code compiled into each target by `boot`, so `boot` is the bootstrap.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/unit-tests-in-source-file · Unit tests sit in their source file — Unit tests sit in a test-only module inside the source file; a module moved out of line sits in its own file declared test-only by path at its parent, its test paths unchanged. · except: the Dioxus bridge tests are a child of the Dioxus document module, because the event handler's two fields are private to it and it has no constructor

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again; a control that removes a bound loosens it to twice the budget rather than removing it, so the mutated run stays bounded.
decision/assertions-print-no-argument-value · Test failures print no argument value — An assertion over a driver call names only its row and index and never an argument value. · why: A failing test log then prints no argument value.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/no-wall-time-in-session-checks · No wall time in session checks — Session checks hold no sleep and no wall-clock read; a check waits on a load through the driver's load wait, and the Timer flow advances by the driver only. · except: The checks whose subject is wall time: the virtual-clock check's 800 ms sleep, the load-wait check's clock reads and stand-in sleep, and the lifecycle check's wait on the host's idle expiry.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/file-urls-only-for-file-documents · file: URLs loadable only from a file: document — A `file:` URL is loaded only for a `file:` document: otherwise a script's `fetch()`, `import` or `<script src>` is refused with a `FetchError`, a blitz-net sub-resource load reads no local file, and the browser app drops a `file:` favicon or link navigation silently; user loads are not gated. · guard: remote_document_cannot_load_file_urls, file_document_can_load_sibling_file_urls, local_files_for_local_documents
decision/no-sleep-or-wall-clock-in-checks · No sleep or wall-clock read in checks — Session checks take no sleep and read no wall clock; time moves by `advance` alone. · except: stand_virtual_clock's 800 ms sleep, stand_load_wait's clock reads and 200 ms stand-in sleep, stand_session_lifecycle's idle-expiry wait and stand_contrast's bound on a load's answer.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/ui-driven-through-session-binary · App UI driven through the session binary — No test in the apps launches a window; the UI is driven through the escher-session binary.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/test-data-in-code · Test data stated in code — Test data is stated in code: inline HTML, literals and byte arrays, directly injected images, fixed viewports, embedded fonts and pages generated in the test. · except: one external benchmark page supplied through PAINT_TREE_BENCH_HTML

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/hand-written-stand-ins · Hand-written stand-ins, no mocking library — Test doubles are hand-written stand-ins; no mocking library is used in CI, the build configuration or the apps.
decision/network-stand-ins-counted · Network stand-ins keep their loads counted — Network stand-ins do not override is_noop, so the harness counts their loads.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/real-provider-inside-stand-in · Real network provider inside a hand-written stand-in — A blitz-tests check may read `file:` loads through blitz-net's real provider inside a hand-written stand-in, blocking on a current-thread tokio runtime the stand-in owns, with blitz-net and tokio taken as blitz-tests dev-dependencies for it. · guard: a_remote_page_reads_no_local_file_and_a_file_page_reads_them_all

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/local-baseline · Local baseline for CI comparison — The local baseline is the dev host, the dev profile with line-tables-only debug info, workspace default features, every cargo command locked and no opt-level rewrite; cold is the first run after a clean, warm an immediate re-run. · why: It is the reference the fork's CI is compared against.
decision/local-pre-push-gate · Local pre-push gate — The fast leg runs fmt, clippy, test, ci-scripts and embolden in that order, stopping at the first red, as the local pre-push gate.
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/clippy-legs · Clippy over the workspace and upstream blitz-dom — The clippy leg runs over the workspace, then, only when green, over upstream's blitz-dom no-default-features line with warnings denied.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/mutation-baseline-read-per-unit · Mutation baseline read per unit — A mutation run skips cargo-mutants' own baseline and first reads each unit's test command green on the unmutated tree. · why: cargo-mutants' baseline tests the mutated package alone.
decision/mutation-testing-dev-host-only · Mutation testing on the dev host — Mutation testing is cargo-mutants run with nextest under a fail-fast mutants profile, on the dev host alone and never in CI.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/mutation-baseline-skipped · Mutation baseline skipped, read green by hand — Mutation runs skip cargo-mutants' baseline; each unit's mutant test command is first read green on the unmutated tree. · why: cargo-mutants' baseline tests the mutated package alone yet passes every test argument, so a test set naming other packages cannot be its baseline and would read every mutant caught.
decision/mutation-runs-on-dev-host-only · Mutation runs on the dev host only — Mutation runs are made on the dev host alone, never in CI. · why: cargo-mutants and cargo-nextest are host tools no lockfile resolves.
decision/mutation-runs-under-nextest-profile · Mutation runs with cargo-mutants under nextest — A mutation run is cargo-mutants with nextest as its test tool under a dedicated mutants profile that stops at the first failure and runs the direct tests first, the default profile untouched. · why: Under plain cargo test, hung integration binaries run before the library tests and make mutants read timeout.

# test-plan · its absences, names and overviews, whole

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
> NOT YET MEASURED — the reading gathered no tier justification, critical-path list or coverage triggers for this summary

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
> NOT YET MEASURED — windowed or browser-driven end-to-end runs lay outside every slice read

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
absence/windowed-end-to-end-tests — An automated test that launches a window

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
absence/mocking-library — A mocking library

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
> NOT YET MEASURED — a performance budget: none is set

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## 12-test-decisions-log · ## 12. Test Decisions Log
> NO RECORDED INTENT

## session-lifecycle · key of ## 3. Test Harness Contract
overview/session-lifecycle — The session checks drive one headless app instance held by one process, and a lifecycle another process drives, with the calls it sends, over a Unix-domain socket. The session is not part of the five-command agent-run contract, which starts and binds nothing.
