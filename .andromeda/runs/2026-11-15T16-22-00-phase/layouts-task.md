@task_prose part=chunk
Browser corrective: the browser app's own tests prove the page-URL hand-off that every browser document loads with (make_doc_config building the DocumentConfig, its net_provider and its navigation_provider).
The new tests fail when the config is replaced by a default, when its net_provider is dropped, or when its navigation_provider is dropped, and read through that provider that a remote page's navigation to a file: URL loads nothing while a file: page's does.
Every browser expect_used allow gets a written reason beside it: the favicon test module's and TabWebView's, so the justified-expect-unwrap rule's exception then names no exception.
No behaviour change: the hand-off and the file: navigation gate stay as the remote-pages-local-files chunk built them; this chunk adds only tests and justification text.
It re-measures the three mutation survivors in make_doc_config with a cargo-mutants run on the document loader once the host is free (dev host only, no CI leg).
@task_file n=1 path=apps/browser/src/document_loader.rs mark=edits
what: the browser's document loader, building the DocumentConfig every browser document loads with (its network provider and its navigation provider carrying the page URL) — edits: make_doc_config's new tests
@task_file n=2 path=apps/browser/src/favicon.rs mark=edits
what: the browser's favicon loading and its test module — edits: the test module's expect_used allow, given a written reason
@task_file n=3 path=apps/browser/src/tab.rs mark=edits
what: the browser's tab and its TabWebView component — edits: the TabWebView expect_used allow, given a written reason
@task_item n=1 part=needs
The DocumentConfig a browser document loads with, as make_doc_config builds it: the new tests fail when that config is replaced by a default.
@task_item n=2 part=needs
The config's network provider (net_provider): a test fails when it is dropped from the config.
@task_item n=3 part=needs
The config's navigation provider (BrowserNavProvider built for the document from the history and the page's base URL), whose document URL is the page URL the file: navigation gate reads: a test fails when it is dropped from the config.
@task_item n=4 part=needs
The file: navigation gate read through the config's own navigation provider: a remote page's navigation to a file: URL loads nothing, a file: page's navigation to a file: URL loads.
@task_item n=5 part=needs
Indirect, through the shell context: make_doc_config reads the ShellProvider from the Dioxus context, so its tests run inside a Dioxus runtime (a VirtualDom) that provides a ShellProvider, as the history tests run navigate_to; P3 closes the harness shape.
@task_item n=6 part=needs
The new tests are CI-gated: the browser app's tests run in the workspace test leg (P3 confirms against the CI leg script and the workspace members).
@task_item n=7 part=needs
The mutation measure the chunk is read by: a cargo-mutants run on the document loader kills the three survivors (config replaced by a default, net_provider deleted, navigation_provider deleted); the mutation tier is no CI leg and runs on the dev host only.
@task_item n=8 part=needs
Every expect_used allow in the browser app carries its written justification beside it (a comment line or the allow's reason), per the justified-expect-unwrap rule; the favicon test module's and TabWebView's are written, the other tab and history allows already stand beside a comment.
@task_item n=9 part=needs
The justified-expect-unwrap rule's exception for the favicon test module names no exception once the reason is written; the wrap's reconcile drops it.
@task_item n=10 part=needs
No behaviour change to the browser: the page-URL hand-off and the file: navigation gate stay as built; the chunk's code is tests and justification text only, inside the browser app.
@task_item n=11 part=needs
Run constraint: the host is loaded by another run until about 2026-11-15 16:35Z (a planned future time, as the operator gave it); no cargo build, test, clippy or mutants run before then.
@task_item n=12 part=needs
The browser app only: no engine crate, no driver, and no carrier, verb, field, log site or span is added or changed.
@task_item n=1 part=not_needed
The stand's stable element ids (stand-element-ids contracts): the chunk adds and renames no element id on any stand page.
@task_item n=2 part=not_needed
The stand's page-shape decisions (accent colour, keyed controls, attribute-only names, roles and ids, pinned headless viewport, lints at boot, CRUD keyboard region, flight type select, open list row bounds, into-view scroll, visible region through clipping boxes, session roles sharing the boot, held instance as fresh boot): the chunk touches no stand page.
@task_item n=3 part=not_needed
The escher-session command line and MCP server contracts (actions, command form, coverage, exit codes, graph, help, host log, lint, recording, refusals, screenshot, snapshot, served tasks, MCP tools, protocol revisions, result bound): the chunk adds no verb, field or carrier.
@task_item n=4 part=not_needed
The cold-agent and agent-run script contracts and the cold-agent decisions (pipe builds nothing, prompt names no id, drives through the driver's MCP server): no script or agent run is changed.
@task_item n=5 part=not_needed
The multi_doc and paint_bench command lines: no bench or its arguments, environment or output is touched.
@task_item n=6 part=not_needed
The MCP-server-local-over-stdio, no-caller-input-echoed and session-recording-as-replayable-check decisions: the chunk writes no server, echo or recording path.
@task_item n=7 part=not_needed
The mobile-native and web-spa surfaces: the chunk changes no mobile or web layout.
