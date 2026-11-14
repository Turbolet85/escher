@task_prose part=chunk
The reference browser app (apps/browser, the blitz binary) closes the two paths by which a remote page still makes the browser read a local file.
The favicon path: a favicon candidate a non-file: document names under file: is never chosen, so it is never probed nor served from the shared favicon cache; a file: document's favicon still loads.
The navigation path: a link click or form submission a non-file: document starts to a file: URL loads nothing, silently; one a file: document starts still loads, and the user's own top-level loads are unchanged.
Both gates read the engine's existing document_may_load rule as it stands; no engine crate is edited.
It also drops two stale provisional clauses from the capability statement, where the founder has ruled.
@task_file n=1 path=apps/browser/src/favicon.rs mark=edits
what: the browser app's favicon component — choosing a page's favicon candidate and probing it, with its inline unit test module — edits: the function favicon_candidate, its existing test module
@task_file n=2 path=apps/browser/src/history.rs mark=edits
what: the browser app's navigation provider over the tab's history store, through which every top-document link click and form submission navigates — edits: BrowserNavProvider and its navigate_to, a new test module at the end of the file
@task_file n=3 path=apps/browser/src/document_loader.rs mark=edits
what: the browser app's document loader, which builds each document's config and its navigation provider once per document, holding that document's base URL — edits: the function make_doc_config, the one construction site of BrowserNavProvider
@task_file n=4 path=docs/capabilities.md mark=edits
what: escher's capability statement, the document of what escher can and cannot do, pinned to the driver schema — edits: the provisional clause on the fixture-state absence and the provisional clause on exploration on no carrier and following no navigation
@task_item n=1 part=needs
The favicon gate: favicon_candidate answers no candidate when document_may_load with the page's base URL refuses that candidate, so a non-file: page naming a file: icon (by link href or by /favicon.ico joined to a file: base) starts no probe; a file: page's file: favicon is still chosen and loads.
@task_item n=2 part=needs
The shared favicon cache, keyed by candidate and shared across tabs: because the gate sits before the cache is reached, a file: icon a file: page cached is never served to a remote page naming the same URL.
@task_item n=3 part=needs
The navigation gate: BrowserNavProvider takes its document's URL when make_doc_config builds it, and navigate_to returns early, loading nothing, for a navigation document_may_load(document URL, target) refuses; a document with no URL (the error page) counts as no file: document; a file: document's navigation to a file: URL still loads.
@task_item n=4 part=needs
The navigating document is the provider's own document, never the tab's current history entry: a remote page still displayed while a file: page the user typed loads must not reach a file: URL. The user's own top-level loads (URL bar, history, home, the new-tab and history pages, the initial URL) do not pass through the gate.
@task_item n=5 part=needs
Indirect, through the engine's net traits: the engine rule that file: URLs are loadable only from a file: document (document_may_load, FileForbidden) is used as it stands, neither widened nor narrowed; the app reads the public function and adds no rule of its own.
@task_item n=6 part=needs
A refusal is silent: no carrier, verb, argument, refusal cause, session error, exit code, log field, span, tracing call, env read, listener or dependency is added; a refused favicon or navigation is a filter and an early return, reaching no error path and no log line.
@task_item n=7 part=needs
Every new check carries a control: the mutation or hand edit that turns it red (dropping the favicon filter, dropping the navigate_to early return) and the reading that it did.
@task_item n=8 part=needs
The favicon gate is proved as a pure unit test of favicon_candidate inside favicon.rs's existing inline test module: remote base with a file: href and a file: favicon fallback refused, file: base with a file: candidate kept, remote candidates kept.
@task_item n=9 part=needs
The navigation gate is proved in a new inline test module at the end of history.rs, calling BrowserNavProvider::navigate_to on a History store built inside a VirtualDom the way the tab lens test builds one, and reading that the history gained no entry for a refused file: target and gained one for an allowed target.
@task_item n=10 part=needs
The new tests read no disk and need no real network provider, temp file or TMPDIR: they run in the workspace test leg and read alike on the three CI OSes.
@task_item n=11 part=needs
The capability statement edit only drops the two provisional clauses; the statement stays pinned to the driver schema, its claim section names and counts unchanged, and the browser's two closed paths are not added to it.
@task_item n=12 part=needs
No capability is claimed by this chunk: no 0.3.0 capability names this entry.
@task_standing n=1
convention: every new test is controlled red-before-green — the mutation or hand edit that turns it red, and the reading that it did
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=2
convention: unit tests sit in an inline test module of their own source file, not in a new test target
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: a regression test documents and is named for the bug it pins
home: 2-test-strategy
@task_standing n=4
convention: test data is stated in code — fixture URLs and documents written in the test, nothing seeded or read from disk
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=5
convention: test failures print no argument value, and a table-driven test asserts its row count
home: unit-test-strategy
@task_standing n=6
convention: stand-ins are hand-written, with no mocking library
home: 8-mocking-stubbing-discipline · mocking-stubbing-discipline
@task_standing n=7
convention: no sleep or wall-clock read in a check
home: integration-test-strategy
@task_standing n=8
convention: workspace tests run in the fast CI leg across the native platform matrix
home: 1-test-scope-summary
@task_item n=1 part=not_needed
The driver session, its session host, socket wire, state directory, recording and session errors: the driver is untouched and no session verb, error or exit is added.
@task_item n=2 part=not_needed
The escher-session command line and MCP role contracts (commands, exit codes, stderr, tools/list, snapshot, refusals with remedy): no command, exit or output line changes.
@task_item n=3 part=not_needed
The seven_guis headless stand, its boot functions, stand log and stand-check rules (both layout modes, bundled font, three-round repeat): the stand and the seven_guis app are untouched.
@task_item n=4 part=not_needed
The agent-run and cold-agent pipe contracts, events, verdicts and exit codes: neither pipe is touched.
@task_item n=5 part=not_needed
WPT conformance runner, its CLI, outputs, status set and suites: no engine conformance behaviour moves.
@task_item n=6 part=not_needed
Harness construction, core, settle, input, inspection, contrast and screenshot contracts: the new tests drive no escher harness and take no screenshot.
@task_item n=7 part=not_needed
Lint defects, driver refusal causes, snapshot nodes and handler coverage limits: no lint, cause or coverage reading is added or changed.
@task_item n=8 part=not_needed
Recorded and committed checks, their freshness and replay: no check is recorded.
@task_item n=9 part=not_needed
Pixel and font tests, buffered render, select painting and scrollbar fade: nothing painted changes.
@task_item n=10 part=not_needed
CI workflow plumbing — pinned actions, package installs, cache, secrets, artifacts, coverage-floor and apt-install script exit codes: no CI job or script is added or edited.
@task_item n=11 part=not_needed
Virtual clock and advance rules: neither gate reads or moves time.
