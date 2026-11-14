@task_prose part=chunk
The reference browser app (the blitz binary, apps/browser) closes the two app-side paths by which a remote page still makes the browser read a local file.
Favicon: the favicon candidate chooser answers no candidate when the engine's file-URL rule refuses that candidate for the page's base URL, so no probe starts and no cached file: icon is served to a remote page; a file: page's favicon still loads.
Navigation: the browser's navigation provider holds its own document's URL from where the document config builds it, and silently loads nothing for a link click or form submission the file-URL rule refuses; a file: document's navigations and the user's own top-level loads are unchanged.
Folded carry: the capability statement drops its two clauses that still call the founder-ruled fixture-state and exploration absences provisional.
@task_file n=1 path=apps/browser/src/favicon.rs mark=edits
what: the reference browser's favicon component: the favicon candidate chooser, the favicon probe fetch and the shared favicon cache — edits: favicon_candidate, its existing unit-test module
@task_file n=2 path=apps/browser/src/history.rs mark=edits
what: the reference browser's navigation provider BrowserNavProvider over the tab's in-memory History — edits: BrowserNavProvider (the document URL it holds), BrowserNavProvider::navigate_to, a new unit-test module at the file's end
@task_file n=3 path=apps/browser/src/document_loader.rs mark=edits
what: the reference browser's document loader, which builds each document's config and its navigation provider and asks for its favicon candidate — edits: make_doc_config
@task_file n=4 path=docs/capabilities.md mark=edits
what: escher's capability statement — edits: the provisional clause of the fixture-state absence, the provisional clause of the exploration-on-no-carrier and no-navigation-following absences
@task_item n=1 part=needs
Favicon gate: the favicon candidate chooser answers none for a candidate (the page's link rel=icon href, else /favicon.ico joined to its base URL) that the engine's document_may_load refuses for that base URL, so the probe is never started; a file: document's favicon candidate is still chosen and loads.
@task_item n=2 part=needs
The favicon cache is keyed by candidate and shared across tabs and stores positive results only: gating at the candidate chooser, before the cache is reached, keeps a file: icon a file: page cached from being served to a remote page naming the same URL.
@task_item n=3 part=needs
Navigation gate: the browser navigation provider, built once per document by the document config builder, takes that document's URL there and refuses, loading nothing, a navigation document_may_load(document URL, target) refuses; a document with no URL (the error page) counts as no file: document. The navigating document is not read from the navigation options' source document id nor from the tab's current history entry.
@task_item n=4 part=needs
The navigation gate binds only document-started navigations that reach navigate_to (link clicks, keyboard-activated links and submits, form submissions of the top document); the user's own top-level loads (URL bar, history, home, new-tab and history pages, initial URL) keep going out as top-level requests with no initiator.
@task_item n=5 part=needs
The engine's file-URL rule (file URLs only for file documents, refused FileForbidden before any disk read) and the public blitz-traits document_may_load are used as they stand, neither widened nor narrowed; the request initiator meaning and the engine's sub-resource initiator stamping are not changed and no engine crate is edited.
@task_item n=6 part=needs
A refused favicon or navigation is simply not started: a filter and an early return, with no new error type, cause, log line, tracing call, span, env read, listener or dependency.
@task_item n=7 part=needs
Behave like a browser and cite the spec: the silent refusal follows Chromium and Firefox, and the Fetch standard's scheme-fetch (which leaves file: to the implementation) is cited at the implementation.
@task_item n=8 part=needs
Favicon check: a unit test of the pure favicon candidate chooser in the favicon component's existing test module, covering a remote page naming a file: candidate (none) and a file: page's file: candidate (kept); no network provider, temp file or TMPDIR read.
@task_item n=9 part=needs
Navigation check: a new unit-test module at the end of the navigation provider's file calls navigate_to on a History store built inside a VirtualDom, the way the tab store lens test builds one, covering a remote document navigating to a file: URL (nothing loads) and a file: document doing so (it loads).
@task_item n=10 part=needs
Both new checks run in the workspace test leg with no disk read, so the three CI operating systems read them alike.
@task_item n=11 part=needs
Every new check carries a control: the mutation or hand edit that turns it red, and the reading that it did.
@task_item n=12 part=needs
The capability statement's only change is dropping the two provisional clauses (the fixture-state absence, and exploration on no carrier and following no navigation); the browser's two closed paths are not added to it and its Loading pages limit, which speaks of the engine, is unchanged.
@task_standing n=1
convention: unit tests sit in a test module at the end of the source file they test
home: conventions
@task_standing n=2
convention: tests carry descriptive names and assert their fixture's precondition first
home: conventions
@task_standing n=3
convention: expect and unwrap in new code and tests carry a justification
home: conventions
@task_standing n=4
convention: new and changed items carry docs, with the spec URL cited at the implementation
home: conventions
@task_standing n=5
convention: std Result is spelled out where the Dioxus prelude glob is in scope
home: conventions
@task_standing n=6
convention: every change passes the format, clippy and rustdoc-warnings gates
home: inherited-defaults
@task_item n=1 part=not_needed
The driver, its session, verbs, carriers, MCP role, session socket, kept checks, recordings, refusal causes, exploration and seeded randomness: the scope leaves the driver untouched.
@task_item n=2 part=not_needed
The seven_guis stand, the headless stand, the cold-agent and agent-run scripts and their verdicts: untouched by this chunk.
@task_item n=3 part=not_needed
The engine's style, layout, paint, hit-testing, select list, accessibility tree and DOM event dispatch rows: the pointer and form paths that reach the navigation provider are used as they stand.
@task_item n=4 part=not_needed
The browser's persistent history (SQLite store, history entries table, migrations, visit folding, entry cap, HistoryService): not in scope.
@task_item n=5 part=not_needed
Iframes (nesting cap, iframe navigation events, the engine's iframe navigation provider): already under the engine's rule and not in scope.
@task_item n=6 part=not_needed
Script fetching, same-origin and CORS checks, and data: and about: schemes: not in scope.
@task_item n=7 part=not_needed
The WPT runner, its outputs, result codes, diff tooling and CI artifacts: not reached.
@task_item n=8 part=not_needed
Upstream sync, cherry-pick and take rows: the chunk takes nothing from another tree.
@task_item n=9 part=not_needed
Dependency versioning and bumps (Stylo, Taffy, Dioxus, toolchain, lockstep versions): no dependency or version changes.
