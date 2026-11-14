@task_prose part=session
The browser reference app now refuses local files to remote pages: a non-file page's favicon candidate that resolves to a file URL is dropped, so no favicon probe or cache is reached for it.
A navigation a non-file document starts toward a file URL (a link click through the navigation provider) is silently dropped, loading nothing and logging nothing; any other navigation pushes onto history as before.
Both refusals use the engine's existing document-may-load predicate; a file page keeps its file favicon and file navigations, and a remote page keeps its own default favicon.
The user's own loads (URL bar, history, home, about pages, first tab) are untouched; no engine crate, dependency, log field, colour, font or UI element changed.
The capability statement lost its two clauses marked provisional pending the founder's choice; browser unit tests rose from 29 to 35.
The Epoch 4 boundary batch discharges the seed row's provisional mark and, at consolidation, words ratified the delegate confirms on design-system's Self-Validation Protocol and cli Component Patterns entries.
@task_file n=1 path=apps/browser/src/favicon.rs
what: the browser app's favicon reader, choosing the favicon from the first icon link of a page and probing it through the shared favicon cache
@task_file n=2 path=apps/browser/src/history.rs
what: the browser app's navigation provider and its history store, which receives link-click navigations a document starts
@task_file n=3 path=apps/browser/src/document_loader.rs
what: the browser app's document loader, building each loaded document's configuration (base URL, navigation provider, the error page's config)
@task_file n=4 path=docs/capabilities.md
what: the project's capability statement, the list of what escher does and does not do
@task_item n=1 part=touches
decision/favicon-from-first-icon-link: which href is read is unchanged, but the resolved candidate is now dropped where it is a file URL on a non-file page; a remote page's default /favicon.ico and a file page's file favicon stand as before.
@task_item n=2 part=touches
Iconography as a whole: the favicon a remote page shows can now be none where its first icon link names a local file, a case no row states.
@task_item n=3 part=touches
Any surface decision on the browser app's navigation provider or link-click navigation: a non-file document's navigation to a file URL now loads nothing and gives no feedback (no focus move, no error page, no message).
@task_item n=4 part=touches
Self-Validation Protocol and Surface cli Component Patterns: the overseer's delegate confirms on these entries (the graph subjects list, the coverage values) are worded ratified on the founder's own word at this epoch's consolidation; the row text itself does not move.
@task_item n=5 part=touches
Any row citing the capability statement's provisional or pending-the-founder clauses: those two clauses are gone (2 to 0).
@task_item n=6 part=touches
Any row that states a refusal is silent (no log, no error, no visible signal): the two new refusals follow that convention and add no log site.
@task_fact n=1 slug=iconography
fact: A page that is not itself a file document never shows a favicon from a file URL; its icon-link candidate naming a local file is dropped, while its default /favicon.ico and a file page's local favicon are kept.
@task_item n=1 part=untouched
Color palette, typography, spacing, depth strategy and border radius: no colour, font, spacing, paint order, hit-testing or radius changed.
@task_item n=2 part=untouched
Motion: no animation, scroll behaviour or virtual clock changed.
@task_item n=3 part=untouched
The select list rows (its overlay, events, colours, accessibility, screenshot and snapshot bounds): untouched.
@task_item n=4 part=untouched
The escher-session commands, exit statuses and codes, stdout and stderr lines, mcp role, tasks, actions, coverage and UI graph lines: no verb, argument, exit or output changed.
@task_item n=5 part=untouched
The WPT runner output and result colours, the JS window object, the shell provider, cursor icons and cursor resolution, the dioxus-native config items: untouched.
@task_item n=6 part=untouched
The web-spa surface and the WASM font context: no WASM or canvas path changed.
@task_item n=7 part=untouched
The engine's own file-URL load rule and its predicate: the predicate is reused unchanged; no engine crate moved.
