@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (per-leg CI failure logs; WPT and publish telemetry upstream-only) — marked the WPT report, `wptscores.json`, the §5 WPT-scores metric and publish-build trace logging upstream `DioxusLabs/blitz` only, their jobs repository-guarded; per-leg failure logs confined to `target/ci-logs/`. Matters: this take carries four WPT-only commits (`ea07ee716`, `e7cca1d37`, `c30096724`, `d2ee20b98`), read against `decision/wpt-scores-upstream-only`, `decision/wpt-publishing-upstream-only` and `decision/ci-platform-github-workflows` — the guards must stay and no new upload may leave fork CI.

@history_item n=2 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — built escher-telemetry: `RUST_LOG` `EnvFilter` defaulting to `warn`, one non-ANSI stderr-only layer, the `LogTracer` bridge, the engine-prefix safe-field scrub and the content-named redaction set (`url`, `error` among them); engine/upstream tracing compiled only behind the `tracing` feature. Matters: the origin of `contract/telemetry-sink/*`, `decision/engine-target-safe-fields`, `decision/escher-target-content-named-set` and `decision/telemetry-via-tracing` — the sink allowlist re-check reads the taken delta's new `tracing`/`log` sites against these; the upstream apps' `fmt::init()` stdout sinks stay unscrubbed.

@history_item n=3 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand installs no subscriber, reads no env var; only the windowed binary installs the sink. Matters: basis of `decision/headless-stand-no-sink`; upstream's new test targets joining blitz-tests must keep this silence.

@history_item n=4 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (two entries: citations re-pointed after the merge; root-manifest citations past line 61) — the first upstream merge re-pointed 20 `file:line` citations by the merge's measured line map and found 6 root `Cargo.toml` citations a prior wrap had left stale. Matters: precedent and trap for this sync — a take moves cited lines in merged upstream files and the root manifest, and every citation must be re-pointed, none left behind.

@history_item n=5 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — OTel export out: no OTel crate, egress or `OTEL_EXPORTER_OTLP_HEADERS` credential path; coverage-report upload ratified. Matters: source of `decision/no-otel-export` and the telemetry-backend absences — a take must bring in no OTel/metrics crate (also covered by the audit leg's dependency review).

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — a second sink installer (`escher-session`); found outside-target records printed as written at `debug`/`trace` (ids, names). Matters: standing trap for the sink re-check — "a zero read from a host that installs no sink says nothing about one that does"; the re-check must read a sink-installing host (`host_log`, `mcp_log`), per `decision/sink-host-read-at-trace`.

@history_item n=7 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — every target outside `ENGINE_TARGET_PREFIXES` / `ESCHER_TARGET_PREFIXES` dropped whole at every level, whatever `RUST_LOG` names; a third-party WARN/ERROR no longer prints. Matters: the allowlist this chunk re-checks (`decision/sink-target-allowlist`, guard `telemetry_drop`); its rule for later chunks — a target is admitted only together with a scrub rule for it — binds any new upstream target the take brings.

@history_item n=8 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span as built; `tracing` ungated in escher-driver) — recorded that a workspace-wide build turns `blitz-dom/tracing` on, so a test child with the sink at `info` also receives engine events. Matters: basis of `decision/engine-tracing-by-invocation`; the take's new engine log sites reach the sink under the workspace build, so `host_log`/`mcp_log` must be read under both builds.

@history_item n=9 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the closed span's line, PROVISIONAL) — added the closed-span record class: one line per closed span whose target is admitted, every pair judged by the same scrub; content-named span values never stored. Matters: origin of `decision/closed-span-lines`, `contract/escher-log-line/span-on-close-only`, `contract/span-line/*` and `decision/content-span-fields-never-stored` — any upstream span the take adds is judged by this rule.

@history_item n=10 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the closed span's line ratified by the founder as a second record class. Matters: the span-line rows the re-check stands on are ratified, not provisional.

