@history_item n=1 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** (Conventions, amd-4nm4q3gr) — `~ decision/justified-expect-unwrap` except narrowed from "the browser app's test module" to "the browser app's favicon test module", because history.rs's new test module stood beside a justification. This chunk writes the favicon test module's justification (favicon.rs:76) and tab.rs:178's, so the except names no exception any more and the wrap's reconcile drops it. tab.rs's TabWebView allow was never named in the except, so the row was already silently contradicted (the cry-775jieku CARRY).

@history_item n=2 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** (Standard Contracts, amd-jdzhcam4) — `~ decision/file-url-load-rule`: in the browser app, a navigation that a non-`file:` page starts to a `file:` URL is not a top-level load and is dropped silently. The navigation provider drops it and loads and logs nothing. This chunk's tests prove exactly that gate through `make_doc_config`'s navigation provider: a remote page's `file:` navigation loads nothing and a `file:` page's does. The rule must stay as built (Boundaries: no behaviour change).

@history_item n=3 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** (Established Decisions, amd-wa4jxqrv) — `~ decision/sub-resource-requests-carry-initiator`: a request with no initiator is now "the user's own load, or a navigation its page may start", no longer any navigation. This is the companion of the gate that `BrowserNavProvider::for_document(history, base_url)` feeds through its `document_url`. The new tests rest on this split between user loads and page-started navigations.

@history_item n=4 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Established Decisions, amd-tbrxc4nl) — `~ decision/url-scheme-routing`: `file:` is read from disk only for a top-level load or a `file:` loading document and is otherwise refused before any disk read. `~ decision/file-script-load-refused` and `+ decision/sub-resource-requests-carry-initiator` were also added. This is the engine half of the gate whose browser-side hand-off this chunk proves. The net_provider in `make_doc_config` is the path these refusals ride on.

@history_item n=5 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Standard Contracts, amd-y3dq25xu) — `~ decision/file-url-load-rule` widened from script loads to every engine request. blitz-net refuses with FileForbidden before reading the disk, and top-level loads are exempt. It also added `contract/blitz-traits-net/request-initiator` and `contract/stylesheet-handler/document-url`. This is the row history this chunk's tests must read consistently: a remote page's navigation loads nothing.

@history_item n=6 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`document_may_load`, amd-xyrrgurw) — registered the public `document_may_load(document_url, url)`, re-exported from blitz-vibey-script's crate root, and `FetchError::Forbidden`, and recorded the change as a narrowing, not a widening. These are the origin of `contract/blitz-vibey-script-api/document-may-load`, `contract/script/document-may-load` and `contract/script-fetcher/errors`. `history.rs` `navigate_to` asks this same predicate, and the chunk's tests read its answer through the config's navigation provider.

@history_item n=7 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`TMPDIR` test-only site, amd-6yml6hnj) — registered blitz-vibey-script's `temp_dir_with_files`, which writes `file:` fixtures under the system temp dir (`$TMPDIR/…`, test code only). Its tests are `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls`. This is the precedent for remote-versus-`file:` pair tests. If the new browser tests touch the disk, they need the same test-only registration; a URL-only test needs none.

@history_item n=8 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (browser tab lenses, amd-hfn4sexj; tab.rs citations, amd-764rqx7s) — added the browser-app decision block (`tab_lens_survives_webview_props_replacement`, upstream's, upstream-only app) and stretched the `tab.rs` chrome-write citation to `:126-154`. This history touches `apps/browser/src/tab.rs`, where this chunk writes the TabWebView allow's justification (`:178`). The edit is comment-only, but any line shift moves those citations.

@history_item n=9 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`Parallel` style traversals, amd-ifh4aulq) — records that the browser's document loader and toolbar build their documents with `style_threading: Parallel`, serialised process-wide by `PARALLEL_TRAVERSAL_LOCK`, so two documents no longer panic. `make_doc_config` in `document_loader.rs` is that builder. A test that builds a config or document in process should know `Parallel` is the browser's setting and that the lock is process-wide state.

@history_item n=10 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (default sheet from the config, amd-w3z3f3db) — `DocumentConfig`'s hand-written `Default` now carries the one default user-agent sheet (`ua_stylesheets: Vec<UserAgentStylesheet>`), and a config that sets the field is used as given. This is the shape behind `contract/document-core/documentconfig` and `decision/document-config`. The chunk's first mutation survivor is "the config replaced by a default", so its test must tell `make_doc_config`'s result from `DocumentConfig::default()`. A Default with no providers has `net_provider`/`navigation_provider` at their no-op `Dummy*` defaults (`decision/embedder-services-default-noop`).

@history_item n=11 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Document core, amd-3rlakvkd) — `DocumentConfig` gained `clock: Option<Arc<dyn Clock>>`, `None` in `Default`. This was the last change to the field list that `contract/document-core/documentconfig` states (viewport, providers, sheets, fonts, media, threading, incremental, abort, depth, clock). A test comparing the browser's config against a default should read the provider fields it means to pin, not compare the whole struct.

@history_item n=12 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`NetHandler::error`, amd-4suux6zc) — a trait default method that does nothing hid a fix behind a wrapper (`CountedHandler`). The recorded trap: a do-nothing default compiles silently, so the implementers of an extended trait must be swept. This is the same hazard this chunk's survivors show: deleting `net_provider` or `navigation_provider` from `make_doc_config` falls back to the no-op provider defaults (`decision/injected-embedder-services`, `contract/provider-traits/shellprovider`, `contract/net-provider/is-noop`) and stays green unless a test reads the injected provider.

@history_item n=13 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** (mutation tier host instruments, amd-5hxke3kf) — set up the source of `decision/mutation-tier-host-only` and `decision/nextest-mutants-profile`: cargo-mutants with `--test-tool nextest` under the `mutants` profile of `.config/nextest.toml`, run on the dev host only and never in CI, with each unit's mutant test command read green on the unmutated tree first. The survivors' kill on `document_loader.rs` is read by such a host run once the host is free; it is no CI leg.

@history_item n=14 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Stack and Technologies, amd-fijsdyn4) — `+ decision/mutation-tier-excludes-demo-shells`: the founder ruled two test-less demo units out of the mutation tier, and the code audit reads its extra exclusions only from an architecture decision row. The browser app is not excluded: its score of 50.0 and its three survivors stand in scope, which this chunk kills.

@history_item n=15 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (fork CI through one leg script, amd-gh222uio) — the origin of `decision/one-ci-leg-runner`, `contract/ci-leg-legs/test`, `contract/ci-leg-script/exit-status` and `decision/linux-tested-by-fast-leg`. Every linux job runs `bash .github/scripts/ci-leg.sh {leg}`, whose `test` leg runs the workspace tests, and the matrix tests only windows and macos. The browser app's new tests are CI-gated through the workspace `test` leg, the `[inferred]` boundary P3 closes.

@history_item n=16 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** (the masters as rows, amd-yhg3j6hb) — the master's text became the rows this chunk is bound by, each verified against the code. That conversion created the duplicate pairs among the picked rows (`decision/justified-expect-unwrap` / `decision/unwrap-expect-justified`; `decision/clippy-allow-scoping` / `decision/local-lint-allows`; `decision/net-feature-noop-on-wasm` / `decision/net-noop-on-wasm`). When the wrap drops the favicon exception, it must reconcile every twin that states the expect/unwrap rule.
