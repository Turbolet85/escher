@history_item n=1 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** (the changed set, the shell's refresh on change) — created the changed set (`take_changed_nodes`, marked by mutation flags, `FOCUS`/`CHECKED` and typed text, never by creation/hover/scroll/layout). This is the origin of `decision/changed-set-write-sites` and `decision/changed-set-engine-touched`. A focus clear for a control that no longer renders (item 1) is a focus change, so it must write the changed set at a sanctioned site. Adding a new write site amends that closed list, as the later select-list chunks did.

@history_item n=2 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the bridge drops a removed node; the 28-name falsy predicate) — a node removed by a re-render is dropped with its subtree and no longer resolves (`decision/bridge-removal-drops-subtree`). A falsy `hidden`/`inert` is cleared and a truthy one is written (`decision/falsy-boolean-attribute-cleared`). Item 1's "hidden by `hidden`" case reaches the engine through this bridge path. Item 2 concerns nodes the render creates and drops.

@history_item n=3 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (keyboard activation and a click's focus) — Enter/Space activation (`handle_key_up`, `dispatch_activation_click`) and a click focusing a button were made direct engine changes for every document. These are the key paths a hidden-but-focused control still takes (item 1), and the rows `decision/keyboard-activation-enter`/`-space` and `decision/click-focuses-button`. The precedent: each such change is its own founder fork and is flagged upstreamable.

@history_item n=4 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (keyboard-types 0.8) — this move left two keyboard-types versions in the graph (Dioxus 0.7.3 carries 0.7). A module that glob-imports `dioxus::prelude::*` names `::keyboard_types`. Trap for item 2's probe and bridge tests in dioxus-native-dom (`decision/keyboard-types-named-by-absolute-path`).

@history_item n=5 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (seven engine changes: `inert`, `disabled` on form controls) — `inert` is honoured by presence through `Node::is_inert` (an ancestor walk, no mutator hook), and `disabled` is read through `ElementData::is_disabled` on a form control alone. It also stated the focus-fixup limit with a residual owner. This is the earlier engine-debts chunk this one follows: each fork was answered by the delegate PROVISIONAL and discharged later by the founder's batch (`amd-yiemoprt`). Item 1's fixup must reuse these one readings (`decision/disabled-one-reading`, `decision/inert-honoured`).

@history_item n=6 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Document core: the document's clock) — added `DocumentConfig.clock`, `None` by default so windowed and embedder documents keep wall time, and passed a sub-document its parent's clock. It is ratified as an API addition, flagged upstreamable. This is the nearest precedent for item 3's document-level seed: a config field whose default keeps behaviour, threaded to sub-documents (`decision/document-clock`, `contract/document-config/clock`). It also set the rule "No `Instant::now()` outside `clock.rs`" (`decision/no-wall-time-outside-clock`).

@history_item n=7 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Test harness: the harness's time is the documents' clock) — the harness holds one `VirtualClock` its built documents share through `HarnessOptions::into_config`, while `wrap(doc)` keeps the document's own clock. The harness's run seed (item 3) would follow the same `from_vdom` vs `wrap` split (`decision/one-virtual-clock`, `contract/test-harness/virtual-clock`).

@history_item n=8 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** (`:focus-within`, focus outside the document, the focus redraw) — `set_focus_to` returns `false` for a node not in the document. `FOCUS_WITHIN` moves along the ancestor chain, `clear_focus` removes it and requests a redraw, and these writes mark no changed-set entry. Item 2's limit is exactly `set_focus_to` returning `false` for a node the render has not mounted yet. A focus clear for item 1 must keep the `:focus-within` chain and the redraw (`decision/focus-within-ancestors`, `decision/redraw-requests`).

@history_item n=9 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** (the select model, an engine change for every document) — the select keyboard model, with typeahead on the document's clock and choices through `apply_select_choice`. It was ratified per fork by the founder and its merge surface rides the upstream-sync CARRY. This is the precedent for how an engine change for every document is recorded (`decision/select-keyboard-model`).

@history_item n=10 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`Parallel` traversals serialised) — the static `PARALLEL_TRAVERSAL_LOCK` serialises `Parallel` traversals. No headless document (harness, stand or driver) sets `style_threading` (`decision/headless-documents-sequential-style`, `decision/headless-documents-default-threading`). Any new headless config field for items 1 and 3 must leave threading at its default.

