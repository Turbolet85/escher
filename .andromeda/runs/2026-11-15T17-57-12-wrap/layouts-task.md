@task_prose part=session
The session added test code and comments to the browser app only; no public API, dependency, schema, harness or spec changed, and nothing was retired.
It appended an inline test module to the browser's document loader that proves the document config handed to each loaded page: it carries the page URL and the passed network provider, and its navigation provider refuses a file URL from a remote page while loading it from a file page.
The test module builds a shell provider with no overrides, every ShellProvider method left at its default.
Every browser clippy expect_used allow now stands beside a justification comment: bare allows went 2 to 0, among them the favicon test module's and the tab webview's.
Measured: the browser crate's unit tests went 35 to 38, and the document config builder's mutants went 11 missed to 7 missed with 4 caught.
@task_file n=1 path=apps/browser/src/document_loader.rs
what: the browser app's document loader — builds the document config a page is loaded with (page URL, network provider, navigation provider, shell provider) and loads the page; now holds an inline test module
@task_file n=2 path=apps/browser/src/favicon.rs
what: the browser app's favicon fetch for a page, and its test module's expect_used allow
@task_file n=3 path=apps/browser/src/tab.rs
what: the browser app's tab and its TabWebView, which hosts a page's document in the browser window
@task_item n=1 part=touches
Any row on how the browser loads a page and the document config it hands it (page URL, network provider, navigation provider): the hand-off is now proven by unit tests, the builder itself unchanged.
@task_item n=2 part=touches
decision/load-page-only-network and any row on which page gets a network provider: a new test reads that the config carries exactly the network provider the loader passes in.
@task_item n=3 part=touches
Any row on navigation from a page to a file URL: tests now read that a remote page's config loads nothing for a file URL and a file page's config loads it; a row whose guard names the navigation tests may name these two.
@task_item n=4 part=touches
The ShellProvider window chrome contract items (drag_window, is_window_maximized, request_window_close, set_window_decorations, set_window_maximized, set_window_minimized): a test shell provider now implements ShellProvider with every method left at its default, relying on each item's default being a no-op.
@task_item n=5 part=touches
Any convention on justified expect or unwrap allows in the browser app, or an exception naming the browser's favicon test module or tab webview as a bare allow: no bare allow remains, the exception is gone.
@task_item n=6 part=touches
absence/arbitrary-index-html-templates: the document loader it rules over gained tests only; no template capability was added, the absence still stands.
@task_item n=1 part=untouched
The stand stable element ids contract items: no stand element, id or task card changed.
@task_item n=2 part=untouched
The stand decisions (accent colour, keyed controls, attributes-only a11y and ids, lints clean at boot, headless pinned viewport, session roles sharing the boot, held instance as fresh boot, document skeleton with main mount, flight type select): no stand code or stand boot changed.
@task_item n=3 part=untouched
The layout and hit-testing decisions (CRUD list keyboard region, open list row bounds, into-view scroll, visible region through clipping boxes): no layout, scroll or clipping code changed.
@task_item n=4 part=untouched
The cli surface: escher-session, its MCP server, recording, refusals, exit codes and served tasks, cold-agent and agent-run pipes, multi_doc and paint_bench command lines; none changed.
@task_item n=5 part=untouched
The mobile-native and web-spa surfaces and the decisions log: nothing reached them.
