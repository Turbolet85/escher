@history_item n=1 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — (Input Validation) `~ decision/file-urls-only-from-file-documents`: the rule widened to name the browser app's favicon and link navigation, a non-file document's `file:` favicon and its `file:` navigation refused, a navigation dropped silently. This is the hand-off and gate this chunk proves: the chunk's tests must show the `make_doc_config` navigation provider carries the page URL the gate reads, so a remote page's `file:` navigation loads nothing and a `file:` page's does. No behaviour change is allowed, so the row's wording must not move.

@history_item n=2 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — (Threat Model Summary) `~ decision/file-url-only-for-file-documents`: the threat row's reach gained "the browser app names no file: favicon and drops a file: navigation silently". The row's browser half rests on the hand-off whose three mutation survivors (`document_loader.rs:52`, `:56`, `:57`) this chunk kills. Until they are killed, nothing pins the threat claim against the config being replaced by a default or losing its `net_provider` or `navigation_provider`.

@history_item n=3 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (Threat Model Summary) `~ decision/file-url-only-for-file-documents`: the rule gained "or a top-level load" and blitz-net's refusal of engine stylesheet, @import, font, image and iframe loads before any disk read. The top-level-load exception is why a `file:` page's own navigation must still load. The chunk's positive test (a `file:` page's navigation loads) reads that exception, and the `net_provider` test reads blitz-net's refusal.

@history_item n=4 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (Input Validation) `~ decision/file-urls-only-from-file-documents` widened to every engine sub-resource load stamped with its loading document; `+ decision/one-stamping-request-constructor` (a crate clippy rule: no request built in blitz-dom outside the stamping constructor). The `net_provider` that `make_doc_config` hands each document is where that stamping is enforced. A test that builds its own config or request must stay inside this rule, not route around the constructor.

@history_item n=5 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (Error Handling) `+ contract/blitz-net-provider-error/file-forbidden`: blitz-net's refusal of a `file:` load for a non-file document has a fixed, path-free text and reaches the handler only through the existing error path, with no log site of its own. A browser test asserting the refusal through the config's `net_provider` must read the fixed text («file: load refused for a non-file document») and must not add a log site or print.

@history_item n=6 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the script fetch's `file:` refusal by document, a narrowing) This entry is where `document_may_load(document_url, url)` came from: a `file:` URL loads only for a `file:` document, and fetch(), module import and script src are each refused with a `FetchError`. It is the precedent for `decision/default-script-fetcher-local-only` and the gate `history.rs` `navigate_to` reads. It was recorded as a narrowing, not a boundary widening (same-origin and CORS checks absent), with the witness `remote_document_cannot_load_file_urls`. Chunk tests that only prove the narrowing keep that classification.

@history_item n=7 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (`TMPDIR` read at a second test-only site) blitz-vibey-script's `temp_dir_with_files` helper for the `file:`-load tests reads `TMPDIR` through `std::env::temp_dir()`, registered in the environment-values list as test-only. If the chunk's browser tests make temporary `file:` pages the same way, that is a third test-only env read, and the list (the one secret sources are measured against) must name it.

@history_item n=8 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (citations re-pointed by the take's line shifts) 52 `file:line` citations re-pointed, `apps/browser/src/tab.rs` among the files. The chunk adds justification lines beside `tab.rs:178`'s allow and `favicon.rs:76`'s, shifting lines below them. Any security-plan citation into `tab.rs`, `favicon.rs`, `document_loader.rs` or `history.rs` must be re-read at wrap even though no claim moves.

@history_item n=9 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (input rows re-read on the merged engine: the `file:` read) blitz-net reads a `file:` URL through `Url::to_file_path` (a URL that is no file path is a typed `InvalidInput`, no panic), with no path restriction as a standing gap. The later file-document gate is the refusal layered on this read. It is the disk read that the `net_provider` the chunk's tests keep in the config refuses "before a disk read".

@history_item n=10 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a failed load reaches the handler and ends the wait) `NetHandler::error` is a do-nothing default on the public trait, so a wrapping implementer that does not forward it compiles and drops the failure silently. The entry states a trap: sweep the implementers of a trait a change extends. This is the "existing error path" the FileForbidden contract rides. A test reading the refusal through a wrapper or a test handler must forward `error`, or it reads nothing.

@history_item n=11 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (where a failed load's error text stops) A failed load's error text stops in `ResourceLoadResponse` and the `Resource load failed` line, where the sink redacts the message, `url` and `error`, and `NetHandler::error` holds no `tracing` site. This bounds how the FileForbidden refusal may surface. The chunk's tests add no log site or span, as its boundaries require.
