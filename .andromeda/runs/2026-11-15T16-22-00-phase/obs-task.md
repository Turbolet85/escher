@task_prose part=chunk
Browser corrective: adds tests in the browser app that prove the document loader's config hand-off — the DocumentConfig every browser document loads with, its net provider and its navigation provider built for the page URL.
The tests fail when the config is replaced by a default, when its net provider is dropped, or when its navigation provider is dropped, killing the three mutation survivors the Epoch 4 code audit measured.
Through the config's own navigation provider they read that a remote page's navigation to a file: URL loads nothing while a file: page's navigation does.
Every browser expect_used allow gets a written justification beside it (the favicon test module and the TabWebView allow); no browser behaviour changes.
The survivors' kill is re-measured by a cargo-mutants run on the document loader once the host is free (dev host only, no CI leg).
@task_file n=1 path=apps/browser/src/document_loader.rs mark=edits
what: the browser's document loader, building the DocumentConfig each browser document loads with — its net provider and its navigation provider for the page URL — edits: make_doc_config's tests (new tests beside it; the function itself unchanged)
@task_file n=2 path=apps/browser/src/favicon.rs mark=edits
what: the browser's favicon fetch for a page — edits: the test module's expect_used allow, given its written reason
@task_file n=3 path=apps/browser/src/tab.rs mark=edits
what: the browser's tab component and its web view — edits: the TabWebView expect_used allow, given its written justification
@task_item n=1 part=needs
The file: navigation gate the new tests read: no file: URL loads from a document that is not itself a file: document, while a file: page's navigation to a file: URL loads — read through the navigation provider make_doc_config builds, whose document URL is the page URL.
@task_item n=2 part=needs
The DocumentConfig hand-off: every browser document loads with the config make_doc_config builds, carrying the net provider and the navigation provider built for the page's base URL; the new tests must fail when the config is a default, when the net provider is missing, or when the navigation provider is missing.
@task_item n=3 part=needs
No behaviour change: the hand-off and the gate stand as the remote-pages-local-files chunk built them; the change is tests and justification text only, held by the browser app's existing tests.
@task_item n=4 part=needs
The new tests run inside a Dioxus runtime that provides a ShellProvider context, as the browser's history tests run navigation inside a VirtualDom; they add no shared module the shipped app reads.
@task_item n=5 part=needs
The new tests are silent on success and read no environment-dependent log output: they install no telemetry sink and leave no subscriber behind.
@task_item n=6 part=needs
The new tests are CI-gated through the workspace test leg; the mutation re-measure stays off CI on the dev host, and nothing it writes is uploaded or committed.
@task_item n=7 part=needs
No new log site, span, field or carrier: the browser change adds no tracing call, so the new tests assert on loaded documents, never on emitted log lines.
@task_item n=8 part=needs
A test fixture page URL or file: path the new tests use stays a synthetic value; no real local file content is read or printed by a test.
@task_standing n=1
convention: each new test is a silent check — it prints nothing on success and any shared helper it adds stays test-only
home: 3-observability-harness-contract
@task_standing n=2
convention: each new test runs in a CI leg whose log is uploaded only on that leg's failure, and the tests keep no local state for upload
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
The driver's command and tool_call spans, their fields and endings, and refusal cause names: the chunk touches no driver and adds no span.
@task_item n=2 part=not_needed
Metric emission, metrics exporters and WPT score publishing: the chunk adds no counter, timing or published figure.
@task_item n=3 part=not_needed
The agent-run and cold-agent harness events, the cold-agent verdict and its counted verbs: the chunk touches neither harness.
@task_item n=4 part=not_needed
escher-session's roles, stdout answers, exit codes, error object, host wire, coverage line and session recording or lifecycle: the chunk touches no session binary.
@task_item n=5 part=not_needed
escher's telemetry sink, its log line layout, scrub set, target allowlist, resource keys and panic hook: the chunk installs no sink and adds no log site.
@task_item n=6 part=not_needed
Script error surfaces — window error events, onerror arguments, script error source labels and the script diagnostics drain — and WPT report messages: the chunk changes no engine script handling.
@task_item n=7 part=not_needed
Page-content privacy rows for the snapshot, graph, actions answers, select lists and page script events: the chunk reads no snapshot or answer and adds no engine field.
