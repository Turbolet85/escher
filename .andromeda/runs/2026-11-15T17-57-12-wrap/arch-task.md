@task_prose part=session
The session added a test-only module to the browser app's document loader. Its three tests show that the document config handed to each page load carries the page URL as base URL and the net provider that was passed in, and that its navigation provider applies the file-URL rule both ways (a remote page loads nothing from a file URL, a file page does).
It added a justification comment beside the two browser clippy::expect_used allows that had none, in the favicon test module and in the tab web view. Every browser expect_used allow (seven of them) now has its justification on the line above, and the probe counts zero bare allows.
No public API, crate, dependency, manifest, lockfile, schema, CI leg or gate changed. The function that builds the document config and its two callers in the document load are unchanged.
Measured: browser unit tests went from 35 to 38. cargo-mutants over the document-config builder went from 11 missed to 7 missed, 4 caught. Of the 7 left, 2 are equivalent (viewport and media type, where None equals the default) and 5 are field deletions no browser test reads (shell provider, HTML parser provider, font context, abort signal, style threading).
CI on the pushed commit was green with 17 of 17 checks.
@task_file n=1 path=apps/browser/src/document_loader.rs
what: the browser app's document loader, which builds each page's document config (base URL, net provider, navigation provider with the file-URL load rule) and loads the document. It edited this file by appending an inline unit-test module with a no-op shell provider stub.
@task_file n=2 path=apps/browser/src/favicon.rs
what: the browser app's favicon fetch and favicon cache, with its test module. It edited one comment line, a justification beside the test module's clippy::expect_used allow.
@task_file n=3 path=apps/browser/src/tab.rs
what: the browser app's tab and its tab web view (the tab store lens over webview props). It edited two comment lines, a justification beside the tab web view's clippy::expect_used allow.
@task_item n=1 part=touches
The expect and unwrap justification convention (justified-expect-unwrap and its duplicates unwrap-expect-justified). Its except cell names the browser favicon test module's unjustified expect_used allow as the exception. That allow is now justified, so the exception is stale and no browser allow is left bare.
@task_item n=2 part=touches
The rule that file URLs are only for file documents (file-url-load-rule). The browser navigation clause is now proven through the config the document loader hands over, by two new tests: remote_page_config_navigation_to_file_url_loads_nothing and file_page_config_navigation_to_file_url_loads. Any guard cell that names the tests for this rule can add them.
@task_item n=3 part=touches
The unit-test placement conventions (unit-test-placement, unit-tests-at-file-end, split-test-module-as-child). The browser document loader now holds an inline test module at the end of the file.
@task_item n=4 part=touches
The lint-allow locality conventions (clippy-allow-scoping, local-lint-allows). The new test module puts a justified expect_used allow on the test item itself, not at crate level.
@task_item n=5 part=touches
The test-naming and fixture conventions (descriptive-test-names, fixture-precondition-first, tests-assert-fixture-first). The three new browser tests follow them, each named for the behaviour it shows.
@task_item n=6 part=touches
Embedder services injected with no-op defaults (injected-embedder-services, embedder-services-default-noop, shell-and-navigation-providers, and the ShellProvider default-methods contract). The test builds a shell provider stub whose every method is defaulted, and checks that the document config carries the passed net provider and a navigation provider.
@task_item n=7 part=touches
blitz-net as the browser's networking (blitz-net-networking and the blitz-net Provider contract). The test builds the existing blitz-net provider with no handler as the browser's net provider and checks that the config hands that same provider on.
@task_item n=8 part=touches
The document configuration struct (document-config, the DocumentConfig contract and the document defaults for media type and URL). Base URL, net provider and navigation provider are now asserted in the browser config. Viewport and media type stay at None, which equals the default, so their mutants are equivalent.
@task_item n=9 part=touches
The host-run mutation tier and its nextest profile (mutation-tier-host-only, nextest-mutants-profile). It was run over the browser's document-config builder, and 7 missed mutants are left with no browser test that reads them. A row that holds a mutation bound or a list of survivors for the browser would be reached.
@task_item n=10 part=touches
Per-crate coverage floors (per-crate-coverage-floors, coverage-floors, per-crate-coverage-floor). Browser line coverage can rise with the new test module. This is a touch only where a row's cell holds the browser crate's floor value.
@task_item n=11 part=touches
The browser chrome and content split and browser load behaviour (browser-dioxus-native-ui, browser-load-aborts-previous, default-document-url). The page load path is unchanged and is now covered by tests at its config hand-off.
@task_item n=12 part=touches
The browser crate's name (name/crate/browser). The name is unchanged, and no crate or binary name was taken or freed.
@task_item n=1 part=untouched
The engine crates' layout, style, paint, hit testing, select list, accessibility tree and event dispatch decisions and contracts. No engine source changed.
@task_item n=2 part=untouched
The escher driver, the session host and socket wire, the MCP role, kept checks, recording, refusal causes and every driver verb or carrier contract.
@task_item n=3 part=untouched
CI workflow, ci-leg legs and their contracts, cargo-deny, fmt and clippy gates, the MSRV job and the Nix flake. No gate surface or workflow changed.
@task_item n=4 part=untouched
The stack and dependency decisions, workspace dependencies, feature gating and version lockstep. No manifest, lockfile, feature or version moved.
@task_item n=5 part=untouched
Browser history persistence (SQLite store, append-only migrations, history entry cap, consecutive-visit folding) and the about pages. They were not edited.
@task_item n=6 part=untouched
The WPT runner, its result contracts and the PR-body WPT section. Also escher telemetry, the 7GUIs stand and the cold-agent and agent-run scripts.
@task_item n=7 part=untouched
Occupied resources (listeners, state directories, environment variables) and absences. No capability was added or removed.
