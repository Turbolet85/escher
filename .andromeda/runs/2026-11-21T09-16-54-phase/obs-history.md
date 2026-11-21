@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — set the rule that tracing is compiled in only with the `tracing` feature in the engine and upstream crates (escher-telemetry ungated), and set up the allowlist scrub: engine prefixes (`blitz`, `dioxus_native`, ...) print only safe fields. This is where `decision/telemetry-via-tracing`, `decision/engine-tracing-feature-gated` and the scrub rows come from. Any focus-fixup, render-task, seeding or registry log line this chunk adds in blitz-dom, dioxus-native-dom or blitz-vibey-script must be feature-gated and print only safe fields through the sink.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — established that the in-process headless stand and its checks install no subscriber, read no env var and print nothing (the origin of `decision/headless-stand-no-sink`). The new focus, render-task, seeding and registry checks must follow it.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — the first exception to "prints nothing": a re-executed child prints ids to stdout, and the parent reads them into assertions only, never as a log. This is the precedent for the `decision/checks-silent` exception that a later chunk widened to digests (seeded randomness). A seeded `Math.random` proof across processes would use this exception.

@history_item n=4 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — extended the headless-stand silence census to the checks' shared modules (`tests/blitz-tests/tests/common/mod.rs`). This is the "Checks, their shared modules" part of `decision/checks-silent`. A new helper this chunk puts in `common`, `fixtures` or `session_common` falls under the census.

@history_item n=5 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — scoped the scrub's reach to the two seven_guis binaries (`seven_guis_native`, `escher-session`), the origin of `decision/scrub-sink-reach`, and measured host stderr by level. Trap it records: a zero read from a host that installs no sink says nothing about one that does.

@history_item n=6 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — origin of `decision/sink-target-allowlist`. Records from outside targets are dropped whole at every level, at the cost that third-party WARN/ERROR never print. Rule for later chunks: a target is admitted only together with a scrub rule for it. A new log target (for example from a dioxus-core or Boa change) prints nothing unless it is admitted.

@history_item n=7 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the first statement of `decision/debug-types-never-logged`. `Command`, `Call` and `ArgValue` derive `Debug` and print ids and text, so they are never fielded with `Debug`. Any new type this chunk adds that holds ids or page text (a deferred focus act, a registry entry) falls under the same rule.

@history_item n=8 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — added `Outcome` to the `Debug`-deriving types that are never printed, logged or fielded, and kept the session executor silent. Rule kept: field a cause by `Cause::name`, never an `Outcome` with `Debug`. This bears on the focus fixup and the `set_focus` change if their result reaches an answer.

@history_item n=9 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — records that a workspace-wide build turns `blitz-dom/tracing` on, so a sink at `info` also receives engine events. Feature-gated engine sites in blitz-dom (the focus fixup's area) therefore do reach host logs under the workspace build. This qualifies `decision/engine-tracing-feature-gated`.

@history_item n=10 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (typed text measured in a host's log) — first measured `host_log` at `trace` under both builds with 0 typed, id and name needles, the basis of `decision/session-logs-carry-no-page-content` (guard `host_log`, `mcp_log`). An engine change for every document (focus, seeding) must keep these guards green under both builds.

@history_item n=11 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the workspace-built host printed the `blitz_dom::document` `focussed node` INFO line twice. Why is recorded but not established. A focus-fixup change for hidden controls moves this line's count.

@history_item n=12 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — the earlier engine-debts chunk's silence census, including the new `inert_subtree` check and the rewritten `stand_keyboard`. Its engine changes added no log site, span, field or env read. This is the direct precedent for this chunk's checks beside `inert_subtree.rs`.

@history_item n=13 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — the upstream focus checks `focus_within` and `focus_redraw` are recorded silent. The engine focus changes they pin (the not-in-document focus refusal, the focus-move redraw) add no log site. The `Focussed node` INFO line stands. This is the precedent for a new focus give-up path.

@history_item n=14 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — recorded `inert_subtree` and its shared modules silent among the `a11y` leg's checks. A new hidden-control focus check placed beside it joins that census.

@history_item n=15 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations re-pointed) — moved the script error surface's citations (window `error` event, `window.onerror`, a handler's exception, labelled error sources) to blitz-vibey-script `runtime/events.rs` and `runtime/timers.rs`. These are the files behind the `contract/script-error-source-labels/*`, `contract/window-error-event/*` and `contract/window-onerror/arguments` rows. Seeding `Math.random` in the runtime's set-up may move those cited lines.