@history_item n=11 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (typed text measured in a host's log; the session host by level under both builds) — the workspace-built host writes one extra `blitz_dom::document` INFO line; 0 ids, names and sentinels at every level under both builds. Matters: the by-build baseline the take's sink re-check compares against (`decision/sink-host-read-at-trace`, `decision/engine-tracing-by-invocation`).

@history_item n=12 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role on the sink; `mcp_log` reads the role at `trace`. Matters: `mcp_log` is the second guard of `decision/sink-host-read-at-trace`, run under both builds after each take.

@history_item n=13 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the merge's five upstream integration targets joined the no-subscriber census; fixed-sha census 0; `host_log`/`mcp_log` pass under both builds after the merge. Matters: precedent for how a sync records upstream's new test targets as silent.

@history_item n=14 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the workspace-built host's `focussed node` engine line printed twice (5 → 6 lines); dated readings kept, re-reads appended. Matters: a moved engine line count under the workspace build is recorded and explained, not loosened — the same rule this chunk's per-OS tally and sink re-reads follow.

@history_item n=15 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the coverage report's lcov excludes `keyboard_compat.rs` by filename. Matters: a prior sync's CI coverage artifact change, kept bound to test-plan's coverage leg; the take must not move it silently.

@history_item n=16 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — upstream's focus checks `focus_within` / `focus_redraw` recorded silent; the engine take PROVISIONAL until the founder's word. Matters: precedent — upstream test targets read silent, and engine changes for every document marked PROVISIONAL when the founder is away, as this chunk's boundary widenings are.

@history_item n=17 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the ten-commit take read silent: no tracing site over `packages/`, no wall-clock read outside `clock.rs`, `escher-telemetry` unchanged, `host_log`/`mcp_log` pass under both builds; the sink allowlist re-check holds (no scrub set, redaction shape or violation schema moves); `multi_doc`'s printed timings recorded. Matters: the template for this chunk's "sink allowlist re-checked" acceptance.

@history_item n=18 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations re-pointed into the code the take moved) — a citation whose code left its file read false and was re-pointed by hand at Validate, the rest by the take's line map. Matters: trap — a take that splits or moves upstream files breaks citations beyond the line map's reach.

@history_item n=19 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take read at `ebacd68bd` silent, and the sink re-check restated) — fixed-sha census over the taken `packages/*.rs` lines read its named hits (the feature-gated uncaught-JS-error `tracing::error!`, a test-only `TMPDIR` read); the census line replaced, not given a sibling (body holds current truth, history in the sidecar). Matters: method and body-replacement convention for this take's silence line and sink re-check.

@history_item n=20 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (slice absences after the module split; test slice holds `pseudo_element_flow_classification`) — slice absences restated with their counts when a take adds or moves files. Matters: upstream's new or edited test files and modules may need the slice counts behind §2/§4/§5/§8 absences re-read.

@history_item n=21 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the `file:` refusal in the script error surface) — upstream's `document_may_load` refusal recorded in §7, with no log site of its own. Matters: origin of `decision/no-file-loads-from-non-file-documents`; the take's SVG image and `@font-face` (`f5ea1caf1`) loading must keep the refusal and add no log site.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (where a `multiple` select's set, an `optgroup`'s label and the engine's `change` leave) — chosen set only as form entries, snapshot value and tree nodes; engine `change` after `input`, a script hearing one. Matters: source of `decision/select-chosen-set-never-logged`, `contract/multiple-select-form-entries/chosen-option`, `contract/page-events/*` and `contract/snapshot/multiple-select-value` — conflict resolutions must keep the select model and change semantics.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`focussed node` re-read beside the focus fixup) — the engine INFO line holds a node number, no id or name, once per focus move under the workspace build. Matters: the one engine line the workspace-built host prints; a take touching focus or `document.rs` re-reads it.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a page's labels as pixels and the size read, the widening ratified) — open list shot carries labels as pixels; `size` read capped at 65534; `optgroup` label painted as a header — each an engine change for every document, ratified. Matters: source of `decision/open-list-shot-carries-labels`, `decision/select-size-attribute-read`, `decision/optgroup-label-crossing`; precedent that new attribute reads on every document (as the take's SVG presentation attributes and `hr` `color`/`noshade`/`size`) are boundary widenings listed for the founder, never settled silently.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open row's bounds and `SelectListRow.label`) — row bounds cross by their own `bounds`; `SelectListRow.label` page text never logged. Matters: source of `contract/snapshot-diff/open-list-row-bounds` and `decision/select-list-label-never-logged`, kept through conflict resolution in the select list.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the Focussed node line's count, not settled) — a sink count with no base reading settles nothing. Matters: trap for the per-OS and sink re-reads — a reading needs a base to call a move.

