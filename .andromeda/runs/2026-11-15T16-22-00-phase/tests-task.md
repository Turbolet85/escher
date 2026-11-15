@task_prose part=chunk
Browser corrective, in the browser app only: it adds tests in the browser app that prove the document loader's page-URL hand-off, the config every browser document loads with, which carries its network provider and a navigation provider whose document URL is the page URL the file: navigation gate reads.
The new tests fail when that config is replaced by a default, when its network provider is dropped, or when its navigation provider is dropped, and read through the config's own navigation provider that a remote page's navigation to a file: URL loads nothing while a file: page's does.
That kills the three mutation survivors the Epoch 4 code audit measured in the hand-off; a cargo-mutants run on the document loader, on the dev host once it is free, re-measures the kill.
It writes a justification beside the two browser expect_used allows that stand bare (the favicon test module and the tab's webview component), so every browser expect_used allow carries its reason.
No behaviour of the browser changes: it adds tests and justification text only.
@task_file n=1 path=apps/browser/src/document_loader.rs mark=edits
what: the browser's document loader — the page-URL hand-off that builds the document config (network provider, navigation provider for the document's page URL), and the unit its mutation run reads — edits: make_doc_config's tests
@task_file n=2 path=apps/browser/src/favicon.rs mark=edits
what: the browser's favicon fetch component and its inline test module — edits: the test module's expect_used allow, given its written reason
@task_file n=3 path=apps/browser/src/tab.rs mark=edits
what: the browser tab component and its webview — edits: the TabWebView expect_used allow, given its written reason
@task_item n=1 part=needs
The hand-off proven by the browser's own tests: a test fails when the document config is replaced by a default, one when its network provider is dropped, one when its navigation provider is dropped — the three mutation survivors killed.
@task_item n=2 part=needs
The file: navigation gate read through the hand-off: through the config's own navigation provider, a remote page's navigation to a file: URL loads nothing, and a file: page's navigation to a file: URL loads — file: URLs loadable only from a file: document.
@task_item n=3 part=needs
The mutation re-measure: a cargo-mutants run on the document loader, read per unit, on the dev host only (the mutation tier is no CI leg), under its nextest profile; the three survivors read killed. Run only once the host is free of the other run.
@task_item n=4 part=needs
Each new test controlled red-before-green: it is shown to fail against the mutation it targets (the config defaulted, a provider dropped) before it is read green.
@task_item n=5 part=needs
The hand-off reads a shell-provider context from the Dioxus runtime, so its test runs inside a Dioxus virtual DOM that provides that context, as the gate's existing tests drive navigation inside one; the stand-in context is hand-written, no mocking library.
@task_item n=6 part=needs
The network provider the test hands in or reads is a hand-written stand-in that keeps its loads counted, and the test reaches no network: the remote page URL is a value stated in code, no request leaves the host.
@task_item n=7 part=needs
Test data stated in code: page URLs and file: URLs the tests use are literals in the test, nothing seeded on disk beyond what the test itself states.
@task_item n=8 part=needs
The new tests sit in an inline test module in their source file and run on Rust's built-in test harness, so they run in the workspace test leg and are CI-gated.
@task_item n=9 part=needs
Justified expect/unwrap: every expect_used allow in the browser app stands beside a written justification (a comment line or the allow's reason); the favicon test module's and the tab webview's allows gain theirs, so the rule's except over the favicon test module then names no exception. Any expect the new tests use falls under an allow that carries its reason.
@task_item n=10 part=needs
The new test code passes the workspace clippy leg, the expect_used lint included, with no new bare allow.
@task_item n=11 part=needs
No behaviour change, the invariant: the hand-off (the document config with its network and navigation providers) and the file: navigation gate stand as the remote-pages-local-files chunk built them; its existing gate tests still hold them, and the justification edits change no code path.
@task_item n=12 part=needs
The new tests document the survivors they kill: each names or states the mutation it guards against (config defaulted, network provider dropped, navigation provider dropped), as a regression test documents its bug.
@task_item n=13 part=needs
No sleep or wall-clock read in the new tests: any wait on the virtual DOM is driven, never timed.
@task_item n=14 part=needs
A failing assertion in the new tests prints no argument value beyond fixed text: no page URL or path echoed beyond what the test itself states.
@task_standing n=1
convention: unit tests sit in an inline test module in their source file
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: each new test is controlled red-before-green
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=3
convention: test data stated in code, nothing seeded
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=4
convention: hand-written stand-ins, no mocking library; network stand-ins keep their loads counted
home: 8-mocking-stubbing-discipline · mocking-stubbing-discipline
@task_standing n=5
convention: regression tests document and are named for their bug
home: 2-test-strategy · test-strategy
@task_standing n=6
convention: test failures print no argument value
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=7
convention: Rust tests on the built-in harness
home: 3-test-harness-contract
@task_item n=1 part=not_needed
The driver and session surfaces — escher-session command line, MCP role, session wire, session host, session recordings, refusal causes, driver verbs, command and tool_call spans: the chunk stays in the browser app and adds no driver, carrier, verb, log site or span.
@task_item n=2 part=not_needed
The WPT conformance runner, its command line, outputs, status set and pinned commit: the chunk touches no engine crate and runs no conformance suite.
@task_item n=3 part=not_needed
The cold-agent pipe and agent-run — their verbs, events, exit codes, verdict and state: the chunk adds no harness run and no agent check.
@task_item n=4 part=not_needed
The engine harness's lint defects, text contrast, screenshot and pixel tests, layout modes and paint expectations: no engine crate, no rendering change.
@task_item n=5 part=not_needed
The seven_guis headless stand — its boot, timer, load page and stand-repeat rounds: the chunk adds no stand check.
@task_item n=6 part=not_needed
Recorded checks, check replay and handler coverage: the chunk records and replays no check.
@task_item n=7 part=not_needed
The virtual clock, advance and timer-tick rows: the hand-off and gate hold no timer and the chunk drives no time.
@task_item n=8 part=not_needed
The CI helper scripts' own contracts — the apt-install script's exit codes, the coverage floor script's exit codes, CI artifacts and log names, CI action pinning, secrets and permissions: the chunk adds no CI job or script.
@task_item n=9 part=not_needed
The blitz-tests manifest, blitz-dom fixtures and the upstream buffered-render tests: upstream engine crates the chunk does not reach.
