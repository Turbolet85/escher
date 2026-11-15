@task_prose part=chunk
Adds tests inside the browser app that prove the document-config hand-off: they fail when the config every browser document loads with is replaced by a default, or when its network provider or its navigation provider is dropped.
The same tests read, through that config's own navigation provider, that a remote page's navigation to a file: URL loads nothing while a file: page's navigation does.
These kill the three mutation survivors the Epoch 4 code audit measured in that function; a cargo-mutants run on the document loader re-measures the kill once the host is free.
It also writes a justification beside two bare expect_used allows in the browser app, the favicon test module's and the tab web view's. It changes no browser behaviour.
@task_file n=1 path=apps/browser/src/document_loader.rs mark=edits
what: the browser's document loader, which builds the document config (network provider, navigation provider holding the page URL) every browser document loads with — edits: tests of make_doc_config
@task_file n=2 path=apps/browser/src/favicon.rs mark=edits
what: the browser's favicon loading and its test module — edits: the test module's expect_used allow, given a written reason
@task_file n=3 path=apps/browser/src/tab.rs mark=edits
what: the browser tab and its web view component — edits: the TabWebView expect_used allow, given a written reason
@task_item n=1 part=needs
The document config the browser builds for each document: tests must show it is the built config and not a default, and that it carries both its network provider and its navigation provider.
@task_item n=2 part=needs
The navigation provider's document URL is the page URL the file: navigation gate reads. The tests must show, through the config's own provider, that a remote page loads no file: URL and a file: page does.
@task_item n=3 part=needs
Restructure invariant: no browser behaviour changes. The hand-off and the file: navigation gate stay as the remote-pages-local-files chunk built them. The chunk adds only tests and justification text, and the browser's existing tests keep holding them.
@task_item n=4 part=needs
The config builder reads a shell provider from the Dioxus context, so its tests run inside a Dioxus runtime (a virtual DOM) that provides that context, the same way the browser's history tests run navigation.
@task_item n=5 part=needs
Every clippy expect_used allow in the browser app carries a written justification, as a comment beside it or as the allow's reason. The favicon test module's exception then lapses.
@task_item n=6 part=needs
The new tests run in the workspace test leg and are CI-gated. The mutation tier is a dev-host run and not a CI leg.
@task_item n=7 part=needs
Scope bound: the browser app only. No engine crate, driver, carrier, verb, field, log site or span changes.
@task_item n=1 part=not_needed
Accessibility tree construction, role mapping, accessible names and ARIA state or property reads: the chunk builds and reads no accessibility tree.
@task_item n=2 part=not_needed
Keyboard navigation, focus, activation, and select, range or grid key contracts: the chunk adds no input handling.
@task_item n=3 part=not_needed
Contrast, visual lint findings, select list colours, scrollbar and stand visual rules: the chunk paints nothing and measures no colour.
@task_item n=4 part=not_needed
Snapshot, snapshot text and diff, actions answer, driver refusals, screenshots, MCP results and UI graph rows: the chunk touches no driver or agent reader.
@task_item n=5 part=not_needed
Stand expectation tables, stand tab order, stand contrast assertions and the stand's three-round checks: the chunk touches no stand app.
@task_item n=6 part=not_needed
Platform adapter, AccessKit adapter wiring, accessibility feature defaults and Dioxus accessibility feature selection: the chunk changes no feature or adapter.
@task_item n=7 part=not_needed
Cognitive accessibility rules (invalid state beyond colour or class): the chunk renders no form state.