@history_item n=11 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the dropped-node hit skip) — `hit_inner` and the paint tree's walk skip a node dropped since the last resolve (`decision/hit-and-paint-skip-dropped-nodes`, `contract/scroll-hit/hit-dropped-node`). This is the precedent for a reader tolerating nodes gone between mutation and resolve. Items 1 and 2 both act between a mutation and the next layout.

@history_item n=12 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (blitz-vibey-script's runtime in child modules) — `runtime` is split into `bootstrap`, `css`, `events`, `fetch`, `modules`, `timers` and `window`, with `BOOTSTRAP_JS` concatenated from `runtime/bootstrap/*.js`. These are the files where item 3's Boa `Math.random` hook and its runtime set-up would land, beside the runtime's own timers (`decision/own-timers-on-document-loop`).

@history_item n=13 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`document_may_load`: a `file:` script load only for a `file:` document) — upstream's narrowing on the registered script-to-DOM crossing, ratified for its take. This is the script runtime's crossing rule, which the scope says item 3 touches (`decision/file-script-load-refused`, `contract/script/document-may-load`, `contract/default-script-fetcher/schemes`). Seeding the runtime must stay inside this crossing and its `ScriptFetcher` rules.

@history_item n=14 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (six upstream behaviour changes ratified for this take) — set the pattern behind `decision/upstream-behaviour-changes-ratified-per-take`: an upstream behaviour change is ratified per take and is not counted among escher's direct engine changes. Fixes to items 1–3 are escher's own direct engine changes and are counted separately (`decision/direct-engine-changes-flagged-upstreamable`).

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the focus fixup under inert) — `DocumentMutator::flush` calls `clear_focus` where the focus node `is_inert()`, firing no `blur`/`focusout`. This covers a subtree moved under an inert box, with `disabled` and `hidden` reading as before. It is the exact mechanism item 1 would widen to a control that no longer renders. The precedent: no-event clearing (`decision/focus-fixup-without-events`, `decision/inert-focus-fixup`, `contract/dom-events/inert-focus-fixup`), and the scope cites this fixup.

@history_item n=16 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the eight engine changes of this chunk) — the tally of escher's direct engine changes, each flagged upstreamable with its merge surface owned on the next upstream sync (`decision/hit-focus-form-engine-changes`). A fix to item 1, 2 or 3 joins this tally on the same terms.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — `build_accessibility_tree` names focus only where the focused node was built, else the window. Item 1's hidden-but-focused control is exactly such a node: the tree already reads the window while the engine keeps focus (`contract/accessibility-tree/focus`, `contract/snapshot/focused`).

@history_item n=18 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the MountedData hazard of a spawned task) — first recorded the panic when a key-handler-spawned task calls `MountedData` inside `render_immediate`. This is the origin of item 2's lineage.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list recorded) — an open list closes with no event on removal, `disabled`, `hidden` or `inert` on its select. It is the closest precedent for item 1's fix: interaction state given up when its node stops rendering (`decision/select-list-closes-without-event`), ratified as a direct engine change.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's state and placement in the engine) — `resolve` calls `reconcile_select_list` after layout and before `refresh_hover`, and `clear_interaction_state_for_removed_node` clears the list. `SelectList.fresh` guards against a `:focus` restyle. This is the precedent for item 1's option B, focus given up after the next layout, at a known point in `resolve` (`decision/resolve-reconciles-select-list`, `decision/interaction-state-cleared-on-removal`).

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic's fix) — `deallocate_anonymous_block` clears its children's `layout_parent`, so a key press on a control hidden by `hidden`, `display:none` or CSS no longer panics. Focus stays and the key still acts, because `DocumentMutator::flush` reads `is_inert()` alone. The founder's F1 A left this limit, and it is item 1's residual. His option B was not built. Trap: assert the fixture reaches the path before trusting a green probe (`decision/anonymous-block-dealloc-detaches`, `decision/anonymous-block-drop-clears-layout-parent`).

@history_item n=22 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the queued `mounted` event's guard) — `flush_queued_mounted_events` skips an element removed before the flush. Upstream's test was left out because Dioxus 0.7 delivers one mounted event. This is the bridge's mounted-flush behaviour, which item 2's deferred or replayed act would sit beside (`decision/stale-mounted-event-skipped`).

