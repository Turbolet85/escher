# test-plan · the rows that bind 2026-11-14-remote-pages-local-files — picked 20 of 605 (5 by every draw) · rule any · pin 984

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/capability-statement-pinned · Capability statement pinned to the driver schema — The capability statement names every driver verb and refusal cause as a code span and states the schema's verb, cause, session-error, node-key and exit-status counts in words; the README states the count of driver commands an MCP client gets as tools. · guard: test_capability_statement.py
decision/native-platform-ci-matrix · Native platform CI matrix — CI tests on windows and macos and only builds on ios and android; linux is tested by the fast test leg, not the matrix.
decision/test-tier · Test tier — The project's test tier is 0.
decision/workspace-tests-in-fast-ci-leg · Workspace tested in the fast CI leg — Every CI run tests the whole workspace with a locked lockfile on ubuntu with default features, as the fast test leg.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/capabilities-doc-pinned-to-schema · Capability statement pinned to the driver schema — The capability statement's verb and cause names and its counts, and the README's count of MCP tools, are held equal to the driver schema's tables. · guard: CapabilityStatementTest

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/inline-unit-test-modules · Inline unit test modules — Unit tests are Rust test functions in cfg(test) modules beside the code under test. · except: modules held in files of their own through a path attribute

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/capability-check-names-held-to-schema · Claim section names and counts held to the schema — Only the claim section's names and counts are held to the schema's tables, so a check renamed or moved changes that section by hand. · guard: test_capability_statement.py
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
decision/inline-cfg-test-modules · Unit tests in inline test modules — Unit tests sit in cfg(test) modules inside their source files. · except: cfg(test) modules held in files of their own through a path attribute at their parent.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/unit-tests-in-source-file · Unit tests sit in their source file — Unit tests sit in a test-only module inside the source file; a module moved out of line sits in its own file declared test-only by path at its parent, its test paths unchanged. · except: the Dioxus bridge tests are a child of the Dioxus document module, because the event handler's two fields are private to it and it has no constructor

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again; a control that removes a bound loosens it to twice the budget rather than removing it, so the mutated run stays bounded.
decision/capability-statement-from-schema · Capability statement read from the schema — The capability statement names every verb and every refusal cause and states its counts in words read from the schema's tables, none hand-copied. · guard: CapabilityStatementTest

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/file-urls-only-for-file-documents · file: URLs loadable only from a file: document — A `file:` URL is loaded only for a `file:` document: a script's `fetch()`, module `import` or `<script src>` is refused with a `FetchError` otherwise, and an engine sub-resource load (stylesheet, `@import`, font, image, iframe) through blitz-net reads no local file for a non-`file:` document. · guard: remote_document_cannot_load_file_urls, file_document_can_load_sibling_file_urls, local_files_for_local_documents

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/mutation-baseline-read-per-unit · Mutation baseline read per unit — A mutation run skips cargo-mutants' own baseline and first reads each unit's test command green on the unmutated tree. · why: cargo-mutants' baseline tests the mutated package alone.
decision/mutation-testing-dev-host-only · Mutation testing on the dev host — Mutation testing is cargo-mutants run with nextest under a fail-fast mutants profile, on the dev host alone and never in CI.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/mutation-baseline-skipped · Mutation baseline skipped, read green by hand — Mutation runs skip cargo-mutants' baseline; each unit's mutant test command is first read green on the unmutated tree. · why: cargo-mutants' baseline tests the mutated package alone yet passes every test argument, so a test set naming other packages cannot be its baseline and would read every mutant caught.
decision/mutation-runs-on-dev-host-only · Mutation runs on the dev host only — Mutation runs are made on the dev host alone, never in CI. · why: cargo-mutants and cargo-nextest are host tools no lockfile resolves.
decision/mutation-runs-under-nextest-profile · Mutation runs with cargo-mutants under nextest — A mutation run is cargo-mutants with nextest as its test tool under a dedicated mutants profile that stops at the first failure and runs the direct tests first, the default profile untouched. · why: Under plain cargo test, hung integration binaries run before the library tests and make mutants read timeout.

## session-lifecycle · key of ## 3. Test Harness Contract
contract/session-recording/exploration-checks · session recording · exploration checks — With a recording directory an exploration keeps one check per path, each check's calls equal to its path's answered calls and replaying green; a panicked path's check holds the steps before the panic.

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
