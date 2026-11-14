@task_prose part=chunk
Narrows blitz-net's file: branch so a file: URL is served only to a request whose loading document is itself a file: document; any other loading document (http, https, data, about, a Dioxus document with no URL) reads nothing from disk and gets an error through NetHandler::error.
Makes the engine's request say which document loads it: blitz-traits' Request carries the loading document's URL, stamped by blitz-dom's one stamping function from the document's URL for every sub-resource fetch (stylesheet, @import, image, font, iframe src).
The rule script loads already keep (file URLs only for file documents) now holds for the engine's own sub-resource fetches through one shared predicate, its home left to the plan.
A request with no loading document (an embedder's own top-level load) keeps loading as today; data: and http(s): loads are unchanged.
Proves it with a blitz-net unit test and an offline check through the real Provider in both layout modes.
@task_file n=1 path=packages/blitz-net/src/lib.rs mark=edits
what: blitz-net Provider, the embedder's networking component that fetches by URL scheme — edits: Provider::fetch_inner's file: arm, its unit test delivers_network_and_bad_url_errors
@task_file n=2 path=packages/blitz-traits/src/net.rs mark=edits
what: blitz-traits net boundary, the non-exhaustive Request every fetch carries — edits: Request (a field for the loading document's URL), possibly the shared file-URL predicate's home
@task_file n=3 path=packages/blitz-dom/src/net.rs mark=edits
what: blitz-dom's engine network glue, where every engine fetch is stamped and two sub-resource fetch sites live — edits: stamped_request
@task_file n=4 path=packages/blitz-vibey-script/src/fetch.rs mark=edits
what: blitz-vibey-script public API's script fetcher and its document_may_load rule, refusing file script loads for non-file documents — edits: document_may_load
@task_item n=1 part=needs
The blitz-net Provider's fetch: its file: scheme arm now refuses a request whose loading document is not a file: document and answers through NetHandler error; data: and http(s): arms unchanged.
@task_item n=2 part=needs
The NetHandler error contract: the refusal is delivered as an error, whose text carries nothing beyond what that error already carries.
@task_item n=3 part=needs
The Request type in blitz-traits gains the loading document's URL (or an equivalent); the type is non-exhaustive and new fields keep it so.
@task_item n=4 part=needs
Every engine fetch is stamped by one function in blitz-dom, which now reads the document's URL (BaseDocument url, the DocumentUrl default) and puts it on the request for all sub-resource sites: stylesheets, imports, images, fonts, iframe src.
@task_item n=5 part=needs
File URLs only for file documents: the rule that file script loads are refused for a non-file document, held by document_may_load, now governs the engine's sub-resource fetches through one predicate, not two that can drift.
@task_item n=6 part=needs
The blitz-vibey-script public API's document_may_load stays callable as today if the predicate moves to blitz-traits beside Request; the default script fetcher's scheme handling stays as it is.
@task_item n=7 part=needs
A request with no loading document, an embedder's own top-level load such as the browser's or the markdown viewer's navigation to a file path, keeps loading from disk as today.
@task_item n=8 part=needs
A file: document keeps loading its file: sub-resources exactly as today; the browser opening a local HTML file, the markdown viewer and the WPT runner reading local fixtures stay green.
@task_item n=9 part=needs
The Dioxus document's default base URL and a Dioxus document with no URL count as non-file loading documents: their file: sub-resource fetches are refused.
@task_item n=10 part=needs
Iframe sub-document loads (the NavigateIframe document event, srcdoc versus src) go through the same refusal when the loading document is not a file: document: no sub-document is built from a local file.
@task_item n=11 part=needs
A failed stylesheet stops blocking render: a refused file: stylesheet must still unblock the page as any failed load does; image cache by URL must not cache a refused load as a success.
@task_item n=12 part=needs
The security limit on blitz-net's file: branch (no path restriction, no response-size cap, no request timeout) keeps holding for a file: document; no such limit is added here.
@task_item n=13 part=needs
No log site, print, environment read or panic path is added on the refusal; unresolvable or refused URLs never panic.
@task_item n=14 part=needs
The check through blitz-net's real Provider runs offline, with no listener: no network port is served by any test.
@task_item n=15 part=needs
At wrap the masters restate the rule: decision file-script-load-refused takes the clause that a remote page cannot read local files, and the file-URL load rule names the engine's sub-resource fetches beside script loads.
@task_standing n=1
convention: A new integration test lives in blitz-tests, one test target per file, one behavior per file, its fixtures stated once with relative asset URLs.
home: conventions
@task_standing n=2
convention: A scenario where a document is laid out runs in both layout modes.
home: conventions
@task_standing n=3
convention: A new test asserts its fixture's precondition first and carries a descriptive name.
home: conventions
@task_standing n=4
convention: Unit tests sit at the end of their source file.
home: conventions
@task_standing n=5
convention: New public items and modules carry docs, cite the spec URL where they implement one, and justify any unwrap or expect.
home: conventions
@task_item n=1 part=not_needed
The escher driver, the session socket, the MCP role and their verbs, arguments, key names, answer fields, refusal causes, session errors and exit codes: the stand and the driver do not use blitz-net and none of these change.
@task_item n=2 part=not_needed
Telemetry init, scrubbing, spans and log-line contracts: the chunk adds no log site or span.
@task_item n=3 part=not_needed
The accessibility tree, snapshot, diff, stable element ids and UI graph contracts: the chunk changes nothing a screen reader reads.
@task_item n=4 part=not_needed
Select list, form controls, event dispatch, focus, scrolling and hit testing decisions: no interaction behaviour is touched.
@task_item n=5 part=not_needed
Paint, layout and CSS feature decisions and the CSS absences: no layout or paint rule changes.
@task_item n=6 part=not_needed
Navigation through the embedder's navigation provider (links, form submission, navigate_to towards a file URL): out of this chunk.
@task_item n=7 part=not_needed
The Dioxus scheme served from local files: the embedder's own asset path, not narrowed.
@task_item n=8 part=not_needed
Browser history, about pages, bookmarks, favicon cache and browser persistence: untouched.
@task_item n=9 part=not_needed
Cold-agent, agent-run and recording contracts and their state directories: untouched.
@task_item n=10 part=not_needed
The names register (binaries, crates, other names): the chunk adds no binary, crate or named value.
@task_item n=11 part=not_needed
Same-origin and CORS checks on script loads and the HTTP cache: the chunk adds no origin check to http(s) loads and does not touch caching.
