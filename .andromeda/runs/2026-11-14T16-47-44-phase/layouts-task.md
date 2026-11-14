@task_prose part=chunk
The reference browser app (apps/browser, the blitz binary) closes the two app paths by which a remote page still makes the browser read a local file.
The favicon path: favicon_candidate answers no candidate that document_may_load(Some(base), candidate) refuses, so a non-file: page's file: favicon is never probed nor served from the shared favicon cache; a file: page's favicon still loads.
The navigation path: BrowserNavProvider takes its document's URL where make_doc_config builds it and silently refuses, loading nothing, a link-click or form navigation document_may_load(document_url, target) refuses; a file: document's navigation and the user's own top-level loads (URL bar, history, home, new-tab and history pages, initial URL) are unchanged.
It also drops two stale provisional clauses from escher's capability statement, and adds a unit test for each gate, each with its mutation control.
@task_file n=1 path=apps/browser/src/favicon.rs mark=edits
what: the browser app's favicon probe — choosing a page's favicon candidate and fetching it through the shared favicon cache — edits: favicon_candidate, its existing test module
@task_file n=2 path=apps/browser/src/history.rs mark=edits
what: the browser app's navigation provider and history store — the provider every link-click and form-submission navigation of the top document reaches — edits: BrowserNavProvider (its document URL), BrowserNavProvider::navigate_to, a new test module at the file's end
@task_file n=3 path=apps/browser/src/document_loader.rs mark=edits
what: the browser app's per-document loader, building each document's config and navigation provider and calling the favicon candidate choice — edits: make_doc_config (the provider's one construction site)
@task_file n=4 path=docs/capabilities.md mark=edits
what: escher's own capability statement — edits: the fixture-state absence's provisional clause (line 344), the exploration clauses' provisional sentence (lines 346–347)
@task_item n=1 part=needs
The browser app's favicon candidate choice: a candidate a non-file: document names under file: yields no candidate, so no probe starts and no cached icon is served; a file: document's favicon and every other candidate are chosen as before.
@task_item n=2 part=needs
The browser app's navigation provider: a navigation a non-file: document (or a document with no URL, such as the error page) starts to a file: URL is refused silently and loads nothing; a file: document's navigation still loads.
@task_item n=3 part=needs
The user's own top-level loads in the browser app — URL bar, history, home, the new-tab and history pages, the initial URL — keep loading as before, file: URLs included.
@task_item n=4 part=needs
Indirect, through the engine's public loading rule: both gates call document_may_load as it stands, neither widened nor narrowed; no engine crate is edited.
@task_item n=5 part=needs
Indirect, through the per-document config construction: the navigation provider learns its navigating document's URL where the document is built, not from the navigation options (whose source document is an id) nor from the tab's current history entry.
@task_item n=6 part=needs
A refusal is a filter or an early return only: no carrier, verb, argument, cause, session error, exit, log field, span, env read, listener or dependency is added, and no log line is reached.
@task_item n=7 part=needs
Each new check carries a control: the mutation or hand edit that turns it red, and the reading that it did.
@task_item n=8 part=needs
The navigation gate's test reaches navigate_to on a History store built inside a VirtualDom, as the tab lens test already builds one; neither test reads the disk, a temp file or TMPDIR, so the workspace test leg reads them alike on all three CI OSes.
@task_item n=9 part=needs
Escher's capability statement loses its two provisional clauses (the fixture-state absence, and exploration on no carrier following no navigation); the browser's closed paths are not added to it and its page-loading limit is unchanged.
@task_item n=1 part=not_needed
Stand element ids, stand decisions and the headless stand's layout, keys, accent and accessibility attributes: the stand is untouched by this chunk.
@task_item n=2 part=not_needed
The ShellProvider window chrome controls: the chunk touches no window chrome.
@task_item n=3 part=not_needed
The command-line contracts — escher-session verbs, answers, refusals, exit codes, MCP server, recording and served tasks, the agent-run and cold-agent pipes, the multi_doc and paint_bench command lines: the driver and its tools are untouched and no verb, exit or argument is added.
@task_item n=4 part=not_needed
The cold-agent and MCP decisions on the cli surface: no agent run or MCP behaviour changes.
@task_item n=5 part=not_needed
The mobile-native and web-spa surfaces: unmeasured areas the chunk does not reach.
@task_item n=6 part=not_needed
Document templates and skeletons (the fixed document skeleton, arbitrary index.html templates): the chunk changes no page markup or mount.
@task_item n=7 part=not_needed
Same-origin or CORS rules, other schemes (data:, about:), the browser's history store contents and iframe navigation: out of scope, iframes already under the engine's own rule.
