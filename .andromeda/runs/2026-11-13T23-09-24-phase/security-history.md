@history_item n=1 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the script fetch's `file:` refusal by document, a narrowing) — added `document_may_load(document_url, url)`: a `file:` URL loads only for a `file:` document, checked by JS `fetch()`, module `import` and `<script src>`, the refusal a `FetchError` variant, witnessed by `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls`; recorded as a narrowing, not a widening, on the founder's word. This is the rule the chunk extends to blitz-net's sub-resource fetches (rows decision/file-url-only-for-file-documents, decision/file-urls-only-from-file-documents, decision/default-script-fetcher-local-only). It is the precedent for keeping one predicate and for recording the change as a narrowing that needs no ratification as a widening.

@history_item n=2 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`TMPDIR` read at a second test-only site) — blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files` reads `TMPDIR` through `std::env::temp_dir()` for the `file:`-load tests. Every env read is registered, so if the chunk's offline witness (a temp file read by a `file:` document versus an `https:` one) adds another such site, it has to be registered as test-only in the same way.

@history_item n=3 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the script runtime's module split, and citations re-pointed by the take's line shifts) — the script fetch's code moved to `runtime/fetch.rs` and `runtime/bootstrap/fetch.js`. `FetchError`'s `Display` range was stretched to cover its forbidden-load arm, and the script-skip records now include a forbidden load. If the chunk moves `document_may_load` (for example into blitz-traits `net`), the Input Validation and Threat Model citations of `fetch.rs` must follow it.

@history_item n=4 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (input rows re-read on the merged engine: the `file:` read) — blitz-net's `file:` branch reads through `Url::to_file_path` off wasm. A non-path `file:` URL is a typed `InvalidInput` error with no panic, and the branch reads `request.url.path()` on wasm. This was recorded as a narrowing while the standing gap, no path restriction, stayed. That is the `file:` arm (`Provider::fetch_inner`) this chunk edits, and the scope keeps that gap as it is.

@history_item n=5 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (a hint-less `@font-face` source is fetched and sniffed, PROVISIONAL) — a source with no format hint and no extension is fetched and its format sniffed, a boundary widening that arrived by upstream merge. The entry states that escher routes no agent- or user-supplied URL through blitz-net. Font loads are one of the sub-resource sites the new `file:` gate covers (row decision/hintless-font-face-source-sniffed).

@history_item n=6 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the hint-less `@font-face` source: the PROVISIONAL mark comes off) — the founder ratified the widening as upstream's behaviour, with a check owed, later `font_face_sniffed_source`. The entry keeps the rule that escher routes no agent- or user-supplied URL through `blitz-net`. The chunk must not touch this ratified row while gating font fetches by the loading document.

@history_item n=7 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load reaches the handler and ends the wait) — added `NetHandler::error` as a do-nothing default, and blitz-net's error arm calls it. `CountedHandler` forwards it, so a failed load ends a settle's wait, and `ResourceHandler<StylesheetHandler>::error` drops the request from `pending_critical_resources`. Recorded trap: a trait's do-nothing default can hide a fix behind a wrapper, so sweep a trait's implementers. The chunk's refusal is answered through this same `NetHandler::error`, and the image, font-face and document-src handlers take the no-op default (rows decision/net-handler-error-default-noop, decision/failed-load-ends-wait).

@history_item n=8 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a stylesheet that fails blocks no render) — in every document, a failed, empty or non-CSS stylesheet blocks no render, and an unresolvable `href` starts no load. The guards are `failed_stylesheet_unblocks_rendering`, `empty_or_non_css_stylesheet_unblocks_rendering` and `invalid_stylesheet_url_does_not_block_rendering`. A stylesheet refused because a non-`file:` document asks for a `file:` URL must take this same path: it errors, blocks no render and leaves no document waiting on Loads.

@history_item n=9 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (where a failed load's error text stops) — a failed load's text stops in `ResourceLoadResponse` and the `Resource load failed` line. The sink redacts the message, `url` and `error`, and prints `waiting_nodes`, and `NetHandler::error` holds no `tracing` site. The scope promises the refusal adds no log site and no page content beyond what `NetHandler::error` already carries (contracts resource-load-failure-log/*, network-tracing-log/*).

@history_item n=10 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no served port, two test-only TCP binds) — two `blitz-net` tests in its `#[cfg(test)] mod tests` bind TCP on loopback. They were registered as test-only, with no served port. The chunk edits blitz-net's unit tests (`delivers_network_and_bad_url_errors`), and its witness must stay offline with no listener. Any new bind there would need the same test-only registration.

@history_item n=11 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six served slugs, the load page's in-process stand-in) — the load page's one fixed stylesheet request is answered by the in-process `LoadNet`, with no `blitz_net`, socket, listener or agent-supplied URL, and every other stand boot holds `net_provider: None`. This is the basis for the scope's boundary that the stand and the driver do not use blitz-net, so no driver carrier, verb or field moves.

@history_item n=12 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle reports a load in flight at once as `NotSettled(Loads)`, never waits on one and reads no clock, so the absent request timeout cannot hang it (row decision/settle-never-waits-on-loads). The chunk adds no request timeout to the `file:` branch, so this bound and absence/request-timeout still stand.

@history_item n=13 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the driver's acting step waits on a load in flight for at most `LOAD_WAIT` (5 s of wall time), moving no time, then returns `settled: false` (rows decision/load-wait-bound, decision/bounded-load-wait). A refused `file:` load has to end through `NetHandler::error` rather than hang until this bound.

@history_item n=14 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — escher's sink drops every record from a target outside its allowlist at every level, and redacts content-named fields (`url`, `error` among them) of escher and engine targets (row decision/telemetry-sink-target-allowlist). Recorded trap: a "nothing in the logs" check has to run against a host that installs the sink. Relevant if the chunk checks that a refused `file:` load leaks no path into a log.

@history_item n=15 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the closed-field rule) — set the rule that a new field of a closed value set inside an already ratified carrier is not a widening, while authored text, an open value set, a new carrier or a new exit still is (rows decision/boundary-widening-rule, decision/closed-value-field-no-widening). This applies to how the loading document's URL rides on `net::Request`: it is an in-process field that narrows access and adds no carrier or exit.

@history_item n=16 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** — `~ decision/unresolvable-stylesheet-href-no-load`: the row's why was dropped as measured false (the URL resolver answers none for an unresolvable URL), while the rule still holds. This is the most recent change to a picked row on the stylesheet load path the chunk's witness exercises.

@history_item n=17 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the security plan's text became rows (decision, contract, absence, overview), each checked against the code. At wrap, the scope's planned restatement of the blitz-net limit and the file-URL decisions beside the script loads lands as `~`/`+` row edits in this form, not as prose.
