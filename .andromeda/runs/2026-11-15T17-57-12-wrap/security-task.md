@task_prose part=session
The session added test code and comments only in the browser app; no runtime behaviour, public API, dependency, config or CI gate changed.
It proves the browser's document-config hand-off by three new unit tests: the config carries the page URL and the passed net provider, and its navigation provider loads a file URL from a file page but nothing from a remote page.
It placed a written justification beside every clippy expect_used allow in the browser source (bare allows 2 to 0), in the favicon prober and the tab web view.
Measured: the browser crate's unit tests 35 to 38; the document-config builder's mutants 11 missed to 7 missed, 4 caught; CI green 17 of 17 checks.
No log, print or environment read was added; the lockfile and the browser manifest are unchanged; nothing was retired.
@task_file n=1 path=apps/browser/src/document_loader.rs
what: the browser app's document loader — builds the document config handed to each loaded page (base URL, the blitz-net net provider, the navigation provider that gates file URLs to file documents, abort signal) and loads documents; edited: a test module appended
@task_file n=2 path=apps/browser/src/favicon.rs
what: the browser app's favicon probing for a loaded page; edited: one justification comment above its test module's expect_used allow
@task_file n=3 path=apps/browser/src/tab.rs
what: the browser app's tab and its web view — in-document navigation, history entry and page loading per tab; edited: two justification comment lines above an expect_used allow
@task_item n=1 part=touches
decision/file-url-only-for-file-documents: the browser-navigation clause of the file-URL gate is now guarded by two new tests reading the navigation provider in the document config; a guard cell naming its tests may add remote_page_config_navigation_to_file_url_loads_nothing and file_page_config_navigation_to_file_url_loads.
@task_item n=2 part=touches
decision/file-urls-only-from-file-documents: the same rule, same two new tests as guards of the browser's navigation direction (remote page loads nothing from a file URL, file page loads it).
@task_item n=3 part=touches
contract/blitz-net-provider-error/file-forbidden: indirect — the remote-page test reads that a file-URL navigation from a remote page loads nothing, the outcome this contract item names; no wording or value moved.
@task_item n=4 part=touches
decision/outbound-http-through-blitz-net and decision/outbound-http-via-reqwest-or-blitz-net: indirect — a new test asserts the document config carries the very blitz-net provider the loader is passed; the rule holds, now with a test beside it.
@task_item n=5 part=touches
decision/document-abort-signal-cancels-fetches: indirect — the abort signal field of the document config stays untested; its deletion is among the 7 surviving mutants, so no guard of this rule was added.
@task_item n=6 part=touches
decision/error-pages-skip-history-and-favicon: indirect — a rule over the favicon probing and tab loading components the session edited; only comments changed there, behaviour unchanged.
@task_item n=7 part=touches
decision/history-store-errors-logged-and-swallowed and decision/failed-fetch-error-page-shows-debug-text: indirect — rules over the tab and document loading components the session edited; behaviour unchanged.
@task_item n=8 part=touches
decision/browser-javascript-opt-in and decision/javascript-opt-in: indirect — rules over the browser's document loading the session tested; the new tests build a config without touching the script opt-in.
@task_item n=9 part=touches
decision/iframe-navigation-to-parent and decision/external-targets-to-system-browser: indirect — navigation rules over the browser's navigation provider the new tests exercise; neither rule moved.
@task_item n=1 part=untouched
The driver session socket, its wire, refusal causes, exit codes and the MCP server rows: the session reached no driver, session or MCP component.
@task_item n=2 part=untouched
Stable element id, snapshot text, actions and graph answers, select list and control-state rows: no engine reading or driver answer changed.
@task_item n=3 part=untouched
Recording, kept check, cold-agent and coverage rows: no recorder, check file or harness script changed.
@task_item n=4 part=untouched
Dependency security rows (audit, licence gate, locked builds, pins): no dependency, lockfile or manifest changed.
@task_item n=5 part=untouched
Secret management and CI token rows: no workflow, secret or environment read changed.
@task_item n=6 part=untouched
Logging and monitoring rows (telemetry sink, allowlists, span lines): the session added no log, print or tracing call.
@task_item n=7 part=untouched
Data protection rows on history storage, HTTP cache, cookies, clipboard and TLS: no stored-data or network-client behaviour changed.
@task_item n=8 part=untouched
Engine input-validation rows on CSS, HTML parsing, serialization, layout and form controls: no engine crate was touched.
