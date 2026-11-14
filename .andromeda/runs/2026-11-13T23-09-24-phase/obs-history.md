@history_item n=1 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the `file:` refusal in the script error surface) §7 gained the bullet behind `decision/no-file-loads-from-non-file-documents`: a `file:` load asked by a non-`file:` document is refused by `document_may_load` as a typed `FetchError::Forbidden`, with no log site of its own (`fetch()` TypeError, module import fails, `<script src>` skipped and recorded as a script error drained and logged per §6). This chunk extends that same rule to the engine's sub-resource fetches through blitz-net; the row's wording names script loads only, so the plan must say whether the row widens to stylesheet, image, font and iframe loads, and the refusal must again add no log site of its own.

@history_item n=2 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the take read at `ebacd68bd` silent, the sink re-check restated) `document_may_load` was recorded as adding no log site, span, field or env read; a §6 observed-absent row records blitz-vibey-script `tests/dom.rs`'s `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` holding no print or log site, the census's one hit there being `temp_dir_with_files`'s test-only `TMPDIR` read through `std::env::temp_dir()`. Precedent for this chunk's witness: a check that writes a local fixture file through the temp dir is a recorded test-only env read, not a silence break, and the moved or reused predicate must keep `document_may_load`'s no-site record true.

@history_item n=3 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a failed load reaches its handler, and where its error text stops) blitz-net's error arm calls `handler.error(url, e.to_string())` with no log site of its own, beside the standing `Error fetching` and `Resource load failed` lines; a failed stylesheet responds `Err(error)` and leaves `pending_critical_resources` (the source of `decision/failed-stylesheet-leaves-pending-set`); `url` and `error` are in escher-telemetry's scrub set, so the sink prints them `[redacted]` and no other carrier takes the text (`contract/telemetry-scrub-set/url`, `/error`). This chunk's refusal answers through this same `NetHandler::error` path, so its error text must reach no new carrier and a refused stylesheet must still unblock loading. Standing trap: `NetHandler::error` is a do-nothing trait default that can hide a fix (it stopped at `CountedHandler` before); sweep every `NetHandler` implementer, iframe and image handlers included.

@history_item n=4 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the upstream take read at `e42ee1f25` is silent) recorded that the only binds added were two loopback `TcpListener` binds in blitz-net's `#[cfg(test)] mod tests`, served by no binary, and that a failed load reaches only the standing `Error fetching` and `Resource load failed` lines. This chunk changes those blitz-net unit tests (`delivers_network_and_bad_url_errors`) and adds a `file:` request test; its witness is offline with no listener, so it must add no bind beyond those, and the failed-load line set must stay those two.

@history_item n=5 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (citations re-pointed past the take's line moves) the §6 and §7 citations into `blitz-dom/src/net.rs` and blitz-net's `lib.rs` (the blitz-net error class) were re-pointed by digits. This chunk edits `stamped_request` in `blitz-dom/src/net.rs`, blitz-net's `fetch_inner` and blitz-traits' `net.rs`, so the same citations will move again and are owed a re-point at wrap.

@history_item n=6 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — introduced the allowlist scrub: engine prefixes (`blitz`, `dioxus_native`, …) print only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`; `url`, `href`, `src`, `path`, `request`, `error` and the rest redacted everywhere. The origin of `decision/engine-target-safe-fields` and the scrub-set contracts: a refused `file:` load's URL, path or error text, if ever fielded under a `blitz_*` target, prints only `[redacted]`, and this chunk adds no field to them.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (a second sink installer; the scrub's reach scoped to the measurement) scoped the silence census and recorded the trap «a zero read from a host that installs no sink says nothing about one that does». Bears on this chunk's claim that the stand and driver are untouched because they do not use blitz-net: a silence reading in a sink-less check does not speak for the sink-installing binaries.

@history_item n=8 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — (outside-target records dropped) the formatter's three outcomes behind `decision/sink-target-allowlist`, `contract/escher-log-line/scrub` and `contract/escher-log-line/bridged-log-record`: engine targets scrubbed to the safe fields, `escher_` targets minus the content-named set, every other target dropped whole at every level whatever `RUST_LOG` names; rule for later chunks: a target is admitted only together with a scrub rule for it. blitz-net logs through `log`/`tracing` under its engine target; any message this chunk touches stays under an admitted, scrubbed target.

@history_item n=9 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (the closed span's line, PROVISIONAL; a span's fields under the scrub) a closed span's every pair is judged by the same rule as an event's field — the source of `contract/escher-log-line/scrub`'s «every pair of a span line». This chunk adds no span; the clause stands as the contract any added instrumentation would fall under.

@history_item n=10 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand and its in-process checks install no subscriber, print nothing and read no env var, so a headless boot has no escher sink: the root of `decision/checks-silent`, which this chunk's new witness check (both layout modes, real blitz-net `Provider`) must keep.

@history_item n=11 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — the one `println!` exception in the stand checks (`stand_id_persistence`'s re-executed child printing ids the parent reads into assertions only), now the «re-run child prints ids its parent reads» clause of `decision/checks-silent`'s except. The new witness gets no such exception: its readings go into assertions only.

@history_item n=12 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — (the headless-stand census covers the checks' shared module) the silence census names the checks' shared modules, not only the check files — the «and shared modules» of `decision/checks-silent`. If the witness reads or extends `tests/blitz-tests/tests/common/mod.rs` or `fixtures/mod.rs`, the census must be fired over those modules too.

@history_item n=13 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — the third shared module `tests/blitz-tests/tests/fixtures/mod.rs` (with the network stand-in `HeldNet`) recorded silent with no file, socket, listener or `blitz_net`. Trap for this chunk: the witness runs through blitz-net's real `Provider` and reads a local file, so it must not land that use inside the shared fixtures module, or that module's recorded «no `blitz_net`, no file» claim breaks and needs an amendment.

@history_item n=14 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the host's six slugs, the served stand's silence) the stand's load page answers through the in-process `LoadNet` (a stand-in beside `HeldNet` and `LateNet`), with no `blitz_net`, socket or listener — the fact this chunk's scope leans on to say the stand and driver are untouched; every other stand boot keeps `net_provider: None`.

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the chunk's surfaces in §6, and the census's carve-out) an inline `#[cfg(test)]` module under `packages/` was held out of the added-lines census, so that census's 0 did not speak for it, and the carve-out was recorded beside the crate's log sites. blitz-net's unit tests are an inline `mod tests` in `lib.rs`; the plan's added-lines census over this chunk's `packages/*.rs` lines must state whether it covers or holds out that test module.

@history_item n=16 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the Dioxus runtime probe's silence row) precedent for a new blitz-tests target with no `stand_` prefix: its silence (print, env read, subscriber, listener) is recorded as one observed-absent row in log-coverage alone, searched by the named silence-census pattern, with §3's body taking no row. The likely home for this chunk's new witness check's silence record.

@history_item n=17 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (decision · contract · absence · overview), each verified against the code; the picked rows (`decision/no-file-loads-from-non-file-documents`, `decision/failed-stylesheet-leaves-pending-set`, the scrub-set and sink contracts) took their present wording here, so a change this chunk makes lands as a `~`/`+` on those handles, not as prose in a section.
