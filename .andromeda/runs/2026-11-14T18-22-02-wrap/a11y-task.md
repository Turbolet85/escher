@task_prose part=session
The session gated local files in upstream's browser reference app: a remote page's favicon candidate never names a file: URL, and a navigation a non-file: document starts toward a file: URL is dropped silently, loading nothing and logging nothing.
Both refusals ride the engine's existing document_may_load predicate; a file: page keeps its file: favicon and its file: navigations, and a remote page keeps its own /favicon.ico and its https navigations.
The browser app's navigation provider gained a constructor taking the document's URL; the user's own loads (URL bar, history, home, about pages, first tab) are untouched.
The capability statement lost its two provisional, pending-the-founder clauses; its hand-written reach is otherwise unchanged.
Six browser unit tests were added (29 to 35); no engine crate, dependency, feature, log field or UI element changed, and no accessibility work was done (a11y n/a in the report's coverage).
@task_file n=1 path=apps/browser/src/favicon.rs
what: the browser reference app's favicon chooser, picking the icon candidate a page's first icon link or default /favicon.ico names, with its unit tests
@task_file n=2 path=apps/browser/src/history.rs
what: the browser reference app's navigation provider and history store, which receives link-click navigations and pushes them, with its unit tests
@task_file n=3 path=apps/browser/src/document_loader.rs
what: the browser reference app's document loader, which builds each loaded document's config (base URL, navigation provider) and its error page
@task_file n=4 path=docs/capabilities.md
what: the capability statement, the hand-edited list of what escher can and cannot do, including its accessibility reach
@task_item n=1 part=touches
decision/capability-statement-reach-by-hand: the capability statement was edited by hand this session (two provisional clauses removed, a few lines added on the browser app's local-file refusals); its stated accessibility reach is not changed, so check the row still reads true.
@task_item n=2 part=touches
decision/browser-accessibility-off-by-default: the browser reference app changed (favicon chooser, navigation provider, document loader); its features and dependencies did not, so the accessibility feature stays off by default there.
@task_item n=3 part=touches
decision/accessibility-feature-default-on and decision/dioxus-accessibility-feature-by-name: indirectly, through the browser app's Dioxus-backed navigation provider; no feature name or default moved.
@task_item n=4 part=touches
Rows on what a navigation does to focus or the tree (keyboard-navigation overview's route-change focus restoration, decision/removed-focused-node-drops-focus): indirectly, a refused link-click navigation to a file: URL now loads nothing, so the current document, its focus and its accessibility tree stay as they were.
@task_item n=5 part=touches
Rows that state a count of the browser app's tests or a test list of the a11y legs (decision/a11y-test-leg, decision/a11y-ci-gate): the browser crate's unit tests moved 29 to 35, none of them an accessibility test; a touch only where a row's cell holds that count.
@task_item n=6 part=touches
absence/assistive-technology-reach and decision/stand-builds-no-platform-adapter: the session adds no platform adapter and no binary; the browser app's change leaves assistive technology reach as stated.
@task_item n=1 part=untouched
The accessibility tree, its states, hidden content and role mapping contracts: no engine crate or accessibility tree builder changed.
@task_item n=2 part=untouched
ARIA name and property reads and accessible name precedence: no attribute read changed.
@task_item n=3 part=untouched
Keyboard contracts for select, range input, CRUD list and Cells grid, keyboard activation and Tab order: no input handling changed.
@task_item n=4 part=untouched
Contrast, visual lint findings, scrollbar, select list colours and stand accent rows: no style, colour or lint changed.
@task_item n=5 part=untouched
Snapshot, actions answer, screenshot, driver refusal, MCP result, UI graph state and handler coverage contracts: the driver session and its carriers did not change.
@task_item n=6 part=untouched
Stand checks, stand expectation tables and the three-round repetition: no stand app or harness changed.
@task_item n=7 part=untouched
Cognitive accessibility and the invalid-state rows: nothing in form validation changed.