@history_item n=23 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the MountedData hazard re-measured, its fix routed to a port) — gave the mechanism: `poll` held the `RefCell` across `render_immediate`, where dioxus-core 0.7.10 first polls a queued task. It was pinned by `dioxus_runtime_probe` in both layout modes, and the fix was routed to a port of upstream `286874cc1`. This is item 2's direct ancestor.

@history_item n=24 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (a node's component and a held signal's readers through Dioxus public API) — showed that the owning component and `GlobalSignal` readers are reachable through public API alone, while hook-local `use_signal` state is not: dioxus-core's `Scope.hooks` is `pub(crate)`, so reaching it needs a fork-level change. This is the same class of trap as item 4's private `ErrorContext` fields (`decision/owning-component-via-public-api`).

@history_item n=25 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the bridge's buffered render recorded as a contract) — the vdom renders into a crate-private `MutationBuffer` with the document unborrowed, then borrows the document once and replays, dropping the borrow before `flush_queued_mounted_events`. This is the design item 2's limit comes from (`decision/bridge-buffered-render`, `decision/dioxus-mutation-buffer-replay`, `decision/render-with-document-unborrowed`).

@history_item n=26 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the MountedData hazard closed as an engine change) — a ratified direct engine change for every Dioxus document, a port of `286874cc1` that is not upstreamable (upstream holds it in its 0.8 form). Its merge surface is `dioxus_document.rs`, `mutation_buffer.rs` (to be replaced by upstream's 0.8 file) and `lib.rs`. Item 2's fix must survive that 0.8 replacement (`decision/dioxus-bridge-buffered-render`).

@history_item n=27 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render's limit and dioxus-core 0.7.10's first-poll paths) — states item 2's residual exactly: a task polled during render sees the previous render's document, and `set_focus_to` returns `false` for a node the same render created. A task spawned from a component body is first polled by the next `poll`'s `poll_tasks`. A task spawned from a handler that dirties a scope is polled inside `render_immediate`'s `Work::PollTask` arm, the one path that matters for a deferred or replayed act.

@history_item n=28 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render's witness on Dioxus 0.7) — the discriminating witness is the probe's two `mounted_data_in_a_task_a_key_handler_spawned_acts_*` tests. Upstream's ported `task_polled_during_render_can_access_document` does not detect a regression to the borrowed render. Item 2's checks must likewise be shown to fail when the fix is mutated away.

@history_item n=29 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (async handler writes land inside their dispatch) — in Dioxus 0.7.10 an async handler's spawned task runs inside the dispatch that started it. This is scheduling that a deferred or replayed act for item 2 must account for.

@history_item n=30 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph registry and the bridge's dispatch) — `SignalRegistry` is provided as a root context at `ScopeId::ROOT` by the first registering hook, with entries leaving through `use_drop`. The dispatch change is inert without a registry, and the buffered render, `initial_build`, `poll` and the mounted flush were left untouched. This is the registry pattern item 4's pruning would follow (`decision/dioxus-root-contexts`, `decision/listeners-read-from-vdom`).

@history_item n=31 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached record on the bridge's dispatch) — a crate-private `ReachedRecord` at `ScopeId::ROOT`, ungated and compiled on the wasm build without `accessibility`, with its `Debug` printing a count alone. This is the precedent for an ungated root-context registry and its gating row (`decision/accessibility-feature-gating`), the row a change to item 4's ungated `boundary` module touches.

@history_item n=32 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** (Cross-cutting Patterns) — created `decision/one-run-seed`: one seed per run held as a Dioxus root context, ungated, reproducible across layout modes and processes. It also added the ungated `seed` module to `decision/accessibility-feature-gating`. Item 3 extends this seed from Dioxus components to page scripts.

@history_item n=33 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** (Standard Contracts) — registered `SeededRandom` (splitmix64, shared-state clones, draw order), `use_seeded_random`/`DEFAULT_SEED` (host fallback outside a run) and `contract/test-harness/run-seed` (`from_vdom` provides seed 0, `wrap` none). These are the contracts a document-level seed for Boa's `Math.random` must agree with.