@history_item n=16 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take read silent at `ebacd68bd`) — the uncaught-JS-error `tracing::error!` stands in `report_js_error` under `#[cfg(feature = "tracing")]` at engine target `blitz_vibey_script`, and the sink prints its message `[redacted]`. blitz-vibey-script `tests/dom.rs` has a test-only `TMPDIR` read. This is the state of the script runtime that the seeding fork's code sits beside.

@history_item n=17 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (blitz-vibey-script slice's absences after the module split) — the slice is split into the `runtime/` and `dom/element/` module trees. Its metric, scrub and span absences are stated by file count. A seeding hook in the runtime is new code in this slice, and these absences must still hold.

@history_item n=18 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the `file:` refusal in the script error surface) — first placed the `file:` refusal in §7 with no log site of its own. It is the ancestor of `decision/no-file-loads-from-non-file-documents`.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`focussed node` re-read beside the focus fixup) — built the inert focus fixup: `DocumentMutator::flush` calls `BaseDocument::clear_focus` where the focused node reads inert. It adds no log site, and neither `clear_focus` nor the flush prints. The `focussed node` INFO line holds a node number and no id or name. This chunk's item 1 extends exactly this fixup to controls that no longer render, so it must keep that no-site and needle-0 state.

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the chunk's surfaces in §6) — the `no site added` row covers the inert focus fixup and the tree's focus fallback. It also carves `blitz-shell/src/event.rs`'s inline test module out of the added-lines census. A widened fixup must be re-read under the same added-lines census.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed past the chunk's added lines) — re-pointed citations into blitz-dom `document.rs`, `mutator.rs` and blitz-vibey-script `runtime/events.rs`. This chunk edits the same files, so expect the same re-point.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the Focussed node line's count, not settled) — a sink count of the focussed-node line on the workspace build read 0 with no base reading, so whether it moved is unsettled. A focus-fixup change should take a base reading before claiming a move.

@history_item n=23 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`URLSearchParams` and the tenth bootstrap script) — precedent for adding a native helper to Boa's runtime. It is registered as `__blitz_parse_search_params` and deleted from `globalThis` before any page script, with no log site or carrier, and it moved the slice's bootstrap count. A seeded `Math.random` hook would follow the same pattern.

@history_item n=24 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the take read silent at `e42ee1f25`) — the `flush_queued_mounted_events` guard and `set_custom_widget`'s replay in dioxus-native-dom open no span. Standing trap from the failed-load entry of the same chunk: a trait default can hide a fix, so sweep every implementer of a trait the change extends. This applies to item 2's mounted-event and render path.

@history_item n=25 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (silence row) — `dioxus_runtime_probe` is recorded silent in log-coverage alone, not in §3, under the silence census pattern. Item 2's probe pins live in this target.

@history_item n=26 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (read values stay in the test) — the probe's failure messages name a layout mode, a task or page and a link or index only, never a component, listener, signal value or node id. A rewritten `…_spawned_acts_*` pin must keep that.

@history_item n=27 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (mutation buffer) — origin of `decision/mutation-buffer-memory-only`. `MutationBuffer` holds page content in memory only from a render to its replay, its `Debug` prints `len` alone, and the replay re-fires `MutationWriter`'s own `trace!` calls. Item 2 (a deferred or replayed act for tasks polled during a render) builds directly on this buffer and must keep the same hold and log stream.

