@task_prose part=chunk
Narrows blitz-net's file: branch: a file: URL is served only to a request whose loading document is itself a file: document; any other loading document (http, https, data, about, a Dioxus document with no URL) gets an error and nothing is read from disk.
The request learns which document loads it: blitz-traits' Request gains the loading document's URL, which blitz-dom's one request-stamping function fills in from the document's URL for every engine sub-resource fetch.
The rule script loads already keep (file: URLs loadable only from a file: document) becomes one predicate shared by script loads and engine sub-resource fetches, not two that can drift.
A request with no loading document (the embedder's own top-level load) keeps loading as today; file: documents keep loading their file: sub-resources, and data: and http(s): loads are unchanged.
Adds a blitz-net unit test of the three cases and an offline check through the real network provider that a remote document's stylesheet, image and iframe read no local file while a file: document's same loads succeed.
@task_file n=1 path=packages/blitz-net/src/lib.rs mark=edits
what: the engine's network provider, its file: branch and its unit tests — edits: Provider::fetch_inner's file: arm, the unit test delivers_network_and_bad_url_errors (or a sibling test beside it)
@task_file n=2 path=packages/blitz-traits/src/net.rs mark=edits
what: the shared network boundary types, the non-exhaustive Request a fetch carries, and possibly the home of the shared file-load predicate — edits: Request (a field for the loading document's URL)
@task_file n=3 path=packages/blitz-dom/src/net.rs mark=edits
what: the document's sub-resource fetching, the request-stamping function every engine fetch goes through, and two of its call sites — edits: stamped_request, the fetch sites at lines 228 and 438
@task_file n=4 path=packages/blitz-dom/src/document.rs
what: the base document, its URL (BaseDocument::url) and one sub-resource fetch site that stamps its request
@task_file n=5 path=packages/blitz-dom/src/iframe.rs
what: the iframe sub-document load, a sub-resource fetch site that stamps its request
@task_file n=6 path=packages/blitz-dom/src/layout/damage.rs
what: the layout damage pass, a sub-resource fetch site that stamps its request
@task_file n=7 path=packages/blitz-dom/src/mutator.rs
what: the document mutator, two sub-resource fetch sites (stylesheet, image and similar loads) that stamp their requests
@task_file n=8 path=packages/blitz-vibey-script/src/fetch.rs
what: the script fetch path and its rule that a file: script loads only into a file: document (document_may_load), the predicate to be shared or re-used
@task_item n=1 part=needs
blitz-net's file: arm reads from disk only when the request's loading document is a file: document; for an http, https, data or about loading document, or a Dioxus document with no URL, it reads nothing and answers through NetHandler::error — covering stylesheets, @import, images, fonts, iframe sources and every other engine sub-resource fetch.
@task_item n=2 part=needs
Request (non-exhaustive, in the shared network types) carries the loading document's URL, or an equivalent, so a provider can tell which document asked.
@task_item n=3 part=needs
The request-stamping function fills the loading document's URL from the document's own URL for all seven engine sub-resource fetch sites; no engine fetch leaves it unstamped.
@task_item n=4 part=needs
A request with no loading document, the embedder's own top-level load (the browser's and the markdown viewer's navigation of a file: path the user named), keeps loading a file: URL as today.
@task_item n=5 part=needs
One predicate decides file: loadability for both script loads and engine sub-resource fetches; the script-load rule (file: URLs loadable only from a file: document) keeps its current behaviour and is now the same code the network provider calls.
@task_item n=6 part=needs
A file: document keeps loading its file: sub-resources exactly as today: the browser opening a local HTML file, the markdown viewer and the WPT runner reading local fixtures stay green.
@task_item n=7 part=needs
data: and http(s): loads are unchanged for every loading document.
@task_item n=8 part=needs
No path restriction, response-size cap or request timeout is added to the file: branch; the standing limit holds for a file: document and only who may reach it narrows.
@task_item n=9 part=needs
The refused load's error carries no page content beyond what NetHandler::error already carries; no log site, print, environment read or panic path is added on the refusal.
@task_item n=10 part=needs
blitz-net unit test: a file: request whose loading document is https errors, one whose loading document is file: delivers the bytes, one with no loading document delivers them.
@task_item n=11 part=needs
Integration check through blitz-net's real Provider, offline with no listener: a non-file: document's stylesheet, image and iframe pointing at an existing local file read nothing (the load errors; no style, no image, no sub-document), and a file: document's same three loads succeed — in both layout modes where the document is laid out.
@task_item n=12 part=needs
The check's local file and documents are built by the test itself (a temporary file or a committed fixture read by path), with no wall-clock sleep or network access; a failed load must still end any settle or load wait the check uses.
@task_item n=13 part=needs
At wrap the test plan's file-URL decision (file: URLs loadable only from a file: document) is restated to cover the engine's sub-resource fetches beside script loads; the architecture decision on refused file script loads and the security plan's blitz-net limit are restated alike.
@task_standing n=1
convention: unit tests sit in an inline test module in their own source file
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=2
convention: each new integration test file is its own test target, and a shared helper module is no target
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: a new test is controlled red-before-green: it fails against the unchanged code and passes with the change
home: unit-test-strategy
@task_standing n=4
convention: test failures print no argument value; table-driven tests assert their row count
home: unit-test-strategy
@task_standing n=5
convention: a regression test documents and is named for the behaviour it guards
home: 2-test-strategy
@task_standing n=6
convention: layout assertions run in both layout modes; checks use no sleep or wall-clock read
home: integration-test-strategy
@task_standing n=7
convention: test data is stated in code, nothing seeded from outside; stand-ins are hand-written, no mocking library
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=8
convention: a test file joining a CI leg keeps its legs, and the per-crate coverage floor still holds for the crates the change touches
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Driver session, session wire, session errors, refusal causes and driver verb contracts: the stand and the driver do not use blitz-net, and no verb, argument, key name, result field, cause, session error, exit or carrier changes.
@task_item n=2 part=not_needed
escher-session command line, MCP role, CLI exit codes, help and snapshot contracts: nothing of the session binary or its outputs changes.
@task_item n=3 part=not_needed
Agent-run and cold-agent contracts, verbs, events, exit codes and verdicts: the chunk adds no harness run, event or verdict.
@task_item n=4 part=not_needed
Session recording, committed recorded checks and session state directory rows: no recording or check file is written or changed.
@task_item n=5 part=not_needed
Handler coverage, lint defects, text contrast and snapshot budget rows: none of these readings is touched.
@task_item n=6 part=not_needed
Driver advance, virtual clock and stand timer rows: the chunk changes no time or timer behaviour.
@task_item n=7 part=not_needed
Navigation: a link, form submission or navigate_to towards a file: URL goes through the embedder's navigation provider and is out of this chunk.
@task_item n=8 part=not_needed
Pixel, screenshot, font-metric, select-list and scrollbar paint rows: the chunk changes no painting and takes no screenshot.
@task_item n=9 part=not_needed
The Dioxus asset scheme served from local files is the embedder's own asset path and is not narrowed.
