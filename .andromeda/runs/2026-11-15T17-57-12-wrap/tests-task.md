@task_prose part=session
The session added an inline unit test module to the browser app's document loader: three tests read the config make_doc_config returns, with no change to make_doc_config or its callers.
One test proves the config carries the page URL and the network provider passed in; two prove the navigation gate through that config: a remote page's config loads nothing for a file: URL, and a file page's config loads it.
The tests use a hand-written shell stand-in (NoShell, every method defaulted) and the browser's real std network provider; no dependency, crate, schema, public API or harness surface moved.
Two comment lines put a justification above the browser's two bare clippy::expect_used allows (the favicon test module and TabWebView), so no bare expect_used allow is left in the browser app.
Measured: the browser crate's unit tests 35 to 38; make_doc_config's mutants went from 11 missed to 7 missed and 4 caught, read with cargo-mutants on the dev host under nextest.
The 7 still missed: two equivalent (viewport, media_type) and five field deletions no browser test reads (shell_provider, html_parser_provider, font_ctx, abort_signal, style_threading). The fast CI leg and full CI were green.
@task_file n=1 path=apps/browser/src/document_loader.rs
what: the browser app's document loader, which builds the document config a page loads under (make_doc_config: base URL, network provider, navigation provider with its file: URL gate) and loads the document. Edited: an inline unit test module appended.
@task_file n=2 path=apps/browser/src/favicon.rs
what: the browser app's favicon loader and its inline test module. Edited: one justification comment above the test module's clippy::expect_used allow.
@task_file n=3 path=apps/browser/src/tab.rs
what: the browser app's tab and its TabWebView. Edited: two justification comment lines above TabWebView's clippy::expect_used allow.
@task_item n=1 part=touches
decision/file-urls-only-for-file-documents: its guard cell names remote_document_cannot_load_file_urls, file_document_can_load_sibling_file_urls and local_files_for_local_documents; two new tests now guard its browser-navigation clause through the config the browser's loader returns: remote_page_config_navigation_to_file_url_loads_nothing and file_page_config_navigation_to_file_url_loads. The guard should name them.
@task_item n=2 part=touches
The unit-tests-in-their-source-file rules (decision/inline-unit-test-modules, decision/inline-cfg-test-modules, decision/unit-tests-in-source-file): the browser's document loader now holds an inline cfg(test) module, which follows them. No row lists the browser's tested source files.
@task_item n=3 part=touches
The hand-written stand-in rules (decision/hand-written-stand-ins, absence/mocking-library, decision/real-provider-inside-stand-in, decision/network-stand-ins-counted): the new tests use a hand-written shell stand-in with every method defaulted and the real std network provider, with no mocking library.
@task_item n=4 part=touches
decision/assertions-print-no-argument-value: the new assertion messages name only the reading (base_url, net_provider, navigation_provider, current url). An assert_eq failure still prints its two operands, which are fixture URL literals the test writes, the same form the browser's history tests take. No call-supplied value is involved.
@task_item n=5 part=touches
The mutation rules (decision/mutation-baseline-read-per-unit, decision/mutation-testing-dev-host-only, decision/mutation-runs-on-dev-host-only, decision/mutation-runs-under-nextest-profile, decision/mutation-baseline-skipped): cargo-mutants was run on the dev host under nextest over one unit, make_doc_config. It went from 11 missed to 7 missed and 4 caught; the 7 still missed are named in the session lines.
@task_item n=6 part=touches
decision/mutation-controlled-tests: the new tests are controlled by mutants caught on make_doc_config (body replaced by Default, base_url, net_provider and navigation_provider each deleted).
@task_item n=7 part=touches
The test-data rules (decision/test-data-in-code, decision/test-data-is-code, decision/shared-fixture-pages): the new tests' page and target URLs are literals in the test code, with no seeded or shared fixture.
@task_item n=8 part=touches
The fast CI leg and locked cargo rules (decision/workspace-tests-in-fast-ci-leg, decision/cargo-legs-locked, decision/ci-fast-slow-split): the browser crate's three new tests run in the fast leg under --locked. No leg, job or threshold moved.
@task_item n=9 part=touches
decision/assertions-print-no-argument-value and decision/session-error-messages-fixed are not broken. The session added no log, print or env read in the browser app (silence probe 0), so no rule on log or telemetry content is touched by the code.
@task_fact n=1 slug=unit-test-strategy
fact: The browser's document config hand-off (make_doc_config) is unit-tested in its source file: the config carries the page URL and the network provider passed in, and its navigation provider loads nothing for a file: URL from a remote page and loads it from a file: page. The tests use a hand-written shell stand-in with every method defaulted and the real std network provider.
@task_fact n=2 slug=quality-gates-coverage-targets
fact: The mutation reading over make_doc_config leaves 7 missed: two equivalent (viewport and media_type, where None equals the default) and five field deletions no browser test reads (shell_provider, html_parser_provider, font_ctx, abort_signal, style_threading).
@task_item n=1 part=untouched
The session host, wire, lifecycle, errors and state-directory contracts, and the escher-session command line, CLI, MCP role and exit codes: no driver or session code moved.
@task_item n=2 part=untouched
The harness contract (construction, core, input, inspection, screenshot, settle, contrast) and the virtual clock and advance rules: no harness code moved.
@task_item n=3 part=untouched
The agent-run and cold-agent contracts, events, verdicts and exit codes: no harness script or agent-run surface moved.
@task_item n=4 part=untouched
CI integration rows (workflow triggers, cache, pinned actions, apt-install scripts, artifacts, clippy legs, MSRV, dependency audit) and the per-crate coverage floors and coverage exit codes: no workflow, script, floor or threshold moved.
@task_item n=5 part=untouched
WPT runner, status set, expectations and pinned-commit rows; the seven_guis stand, its boot, timer and repeat budget; and the committed recorded checks and their recording.
@task_item n=6 part=untouched
absence/windowed-end-to-end-tests and the e2e strategy rows: the new tests are unit tests that open no window.
@task_item n=7 part=untouched
blitz-dom, blitz-tests and the Dioxus bridge rows (fixtures, one target per file, buffered render, select painting, scrollbar fade): no engine crate moved.
