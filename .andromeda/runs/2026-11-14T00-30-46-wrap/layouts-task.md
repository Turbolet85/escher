@task_prose part=session
The session closed the network provider's file: branch for a document that is not itself a file: document: a remote page's sub-resource fetch now reads no local file.
Every engine sub-resource request (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe) now carries its loading document's URL as the request's initiator; a request with no initiator (an embedder's top-level load, a navigation) or a file: initiator reads the file as before.
A refused read comes back as a fixed-text network error, file: load refused for a non-file document, through the handler's existing error path, with no new log site.
The file-load predicate now has one body in the shared traits crate and the script crate re-exports it under the same name; blitz-dom may build a request only through its one stamping constructor, enforced by a crate-level clippy rule.
One new blitz-tests target witnesses it in both layout modes; nothing was retired, and the favicon probe and file: navigation from a remote page stay open, owed to a provisional route entry.
@task_file n=1 path=packages/blitz-traits/src/net.rs
what: the shared network request type and its builder, now with the loading-document initiator, and the one body of the file-load predicate a document may load
@task_file n=2 path=packages/blitz-traits/src/navigation.rs
what: the navigation options that turn a top-level navigation into a network request, with no initiator
@task_file n=3 path=packages/blitz-vibey-script/src/fetch.rs
what: the script engine's fetch, which re-exports the file-load predicate for script loads
@task_file n=4 path=packages/blitz-net/src/lib.rs
what: the network provider that serves http, data and file URLs, and its typed provider errors, now refusing a file read for a non-file document
@task_file n=5 path=packages/blitz-dom/src/net.rs
what: the document's sub-resource loading: linked stylesheets, @import, @font-face fetches and the one request constructor that stamps the loading document
@task_file n=6 path=packages/blitz-dom/src/document.rs
what: the engine document, which builds its resource requests and adds stylesheets and their fonts
@task_file n=7 path=packages/blitz-dom/src/iframe.rs
what: the iframe sub-document load started from its parent document
@task_file n=8 path=packages/blitz-dom/src/mutator.rs
what: the document mutator that builds a linked stylesheet's load handler when a link element is inserted
@task_file n=9 path=packages/blitz-dom/src/cssom.rs
what: the CSS object model's rule insertion, including @font-face and its stylesheet loader
@task_file n=10 path=packages/blitz-dom/src/layout/damage.rs
what: the style damage pass that fetches CSS background images
@task_file n=11 path=packages/blitz-dom/clippy.toml
what: blitz-dom's lint configuration, forbidding a bare network request constructor outside the stamping one
@task_file n=12 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: the check that a remote page reads no local file and a file page reads them all, and that every engine request carries its document
@task_file n=13 path=tests/blitz-tests/Cargo.toml
what: the blitz-tests crate manifest and its dev-dependencies, now with the network provider and an async runtime
@task_file n=14 path=Cargo.lock
what: the workspace lockfile, moved only in blitz-tests' own dependency list
@task_item n=1 part=touches
The rule that the load page alone has a network provider: that provider now refuses a file: sub-resource for a non-file document, so a row stating what the load page's provider serves may now be partial.
@task_item n=2 part=touches
The load task served by escher-session and its load-fetch and load-done ids: its fetch runs through the network provider whose file: branch changed; a row that says what the fetch can reach, or what a failed fetch shows, is touched where it names a file: URL or a local fixture.
@task_item n=3 part=touches
The fixed document skeleton with its main mount, and any row on how a stand or session document is built from HTML with a base URL: the base URL is now the initiator of every stylesheet, font, image and iframe request, so a stand document served as a non-file document cannot pull a local file sub-resource.
@task_item n=4 part=touches
The absence of arbitrary index.html templates for a document: indirectly, a document's own linked stylesheets, @import, fonts, images and iframes now load local files only when the document itself is a file: document.
@task_item n=5 part=touches
Any row that names the escher-session or stand load path as reading local files, fixtures or a temporary directory through file: URLs: such reads hold for a file: document or a top-level load alone.
@task_item n=6 part=touches
Any row whose cell names blitz-tests' target count or the list of targets a stand or session check runs among: blitz-tests' top-level test files moved 171 to 172 and its targets 170 to 171; the new target is not in the stand repeat list.
@task_item n=1 part=untouched
The ShellProvider window chrome contracts (drag, maximize, minimize, close, decorations): no window code moved.
@task_item n=2 part=untouched
The stand stable element ids of the counter, temperature converter, flight booker, timer, CRUD, circle drawer and cells tasks, and their task cards: no stand page or id changed.
@task_item n=3 part=untouched
The stand's look and access: accent colour, names, roles and states by attributes, ids as attributes, lints clean at boot, pinned headless viewport.
@task_item n=4 part=untouched
The stand's interaction decisions: the CRUD list as one keyboard region, open list rows reading their own bounds, into-view scrolling, visible regions through clipping boxes, the flight type select read at boot, a held instance reading as a fresh boot, keyed controls, shared session roles.
@task_item n=5 part=untouched
The agent-run, cold-agent, multi_doc and paint_bench command-line contracts, and the cold-agent decisions: no script, usage, exit code or stdout shape changed.
@task_item n=6 part=untouched
The escher-session MCP server, command line, exit codes, actions, snapshot, screenshot, graph, lint, coverage, recording, host log and refusals: no session verb, refusal, answer or bound changed, and the new network error is no session refusal.
@task_item n=7 part=untouched
The mobile-native and web-spa surfaces and the decisions log: no layout surface was reached.
