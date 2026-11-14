@task_prose part=session
In the browser app (upstream's reference app, a binary), a remote (non-file) page no longer names a file: URL as its favicon candidate, so no local file is probed for it and none enters the shared favicon cache; a file page's file favicon and a remote page's own /favicon.ico are unchanged.
In the browser app, the navigation provider now knows its document's URL and silently drops a non-file document's navigation to a file: URL (nothing loads, nothing is logged), as Chromium and Firefox do; every other navigation pushes to history as before.
Both refusals use the engine's existing document-may-load predicate; an absent or unparsable document URL (the error page's) reads as no file document, never a panic. The user's own loads (URL bar, history, home, about pages, first tab) pass through no gate.
The committed capability statement lost its two clauses marked provisional pending the founder's choice (2 to 0). Six browser unit tests added (29 to 35); no dependency, env read, log site, span, error or exit added; no engine crate changed.
The Epoch 4 boundary batch discharges, on the founder's word, the one master row still marked provisional (an architecture row on seeds on carriers); security-plan sidecar entries once confirmed by a delegate are worded ratified at consolidation, no security-plan row text changing for it.
@task_file n=1 path=apps/browser/src/favicon.rs
what: the browser app's favicon probing: resolves a page's favicon candidate (first icon link or the default /favicon.ico) and probes it into the shared favicon cache; now keeps a candidate only where the document may load it
@task_file n=2 path=apps/browser/src/history.rs
what: the browser app's history and its navigation provider, which receives a document's link-click navigations and pushes them to the browser history; now refuses a non-file document's file: navigation
@task_file n=3 path=apps/browser/src/document_loader.rs
what: the browser app's document loader, which builds each loaded document's config (its base URL, its navigation provider) including the error page's
@task_file n=4 path=docs/capabilities.md
what: the committed capability statement, the hand-written statement of what escher can and cannot do and of its crossings, read by the capability-statement CI check
@task_item n=1 part=touches
decision/file-urls-only-from-file-documents (input-validation): the rule that file URLs load only from file documents now reaches the browser app's favicon candidate and its in-document navigation; its top-level-load clause named no app path, and a navigation a non-file document starts is now no top-level load in the app
@task_item n=2 part=touches
decision/file-url-only-for-file-documents (threat-model-summary): the same claim as a vector; the browser app's favicon probe and link navigation were two remaining paths by which a remote page could reach a local file, now closed
@task_item n=3 part=touches
decision/error-pages-skip-history-and-favicon (error-handling): over the favicon probing and the document config that changed; the error page's config carries no base URL and now also reads as no file document for the navigation gate; the claim stays true
@task_item n=4 part=touches
decision/external-navigation-get-http-mailto and decision/external-navigation-schemes (input-validation): rules over in-document navigation schemes; the navigation provider now drops a file: target from a non-file document before any push or load
@task_item n=5 part=touches
decision/iframe-navigation-to-parent (api-security): navigations routed to the parent reach the browser app's navigation provider, which now applies the file: gate by its document's URL
@task_item n=6 part=touches
decision/history-in-sqlite and decision/history-store-errors-logged-and-swallowed (data-protection, error-handling): the history module changed; a refused navigation writes nothing to the history store; the store and its error handling are unchanged
@task_item n=7 part=touches
contract/blitz-net-provider-error/file-forbidden (error-handling): the app's two new refusals raise no FileForbidden or any typed error; they return silently, beside that contract item
@task_item n=8 part=touches
contract/resource-load-failure-log and contract/network-tracing-log items (logging-monitoring): a dropped favicon candidate or navigation starts no request and so emits no load-failure or network line; no log site added
@task_item n=9 part=touches
decision/capability-statement-mcp-crossing and any row citing the committed capability statement: the statement was edited (its two provisional pending-the-founder clauses removed); rows sharing its names or sections stay true
@task_item n=10 part=touches
absence/same-origin-cors-checks, absence/same-origin-and-cors-checks, absence/cors-same-origin-policy, absence/same-origin-checks (threat-model-summary, input-validation, api-security): the new gate is a scheme rule (file: from non-file documents), not an origin policy; the absences stand, the gate sits beside them
@task_item n=11 part=touches
decision/attribute-readers-no-log-or-panic and decision/no-panic-on-call-input kin: the favicon href read and the document URL parse in the new provider constructor answer none on bad input, never panic or log
@task_item n=12 part=touches
decision/cookies-and-cache-default-features and decision/private-on-disk-http-cache (data-protection): no local file's bytes reach the shared favicon cache for a remote page any longer; the caches themselves are unchanged
@task_fact n=1 slug=input-validation
fact: The browser app's navigation provider silently drops a navigation a non-file document starts to a file: URL (nothing loads, nothing is logged, no error raised); an absent or unparsable document URL reads as no file document; file documents' file: navigations and every other scheme push as before.
@task_fact n=2 slug=input-validation
fact: The browser app's favicon candidate is kept only where the document may load it: a non-file page names no file: favicon, so no local file is probed or cached for it; a file page's file favicon and a remote page's default /favicon.ico are unchanged.
@task_fact n=3 slug=input-validation
fact: Convention with its exception: the file: gate covers loads a document starts (sub-resources, favicon, link navigation); the user's own loads in the browser app (URL bar, history, home, about pages, first tab) are top-level loads and pass through no gate.
@task_item n=1 part=untouched
Driver session, session socket, MCP server, CLI flags, roles, exit codes and refusal causes: no driver code, verb, argument, cause or exit changed
@task_item n=2 part=untouched
Select lists, accessibility, stable element ids, snapshots, masking, range inputs and control-state rows: no engine or driver reading changed
@task_item n=3 part=untouched
Recorded checks, kept checks, cold-agent, coverage and exploration rows: none of those components changed
@task_item n=4 part=untouched
Dependency-security rows: no dependency, lockfile entry, CI action or audit setting changed
@task_item n=5 part=untouched
Secret-management and authentication-authorization rows: no env read, secret, token or CI permission added
@task_item n=6 part=untouched
Telemetry sink, log targets, span and command-span rows: no log site, field, span or target added
@task_item n=7 part=untouched
Engine-side loading rows (blitz-net scheme routing, the default script fetcher, stylesheet and font loads, iframe nesting, outbound per-host limits, request methods): no engine crate changed; the predicate the app calls already existed
@task_item n=8 part=untouched
JavaScript opt-in, WPT runner, rdme and CI workflow rows: not reached
