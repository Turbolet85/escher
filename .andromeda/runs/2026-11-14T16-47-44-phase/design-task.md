@task_prose part=chunk
The reference browser app (the blitz binary) closes the two paths by which a remote page still makes it read a local file.
Favicon: the favicon candidate chooser answers no candidate when the engine's document-may-load rule refuses it for the page's base URL, so no probe starts and no shared favicon cache entry is served; a file: page's favicon still loads.
Navigation: the browser's navigation provider takes its document's URL where it is built and silently refuses, loading nothing, a link-click or form navigation that rule refuses; a file: page's navigation and the user's own top-level loads are unchanged.
New unit tests prove both gates, each with a mutation control; the capability statement drops two clauses still marked provisional where the founder has ruled.
@task_file n=1 path=apps/browser/src/favicon.rs mark=edits
what: the browser's favicon component — choosing a page's favicon candidate from its first icon link or /favicon.ico and probing it — edits: favicon_candidate, its existing test module
@task_file n=2 path=apps/browser/src/history.rs mark=edits
what: the browser's history store and its navigation provider for link clicks and form submissions — edits: BrowserNavProvider, navigate_to, a new test module at its end
@task_file n=3 path=apps/browser/src/document_loader.rs mark=edits
what: the browser's document loader, building each document's configuration and its navigation provider and calling the favicon candidate chooser — edits: make_doc_config
@task_file n=4 path=docs/capabilities.md mark=edits
what: escher's capability statement, what escher does and does not do — edits: the provisional clause on the fixture-state absence, the provisional clause on exploration on no carrier and following no navigation
@task_item n=1 part=needs
The favicon read from the first icon link (else /favicon.ico joined to the base URL): the candidate is now also gated, none answered when a non-file: page names a file: candidate; a file: page's favicon still loads.
@task_item n=2 part=needs
The favicon cache shared across tabs, keyed by candidate: the gate sits before it, so a file: icon a file: page cached is never served to a remote page naming the same URL.
@task_item n=3 part=needs
Navigation from a page — link click, keyboard-activated link or submit, form submission of the top document — to a file: URL: refused and loads nothing when a non-file: document starts it; still loads when a file: document starts it.
@task_item n=4 part=needs
A document with no URL (the error page) counts as no file: document for the navigation gate.
@task_item n=5 part=needs
The user's own top-level loads — URL bar, history, home, the new-tab and history pages, the initial URL — are unchanged.
@task_item n=6 part=needs
The refusal is silent: no error path, no log line, no tracing call, no new carrier, verb, argument, cause, session error, exit, log field, span, env read, listener or dependency.
@task_item n=7 part=needs
Indirect: the engine's document-may-load rule and its FileForbidden refusal are used as they stand, through its public net traits function — not widened or narrowed, no engine crate edited.
@task_item n=8 part=needs
Indirect: the navigating document's URL comes from where its provider is built (its base URL), not from the navigation options' source document id nor the tab's current history entry.
@task_item n=9 part=needs
New tests: the favicon gate unit-tested as a pure function; the navigation gate reached by calling navigate_to on a history store built inside a virtual DOM; no disk read, no temp file, run in the workspace test leg alike on the three CI OSes; each new check carries a mutation control and the reading that it turned red.
@task_item n=10 part=needs
Mobile targets built, not tested: the browser app's changed code must still build for the mobile targets (Android prelude included).
@task_item n=11 part=needs
The capability statement: two clauses saying provisional, pending the founder's choice, are dropped — its only change; the browser's closed paths are not added to it.
@task_item n=1 part=not_needed
Color palette, color scheme and engine colour rows: the chunk paints nothing and sets no colour.
@task_item n=2 part=not_needed
Typography and font rows (system fonts, font context, type scale): no text is laid out or measured and no font is loaded by the new tests.
@task_item n=3 part=not_needed
Spacing and depth-strategy rows (margins, z-index, hit-testing, stacking): no layout, paint order or hit-test is touched.
@task_item n=4 part=not_needed
Motion rows in either twin (virtual clock, scroll, animation, select list motion): no timing or scroll is involved.
@task_item n=5 part=not_needed
escher-session command, exit status, stdout, stderr, mcp, tasks, actions, coverage and UI graph rows: the driver and its session are untouched and no exit, error or output is added.
@task_item n=6 part=not_needed
Select list rows (overlay, events, accessibility, snapshot and screenshot of rows, gestures): no select behaviour changes.
@task_item n=7 part=not_needed
WPT runner output and result colour rows: the runner is untouched.
@task_item n=8 part=not_needed
Cursor icon and cursor resolution rows, and shell provider window rows (cursor, maximize, minimize, drag, decorations): neither cursors nor window chrome change.
@task_item n=9 part=not_needed
Web-spa surface (WASM pages in a canvas): the chunk changes the desktop browser app only.
@task_item n=10 part=not_needed
JS window object, MountedData and dioxus-native configuration rows (viewport size, device pixel ratio, alpha mode, base colour): no configuration value or script surface changes.
@task_item n=11 part=not_needed
Same-origin/CORS, other schemes (data:, about:), iframe navigation and the history store's own contents: out of scope, iframes already under the engine's rule.
