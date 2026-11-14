@task_prose part=chunk
Narrows blitz-net's file: branch: a file: URL is served only to a request whose loading document is itself a file: document; any other loading document gets an error and nothing is read from disk.
The engine's sub-resource request learns which document loads it: the stamping function every engine fetch passes through sets the loading document's URL on the request.
The engine's sub-resource fetches and the script loader share one predicate for whether a document may load a file: URL.
A request with no loading document (an embedder's own top-level load) and every data: and http(s): load keep working as today.
It adds a blitz-net unit test and an offline check, in both layout modes, that a remote document's stylesheet, image and iframe read no local file while a file: document's do.
@task_file n=1 path=packages/blitz-net/src/lib.rs mark=edits
what: the blitz-net network provider that serves file:, data: and http(s): fetches and its unit tests — edits: Provider::fetch_inner's file: arm, the unit test delivers_network_and_bad_url_errors
@task_file n=2 path=packages/blitz-traits/src/net.rs mark=edits
what: the engine's shared network boundary, the non-exhaustive Request type and the NetHandler that answers it — edits: Request (a field or equivalent carrying the loading document's URL), possibly the shared file-load predicate's home
@task_file n=3 path=packages/blitz-dom/src/net.rs mark=edits
what: the DOM's request stamping that every engine sub-resource fetch goes through — edits: stamped_request
@task_file n=4 path=packages/blitz-vibey-script/src/fetch.rs mark=edits
what: the script loader's fetch gate deciding whether a document may load a URL — edits: document_may_load (kept or re-used as the one shared predicate)
@task_item n=1 part=needs
The engine's Request carries the URL of the document that loads it (or an equivalent), added to a non-exhaustive type; a request with no loading document stays expressible.
@task_item n=2 part=needs
The stamping function sets the loading document's URL from the document's own URL, so all seven engine sub-resource sites (stylesheet, @import, image, font, iframe and the rest) carry it without each site doing it.
@task_item n=3 part=needs
The file: arm of the network provider serves a file: URL only when the loading document is a file: document; for an http:, https:, data:, about: or URL-less loading document it reads nothing from disk and answers through the handler's error path.
@task_item n=4 part=needs
A request with no loading document, the embedder's own top-level load such as the browser or the markdown viewer opening a path the user named, keeps loading a file: URL as today.
@task_item n=5 part=needs
A file: document keeps loading its file: sub-resources exactly as today: the browser on a local HTML file, the markdown viewer and the WPT runner reading local fixtures stay green.
@task_item n=6 part=needs
data: and http(s): loads are unchanged for every document.
@task_item n=7 part=needs
One predicate decides whether a document may load a file: URL, shared by the script loader and the engine's sub-resource fetches so the two cannot drift; where it lives (beside Request in the shared traits, or in the script crate and re-used) is the plan's choice.
@task_item n=8 part=needs
The file: branch gains no path restriction, response-size cap or request timeout: the chunk narrows only who may reach it, and the existing standing limit keeps holding for a file: document.
@task_item n=9 part=needs
The refusal adds no log site, print, env read or panic path, and its error text carries no page content beyond what the handler's error already carries; the change is a narrowing only, no boundary widens.
@task_item n=10 part=needs
Navigation (a link, a form submission or navigate_to toward a file: URL) goes through the embedder's navigation provider and is left untouched; the dioxus scheme served from local files is not narrowed.
@task_item n=11 part=needs
A blitz-net unit test proves three cases: a file: request whose loading document is https: errors, one whose loading document is file: delivers the bytes, one with no loading document delivers them.
@task_item n=12 part=needs
An offline check with no listener, through the real network provider, in both layout modes where a document is laid out: a non-file: document's stylesheet, image and iframe pointing at an existing local file read nothing (load errors, no style, no image, no sub-document), and a file: document's same three loads succeed.
@task_item n=1 part=not_needed
The escher-session driver's contracts (verbs, arguments, answers, refusals, exit codes, recording, MCP server, served tasks, host log): the stand and the driver do not use blitz-net, and the chunk adds or changes nothing on that surface.
@task_item n=2 part=not_needed
The stand's stable element ids and the stand's own decisions (accent colour, keyed controls, attributes-only ids and a11y, pinned viewport, held instance, keyboard regions, scroll and clipping, lints at boot): no stand page or control changes.
@task_item n=3 part=not_needed
The stand's load page and its in-process network provider: it answers through its own provider, not blitz-net, and is not touched.
@task_item n=4 part=not_needed
The shell provider's window chrome controls: the chunk touches no window or shell behaviour.
@task_item n=5 part=not_needed
The agent-run and cold-agent test pipes and their command lines, and the cold agent's decisions: no script, pipe or agent run changes.
@task_item n=6 part=not_needed
The multi_doc and paint_bench command lines: no benchmark binary, argument or environment variable changes.
@task_item n=7 part=not_needed
The absence of arbitrary index.html templates for a document, and the unmeasured mobile-native and web-spa surfaces: the chunk adds no document template and reaches no such surface.
