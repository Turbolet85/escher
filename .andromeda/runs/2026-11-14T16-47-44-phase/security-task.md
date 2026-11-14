@task_prose part=chunk
The reference browser app (apps/browser, the blitz binary) closes the two paths by which a remote page still makes the browser read a local file.
The favicon path: a favicon candidate a non-file document names under a file: URL is never chosen, so no probe starts and no shared favicon cache entry is served; a file document's favicon still loads.
The navigation path: a link click, keyboard activation or form submission that a non-file document starts toward a file: URL loads nothing; one a file document starts still loads, and the user's own top-level loads are unchanged.
Both gates reuse the engine's existing public document_may_load rule as it stands; no engine crate is edited.
It also drops two stale "provisional, pending the founder's choice" clauses from escher's committed capability statement.
@task_file n=1 path=apps/browser/src/favicon.rs mark=edits
what: the browser app's favicon probe — choosing a page's favicon candidate (its link rel=icon href, else /favicon.ico on its base URL), fetching it, and the favicon cache shared across tabs — edits: the favicon_candidate function, its existing unit test module
@task_file n=2 path=apps/browser/src/history.rs mark=edits
what: the browser app's navigation provider, BrowserNavProvider, through which top-document link clicks and form submissions navigate, beside the tab's history store — edits: BrowserNavProvider and its navigate_to, a new test module at the end of the file
@task_file n=3 path=apps/browser/src/document_loader.rs mark=edits
what: the browser app's document loader, which builds each document's config and its navigation provider and picks the favicon candidate once per load — edits: make_doc_config, the one construction site of BrowserNavProvider, handing it that document's URL
@task_file n=4 path=docs/capabilities.md mark=edits
what: escher's committed capability statement — edits: the provisional clause on the fixture-state absence, the provisional clause on exploration riding no carrier and following no navigation
@task_item n=1 part=needs
The rule that a file: URL loads only for a file document now binds the app's own favicon candidate choice and the app's own top-document navigations: each is checked with document_may_load(loading document URL, target) before anything starts.
@task_item n=2 part=needs
The engine's sub-resource rule and its FileForbidden refusal are used as they stand, neither widened nor narrowed; the app reads the public blitz_traits::net::document_may_load and no engine crate changes.
@task_item n=3 part=needs
Favicon candidate choice: favicon_candidate answers none for a candidate the rule refuses for the page's base URL, so no fetch is started and a file: icon a file page cached is never served to a remote page naming the same URL; a file document's favicon still loads.
@task_item n=4 part=needs
The favicon fetch itself goes out through blitz-net's Request::get with no initiator, read as a top-level load; the chunk gates before that fetch rather than relying on the fetch to refuse.
@task_item n=5 part=needs
In-document navigation from a non-file document to a file: URL is refused silently and loads nothing; the navigating document's URL is taken where its navigation provider is built, not from the tab's current history entry, so a remote page still shown while a typed file page loads cannot reach a file: URL.
@task_item n=6 part=needs
A document with no URL, the browser's error page, counts as no file document for the navigation rule; error pages already skip history and favicon probing.
@task_item n=7 part=needs
Link clicks, keyboard-activated links and submits, and form submissions of the top document all reach navigate_to; the refusal sits ahead of the existing external-target and form-submission scheme and method handling, which stays as it is.
@task_item n=8 part=needs
The user's own top-level loads — the URL bar, history, home, the new-tab and history pages, the initial URL — still load whatever they name, file: included.
@task_item n=9 part=needs
No carrier, verb, argument, cause, session error, exit, log field, span, env read, listener or dependency is added: a refused favicon or navigation is a filter or an early return, reaching no error path and no tracing call.
@task_item n=10 part=needs
Each new check carries a control, the mutation or hand edit that turns it red and the reading that it did; the favicon gate is unit-tested as the pure favicon_candidate, the navigation gate by calling navigate_to on a History store built inside a VirtualDom, with no disk read, temp file or TMPDIR read, so the three CI OSes read them alike in the workspace test leg.
@task_item n=11 part=needs
The capability statement loses only its two provisional clauses; the browser's two closed paths are not added to it, since the browser app is upstream's and its loading-pages limit speaks of the engine's rule, which this chunk does not move.
@task_standing n=1
convention: each new test and test module adds no dependency or dev-dependency and builds under the locked build, the audit and the licence gates as they stand
home: dependency-security
@task_standing n=2
convention: each new test installs no tracing subscriber and prints no page content, URL or path to a log
home: logging-monitoring
@task_item n=1 part=not_needed
The driver session, its Unix socket, the MCP server and the escher-session command line — their verbs, flags, exit codes, refusal causes, session errors and reply contracts: the driver, the stand and the seven_guis app are untouched.
@task_item n=2 part=not_needed
Snapshot, accessibility, stable element id, masked control values, select list and other control-state readings: the chunk changes no engine reader and no carrier.
@task_item n=3 part=not_needed
Recorded checks, kept check files, cold-agent runs, coverage floors and the coverage answer: no recording, replay or coverage path is touched.
@task_item n=4 part=not_needed
Script fetcher scheme limits, JS fetch method limits and the same-origin and CORS absences: same-origin, CORS and other schemes such as data: and about: are out of scope, and script fetches are not touched.
@task_item n=5 part=not_needed
Iframe navigation routing, iframe and import nesting limits and stale iframe responses: an iframe's navigation already goes through the engine's own provider and stamping constructor, out of this chunk's scope.
@task_item n=6 part=not_needed
CI tokens, CI secrets, signing material, publish and release jobs and the WPT workflows: no CI job, workflow or secret is added or changed.
@task_item n=7 part=not_needed
Engine parse, style, layout and paint rules (CSS, HTML parsing, presentational hints, transforms, serialization): the chunk edits no engine crate.
