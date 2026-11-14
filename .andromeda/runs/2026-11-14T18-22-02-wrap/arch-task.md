@task_prose part=session
The session closes the last gap of the file URL rule in the browser app, upstream's reference app: a remote page can no longer reach a local file through its favicon or through a navigation it starts.
The favicon candidate is now kept only where document_may_load allows it, so a non-file page names no file: favicon and neither the favicon probe nor the shared favicon cache is reached for one; a file page's file: favicon and a remote page's own /favicon.ico are unchanged.
The browser's navigation provider is now built per document by a constructor that takes the document's URL (an absent or unparsable base reads as no URL, never a panic); a navigation a non-file document starts to a file: URL is silently dropped, loading and logging nothing, as Chromium and Firefox drop it; any other navigation pushes to history as before. The error page's document has no URL and reads as no file document.
The user's own loads (URL bar, history, home, the about pages, the first tab) pass no such gate. No engine crate, dependency, log site, span, env read, carrier, verb or session error was added; six browser unit tests were added (29 to 35).
The capability statement loses its two provisional, pending-the-founder clauses, and the seed absence on the driver's carriers moves from provisional to standing on the founder's own word.
@task_file n=1 path=apps/browser/src/favicon.rs
what: the browser app's favicon candidate resolution (which icon URL a page's favicon is fetched from) and its unit test module, which allows expect with no justification beside it
@task_file n=2 path=apps/browser/src/history.rs
what: the browser app's navigation provider (receives link-click navigations and pushes them to the browser's in-memory history) and its new unit test module
@task_file n=3 path=apps/browser/src/document_loader.rs
what: the browser app's document loader, which builds each loaded document's DocumentConfig with its base URL, navigation provider and net provider
@task_file n=4 path=docs/capabilities.md
what: the capability statement, escher's own document beside upstream's docs, held to the driver schema by a CI script test
@task_item n=1 part=touches
decision/file-url-load-rule (File URLs only for file documents): in the browser app a navigation a non-file document starts is no top-level load; such a navigation to a file: URL is dropped, and a non-file page's file: favicon candidate is refused, both by document_may_load.
@task_item n=2 part=touches
decision/sub-resource-requests-carry-initiator: its wording a navigation asked for now stands only for the user's own loads in the browser app; a document-started navigation is checked against that document's URL.
@task_item n=3 part=touches
decision/shell-and-navigation-providers: the browser app's navigation provider now carries the URL of the document it serves and refuses some navigations silently before pushing.
@task_item n=4 part=touches
decision/no-seed-on-carriers: its state moves provisional to standing and its why clause, which says the absence stands provisional until the founder confirms it, is restated on the founder's own word.
@task_item n=5 part=touches
decision/justified-expect-unwrap: its except names the browser app's test module allowing clippy::expect_used with no justification; the app now holds two such test modules, the favicon one without a justification (the except, with three new tests using expect under it) and a new history one with a one-line justification (inside the rule).
@task_item n=6 part=touches
contract/blitz-vibey-script-api/document-may-load and contract/script/document-may-load: the predicate gains two callers in the browser app (favicon candidate and navigation provider); its API is unchanged.
@task_item n=7 part=touches
decision/file-script-load-refused and decision/url-scheme-routing: the same file: refusal now also covers the browser app's favicon and document-started navigations, beside the script loads and blitz-net's scheme routing.
@task_item n=8 part=touches
decision/favicon-cache-positive-only and decision/favicon-success-by-decoding: the favicon probe and cache are no longer reached for a file: favicon of a non-file page; a refused candidate is no favicon at all.
@task_item n=9 part=touches
decision/link-load-start and decision/browser-load-aborts-previous: a link click from a remote page to a file: URL now starts no load and so aborts no previous load.
@task_item n=10 part=touches
decision/unresolvable-url-skipped: the navigation provider's constructor parses the document URL leniently, an unparsable base reading as no URL, never a panic.
@task_item n=11 part=touches
decision/capability-statement, decision/capabilities-doc-beside-upstream and contract/capability-statement/sections: the capability statement was edited, its two provisional clauses pending the founder's choice removed (2 to 0); sections unchanged.
@task_item n=12 part=touches
decision/one-run-seed and decision/no-prng-crate: rows beside the seed absence whose provisional basis the founder's word now settles; they state no provisional mark themselves.
@task_item n=13 part=touches
contract/blitz-traits-net/request-initiator and decision/blitz-dom-stamped-requests: the initiator field is unchanged; the browser app's new gate reads the document URL it holds, not a request's initiator.
@task_item n=14 part=touches
decision/history-service-single-entry and contract/browser-history/history-store: document-started navigations still reach history through the same push, except the refused file: ones, which push nothing.
@task_item n=15 part=touches
decision/clippy-allow-scoping, decision/local-lint-allows and decision/unit-tests-at-file-end: a new test module at the end of the navigation provider's file with a locally scoped expect allow.
@task_item n=16 part=touches
decision/spec-url-citations and decision/cite-the-spec: the favicon refusal cites the Fetch scheme-fetch section and names Chromium's behaviour.
@task_item n=17 part=touches
decision/fixture-precondition-first and decision/tests-assert-fixture-first: the six new browser tests assert their document's and target's schemes first.
@task_item n=18 part=touches
decision/tracing-gated-per-call-site and decision/browser-state-best-effort: the two refusals are silent, adding no log site or error.
@task_fact n=1 slug=standard-contracts
fact: The browser app's navigation provider is built per document with that document's URL, and silently drops a navigation a non-file document starts to a file: URL (by document_may_load), loading and logging nothing; a document with no or an unparsable URL, as the error page, reads as no file document; the user's own loads from the URL bar, history, home, the about pages and the first tab pass no such gate.
@task_fact n=2 slug=standard-contracts
fact: The browser app keeps a page's favicon candidate only where document_may_load allows it: a non-file page names no file: favicon, so neither the favicon probe nor the favicon cache is reached for one; a file page's file: favicon and a remote page's own /favicon.ico stand.
@task_item n=1 part=untouched
No engine crate changed: blitz-dom, blitz-net, blitz-traits, blitz-html, blitz-paint, blitz-shell and blitz-vibey-script rows (style, layout, paint, hit testing, events, select lists, accessibility, script, networking) stand as they were.
@task_item n=2 part=untouched
The escher driver, session host, escher-session command line, MCP role, session socket wire, kept checks, recording directory, verbs, refusal causes, session errors and exploration rows: nothing on the carriers moved.
@task_item n=3 part=untouched
Telemetry and logging rows: no span, log field, env read or subscriber change was added.
@task_item n=4 part=untouched
Stack and dependency rows, CI legs, gates, coverage floors, stand and cold-agent rows: no dependency, workflow, leg or threshold changed.
@task_item n=5 part=untouched
The browser's persistent history storage (SQLite, migrations, the entry cap, visit folding), bookmarks, about pages, tabs, the URL bar, search input and View Source: unchanged.
@task_item n=6 part=untouched
Names: no binary, crate, server or other name was taken or freed.
@task_item n=7 part=untouched
Absences: no absence was added or retired; the WPT runner, rdme, the 7GUIs stand and the examples were not reached.
