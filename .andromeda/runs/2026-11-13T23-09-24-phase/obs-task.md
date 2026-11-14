@task_prose part=chunk
Closes blitz-net's file: branch to every loading document that is not itself a file: document: a remote, data:, about: or URL-less Dioxus document reads no local file through a stylesheet, @import, image, font, iframe or any other engine sub-resource fetch, and the load answers an error instead.
The engine's request gains the loading document's URL, stamped by the one function every engine sub-resource fetch passes through; a request with no loading document (an embedder's own top-level load) keeps loading as today.
The rule script loads already keep becomes one predicate shared by script loads and engine sub-resource fetches, not two that can drift.
At wrap the masters restate it: the file-script-load decision takes the clause that a remote page cannot read local files, and the security plan's blitz-net limit and the test plan's file-URL decision name engine sub-resource fetches beside script loads.
@task_file n=1 path=packages/blitz-net/src/lib.rs mark=edits
what: blitz-net's network provider, the component that serves file:, data: and http(s): fetches for the engine — edits: the file: arm of Provider::fetch_inner, its unit tests (delivers_network_and_bad_url_errors)
@task_file n=2 path=packages/blitz-traits/src/net.rs mark=edits
what: blitz-traits' net boundary, the engine's Request and the NetHandler the provider answers through — edits: Request (a field or equivalent carrying the loading document's URL), and possibly the shared file:-load predicate's new home
@task_file n=3 path=packages/blitz-dom/src/net.rs mark=edits
what: blitz-dom's engine fetch stamping, the function every sub-resource fetch is built by — edits: stamped_request, which reads the document's URL (BaseDocument::url)
@task_file n=4 path=packages/blitz-vibey-script/src/fetch.rs mark=edits
what: the script engine's fetch path and its rule that a script load of a file: URL is allowed only for a file: document — edits: document_may_load, kept and re-used or moved to the shared home
@task_item n=1 part=needs
The engine's file: sub-resource fetch is refused for any loading document that is not a file: document: nothing is read from disk and the load is answered as an error through NetHandler::error; a file: document's file: sub-resources still load exactly as before.
@task_item n=2 part=needs
The engine's Request carries the loading document's URL, stamped once by stamped_request from the document's URL; a request with no loading document (the browser's or the markdown viewer's navigation to a file: path the user named) keeps loading.
@task_item n=3 part=needs
One predicate decides whether a document may load a file: URL, shared by script loads and engine sub-resource fetches: the decision no file: loads from non-file: documents and the file-script-load refusal now cover the engine's own fetches too.
@task_item n=4 part=needs
data: and http(s): loads stay unchanged for every document; the change is a narrowing only, no boundary is widened.
@task_item n=5 part=needs
The refused load adds no log site, print, environment read or panic path, and its error text carries no page content beyond what NetHandler::error already carries; any URL or error reaching a log stays under the engine targets' safe-field and url/error scrub rules.
@task_item n=6 part=needs
A refused stylesheet is a failed stylesheet load: it must leave the document's pending critical stylesheet set as any failed stylesheet does, so the document still lays out with no style from it.
@task_item n=7 part=needs
No path restriction, response-size cap or request timeout is added to the file: branch: the standing limit on blitz-net's file: reads keeps holding for file: documents, only who may reach it narrows.
@task_item n=8 part=needs
Witness: a check, in both layout modes where a document is laid out, that a non-file: document's stylesheet, image and iframe pointing at an existing local file read nothing (load errors, no style, no image, no sub-document) and a file: document's same three loads succeed, through blitz-net's real Provider, offline with no listener.
@task_item n=9 part=needs
Witness: blitz-net's unit tests show a file: request whose loading document is https: errors, one whose loading document is file: delivers the bytes, and one with no loading document delivers them.
@task_item n=10 part=needs
The markdown viewer, the browser opening a local HTML file and the WPT runner reading local fixtures keep working, as file: documents or top-level loads.
@task_standing n=1
convention: every new test or check this chunk adds runs silent: it prints nothing on success and installs no telemetry sink
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
The driver's command and tool_call spans, their fields and endings, and refusal cause names: the stand and driver do not use blitz-net and no verb, field, cause or span changes.
@task_item n=2 part=not_needed
escher-session's roles, exit codes, host wire, coverage line, MCP stdout and error object: no session error, exit or carrier changes.
@task_item n=3 part=not_needed
Agent-run and cold-agent harness events, verdicts, counted verbs, transcripts and kept checks: this chunk touches no harness.
@task_item n=4 part=not_needed
Metric emission, metrics exporters and WPT score publishing: nothing measured or published changes.
@task_item n=5 part=not_needed
Snapshot, graph, actions answer, select and optgroup rows and page script events: no driver answer or form behaviour changes.
@task_item n=6 part=not_needed
CI artifact uploads, recording bounds, recording and session-state directories and the package-install diagnostic: no CI job or local state is added.
@task_item n=7 part=not_needed
Window error event, window.onerror, script error source labels and the WPT report message: script error reporting is unchanged.
@task_item n=8 part=not_needed
escher-telemetry's panic hook and startup events, the WASM panic hook, OTel export and the sink's resource keys: no sink or hook changes.
