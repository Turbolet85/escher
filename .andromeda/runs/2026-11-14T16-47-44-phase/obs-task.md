@task_prose part=chunk
The reference browser app (the blitz binary) closes the two app-side paths by which a remote page still makes the browser read a local file.
The favicon path: the favicon candidate a non-file: document names under file: is refused before any probe or the shared favicon cache is reached, so no file is read; a file: document's favicon still loads.
The navigation path: a link click or form submission a non-file: document starts toward a file: URL loads nothing; one a file: document starts still loads, and the user's own top-level loads are unchanged.
Both gates reuse the engine's existing document_may_load rule as it stands; the refusals are silent, adding no error path, log line or span.
The capability statement drops its two clauses still marked provisional where the founder has ruled (fixture-state absence; exploration on no carrier and following no navigation).
@task_file n=1 path=apps/browser/src/favicon.rs mark=edits
what: the browser app's favicon component, choosing a page's favicon candidate and probing it — edits: favicon_candidate, its existing unit test module
@task_file n=2 path=apps/browser/src/history.rs mark=edits
what: the browser app's navigation provider over the tab history store — edits: BrowserNavProvider (its construction and navigate_to), a new test module at the end of the file
@task_file n=3 path=apps/browser/src/document_loader.rs mark=edits
what: the browser app's document loader, which builds each document's config and calls the favicon candidate choice — edits: make_doc_config, the one construction site of BrowserNavProvider, passing it the document's URL
@task_file n=4 path=docs/capabilities.md mark=edits
what: escher's capability statement, updated by hand — edits: the provisional clause at the fixture-state absence, the provisional clause on exploration on no carrier and following no navigation
@task_item n=1 part=needs
The favicon gate: favicon_candidate answers no candidate where document_may_load(Some(base), candidate) refuses it, so the probe is never started and the shared favicon cache (keyed by candidate, across tabs) never serves a file: icon a file: page cached to a remote page naming the same URL; a file: document's favicon still loads.
@task_item n=2 part=needs
The navigation gate: BrowserNavProvider::navigate_to refuses, loading nothing, a navigation document_may_load(document_url, target) refuses; a document with no URL (the error page) counts as no file: document; a file: document's navigation to a file: URL still loads.
@task_item n=3 part=needs
The provider holds the navigating document's own URL, taken where make_doc_config builds it once per document from that document's base URL — not the tab's current history entry, which would let a remote page still displayed while a typed file: page loads navigate to a file: URL.
@task_item n=4 part=needs
The rule that no file: load is made from a non-file: document now covers the app's favicon probe and its link and form navigations, reached through the engine's public document_may_load and its FileForbidden refusal used as they stand — neither widened nor narrowed, no engine crate edited.
@task_item n=5 part=needs
The user's own top-level loads keep loading as before: the URL bar, history, home, the new-tab and history pages and the initial URL are not gated.
@task_item n=6 part=needs
A refused favicon or navigation is silent, as Chromium and Firefox refuse local resources: a filter and an early return, with no tracing call, no log line, no span, no error path, cause, exit, log field, env read, listener or dependency added.
@task_item n=7 part=needs
The capability statement is updated by hand: the two provisional clauses are dropped and nothing else changes in it; the browser's two closed paths are not added to it, since the browser app is upstream's and its loading-pages limit speaks of the engine, whose rule does not move.
@task_item n=8 part=needs
Each new check carries a control: the mutation or hand edit that turns it red, and the reading that it did.
@task_item n=9 part=needs
The favicon gate is proved by unit tests on the pure favicon_candidate in its existing test module; the navigation gate by calling navigate_to on a history store built inside a VirtualDom, as the tab's lens test already builds one, in a new test module; no disk read, no temp file, no TMPDIR read, so both run alike in the workspace test leg on the three CI operating systems.
@task_standing n=1
convention: every new test is silent — it prints nothing and needs no telemetry sink
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
The driver's command span and tool_call span, their fields and endings, and the refusal cause names: the driver is untouched and no span is added.
@task_item n=2 part=not_needed
escher-session's stdout, stderr, exit codes, error object, coverage role, MCP role, commands and host wire: no session code is touched.
@task_item n=3 part=not_needed
The agent-run and cold-agent harness events, verdicts, counted verbs, kept checks and transcripts: no harness is touched.
@task_item n=4 part=not_needed
The script error surface, window error event, script error source labels, script diagnostics drain and the WPT report: no script or engine error path changes.
@task_item n=5 part=not_needed
Metric emission, metrics exporters, WPT score publishing and any telemetry backend: none is added.
@task_item n=6 part=not_needed
The recording file, recording and session state directories, session lifecycle and record refusals: the session's recording is untouched.
@task_item n=7 part=not_needed
Snapshot, graph and actions answers, select and page-event rows, the state registry and the shot image: the driver's page reading is untouched.
@task_item n=8 part=not_needed
CI artifacts, the package-install diagnostic line and fork CI uploads: no CI job or artifact is added; the new tests run in the existing workspace test leg.