@history_item n=28 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the probe's renamed spawned_acts pins) — recorded `mounted_data_in_a_task_a_key_handler_spawned_acts_*` and the helper `spawned_focus_acts` silent. These are the pins item 2 changes if `set_focus` on a same-render node starts to act, so the row must be re-read.

@history_item n=29 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's graph modules and bridge dispatch add no log site) — `DioxusEventHandler::handle_event` sets a root-scope `SignalRegistry` dispatch and adds no log site. This is a precedent for a root-scope context in dioxus-native-dom, the pattern of item 4's boundary registry.

@history_item n=30 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the state registry holds no state value) — `SignalRegistry`'s `Debug` prints counts alone, and it holds no `NodeId` beyond its dispatch chain and no log site. This is the template a pruned boundary registry (weak handle or generation) should follow.

@history_item n=31 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read opens no span and adds no site) — origin of `decision/actions-read-changes-nothing`: `Session::actions` changes nothing in the document. A focus fixup that clears focus on hidden controls must not make the actions read mutate state, and `stand_actions`' "changes nothing" pin must stay green.

@history_item n=32 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage module, the reached record and their check log nothing) — dioxus-native-dom's root-scope `ReachedRecord` holds stable ids and event names, no `NodeId`, and its `Debug` prints its count alone. A second precedent for a per-document registry like the error-boundary registry.

@history_item n=33 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into rows, among them the picked decisions and contracts. Text not kept as a row (relied facts, citations) is now held by the code, so this chunk's amendments land as row changes (`~`, `+`, `−`).

@history_item n=34 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — `~ decision/no-file-loads-from-non-file-documents`: added the engine sub-resource fetch refusal, with fixed text naming no URL and no log site. Precedent for engine refusals that add no log site.

@history_item n=35 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — `~ decision/no-file-loads-from-non-file-documents`: the row's current wording, where the browser app's favicon and link navigation drop silently. Shows that a script-runtime crossing change rewords this row.

@history_item n=36 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ decision/checks-silent` except: a re-run child prints ids or digests for its parent, to cover the seeded-randomness check's digest lines. Item 3 extends the run's seeded source (`SeededRandom`) to Boa's `Math.random`. A cross-process proof of page-script seeding would reuse this exception, and any new kind of print would need a further row change.

@history_item n=37 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `~ decision/checks-silent` and `~ decision/headless-stand-no-sink` except: `stand_layout`'s recording subscriber added as a named exception. Precedent: a new check that installs a subscriber must be named in both rows' exceptions.

@history_item n=38 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (§3 and §4) — `~ decision/headless-stand-no-sink` rule: an in-process acting step runs under escher-telemetry's scoped capture, which prints nothing. Engine warn/error records during a focus or render act are therefore captured into the answer, not printed.

@history_item n=39 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (§6) — `+ decision/capture-framework-targets-warn-up`: the scoped capture counts Dioxus framework targets (`dioxus*`, `generational_box`, `warnings`) at warn and above. A dioxus-core error-boundary or render-task warning surfaces in a caused item.

@history_item n=40 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (§3) — `~ decision/headless-stand-no-sink` and `~ decision/checks-silent` except: the error-text founder check installs escher's sink over an in-memory writer. These are the current exception texts of both picked rows.

@history_item n=41 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (§6) — `~ decision/capture-framework-targets-warn-up`: the capture also admits engine targets at warn and error. Any engine warn site this chunk adds reaches the caused answer.

@history_item n=42 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (§8, 3 rows) — `+ decision/engine-sites-fixed-messages`: engine warn/error sites write fixed messages with values in named fields. A new engine or framework warn site this chunk adds (focus give-up, registry prune) must follow this convention.

@history_item n=43 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (§3) — `~ decision/checks-silent` except: "the order page logs", its current form. A pinned page with a log site must be named as an exception, which constrains any fixture this chunk adds.
