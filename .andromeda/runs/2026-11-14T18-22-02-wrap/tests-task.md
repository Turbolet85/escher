@task_prose part=session
The browser app (upstream's reference app) now refuses a file: URL for a non-file: document in two places: its favicon candidate names no file: favicon for a remote page, and its navigation provider silently drops a remote document's navigation to a file: URL.
A file: page keeps its file: favicon and its file: navigations; a remote page keeps its own default favicon and its https navigations; the user's own loads (URL bar, history, home, about pages, first tab) are untouched.
Six new unit tests hold it, in inline test modules: three favicon tests and a new history test module with three navigation tests; the browser crate's unit tests move 29 to 35.
Five mutation controls hold (gate removed, gate refusing all, URL dropped); the one wiring line that hands a document's URL to the navigation provider is reached by no test, as it runs only inside a launched Dioxus runtime.
The capability statement loses its two clauses marked provisional pending the founder's choice; no dependency, log site, env read or CI workflow changed; CI green on 17 of 17 checks including macOS and Windows.
@task_file n=1 path=apps/browser/src/favicon.rs
what: the browser app's favicon candidate (which URL a page's favicon is probed from, beside the shared favicon cache) and its inline unit test module
@task_file n=2 path=apps/browser/src/history.rs
what: the browser app's history store and its navigation provider, which receives link-click navigations a document starts; now with an inline unit test module of its own
@task_file n=3 path=apps/browser/src/document_loader.rs
what: the browser app's document loader, which builds each loaded document's config (its base URL, its navigation provider) and the error page's
@task_file n=4 path=docs/capabilities.md
what: the capability statement, pinned to the driver schema, whose claims name their checks
@task_item n=1 part=touches
decision/file-urls-only-for-file-documents: the rule now reaches the browser app's favicon candidate and its navigation provider; its guard list gains the six browser tests remote_page_favicon_candidate_names_no_local_file, file_page_favicon_candidate_keeps_local_file, remote_page_default_favicon_candidate_unchanged, remote_page_navigation_to_file_url_loads_nothing, file_page_navigation_to_file_url_loads, remote_page_navigation_to_https_url_loads.
@task_item n=2 part=touches
The capability statement rows (capabilities-doc-pinned-to-schema, capability-statement-pinned, capability-statement-from-schema, capability-check-names-held-to-schema, capability-claims-name-their-checks): the statement was edited, its two provisional clauses pending the founder's choice removed; its schema-pinned names and section counts are unchanged and its check stayed green in the fast leg.
@task_item n=3 part=touches
Inline unit test module rows (inline-unit-test-modules, inline-cfg-test-modules, unit-tests-in-source-file): a new inline test module sits in the browser app's history source file, and the favicon source file's module gains three tests.
@task_item n=4 part=touches
decision/mutation-controlled-tests: the six new tests were controlled red-before-green, five controls holding (favicon gate removed, favicon gate refusing all, navigation gate removed, document URL dropped, navigation gate refusing all).
@task_item n=5 part=touches
decision/workspace-tests-in-fast-ci-leg and decision/native-platform-ci-matrix: the browser crate's new unit tests run in the fast leg and passed on the macOS and Windows test jobs.
@task_item n=6 part=touches
Coverage floor rows (per-crate-coverage-floors, coverage-floor-per-crate): the browser crate gains gated code and tests, but the wiring line in the document loader that passes the document's URL to the navigation provider is reached by no test; no floor value moved.
@task_item n=7 part=touches
absence/windowed-end-to-end-tests: the navigation gate's wiring runs only inside a launched Dioxus runtime, so the one line left untested is exactly what a windowed test would reach; the absence still holds, now with a named gap beside it.
@task_item n=8 part=touches
Stand-in and mocking rows (hand-written-stand-ins, absence/mocking-library): the navigation tests drive the real provider inside a Dioxus virtual DOM, reporting the store's current URL out through a thread-local, with no mocking library.
@task_item n=9 part=touches
Test data rows (test-data-is-code, test-data-in-code): the new tests state their document and target URLs in code, nothing seeded.
@task_item n=10 part=touches
Regression test naming rows (regression-tests-named-for-bug, regression-tests-document-bug): the six new tests are named for the behaviour they hold (a remote page's favicon names no local file; a remote page's navigation to a file URL loads nothing).
@task_item n=11 part=touches
decision/no-sleep-or-wall-clock-in-checks and decision/assertions-print-no-argument-value: rules over the new unit tests, which assert schemes and the current URL after a virtual DOM rebuild, with no sleep or wall-clock read.
@task_item n=12 part=touches
Network stand-in rows (network-stand-ins-counted, harness-counts-fetching-providers-only): a remote page's file: favicon no longer reaches the favicon probe or the shared favicon cache, so no load is started for it.
@task_fact n=1 slug=integration-test-strategy
fact: In the browser app a non-file: document names no file: favicon and its navigation to a file: URL is dropped silently, loading and logging nothing; a file: document keeps both, and the user's own loads are not gated.
@task_fact n=2 slug=unit-test-strategy
fact: A Dioxus provider that needs a runtime is unit-tested inside a virtual DOM run, the component reporting out through a thread-local read after the rebuild rather than asserting inside the component.
@task_fact n=3 slug=integration-test-strategy
fact: The line that hands a loaded document's URL to the browser app's navigation provider is reached by no test, running only inside a launched Dioxus runtime; the unit tests control the provider's URL parse, not that call site.
@task_item n=1 part=untouched
The session lifecycle area: the session host, wire, state directory, recording, check replay and its errors are not reached; the change is in upstream's reference browser app alone.
@task_item n=2 part=untouched
Driver, escher-session CLI and MCP role contracts: verbs, refusal causes, exit codes, stdout, stderr, help, snapshot and actions are unchanged.
@task_item n=3 part=untouched
agent-run and cold-agent rows: their events, verbs, exit codes, verdicts and isolation are not touched.
@task_item n=4 part=untouched
WPT rows: the runner, its CLI, status set, pinned commit, report-only reading and reftests are unchanged.
@task_item n=5 part=untouched
CI integration rows over workflows, caches, triggers, package installs, apt-install and coverage floor scripts, clippy legs and the dependency audit: no workflow, script or dependency changed.
@task_item n=6 part=untouched
The headless harness contracts (construction, settle, input, inspection, screenshot, contrast) and the lint defects: no engine crate or harness code changed.
@task_item n=7 part=untouched
Handler coverage, UI graph, exploration and start-in-a-state rows: not reached by this session.
@task_item n=8 part=untouched
Mutation-tool and quality-gate rows over cargo-mutants, the complexity ceiling and the stand repeat budget: no threshold, profile or budget moved.
