@task_prose part=chunk
Adds tests inside the browser app proving the document loader's page-URL hand-off: the DocumentConfig every browser document loads with carries its net provider and its navigation provider, the one built for the document from its history and base URL.
The tests fail when the config is replaced by a default, when its net provider is dropped or when its navigation provider is dropped; read through the config's own navigation provider, a remote page's navigation to a file: URL loads nothing and a file: page's does.
Writes a justification beside the two bare clippy expect_used allows left in the browser app, the favicon test module and the tab's TabWebView, so the justified expect and unwrap rule's exception for the favicon test module can be dropped at the wrap's reconcile.
Changes no browser behaviour: the hand-off and the file: navigation gate stay as built; the three mutation survivors in the config builder are re-measured by a host cargo-mutants run on the document loader once the host is free.
@task_file n=1 path=apps/browser/src/document_loader.rs mark=edits
what: the browser's document loader, which builds the DocumentConfig a browser document loads with, carrying the embedder's net provider and the shell and navigation provider for the page — edits: tests of make_doc_config added in the browser app
@task_file n=2 path=apps/browser/src/favicon.rs mark=edits
what: the browser's favicon fetching and cache — edits: the test module's clippy expect_used allow
@task_file n=3 path=apps/browser/src/tab.rs mark=edits
what: the browser tab and its TabWebView component — edits: the TabWebView clippy expect_used allow
@task_item n=1 part=needs
The DocumentConfig hand-off: the document configuration struct a browser document is built from must carry the embedder's net provider and the navigation provider the browser builds for that document; the new tests must fail when the config is a default, when the net provider is dropped and when the navigation provider is dropped.
@task_item n=2 part=needs
The navigation provider's document URL is the page URL the file: navigation gate reads: file URLs load only for file documents, so a remote page's navigation to a file: URL loads nothing while a file: page's navigation to one does; the tests read this through the config's own navigation provider, via the document_may_load answer the gate uses.
@task_item n=3 part=needs
Embedder services are injected with no-op defaults: a default DocumentConfig carries the no-op net provider and the default navigation provider, so a test tells the browser's config from a default by what its providers report (the net provider's is_noop, the navigation provider's document URL).
@task_item n=4 part=needs
The config builder consumes the ShellProvider from the Dioxus root contexts, so a test of it runs inside a Dioxus runtime that provides a ShellProvider context, as the browser's history tests run navigation inside a VirtualDom; the harness shape is closed in P3.
@task_item n=5 part=needs
expect and unwrap need a justification: every clippy expect_used allow in the browser app carries a written reason beside it, a comment line or the allow's reason field, and the new tests' own expect or unwrap calls carry one too; the rule's exception naming the favicon test module then names nothing and is dropped at the wrap's reconcile.
@task_item n=6 part=needs
Clippy allows are kept local and lints allowed locally: the justifications stand on the existing scoped allows, the favicon test module's and TabWebView's, and no allow is widened to a crate or file level.
@task_item n=7 part=needs
The mutation tier is host-run, no CI leg: the kill of the three survivors (the config replaced by a default, the net provider deleted, the navigation provider deleted) is read by a cargo-mutants run on the document loader under the nextest mutants profile, once the host is free; the browser app's mutation score is re-measured by it.
@task_item n=8 part=needs
The new browser tests run in the workspace test leg of the CI leg runner and are CI-gated there; P3 closes it against the leg script and the workspace members.
@task_item n=9 part=needs
The chunk adds tests and justification text only, in the browser app alone: no new public item, carrier, verb, field, log site or span, and the shell and navigation providers keep their present shape.
@task_standing n=1
convention: each new test carries a descriptive name saying the behaviour it proves
home: conventions
@task_standing n=2
convention: each new unit test sits in the test module at the end of the source file it tests, placed as the unit-test placement rule says
home: conventions
@task_standing n=3
convention: each new test asserts its fixture's precondition first, before the behaviour it proves
home: conventions
@task_standing n=4
convention: each new test module and test passes the formatting and clippy gate and the rustdoc warnings gate, and carries the module docs a new module owes
home: conventions
@task_item n=1 part=not_needed
The engine's style, layout, paint, scroll, hit-testing, select list and accessibility rows: the chunk touches no engine crate.
@task_item n=2 part=not_needed
The driver, session, MCP role, session socket, snapshot, diff, lint and actions contracts and the refusal and verb tables: the chunk adds no carrier, verb or field.
@task_item n=3 part=not_needed
The telemetry bootstrap, its scrubbing and line contracts and the driver's spans: the chunk adds no log site or span.
@task_item n=4 part=not_needed
The WPT runner, its outputs, result codes and diff-to-PR script: the browser app's tests do not reach them.
@task_item n=5 part=not_needed
The occupied resources rows over the session host, recording directories, kept checks and the cold-agent runs: the chunk holds no listener, directory or run.
@task_item n=6 part=not_needed
The names register of crates, binaries and other names: the chunk adds no crate, binary or name.
@task_item n=7 part=not_needed
The upstream sync, cherry-pick and upstream-take rows: the chunk takes nothing from another tree.
@task_item n=8 part=not_needed
The browser's about pages, toolbar View Source and URL suggestions: the chunk changes none of them.
@task_item n=9 part=not_needed
The project intent's capability statement and root README rows: the chunk adds no capability.
@task_item n=10 part=not_needed
The stack and technologies choices, the Nix flake and the agent-run, cold-agent and package-install script contracts: no dependency, tool or script changes.
