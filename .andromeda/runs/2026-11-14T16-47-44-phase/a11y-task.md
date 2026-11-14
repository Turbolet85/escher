@task_prose part=chunk
The reference browser app closes the two paths by which a remote page still makes the browser read a local file.
Its favicon candidate is answered as none when the engine's document_may_load rule refuses the candidate for the page's base URL, so the probe is never started and no cached file: icon is served to a remote page.
A link click or form submission from a non-file: document to a file: URL loads nothing; a file: document's navigation and the user's own top-level loads are unchanged.
The capability statement drops two stale provisional clauses the founder has since ruled on.
@task_file n=1 path=apps/browser/src/favicon.rs mark=edits
what: the browser app's favicon candidate choice and favicon probe, with its unit test module — edits: the function favicon_candidate, its existing test module
@task_file n=2 path=apps/browser/src/history.rs mark=edits
what: the browser app's navigation provider over the tab's history store — edits: BrowserNavProvider and its navigate_to, a new test module at the file's end
@task_file n=3 path=apps/browser/src/document_loader.rs mark=edits
what: the browser app's per-document loader that builds each document's config and starts its favicon probe — edits: make_doc_config (the navigation provider's one construction site), the favicon_candidate call
@task_file n=4 path=docs/capabilities.md mark=edits
what: escher's own capability statement, its limits and absences written by hand — edits: the provisional clause on the fixture-state absence, the provisional clause on exploration on no carrier and following no navigation
@task_item n=1 part=needs
The browser app's favicon candidate: a candidate a non-file: document names under file: is answered as none, the probe never started; a file: document's favicon still loads.
@task_item n=2 part=needs
The browser app's navigation provider: a navigation a non-file: document starts to a file: URL is refused silently with an early return, loading nothing; a document with no URL, such as the error page, counts as no file: document.
@task_item n=3 part=needs
The navigation provider takes its document's URL where it is built per document, never the tab's current history entry.
@task_item n=4 part=needs
Indirect: both gates call the engine's public document_may_load rule as it stands, neither widened nor narrowed; no engine crate is edited.
@task_item n=5 part=needs
The user's own top-level loads (the URL bar, history, home, the new-tab and history pages, the initial URL) stay unchanged.
@task_item n=6 part=needs
No carrier, verb, argument, cause, session error, exit, log field, span, env read, listener or dependency is added; a refused favicon or navigation reaches no error path and no log line.
@task_item n=7 part=needs
The capability statement, edited by hand: two provisional clauses are dropped and nothing about the browser's closed paths is added.
@task_item n=8 part=needs
New tests: a pure unit test of the favicon candidate and a navigation test built on a history store inside a virtual DOM, each with the mutation or hand edit that turns it red, run in the workspace test leg with no disk read.
@task_item n=9 part=needs
Link clicks, keyboard-activated links and submits, and form submissions of the top document all reach the navigation provider, so the one gate covers them.
@task_item n=10 part=needs
The decision log and route records of this Epoch 4 boundary chunk carry no new accessibility claim: no 0.3.0 capability names this entry.
@task_item n=1 part=not_needed
The accessibility tree, its roles, states, names and hidden content: the chunk builds no node and reads no tree.
@task_item n=2 part=not_needed
The snapshot, the snapshot text form, the snapshot diff, the UI graph and the actions answer: the driver is untouched.
@task_item n=3 part=not_needed
The seven_guis Cells grid and CRUD list contracts and the stand's contrast, tab-order and lint checks: the stand and the seven_guis app are untouched.
@task_item n=4 part=not_needed
Select, range input and checkable control behaviour, their key rules and list colours: no form control changes.
@task_item n=5 part=not_needed
Visual design verification: contrast thresholds, the lint's contrast finding, scrollbar and zoom rules; nothing painted changes.
@task_item n=6 part=not_needed
AccessKit adapter choice, the platform adapter on stand binaries and the browser's accessibility feature default: no feature or dependency moves.
@task_item n=7 part=not_needed
The a11y assertion harness, its CI leg and its three-round stand repetition: the new tests are no accessibility assertions.
@task_item n=8 part=not_needed
Cognitive accessibility: invalid-state presentation and stand focus-order tables.
