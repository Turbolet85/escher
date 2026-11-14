@task_prose part=session
The engine's network layer now refuses a file: URL for a request whose loading document is not itself a file: document, so a remote page's sub-resource fetch reads no local file.
Every request the engine builds for a sub-resource now carries the URL of its loading document: linked stylesheet, its reload, @import, @font-face, CSS image, img and iframe (7 paths, from 0).
A top-level load with no loading document, and any load from a file: document, still reads the file as before.
The one predicate that decides whether a document may load a URL moved to the shared network traits crate and is re-exported where the script runtime held it, unchanged in text.
A crate-level lint config now forbids building a request in the DOM crate other than through its one stamping constructor; a new integration test witnesses the refusal and the stamp in both layout modes.
The browser's favicon probe and file: navigation from a remote page stay open, owed to a provisional route entry; nothing visual, no colour, font scale, spacing or motion moved.
@task_file n=1 path=packages/blitz-dom/clippy.toml
what: the DOM crate's lint configuration: forbids the bare request constructor so every sub-resource request is stamped with its loading document
@task_file n=2 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: the integration check that a remote page reads no local stylesheet, import, image or iframe while a file: page reads them all, and that every engine request carries its document as initiator
@task_file n=3 path=packages/blitz-traits/src/net.rs
what: the shared network request type and its initiator field, and the document-may-load predicate deciding which URLs a document may load
@task_file n=4 path=packages/blitz-traits/src/navigation.rs
what: the navigation options that build a top-level request, with no loading document
@task_file n=5 path=packages/blitz-vibey-script/src/fetch.rs
what: the script runtime's fetch and script-load path, which re-exports the document-may-load predicate
@task_file n=6 path=packages/blitz-net/src/lib.rs
what: the blitz-net network provider: URL scheme routing, the file: branch reading from disk, and its typed provider errors
@task_file n=7 path=packages/blitz-dom/src/net.rs
what: the DOM's resource loading: stylesheet, import and font-face fetches and the stylesheet handler and loader
@task_file n=8 path=packages/blitz-dom/src/document.rs
what: the base document: building resource requests, linked stylesheets, images and font faces for its nodes
@task_file n=9 path=packages/blitz-dom/src/iframe.rs
what: the iframe load path, starting a child document's load from its parent
@task_file n=10 path=packages/blitz-dom/src/mutator.rs
what: the DOM mutator, which constructs the stylesheet handler when a link element is added
@task_file n=11 path=packages/blitz-dom/src/cssom.rs
what: the CSSOM, inserting rules including @font-face and building stylesheet loaders
@task_file n=12 path=packages/blitz-dom/src/layout/damage.rs
what: style damage handling, which fetches CSS background image layers from style
@task_file n=13 path=tests/blitz-tests/Cargo.toml
what: the blitz-tests crate manifest and its dev-dependencies
@task_file n=14 path=Cargo.lock
what: the workspace lockfile, the resolved dependency graph
@task_item n=1 part=touches
decision/favicon-from-first-icon-link: the favicon is still read from the first icon link with no loading document stamped, so a remote page's file: icon is still read from disk for the tab strip; this session leaves that path open (owed to a provisional route entry), so a row stating the favicon read may need that limit beside it.
@task_item n=2 part=touches
decision/unavailable-families-use-default-font: an @font-face whose source is a file: URL on a non-file: page is now refused at the network layer, so that family is unavailable and falls back to the default font; the rule over font fallback now also governs refused local font loads.
@task_item n=3 part=touches
contract/script-runtime/document-fonts: font faces fetched for a remote document from file: URLs now fail to load, which is what document.fonts would observe for those faces; check the contract makes no promise that a declared face loads.
@task_item n=4 part=touches
contract/escher-session-tasks/load: a page loaded through the session from a non-file: URL now gets no local file: stylesheet, import, image, font or iframe; a file: page loads them all as before.
@task_item n=5 part=touches
contract/escher-session-commands/screenshot and contract/escher-session-commands/snapshot: for a remote page whose sub-resources point at file: URLs, the rendered and snapshotted result now lacks those images, styles and frames; the verbs themselves are unchanged.
@task_item n=6 part=touches
surface-web-spa overview: the file: refusal runs before the wasm32 branch of the network provider's file: arm, so the rule holds on the WASM pages too; no canvas or sizing behaviour changed.
@task_item n=7 part=touches
decision/default-ua-sheet-via-default-config: the user-agent sheet is not fetched and is untouched, but stylesheet loading in the DOM changed (every linked and imported sheet now carries its document's URL); confirm the row names no fetch path.
@task_item n=8 part=touches
decision/color-scheme-change-full-recascade: a recascade reloads applied stylesheets by href, and that reload request is now stamped with the document's URL, so for a remote document a file: sheet stays refused on reload.
@task_item n=1 part=untouched
color-palette: no colour, engine colour constant, select-list colour or base colour moved.
@task_item n=2 part=untouched
spacing, border-radius and depth-strategy: no margin, cellspacing, paint order, stacking or hit-testing rule moved.
@task_item n=3 part=untouched
motion and its twin: no virtual clock, scroll behaviour, scrollbar fade or select-list motion moved.
@task_item n=4 part=untouched
the select list rows on surface (events, accessibility, overlay, gestures, screenshot and snapshot rows of an open list): unchanged.
@task_item n=5 part=untouched
escher-session exit statuses, exit codes, stdout and stderr shapes, mcp role results, the actions, coverage and UI graph lines: no output shape, status or error text moved.
@task_item n=6 part=untouched
WPT runner output and result colours, JS window sizes, shell provider window controls, cursor icons and resolution: unchanged.
@task_item n=7 part=untouched
typography's system-font test builds, bundled WASM font context, lang locale, root-relative units and font element hints: no font configuration moved, only whether a remote page's local font file is fetched.
