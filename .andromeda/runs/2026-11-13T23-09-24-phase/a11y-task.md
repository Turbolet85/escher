@task_prose part=chunk
Closes blitz-net's file: branch to every document that is not itself a file: document: a remote, data:, about: or URL-less page reads no local file through a stylesheet, @import, image, font, iframe or any other engine sub-resource fetch.
The engine's request gains a way to say which document loads it, stamped by the one function every engine fetch passes through; a request with no loading document (an embedder's own top-level load) keeps loading as today.
One predicate, shared with the rule script loads already keep, decides whether a document may load a file: URL; where it lives is the plan's.
A file: document keeps loading its file: sub-resources exactly as today; data: and http(s): loads are unchanged for every document.
@task_file n=1 path=packages/blitz-net/src/lib.rs mark=edits
what: the engine's network provider, which serves http(s):, data: and file: fetches and answers a failed load through the net handler's error — edits: Provider::fetch_inner's file: arm, its unit tests (delivers_network_and_bad_url_errors)
@task_file n=2 path=packages/blitz-traits/src/net.rs mark=edits
what: the engine's shared network boundary, the request every fetch carries and the handler that answers it — edits: Request (a field or an equivalent the loading document's URL rides on)
@task_file n=3 path=packages/blitz-dom/src/net.rs mark=edits
what: the document's network glue that stamps every engine fetch and issues the stylesheet, @import and font sub-resource loads — edits: stamped_request
@task_file n=4 path=packages/blitz-dom/src/document.rs
what: the engine's base document, whose own URL is the loading document a sub-resource request now carries, and one of the sub-resource fetch sites
@task_file n=5 path=packages/blitz-dom/src/iframe.rs
what: the iframe sub-document loader, a sub-resource fetch site whose load builds a nested document
@task_file n=6 path=packages/blitz-dom/src/layout/damage.rs
what: layout damage handling, a sub-resource fetch site for images reached during layout
@task_file n=7 path=packages/blitz-dom/src/mutator.rs
what: the document mutator, two sub-resource fetch sites reached when elements carrying a source are inserted or changed
@task_file n=8 path=packages/blitz-vibey-script/src/fetch.rs
what: the script engine's fetch, home of the predicate that refuses a file: load to a page that is not a file: document (document_may_load)
@task_item n=1 part=needs
A sub-resource fetch from a non-file: document towards a file: URL now errors through the net handler's error and reads nothing from disk: no style, no image, no sub-document for that load.
@task_item n=2 part=needs
An iframe whose src is a file: URL inside a non-file: document produces no nested document; inside a file: document it loads as today.
@task_item n=3 part=needs
The request names its loading document; every engine fetch gets it from the one stamping function, which reads the document's own URL; a request with no loading document keeps loading.
@task_item n=4 part=needs
One predicate decides file: access for both script loads and engine sub-resource fetches; the existing script-load refusal keeps holding unchanged through it.
@task_item n=5 part=needs
No log site, print, env read or panic path is added on the refusal; the refused load's error text carries no page content beyond what the net handler's error already carries.
@task_item n=6 part=needs
The witness check runs in both layout modes where a document is laid out, through blitz-net's real provider, offline with no listener: a non-file: document's stylesheet, image and iframe at an existing local file read nothing, a file: document's same three load.
@task_item n=7 part=needs
blitz-net's unit test covers three requests for a file: URL: loading document https: errors, loading document file: delivers the bytes, no loading document delivers them.
@task_item n=8 part=needs
The browser opening a local HTML file, the markdown viewer and the WPT runner reading local fixtures stay green: they are file: documents or embedder top-level loads.
@task_item n=9 part=needs
No driver verb, argument, key name, result field, cause, session error, exit, carrier, log field or span is added or changed: the stand and the driver do not use blitz-net.
@task_item n=1 part=not_needed
ARIA patterns and roles, hidden-content and role-mapping rows: the chunk changes which local files a page may fetch, not how any element maps into the accessibility tree.
@task_item n=2 part=not_needed
Keyboard navigation: focus, Tab order, keyboard activation, select lists, range inputs and the seven_guis grid and list keys; no input or focus path is touched.
@task_item n=3 part=not_needed
Visual design verification: contrast thresholds, the lint's contrast finding, scrollbar and select-list colours, stand accent colour, zoom and scheme; nothing painted changes for a document allowed to load.
@task_item n=4 part=not_needed
Screen reader support: the snapshot, its text form and diff, accessible names, the actions answer, driver screenshots and refusal causes, the AccessKit adapter; the driver and the stand are untouched.
@task_item n=5 part=not_needed
Cognitive accessibility: invalid-state presentation and the stand's shared tab-order table.
@task_item n=6 part=not_needed
A11y scope summary rows on the accessibility feature, AccessKit, the platform adapter and the accessibility tier: no feature flag or adapter wiring changes.
@task_item n=7 part=not_needed
UI graph state registration and the Dioxus document's reached-handler record: the Dioxus embedder's own asset scheme and its documents' handlers are not narrowed or changed.