@history_item n=34 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** (Stack and Technologies) — `decision/no-prng-crate`: splitmix64 hand-written, host seed from std `RandomState`, so no package enters the lockfile. Item 3 must not take a PRNG crate either.

@history_item n=35 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** (Standard Contracts: no seed crosses the driver's carriers) — `decision/no-seed-on-carriers`: no verb, argument, flag, MCP field, kept-check key or span field carries a seed, and a seed reaches a run only from in-process code. Item 3 must seed the script runtime without adding any carrier, as the scope's boundary also says.

@history_item n=36 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** (Standard Contracts) — `decision/no-seed-on-carriers` moved from provisional to standing on the founder's own word, «A: нет, сид только при загрузке». Item 3's fork inherits this ratified absence.

@history_item n=37 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Standard Contracts) — added `root_error_mark`/`RootErrorMark` (root error boundary read by identity) and the capture's dispatcher pair. This is the first reader of dioxus-core's `ErrorContext` that item 4's registry grew from (`contract/dioxus-native-dom-api/root-error-mark`).

@history_item n=38 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Cross-cutting Patterns) — the new `boundary` module stays ungated in `decision/accessibility-feature-gating`. A pruned registry (item 4) stays in that module and keeps it ungated.

@history_item n=39 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Established Decisions) — `decision/driver-library-side-effect-free` and `decision/driver-spans-only` were narrowed to "installs no global subscriber" around the scoped capture. Any fix touching the driver's step must keep these.

@history_item n=40 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Standard Contracts, 2 of 2) — registered `contract/dioxus-native-dom-api/boundary-marks`: a boundary registry is provided at the root scope before the first build, and every boundary built under it registers, read by identity. The registry grows for the document's life because a dropped boundary cannot be told. This is the row item 4 would amend (prune) or keep. The caused errors are ordered "new since the boundary marks" (`contract/acting-verb-results/caused`), so pruning must not change which errors read as new.

@history_item n=41 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Established Decisions) — `decision/driver-library-side-effect-free` gained the capture's process-wide chaining panic hook as its exception. `decision/no-node-id-across-session` gained a step's caused text. Item 4's boundary reading feeds this caused text across the carriers.

@history_item n=42 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (Project Intent) — `decision/capability-statement`: the statement's names and counts are held to the schema and leg runner, and no check reads its other claims. Each of the four debts kept as a residual, or fixed, changes a stated limit there by hand.

@history_item n=43 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Standard Contracts) — a focus change, programmatic ones included, now dispatches blur, focusout, focus and focusin. Only the inert focus fix-up and the driver's select focus dispatch none. The platform tree refreshes on a redraw after resolve when the changed set is non-empty. A no-event focus fixup for item 1 would be a third silent path, which this row (`contract/event-dispatch/focus-change`) would have to name. A layout-time fixup (option B) lands right before that redraw (`decision/accessibility-tree-refresh-on-change`).

@history_item n=44 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Cross-cutting Patterns) — `decision/events-queued-on-document` replaced the dispatch callback, so events queue on the document and the driver's flush dispatches them. `decision/a11y-rebuild-on-nonempty-changed-set` moved to the redraw. Item 1's fix, and any act item 2 replays, go through this queue model.

@history_item n=45 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Project Intent) — `decision/dioxus-0-8-move-held`: Dioxus is held at 0.7.3 while Stylo moved to 0.24. Item 4's "wait on a dioxus-core release" option and item 2's 0.8-form `mutation_buffer.rs` both wait on this hold.

@history_item n=46 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (Cross-cutting Patterns) — `decision/changed-set-write-sites` gained a select list's highlight move as a write site, on the founder's word. This is the most recent precedent for widening that closed list when a debt fix adds a state change, as item 1's focus clear would.

@history_item n=47 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (Cross-cutting Patterns and Standard Contracts) — the reached record is now resolved before dispatch and written after, reading the stop and a handler's panic unwind, and `press` gained `ctrl`. This is the sibling debts chunk of this epoch: its forks were put to the founder as one card and confirmed «Подтвердить все 41», the shape this chunk's `founder-forks.md` follows. It also changed the bridge's dispatch, which item 2's render-time act runs beside (`contract/driver-verbs/press`).
