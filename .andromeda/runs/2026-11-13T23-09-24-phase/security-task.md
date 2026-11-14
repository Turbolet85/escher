@task_prose part=chunk
Narrows blitz-net's file: branch: a file: URL is served only to a request whose loading document is itself a file: document; any other loading document (http, https, data, about, a Dioxus document with no URL) reads nothing from disk and gets an error through NetHandler::error.
The request learns which document loads it: blitz-traits' Request carries the loading document's URL (or an equivalent), stamped by blitz-dom's one stamping function from the document's URL at every engine sub-resource site; a request with no loading document (an embedder's own top-level load) keeps loading as today.
The rule script loads already keep (file URLs only for file documents) becomes one predicate shared by the script loader and the engine's sub-resource fetches, its home left to the plan.
Adds a blitz-net unit test and an offline witness check in both layout modes; at wrap the security plan's blitz-net limit and the file-URL decision restate the engine's sub-resource fetches beside script loads.
@task_file n=1 path=packages/blitz-net/src/lib.rs mark=edits
what: blitz-net, the engine's outbound fetch provider and its file: branch — edits: Provider::fetch_inner's file: arm, its unit tests (delivers_network_and_bad_url_errors)
@task_file n=2 path=packages/blitz-traits/src/net.rs mark=edits
what: blitz-traits net, the request type every engine fetch crosses — edits: Request (a field for the loading document's URL), possibly the shared file-URL predicate's new home
@task_file n=3 path=packages/blitz-dom/src/net.rs mark=edits
what: blitz-dom's network layer, the one function that stamps every engine sub-resource request, and two sub-resource sites — edits: stamped_request, the stylesheet/import and resource request sites
@task_file n=4 path=packages/blitz-vibey-script/src/fetch.rs mark=edits
what: the default script fetcher and its file-URL rule for script loads — edits: document_may_load (kept and re-used, or moved to the shared predicate)
@task_file n=5 path=packages/blitz-dom/src/document.rs mark=edits
what: the base document, its URL (BaseDocument::url) and its request-stamping wrapper — edits: the sub-resource request site
@task_file n=6 path=packages/blitz-dom/src/iframe.rs mark=edits
what: the iframe sub-document loader that fetches an iframe src — edits: the iframe src request site
@task_file n=7 path=packages/blitz-dom/src/layout/damage.rs mark=edits
what: the layout pass that starts sub-resource loads (images, fonts) — edits: the sub-resource request site
@task_file n=8 path=packages/blitz-dom/src/mutator.rs mark=edits
what: the document mutator that starts sub-resource loads as elements are added or changed — edits: its two sub-resource request sites
@task_item n=1 part=needs
The file-URL rule for documents (file URLs load only for, and only from, file documents) now binds the engine's own sub-resource fetches through blitz-net, not only script loads: stylesheet, @import, image, font, iframe src and every other sub-resource.
@task_item n=2 part=needs
One predicate decides whether a loading document may load a URL, shared by the default script fetcher and blitz-net's file: branch, so the two cannot drift; the script fetcher's file-and-data-only and local-only rules keep holding unchanged for script loads.
@task_item n=3 part=needs
The engine request type gains the loading document's URL; it is filled by the one stamping function from the document's URL at all seven sub-resource sites, and the stamping keeps carrying the document's abort signal so its fetches still cancel.
@task_item n=4 part=needs
A request with no loading document (the browser's or the markdown viewer's top-level load of a file: path the user named) keeps loading from disk as today.
@task_item n=5 part=needs
A refused file: load reads nothing from disk and is answered only through NetHandler::error; its error text holds no page content beyond what that error already carries, and no log site, print, env read or panic path is added on the refusal.
@task_item n=6 part=needs
A refused sub-resource behaves as any failed load: a failed stylesheet blocks no render, a failed load ends the bounded load wait, an iframe whose src is refused gets no sub-document, and the existing resource-load-failure log carries only its url and error fields.
@task_item n=7 part=needs
A file: document keeps loading its file: sub-resources exactly as today: the browser opening a local HTML file, the markdown viewer and the WPT runner reading local fixtures stay green.
@task_item n=8 part=needs
data: and http(s): loads are unchanged for every document; sub-resource requests stay GETs and the per-host outbound limit is untouched.
@task_item n=9 part=needs
No path restriction, response-size cap or request timeout is added to the file: branch: the standing absences of a response size limit and a request timeout on outbound requests keep holding for a file: document; the chunk narrows only who may reach it.
@task_item n=10 part=needs
The change is a narrowing for every document, remote pages included, read under the boundary widening rule: nothing crosses a boundary it did not cross before.
@task_item n=11 part=needs
Carve-out: the Dioxus scheme served from local files is the embedder's own asset path and is not narrowed by the rule.
@task_item n=12 part=needs
Carve-out: navigation is not a fetch by the page; a link, a form submission or navigate_to towards a file: URL goes through the embedder's navigation provider and is outside the narrowed rule.
@task_item n=13 part=needs
Witness: a check in both layout modes that a non-file: document's stylesheet, image and iframe pointing at an existing local file read nothing (load errors, no style, no image, no sub-document) and a file: document's same three loads succeed, through blitz-net's real provider, offline, with no listener.
@task_item n=14 part=needs
blitz-net's own unit test: a file: request whose loading document is https: errors, one whose loading document is file: delivers the bytes, one with no loading document delivers them.
@task_item n=15 part=needs
At wrap the security plan's blitz-net standing limit and its file-URL decision restate that the engine's sub-resource fetches, beside script loads, take file: URLs only for file: documents.
@task_standing n=1
convention: a new test reaches no network: it runs offline with no listener, and a local fixture it reads is the test's own
home: api-security
@task_standing n=2
convention: a new error or refusal path adds no log line, print, environment read or panic, and carries no page content beyond its existing error
home: error-handling · logging-monitoring
@task_item n=1 part=not_needed
The driver session socket, its wire, replies, refusal causes, session errors, exit codes and CLI flags: the stand and the driver do not use blitz-net and the chunk adds or changes no verb, argument, key, result field, cause, error or exit.
@task_item n=2 part=not_needed
The MCP server, its tool results, protocol errors, transport and spans: nothing of the MCP crossing is touched.
@task_item n=3 part=not_needed
Select lists, form controls, accessibility names and states, the stable element id and snapshot or diff contracts: the chunk touches no control, tree or page reading.
@task_item n=4 part=not_needed
Cold-agent runs, recordings, kept checks and the coverage role: the chunk records nothing and adds no check file of that kind.
@task_item n=5 part=not_needed
CI secrets, publish and release guards, signing material and the WPT post-results workflow: the chunk changes no workflow or secret.
@task_item n=6 part=not_needed
Browser history, cookie store, HTTP cache, clipboard, file dialog and TLS choice in data protection: the chunk stores and encrypts nothing.
@task_item n=7 part=not_needed
External navigation schemes, form submission by method, iframe navigations routed to the parent and external targets opened in the system browser: navigation is out of this chunk.
@task_item n=8 part=not_needed
JS-facing fetch method limits, CSSOM script API contracts and JS module loading errors: the script engine's fetch API and module loading are not changed.
@task_item n=9 part=not_needed
Authentication and authorization rows: there is no application authentication and the chunk adds none.
