@task_prose part=session
The session gated the browser app (upstream's reference app, a binary) so a remote page reads no local file: a non-file: page's favicon candidate never names a file: URL, and a navigation a non-file: document starts toward a file: URL is silently dropped, as Chromium and Firefox drop it.
The browser app's navigation provider gained a constructor that takes the document's base URL (an absent or unparsable one reads as no URL), and the document config builder now builds the provider through it; the error page's config reads as no file: document.
The user's own loads (URL bar, history, home, the about pages, the first tab) pass no gate and are unchanged; no engine crate, dependency, log field, env read, exit or UI element changed.
Six unit tests were added to the browser app (29 to 35); the capability statement lost its two clauses pending the founder's choice (2 to 0).
At the Epoch 4 close the provisional delegate confirms in the masters' history (among them layout-templates' cli escher-session entry and the desktop-native Home and TaskShell screens) are worded ratified on the founder's own recorded word; no row's text moves for them.
@task_file n=1 path=apps/browser/src/favicon.rs
what: the browser app's favicon lookup: picks the tab's favicon candidate from the page's first icon link or its default /favicon.ico, and probes and caches it
@task_file n=2 path=apps/browser/src/history.rs
what: the browser app's history store and its navigation provider, which receives a document's link-click navigations and pushes them onto the tab's history
@task_file n=3 path=apps/browser/src/document_loader.rs
what: the browser app's document loader: builds each loaded document's config (base URL, network, shell and navigation providers) and the error page's
@task_file n=4 path=docs/capabilities.md
what: the project's capability statement, the hand-written list of what the engine and its apps can and cannot do
@task_item n=1 part=touches
Any row on the browser app's navigation provider or link-click navigation: a navigation a remote (non-file:) document starts toward a file: URL now loads nothing and logs nothing; a file: document's navigation to a file: URL and any navigation to an http(s) URL push onto the history as before.
@task_item n=2 part=touches
Any row on the browser app's tab chrome showing a page's favicon: a remote page whose icon link names a file: URL now shows no favicon from it; a file: page's local favicon and a remote page's own default favicon are unchanged.
@task_item n=3 part=touches
Any row on how the browser app builds a loaded document's config (its base URL and providers): the navigation provider is now built per document from the base URL, before the config, and the error page's config carries no base URL so its navigations read as from no file: document.
@task_item n=4 part=touches
decision/load-page-only-network and any row on which providers a document is given: the session added no network provider and changed none; it only made the navigation provider document-aware, which a picker should weigh beside that row.
@task_item n=5 part=touches
The desktop-native surface's overview, which says the browser app's window and page layout code is covered by slice s04's facts: the browser app changed this session, but only in what a page may load, not in any window, page layout, chrome or screen.
@task_item n=6 part=touches
Any row on the user's own loads in the browser app (URL bar, history, home, about pages, first tab): these pass no new gate, so a file: URL typed or chosen by the user still loads.
@task_item n=7 part=touches
contract/shell-provider-window-chrome items: the document config that carries the shell provider was edited around them (the navigation provider's line), with the shell provider and its window chrome calls unchanged.
@task_item n=8 part=touches
absence/arbitrary-index-html-templates: the document loader that builds a document was edited; it gained no template or document skeleton choice, so the absence stands.
@task_item n=9 part=touches
Any row citing the capability statement's provisional clauses pending the founder's choice: the statement now holds none (2 to 0).
@task_item n=10 part=touches
The cli escher-session primary-screen entry and the desktop-native Home and TaskShell screens, whose confirm stood in the history as the overseer's provisional delegate confirm: at the Epoch 4 consolidation those entries are worded ratified on the founder's own recorded word; the rows' text is unchanged.
@task_item n=11 part=touches
Any row naming a count of the browser app's unit tests or its test modules: the count moved 29 to 35, with a new history test module and three favicon tests.
@task_item n=1 part=untouched
The stand's stable element id contracts (cells, circle drawer, counter, crud, flight booker, load, task cards, temp converter, timer): no stand control, id or attribute changed.
@task_item n=2 part=untouched
The stand's layout and accessibility decisions (keyboard regions, open list bounds, into-view scrolling, visible region through clipping boxes, accent colour, headless viewport, lints at boot, held instance, session roles): the stand was not touched.
@task_item n=3 part=untouched
The escher-session command line contracts (actions, exit codes, refusals, coverage role, graph, lint, snapshot, screenshot, recording, MCP server, served tasks, host log): no verb, argument, exit, refusal or answer changed.
@task_item n=4 part=untouched
The cold-agent and agent-run script contracts and the cold-agent decisions: no script, prompt, pipe or exit changed.
@task_item n=5 part=untouched
The multi_doc and paint_bench command line contracts: no bench or example binary changed.
@task_item n=6 part=untouched
The mobile-native and web-spa surfaces and the Decisions Log: nothing measured or decided there.
@task_item n=7 part=untouched
Any row on logging, env reads, dependencies or the engine crates: the session added no log site, env read or dependency and changed no engine crate.