@history_item n=27 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load reaches its handler, and where its error text stops) — `NetHandler::error` from upstream `f6edb9701`; a failed stylesheet responds `Err` and leaves `pending_critical_resources`; `url` and `error` redacted on `Resource load failed`. Standing trap: a trait default can hide a take's fix — sweep the implementers of a trait a take extends. Matters: source of `decision/failed-stylesheet-leaves-pending-set`, `contract/telemetry-scrub-set/error`, `contract/telemetry-scrub-set/url`; the take's `@font-face` and SVG image paths touch loading.

@history_item n=28 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the upstream take read at `e42ee1f25` is silent) — silence line for the twelve commits and the fork's `deallocate_anonymous_block` fix; test-only loopback binds noted; `host_log`/`mcp_log` 0 needles at `trace` under both builds; no scrub set moves; earlier take lines kept beside as history. Matters: the immediate precedent for this take's silence line and sink re-read (the last-but-one take).

@history_item n=29 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`URLSearchParams` and the tenth bootstrap script) — bootstrap count 9 → 10 in the slice absences. Matters: a take adding a bootstrap script or module moves the slice counts the absences carry.

@history_item n=30 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (citations re-pointed past the take's line moves) — digits-only re-point across §9, §6, §5, §4 and the body. Matters: the expected citation re-point at this take's wrap.

@history_item n=31 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render's mutation buffer, silent and held in process only) — `MutationBuffer` holds page content in memory only, `Debug` prints `len`; the replay re-fires `MutationWriter`'s `trace!`. Matters: source of `decision/mutation-buffer-memory-only`; the buffered render is a carried merge surface (`cry-2hvzi4ns`) a conflict resolution must keep.

@history_item n=32 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's carried set) — graph answer carries ids, names, closed flag words and booleans, never a value; a boundary widening. Matters: source of `contract/graph-answer/content` — dioxus-native-dom resolutions keep the graph unchanged.

@history_item n=33 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's graph modules and bridge dispatch add no log site) — a page with no registered state reaches no registry. Matters: source of `decision/no-state-page-skips-registry`, kept through any take touching `DioxusEventHandler::handle_event`.

@history_item n=34 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading carries no page text) — `clickable` a closed bool after `focused`. Matters: source of `contract/snapshot-node/clickable` and `contract/snapshot-text/clickable`, kept through snapshot conflict resolution.

@history_item n=35 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage module, the reached record and their check log nothing) — reached record holds stable ids and event names only, no `NodeId`, `Debug` its count. Matters: source of `decision/reached-record-ids-and-events-only`, a dioxus-native-dom surface the take must not move.

@history_item n=36 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (decisions, contracts, absences, overviews), verified against the code. Matters: since then amendments name rows by handle (`~ decision/...`); this chunk's obs amendments, e.g. to the take's silence or a new boundary widening, are row edits.

@history_item n=37 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — `~ decision/no-file-loads-from-non-file-documents`: an engine sub-resource fetch refused with fixed text naming no URL, no log site. Matters: the scope's `Request::get` ban carry (`cry-nnm3rzya`) — a take edit to `@font-face` loading (`f5ea1caf1`) routes through `stamped_request`, keeping this rule.

@history_item n=38 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — `~ decision/no-file-loads-from-non-file-documents`: the browser app's favicon and link navigation drop a non-file: document's `file:` load silently. Matters: the row's current text; a take touching the browser app (the `expect_used` carry) keeps it.

@history_item n=39 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph answer's carried set) — the graph answer's widening ratified by the founder. Matters: `contract/graph-answer/content` stands ratified, no longer PROVISIONAL, as this chunk resolves dioxus-native-dom conflicts.
