@history_item n=1 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`document_may_load`: a `file:` script load only for a `file:` document) — added the public `document_may_load(document_url: Option<&Url>, url: &Url) -> bool` in blitz-vibey-script, re-exported beside `FetchError`, with `FetchError::Forbidden`; `fetch()`, a module `import` and `<script src>` ask it before the fetcher is called, and `external_script_urls` leaves out what it refuses. This is the rule this chunk extends to blitz-net's own sub-resource fetches (contract/script/document-may-load, contract/script-fetcher/errors, decision/file-url-load-rule, decision/file-script-load-refused). It sets the `Option<&Url>` shape: a document with no URL is refused a `file:` load. It was recorded as a narrowing, not a boundary widening, and that is the precedent for classing this chunk.

@history_item n=2 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (six upstream behaviour changes for every document, ratified for this take) — counted the `file:` script-load refusal as one of upstream's behaviour changes for every document, apart from escher's own direct engine changes, ratified for its take and read as a narrowing. This chunk's blitz-net refusal is escher's own change, not an upstream take, so it is counted and flagged by the direct-engine-change rule (n=13), not by this one.

@history_item n=3 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`TMPDIR` at a second test-only site, and its test directory) — registered blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files`: one `blitz-vibey-script-{name}-{pid}` directory under the system temp dir (`$TMPDIR/…`), read by `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls`. If this chunk's witness or blitz-net's unit test writes a local file to load, a new temp-dir site or directory is an occupied resource to register the same way.

@history_item n=4 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (blitz-vibey-script's runtime and element bindings in child modules) — moved `runtime.rs`'s fetch into the child `runtime/fetch` while `document_may_load` stays in the crate's `fetch.rs`. Bears on where the shared predicate lives, whether kept in blitz-vibey-script or moved beside `Request` in blitz-traits' `net`, and on the citations that would follow it.

@history_item n=5 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed stylesheet through `NetHandler::error`) — added `NetHandler::error(self: Box<Self>, url, error)` with a do-nothing default. It changed blitz-net's `Provider` to call `bytes` on a success or `error(url, e.to_string())` on a failure, then wake the waker, so a failed stylesheet stops blocking render. A bad `href` starts no load. This is the path a refused `file:` load must answer through (contract/net-handler/error, contract/blitz-net-provider/fetch, decision/failed-stylesheet-unblocks). Trap: a trait default method can hide a fix. A new field or method on `Request`/`NetHandler` means sweeping every implementer and wrapper.

@history_item n=6 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load ending the wait) — `CountedHandler` in blitz-test-harness forwards `NetHandler::error`, so a failed load reaches the document on a harness or driver session and ends `settle_waiting` before `LOAD_WAIT`. The witness's refused loads depend on this forward to settle and not read `NotSettled(Loads)` for good (decision/harness-counts-loads-at-net-provider, contract/test-harness/settle).

@history_item n=7 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the test-only binds and no served port) — registered blitz-net's `#[cfg(test)] mod tests` binding `TcpListener::bind("127.0.0.1:0")` in `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css` as test-only binds, no crossing and no served port. The scope extends `delivers_network_and_bad_url_errors`. Any new bind in blitz-net's tests is registered the same way. The scope's witness is to stay offline with no listener (decision/one-local-unix-listener, decision/session-socket-one-listener).

@history_item n=8 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the citations re-pointed past the take) — re-pointed citations into blitz-dom's `net.rs` (+4), blitz-traits' `net.rs` (+3) and blitz-net's `lib.rs` (-3 or -1), the three files this chunk edits. Adding a field to `Request` or a branch to `fetch_inner` moves those cited lines again, and the wrap owes the re-point.

@history_item n=9 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Established Decisions: 4 rows) — added decision/unresolvable-url-skipped: URL resolution is fallible and every caller skips, never a panic. It also moved decision/upstream-take-engine-changes to the 29 picks at `563d3db73`. Reading the loading document's URL to stamp a request has to keep the no-panic rule when a document has no URL (a Dioxus document, `about:`).

@history_item n=10 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Conventions: 1 row) — gave decision/both-layout-modes its exception: upstream-taken blitz-tests targets are kept as taken, and escher's own targets run both modes. This chunk's witness is escher's own, so it must run `for incremental in [false, true]`.

@history_item n=11 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the headless stand serves Cells and a load page) — added the stand's load page. Its options' `net_provider` is the in-process `LoadNet`, answering `LOADED_CSS` from its own thread, while `stand::options` keeps `net_provider: None` elsewhere. This is why the scope says the stand and the driver do not reach blitz-net, so the witness must build documents on blitz-net's real `Provider` and not on the stand.

@history_item n=12 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the cold-agent pipe over six apps) — the load page's one URL `http://stand.test/load.css` (`LOAD_URL`) is answered in process by `LoadNet` and never reaches the network. Confirms that the driver-side surfaces stay out of this chunk.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the eight engine changes of this chunk) — recorded eight more ratified direct engine changes for every document. Each is flagged upstreamable with nothing sent, and their merge surface is a line owned by the next upstream sync. This is the pattern a `Request` field, a `stamped_request` change and blitz-net's `file:` refusal follow (decision/direct-engine-changes-flagged-upstreamable).

@history_item n=14 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic's fix) — the latest single direct engine change: ratified, flagged upstreamable, nothing sent, its merge surface a line owned by «Upstream sync ahead of determinism and exploration». It is the nearest template for recording this chunk's change and placing its merge surface.

@history_item n=15 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Document core and [Scrolling and input]: the document's clock) — added a field to a public engine config (`DocumentConfig.clock`) as an API addition whose default keeps every embedder-built document's behaviour, flagged upstreamable, with its merge surface listed by file. This is the precedent for adding the loading document's URL to `#[non_exhaustive]` `Request` in a way that leaves embedders' top-level loads (no loading document) unchanged (decision/document-config, decision/non-exhaustive-request-types).

@history_item n=16 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** (settle is built) — wrapped a fetching `net_provider` in the harness's crate-private counter, so a request counts as in flight until its handler is consumed or dropped. An offline harness carries none. A `file:` request refused inside blitz-net has to still consume its handler through `error`, or settle reads `Loads` (decision/harness-counts-loads-at-net-provider).

@history_item n=17 marker=2026-10-20-load-wait
**2026-10-20-load-wait** (the acting step's bounded load wait) — added `LOAD_WAIT` (5 s) and `settle_waiting`, which settles again while `busy` is `loads`. A refused load that never answered its handler would cost the full wait on every acting step, and n=6 is the forward that closes this.

@history_item n=18 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (blitz-tests keeps one test target per file) — each file directly under `tests/blitz-tests/tests/` is its own target, with no `autotests = false`, and upstream's `all.rs` is held at `test = false`, pinned by `test_blitz_tests_targets.py`. The witness lands as a new auto-discovered target. Its name settles whether `run stand`, `stand-repeat` or the `a11y` leg reach it (no `stand_` prefix: the workspace `test` leg alone).

@history_item n=19 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (the test-only listener, the manifests, the merge surface read, the pinned driver claims) — set the rule that a test-only bind is registered under Network ports and listeners, never served and so no crossing, and that `TMPDIR` reads via `std::env::temp_dir()` in test code are registered as environment variables. A new local-file fixture or loopback bind in this chunk's tests is registered under this rule.
