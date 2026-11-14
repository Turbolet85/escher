@history_item n=1 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — `~ decision/no-file-loads-from-non-file-documents`: the rule widened from script loads alone to the engine's sub-resource refusal too (blitz-net's fixed-text `FileForbidden` provider error through the handler's error path, naming no URL or path, adding no log site). This is the row this chunk extends to the app's two remaining paths (favicon probe, navigation); the chunk reuses `document_may_load` / `FileForbidden` as they stand, and keeps the row's "adding no log site" — a refused favicon or navigation is simply not started. Whether the row's rule text needs a clause for the app-level refusals (silent, not an error) is the plan's question.

@history_item n=2 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the `file:` refusal in the script error surface) — added the §7 bullet the row descends from: a `file:` load asked by a non-`file:` document is refused by `document_may_load` as a typed `FetchError::Forbidden` with no log site of its own. The refusal is upstream's, taken on the founder's word ratifying an engine behaviour change for every document; this chunk must not widen or narrow that engine rule (its Boundaries), only call it from `apps/browser`.

@history_item n=3 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take read at `ebacd68bd` silent) — recorded that `document_may_load` adds no log site, span, field, log field or env read, and an `observed absent` §6 row for blitz-vibey-script's `remote_document_cannot_load_file_urls` / `file_document_can_load_sibling_file_urls` tests (their one hit a test-only `TMPDIR` read). Precedent for this chunk's new favicon and navigation tests: record them silent; the scope's tests need no `TMPDIR` or temp file, so no new test-only env-read site should appear.

@history_item n=4 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load reaches its handler) — `NetHandler::error` became a public trait method with a do-nothing default; blitz-net's error arm calls it, with the standing `Error fetching` / `Resource load failed` lines as the path's only instrumentation (`url`, `error` redacted by escher's sink). This is the path the engine's `FileForbidden` already rides. Standing trap: a trait default can hide a fix — sweep implementers of a trait a change extends. Relevant if the favicon gate were done by initiator on the fetch instead of the scope's pre-probe filter.

@history_item n=5 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ decision/checks-silent` · except: the re-run-child exception widened from "prints ids" to "prints ids or digests its parent reads". The current text of the row binding this chunk's new checks (`favicon.rs` unit tests, the new test module in `history.rs`): they install no subscriber, read no env var and print nothing, and fall under none of the named exceptions.

@history_item n=6 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — origin of the `decision/checks-silent` exception for a re-executed child printing to stdout that the parent reads into assertions only ("not a log channel"). The exception list is closed; a new check that prints needs its own named exception, which this chunk's tests do not need.

@history_item n=7 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the silence census widened to cover the checks' shared modules (the "their shared modules" clause of `decision/checks-silent`). If this chunk's navigation test reuses `tab.rs`'s `VirtualDom`/`History` setup through a shared helper, that helper falls under the same silence rule.

@history_item n=8 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — hand-written network stand-ins (`HeldNet`) entered the silence census, the basis of the "stand-ins they use" clause in `decision/checks-silent`. Any stand-in this chunk adds for its tests must hold no print, env read, subscriber, listener or `blitz_net`; the scope's pure `favicon_candidate` and in-`VirtualDom` navigation tests need none.

@history_item n=9 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the chunk's surfaces in §6, the census's carve-out) — an inline `#[cfg(test)]` module was held out of the added-lines census, so that census's 0 did not speak for it. Trap for this chunk: both new tests are inline test modules (`favicon.rs`'s existing module, a new module at the end of `history.rs`); the silence reading must name them explicitly rather than lean on an added-lines census that excludes them.

@history_item n=10 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the silence row) — precedent for recording a new non-`stand_` test target's silence: a §6 `observed absent` row with the silence-census pattern and its exit-1/0 reading, the §3 body taking no row. Model for recording this chunk's `apps/browser` tests silent.

@history_item n=11 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — fixed that `fmt::init()` is the upstream apps' stdout install (the `blitz` browser app among them), escher's scrubbed sink installed only by escher's binaries, and left `pii-scrubbing-wire` open for the upstream sinks. A log line added in `apps/browser` would print unscrubbed (a URL or path); the chunk's no-log-line refusals keep the favicon candidate and navigation target out of that open sink.

@history_item n=12 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the obs master's text became rows (decision/contract/absence/overview), the form `decision/checks-silent` and `decision/no-file-loads-from-non-file-documents` now take; a body holds current truth and this sidecar the history, so an edit to either row is a `~ {handle}` amendment, not a new paragraph.

@history_item n=13 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's carried set) — the confirm that its proposals lie inside the founder's ratified widening is PROVISIONAL (delegate the overseer), "for the founder's own word in his batch at the epoch boundary". This chunk's wrap is the Epoch 4 boundary wrap that carries that batch; a wrap's batch, not a plan task, but the plan should not treat the mark as discharged.

@history_item n=14 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role entries: the `escher-session` install row, its endings, the `command` span's reach, the coverage line's carried fields) — each confirmed by the overseer as delegate, PROVISIONAL, for the founder's batch at the epoch boundary. Like n=13, open marks the Epoch 4 boundary wrap at this chunk's end is due to take to the founder; not this chunk's plan work.
