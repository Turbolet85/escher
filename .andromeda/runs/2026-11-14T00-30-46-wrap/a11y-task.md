@task_prose part=session
The network layer now refuses to read a file: URL for a request whose loading document is not itself a file: document; a request with no loading document (an embedder's top-level load) or a file: one reads the file as before.
Every engine sub-resource request now carries the URL of the document that asked for it: linked stylesheet, its reload, @import, @font-face, CSS image, img and iframe — seven paths, 0 before.
The refusal is a new fixed-text network error with no URL or path in it, reaching the document's existing error path with no new log site.
The engine crate now builds a request only through one stamping constructor, enforced by a crate clippy config that disallows the plain request constructor.
A new blitz-tests integration target proves a remote page reads no local stylesheet, import, image or iframe while the same page as a file: document reads all four, in both layout modes; nothing was retired.
@task_file n=1 path=packages/blitz-traits/src/net.rs
what: the shared network request type and the file-URL load predicate (whether a document may load a URL), now carrying the loading document as the request's initiator
@task_file n=2 path=packages/blitz-traits/src/navigation.rs
what: navigation options turned into a top-level request, with no loading document
@task_file n=3 path=packages/blitz-vibey-script/src/fetch.rs
what: the script runtime's fetch, which re-exports the file-URL load predicate for script loads
@task_file n=4 path=packages/blitz-net/src/lib.rs
what: the network provider that fetches http and file: URLs and reports typed provider errors, now refusing a file: read for a non-file document
@task_file n=5 path=packages/blitz-dom/src/net.rs
what: the engine's sub-resource loaders for stylesheets, @import and @font-face fonts, and the one request constructor that stamps the loading document
@task_file n=6 path=packages/blitz-dom/src/document.rs
what: the engine document, which builds image and stylesheet requests and adds stylesheets for nodes
@task_file n=7 path=packages/blitz-dom/src/iframe.rs
what: the iframe sub-document load
@task_file n=8 path=packages/blitz-dom/src/mutator.rs
what: the document mutator, which constructs the stylesheet handler for a linked sheet
@task_file n=9 path=packages/blitz-dom/src/cssom.rs
what: the CSS object model, inserting rules such as @font-face and building the stylesheet loader
@task_file n=10 path=packages/blitz-dom/src/layout/damage.rs
what: the layout damage pass that loads CSS background images named by computed styles
@task_file n=11 path=packages/blitz-dom/clippy.toml
what: the engine crate's lint configuration, disallowing the plain request constructor
@task_file n=12 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: the integration check that a remote page reads no local file and a file: page reads them all, and that every engine request carries its document
@task_file n=13 path=tests/blitz-tests/Cargo.toml
what: the blitz-tests crate manifest, its dev-dependencies now including the network provider and the async runtime
@task_file n=14 path=Cargo.lock
what: the workspace lockfile, moved only in blitz-tests' own dependency list
@task_item n=1 part=touches
Rows that rely on images loading for a document: on a remote (non-file) document an img or CSS background image with a file: URL now stays unloaded; a file: document loads it as before.
@task_item n=2 part=touches
Rows over iframe or sub-document content reaching the accessibility tree or the snapshot: a remote document's file: iframe now gets no sub-document, so no nodes from it.
@task_item n=3 part=touches
Rows over styles read from linked or imported stylesheets and loaded fonts (computed-style contrast measurement, text shaping): on a remote document file: stylesheets, @import and @font-face fonts are refused and their styles never apply; file: stand pages are unaffected.
@task_item n=4 part=touches
Rows over the CI test legs and gates that run every blitz-tests target: the workspace test leg gains one auto-discovered target, CI read green 17 of 17 checks; the target is not in the stand-repeat list of named targets, whose count is unchanged.
@task_item n=5 part=touches
Rows over the engine's network error path or what a failed load leaves in the document: a refused file: load arrives as an ordinary load error through the existing handler, with no log line of its own.
@task_item n=1 part=untouched
Role mapping, ARIA property and state reads, accessible-name precedence and the accessibility tree's construction: no rule or reader of them changed.
@task_item n=2 part=untouched
Keyboard navigation, focus, Tab order, click and activation rules, range input and select-list keys and events: no input or focus path changed.
@task_item n=3 part=untouched
The snapshot model, its text form, the actions answer, driver refusals and the UI graph: no field or reader changed.
@task_item n=4 part=untouched
Contrast thresholds, the WCAG ratio helper, the lint's contrast finding and stand colours: no value or formula moved.
@task_item n=5 part=untouched
The accessibility feature defaults, the platform adapter choice and accessibility tier: no feature flag or adapter changed.
@task_item n=6 part=untouched
Cognitive accessibility rows (invalid state, shared tab-order table): untouched.
