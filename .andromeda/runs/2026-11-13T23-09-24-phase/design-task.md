@task_prose part=chunk
Narrows the engine's network provider so a file: URL is served only to a request whose loading document is itself a file: document; any other loading document (http, https, data, about, a Dioxus document with no URL) reads nothing from disk and gets an error.
Makes the request say which document loads it: the engine's one request-stamping function carries the loading document's URL on every sub-resource fetch it stamps.
Shares one predicate between script loads and the engine's own sub-resource fetches, so the two cannot drift.
An embedder's own top-level load (no loading document) and every file: document's file: sub-resources keep loading as today; data: and http(s): loads are unchanged.
Adds a witness check (stylesheet, image, iframe from a non-file: and from a file: document, both layout modes, offline) and the provider's own unit test.
@task_file n=1 path=packages/blitz-net/src/lib.rs mark=edits
what: the engine's network provider, which fetches file:, data: and http(s): URLs and answers errors through the net handler — edits: the provider's fetch function's file: arm, its unit tests
@task_file n=2 path=packages/blitz-traits/src/net.rs mark=edits
what: the shared network request type and the net handler boundary between the engine and a network provider — edits: the request type (a field or equivalent the loading document's URL rides on)
@task_file n=3 path=packages/blitz-dom/src/net.rs mark=edits
what: the engine's request stamping and its stylesheet and resource fetch handlers — edits: the request-stamping function, the two sub-resource fetch sites in it
@task_file n=4 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine's base document, which holds the document's URL — edits: its sub-resource fetch site
@task_file n=5 path=packages/blitz-dom/src/iframe.rs mark=edits
what: the engine's iframe sub-document loading — edits: its sub-document fetch site
@task_file n=6 path=packages/blitz-dom/src/layout/damage.rs mark=edits
what: the engine's layout damage pass, which fetches resources found while restyling — edits: its sub-resource fetch site
@task_file n=7 path=packages/blitz-dom/src/mutator.rs mark=edits
what: the engine's DOM mutator, which starts fetches for inserted link, image and style elements — edits: its two sub-resource fetch sites
@task_file n=8 path=packages/blitz-vibey-script/src/fetch.rs mark=edits
what: the script runtime's script fetching and the rule that a file: script loads only into a file: document — edits: the document-may-load predicate (kept here and re-used, or moved beside the request type)
@task_item n=1 part=needs
The network provider's file: branch: serves a file: URL only when the request's loading document is a file: document; for any other loading document it reads nothing from disk and answers an error through the net handler.
@task_item n=2 part=needs
The request type gains the loading document's URL (a field or an equivalent); the type stays non-exhaustive and a request with no loading document is still constructible.
@task_item n=3 part=needs
The request-stamping function reads the document's URL and stamps it on every sub-resource request: stylesheets, @import, images, fonts, iframe sub-documents and the other engine fetches.
@task_item n=4 part=needs
A request with no loading document (an embedder's own top-level navigation of a file: path the user named, in the browser and the markdown viewer) keeps loading as today.
@task_item n=5 part=needs
One predicate decides both script loads and engine sub-resource fetches (file: only for file: documents); where it lives, beside the request type or kept in the script runtime and re-used, is the plan's choice.
@task_item n=6 part=needs
A file: document keeps loading its file: sub-resources exactly as today: the browser opening a local HTML file, the markdown viewer and the WPT runner reading local fixtures stay green.
@task_item n=7 part=needs
data: and http(s): loads stay unchanged for every document.
@task_item n=8 part=needs
No path restriction, response-size cap or request timeout is added to the file: branch; the standing no-limit holds for a file: document, only who may reach it narrows.
@task_item n=9 part=needs
The refused load's error carries no page content beyond what the net handler's error already carries; no log site, print, env read or panic path is added on the refusal.
@task_item n=10 part=needs
A refused sub-resource produces no style, no image and no sub-document: the load errors and the document lays out as with any failed fetch.
@task_item n=11 part=needs
Witness check: a non-file: document's stylesheet, image and iframe pointing at an existing local file read nothing, and a file: document's same three loads succeed, in both layout modes, through the real network provider, offline, with no listener.
@task_item n=12 part=needs
The provider's own unit test: a file: request whose loading document is https errors, one whose loading document is file: delivers the bytes, one with no loading document delivers them; the existing network and bad-URL error test keeps passing.
@task_standing n=1
convention: a new test that lays out a document with text is built with system fonts, and an unavailable family falls back to the default font
home: typography
@task_item n=1 part=not_needed
Navigation (a link, a form submission, navigate-to towards a file: URL) goes through the embedder's navigation provider and is out of this chunk.
@task_item n=2 part=not_needed
The Dioxus asset scheme served from local files is the embedder's own asset path and is not narrowed.
@task_item n=3 part=not_needed
The driver and the stand: no driver verb, argument, key name, result field, cause, session error, exit status, carrier, log field or span changes; the escher-session commands, exit statuses, stdout, stderr, mcp role and tasks rows do not bind.
@task_item n=4 part=not_needed
Colour, spacing, depth, border-radius, motion and iconography rows: the chunk changes no paint, colour, layout spacing, stacking, animation, scroll or cursor behaviour.
@task_item n=5 part=not_needed
The select list, its events, accessibility tree, snapshot and screenshot rows: untouched.
@task_item n=6 part=not_needed
The JS window object, MountedData during render and shell provider contracts: untouched.
@task_item n=7 part=not_needed
The web-spa (WASM canvas) surface and mobile targets: no change to how they are built or rendered.
