# test-plan · the rows that bind 2026-11-20-browser-s-surviving-mutants — picked 20 of 637 (12 by every draw) · rule any · pin 2634

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/dedicated-headless-test-harness · Dedicated headless test harness — Blitz documents are tested headlessly through a dedicated, unpublished harness crate.
decision/test-tier · Test tier — The project's test tier is 0.
decision/workspace-tests-in-fast-ci-leg · Workspace tested in the fast CI leg — Every CI run tests the whole workspace with a locked lockfile on ubuntu with default features, as the fast test leg.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/inline-unit-test-modules · Inline unit test modules — Unit tests are Rust test functions in cfg(test) modules beside the code under test. · except: A source file's unit tests may sit out of line in a sibling file with the _tests suffix, declared as a cfg(test) module by path, and fixture builders may sit in a child module of that test module.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/inline-cfg-test-modules · Unit tests in inline test modules — Unit tests sit in cfg(test) modules inside their source files. · except: Unit tests may sit out of line in a sibling file with the _tests suffix, declared as a cfg(test) module by path at their parent, and fixture builders in a child module of that test module.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/bridge-tests-without-widening · Bridge tests without widening visibility — The event-bridge tests sit in a cfg(test) child module of the Dioxus document module, and no function under test gains a constructor or a pub for them. · why: DioxusEventHandler's two fields are private to that module and it has no constructor · except: the one cfg(test) shim, DioxusDocument::dispatch_to_vdom, which dispatches a chain into the vdom for a bridge test
decision/rust-tests-on-built-in-harness · Rust tests on the built-in harness — Rust tests run under the standard built-in harness: #[cfg(test)] modules of #[test] functions asserting with assert! and assert_eq!, and #[tokio::test] for async worker tests. · except: upstream's merged checks that return TestResult<()> and assert with test-that's verify_that!, under the same harness
decision/test-data-is-code · Test data is code, nothing seeded — Nothing is seeded for the agent-run contract: the fixtures are code compiled into their targets, by boot for the blitz-tests targets and by the run's own cargo call for the session selection's seven_guis target.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/unit-tests-in-source-file · Unit tests sit in their source file — Unit tests sit in a test-only module inside the source file; a module moved out of line sits in its own file declared test-only by path at its parent, its test paths unchanged. · except: the Dioxus bridge tests are a child of the Dioxus document module, because the event handler's two fields are private to it and it has no constructor

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/expected-value-pinned-as-literal · Expected value pinned as a literal — A check never derives its expected value from the constant its mutation control changes; it pins the measured value as a literal. · why: A check computing its expectation from the mutated constant reads green under the mutation.
decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again; a control that removes a bound loosens it to twice the budget rather than removing it, so the mutated run stays bounded.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/ui-driven-through-session-binary · App UI driven through the session binary — No test in the apps launches a window; the UI is driven through the escher-session binary.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/test-data-in-code · Test data stated in code — Test data is stated in code: inline HTML, literals and byte arrays, directly injected images, fixed viewports, embedded fonts and pages generated in the test. · except: one external benchmark page supplied through PAINT_TREE_BENCH_HTML

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/hand-written-stand-ins · Hand-written stand-ins, no mocking library — Test doubles are hand-written stand-ins; no mocking library is used in CI, the build configuration or the apps.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.

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
overview/session-lifecycle — The session checks drive one headless app instance held by one process, and a lifecycle another process drives, with the calls it sends, over a Unix-domain socket. The session is no verb of the five-command agent-run contract, which itself starts and binds nothing; its run session selection runs the session binary's cli_flow flows as tests, their hosts started by the flows.
