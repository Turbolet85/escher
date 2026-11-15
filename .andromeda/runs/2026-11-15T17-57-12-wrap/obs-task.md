@task_prose part=session
The session added three unit tests to the browser app's document loader, proving the document config it hands each page carries the page URL as base URL, the net provider passed in, and a navigation provider that keeps the file: gate.
Two of the tests read the gate's two directions: a remote page's config loads nothing on a navigation to a file: URL, a file: page's config loads it.
It added a justification comment beside two clippy expect_used allows in the browser (the favicon test module and the tab's web view), so every browser expect_used allow now carries its reason.
Measured: the browser crate's unit tests 35 to 38; bare expect_used allows in the browser 2 to 0; the document config builder's mutants 11 missed to 7 missed with 4 caught.
No runtime code, log, print, span, environment read, dependency, schema, CI or gate surface changed; the diff is test code and comments only, and CI read green.
@task_file n=1 path=apps/browser/src/document_loader.rs
what: the browser app's document loader, which builds each page's document config (base URL, net provider, navigation provider) on which no file: loads from non-file: documents rests; it now holds an inline unit test module
@task_file n=2 path=apps/browser/src/favicon.rs
what: the browser app's favicon fetch and its test module; one justification comment added above its expect_used allow
@task_file n=3 path=apps/browser/src/tab.rs
what: the browser app's tab and its web view; two justification comment lines added above an expect_used allow
@task_item n=1 part=touches
The decision that no file: loads happen from non-file: documents: two new browser tests now guard its browser-navigation clause through the document config, a remote page's navigation to a file: URL loading nothing and a file: page's loading; a row whose guard cell names that decision's tests may add them.
@task_item n=2 part=touches
Any row whose guard cell names remote_document_cannot_load_file_urls, file_document_can_load_sibling_file_urls or local_files_for_local_documents: the new tests remote_page_config_navigation_to_file_url_loads_nothing and file_page_config_navigation_to_file_url_loads stand beside them.
@task_item n=3 part=touches
Indirect: any rule over the browser's document config or its navigation provider, which the new tests now read (base URL is the page URL, the net provider is the one passed in); the config builder itself is unchanged.
@task_item n=4 part=touches
Any row whose cell holds the browser crate's unit test count (now 38, was 35) or lists the browser's tested source files (the document loader joins them).
@task_item n=5 part=touches
Any row whose cell holds a mutation-testing figure for the browser's document config builder: 7 missed and 4 caught of 11; the survivors are two equivalent fields and five field deletions no browser test reads.
@task_item n=6 part=touches
Any row naming an exception for a bare clippy expect_used allow in the browser's favicon test module: that allow now stands below its justification, so no bare allow remains in the browser.
@task_item n=7 part=touches
Indirect: rules that the browser adds no log, print or environment read hold unchanged; the session's silence probe read none added in the browser.
@task_item n=1 part=untouched
Telemetry strategy and the tracing feature gating: no tracing call, feature or subscriber changed.
@task_item n=2 part=untouched
Span and trace coverage: the driver's command and tool_call spans, their fields and endings, and the refusal cause names are untouched.
@task_item n=3 part=untouched
Metric coverage and the absence of metric emission, exporters, OTel export or an error-reporting service: none added.
@task_item n=4 part=untouched
escher's telemetry sink, its log line layout, scrub set, target allowlist, RUST_LOG handling and resource keys: unchanged.
@task_item n=5 part=untouched
escher-session, its roles, exit codes, host wire, recording, coverage line and session state directory: no change reached them.
@task_item n=6 part=untouched
The agent-run and cold-agent harnesses, their events, verdict fields and kept checks: untouched.
@task_item n=7 part=untouched
PII rules over the snapshot, select, actions and graph answers, and the page script events: untouched.
@task_item n=8 part=untouched
Script error surface, the window error event, WPT reports and the panic hooks: untouched.
@task_item n=9 part=untouched
CI integration: workflows, artifacts and the package-install diagnostic line unchanged; CI only read green.
