@task_prose part=chunk
Proves the browser app's page-URL hand-off: new tests in the browser app fail when the document config every browser document loads with is replaced by a default, loses its network provider, or loses its navigation provider.
The same tests read, through the config's own navigation provider, that a remote page's navigation to a file: URL loads nothing while a file: page's navigation to a file: URL loads.
Writes a justification beside the two bare clippy expect_used allows in the browser app (the favicon test module, the tab's web view); no behaviour of the browser changes.
Re-measures the three mutation survivors of the Epoch 4 code audit in the document config builder by a cargo-mutants run on the dev host once it is free; the mutation tier is no CI leg.
@task_file n=1 path=apps/browser/src/document_loader.rs mark=edits
what: the browser's document loader, which builds the document config every browser document loads with — its network provider and its navigation provider, the provider whose document URL is the page URL the file: navigation gate reads — edits: make_doc_config's test module (new tests of the hand-off)
@task_file n=2 path=apps/browser/src/favicon.rs mark=edits
what: the browser's favicon probing for a page — edits: the test module's expect_used allow (a written justification beside it)
@task_file n=3 path=apps/browser/src/tab.rs mark=edits
what: the browser tab and its web view component — edits: the TabWebView expect_used allow (a written justification beside it)
@task_item n=1 part=needs
The page-URL hand-off: the document config the browser builds for each document carries a navigation provider built for that document from the history and the page's base URL, and that provider's document URL is the page URL the file: navigation gate reads; the new tests fail when the config is a default or the navigation provider is dropped.
@task_item n=2 part=needs
The file: navigation gate — file URLs load only for file documents: the new tests read, through the config's own navigation provider, that a navigation to a file: URL from a remote page loads nothing and from a file: page loads; the gate's rule is asserted, not changed.
@task_item n=3 part=needs
The network provider the document config carries: the browser's provider for a document's outbound and sub-resource requests; a new test fails when the config carries none (the default) or drops it.
@task_item n=4 part=needs
The restructure-free invariant: the hand-off and the gate stand as the remote-pages-local-files chunk built them — make_doc_config still builds the config with its network and navigation providers, the gate still refuses a remote page's file: navigation and lets a file: page's through; the chunk adds tests and justification text only, and the gate's existing browser tests and the new ones hold it.
@task_item n=5 part=needs
A test of the config builder runs inside a Dioxus runtime that provides a shell provider context (the builder consumes it), the way the browser history's tests run navigation inside a virtual DOM; the harness shape is left to P3.
@task_item n=6 part=needs
The new tests run in the workspace test leg and so are CI-gated; P3 closes this against the CI leg script and the workspace members.
@task_item n=7 part=needs
The mutation measure: the three survivors (the config body replaced by a default, its network provider deleted, its navigation provider deleted) are read as killed by a cargo-mutants run on the document loader on the dev host, after the host's load window; no cargo build, test or clippy before about 2026-11-15 16:35Z (a planned future time, as the operator gave it).
@task_item n=8 part=needs
Every expect_used allow in the browser app carries its reason: the favicon test module's and the tab web view's allows get a written justification (a comment line or the allow's reason), so the justified-expect rule's exception for the favicon test module names nothing and the wrap drops it; the new tests' own expect calls stand under the same rule.
@task_item n=9 part=needs
The boundary of the chunk: browser app only — no engine crate, no driver, and no carrier, verb, field, log site or span is added or widened.
@task_standing n=1
convention: each new test of a file: URL navigation reads only local paths the test owns or names fixed, and asserts the refusal as a load that does not happen, never by reading a file's contents
home: input-validation
@task_item n=1 part=not_needed
The driver session socket, its request grammar, replies, bounds, idle expiry and owner-only access: the chunk touches no driver or session.
@task_item n=2 part=not_needed
The MCP server, its transport, requests, results, tool-call span and protocol errors: no MCP surface is touched.
@task_item n=3 part=not_needed
The stable element id, snapshot text, diff node keys, graph and actions answers and their carriers: the chunk adds no carrier or field.
@task_item n=4 part=not_needed
Select list, range input, password masking, accessibility adapter and control-state rows of the engine: no engine crate is touched.
@task_item n=5 part=not_needed
Recorded and kept check files, the recording directory, cold-agent runs and the coverage answer line: none of them is reached.
@task_item n=6 part=not_needed
Refusal causes, refusal JSON, session error JSON and escher-session exit codes and output contracts: no driver output changes.
@task_item n=7 part=not_needed
CI publishing, release signing, publish secrets and the WPT runner and post-results workflow: no workflow or release job is changed.
@task_item n=8 part=not_needed
CSS, HTML parsing, layout, CSSOM and serialization rows of the engine: the chunk changes no engine behaviour.
@task_item n=9 part=not_needed
Authentication and authorization rows: the browser app holds none and the chunk adds none.
