@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — (§3 obs log format) measured escher's sink as non-JSON stderr lines behind the allowlist scrub (`text`/`value`/`html`/`attrs` redacted, only safe fields for `blitz*`/`accesskit_xplat`); this chunk's "scrub held" boundary is that sink, and no a11y violation schema is defined against it, so a captured record entering an answer must not reshape it.

@history_item n=2 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink gained its third outcome: records from a target outside `escher_*`/`blitz*`/`accesskit_xplat` are dropped whole at every level; a per-step capture layer must see what the sink drops versus prints, and an app handler's own log under an app target is exactly such a dropped record today.

@history_item n=3 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — restated the sink as two record classes (events, and one line per closed span of an admitted target) and added `tracing` as escher-driver's dependency; the scope forbids the `command` span gaining any field carrying the new answer content, and this is the span class that would print it.

@history_item n=4 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the closed-span record class ratified by the founder (2026-10-09); one of the "log rulings of 2026-10-07/09" that bound the founder's boundary fork on what of a log may enter an answer.

@history_item n=5 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — every Dioxus element node carries its stable id as `author_id` (row `contract/accessibility-tree/author-id`) and the tree is built through `Document::accessibility_tree` (rows `decision/accessibility-tree-through-document-method`, `decision/a11y-tree-through-document-trait`); the stable element id this chunk ties each error to is that identity, never a `NodeId`.

@history_item n=6 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot reads role, name and focus from the tree with no second mapping (origin of `decision/snapshot-model-single-source`, `decision/readers-of-one-snapshot-model`); the new answer field sits beside the diff and must not become a second source of role or name.

@history_item n=7 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the actionable-key rule (`decision/actionable-key-rule`, guard `unkeyed_actionable`): every actionable element reads an author key; trap: never satisfy the check by adding a `tabindex` or role. A fixture app beside the stand that panics or logs from a handler must key its elements so the error ties to a stable id.

@history_item n=8 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — a password input reads `MASKED_VALUE` (origin of `decision/password-value-masked`); a captured panic payload or log field must never carry typed text a mask hides elsewhere.

@history_item n=9 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — `Snapshot::to_text` added as a reader of the model, and a file input's path masked; precedent that each new answer form reads the model alone and that path-like text is kept out of every field.

@history_item n=10 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the tree refreshes on a poll whose drained changed set was non-empty (`decision/a11y-tree-refresh-on-change`) and the diff an acting call returns reads the snapshot model; the new field rides beside that diff and must not alter the changed-set drain.

@history_item n=11 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the refresh on change and its changed-set contract ratified by the founder (2026-10-07); a caught panic leaving the instance held must keep the changed set and refresh semantics this ruling fixed.

@history_item n=12 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle (`Harness::settle`, the session's `act`) neither reads nor drains the changed set, and a fresh boot holds a non-empty set (`decision/harness-settle-leaves-changed-set`, guard `stand_settle`); trap for a per-step capture around the run path: do not drain the set, and drain first in a check that reads it as one step's change.

@history_item n=13 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the driver's schema defines no role or name set (`decision/driver-schema-has-no-role-set`); a new result field enters only as the schema tables admit, as the scope requires.

@history_item n=14 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — escher-driver names `accessibility` on dioxus-native-dom unconditionally, so both seven_guis binaries compile the gated readers; the graph reader the error-to-handler tie needs is reachable from the driver for that reason, while no stand binary builds an adapter.

@history_item n=15 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `Session::run` refuses `click`/`type` on `enabled == Some(false)` with nothing dispatched, the first of its screen-level checks; a refused step dispatches no handler, so it can cause no handler error to report.

@history_item n=16 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — an MCP result is a reader of the snapshot model only; an acting call's result is its JSON line with the diff (`contract/mcp-result/snapshot`), the crossing ratified by the founder 2026-10-10; the new field on MCP extends this line and his field rule of 2026-10-10 judges whether it is a widening.

@history_item n=17 marker=2026-10-12-self-description
**2026-10-12-self-description** — the password over MCP reads `MASKED_VALUE`, sentinel 0 times, and `help` (the node keys' help) became a closed-words reader of the model; the scope's `help` and MCP listing describing the new field follow this precedent, and a sentinel-count check is the pattern for proving no typed text crosses.

@history_item n=18 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a password's text in no painted frame either (`decision/password-never-revealed`), the founder's fork 2; extends the no-typed-text boundary the error field must respect on every carrier.

@history_item n=19 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — (Enter and Space activate a focused control) Enter/Space dispatch a synthetic Click, the app's handlers run first, then `handle_click` (`contract/keyboard-activation/*`, `decision/keyboard-activation-enter-space`); a `press` can therefore run an app handler that panics or logs, so `press` is one of the acting verbs whose answer carries the error.

@history_item n=20 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (§5, §6: `inert` and `disabled`) a disabled form control takes neither activation key, a `disabled` on a `summary` or `a` gating neither; ratified 2026-10-18 (Epoch 2 close entry); bounds which keyboard steps reach a handler.

@history_item n=21 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the gated part: the `lint` verb) the lint's reading crossed onto every carrier and the kept check, written as ids, closed words and numbers only; precedent for a new answer content on every ratified carrier that the founder ratified as a closed set.

@history_item n=22 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — an MCP `snapshot` result is the text for a whole screen and a `{text, part, parts}` JSON line for a paged one (`contract/mcp-result/snapshot`); an answer that grows by captured records must stay within the answer forms and budgets this set.

@history_item n=23 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs fourteen files by the founder's word, each joining file a witness or a tree reading he took; a new `tests/blitz-tests/tests/` target for this chunk joins no a11y leg list unless so ruled.

@history_item n=24 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the scrollbar's band) right after a `scroll` a driver click in the shown overlay scrollbar band is a thumb drag and dispatches nothing (`decision/driver-click-in-shown-scrollbar-drags`); such a step reaches no handler and must report no handler error.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the `MountedData` hazard in a spawned task) a `MountedData` call in a task spawned from a key handler panicked `RefCell already borrowed`; the stand's one recorded handler panic, the class of panic this chunk would now catch and return.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the driver on a `multiple` select) `type` focuses a select with no click and the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors and exits gain nothing (`decision/driver-type-focuses-select-no-click`); the scope's "no new verb, exit status or session error" rests on those same closed sets.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the driver's `type` on any select) `type` widened from a `multiple` select to any select, closing any list, firing no DOM focus events; the current form of `decision/driver-type-focuses-select-no-click`.

@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (how an open list's labels leave) an option's and a group's label leave only as screenshot pixels and the snapshot's `name`, "with no log site", ratified by the founder (`decision/option-labels-leave-only-as-pixels`); a captured log record entering an answer must not open a new route for page text.

@history_item n=29 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (what the snapshot and an AT read of an open list) no `expanded` node key, `NODE_FIELDS` thirteen (`absence/snapshot-list-open-state`), a row's own `bounds` read as a closed set by the field rule of 2026-10-10; the same rule judges whether the new answer field is a widening.

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (an open list has no motion) motion on an open list observed absent, a screen read after a step needing no `advance` (`absence/select-list-motion`); an error captured on a step is read in that step's answer with no `advance`.

@history_item n=31 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a key press past a dropped anonymous block) a key-press panic in the engine fixed as escher's own, with a check outside both a11y and stand-repeat legs; precedent that an engine panic is fixed, not caught, while this chunk catches only the app handler's panic.

@history_item n=32 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the listener mark's over-report) the `data-dioxus-id` mark and event attributes over-report removed listeners while the vdom walk's dynamic attributes read current listeners exactly; tie a handler error to the handler through the exact listener reading, never the mark.

@history_item n=33 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the MountedData hazard re-measured) the panic is `DioxusDocument::poll` holding `inner.borrow_mut()` across `render_immediate`, pinned by `should_panic` pins in `dioxus_runtime_probe`; the measured site where a panic arises on the poll and run path.

@history_item n=34 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the probe target outside the a11y leg) `dioxus_runtime_probe` joined no `a11y` or `stand-repeat` leg and is reached by the workspace `test` leg and `run`; placement precedent for this chunk's new blitz-tests target and fixture.

@history_item n=35 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (the spawned-task MountedData hazard record, closed by the buffered render) the panic retired by rendering into a `MutationBuffer` unborrowed; the two `should_panic` pins became acting pins. A panic fixture must not lean on that retired panic.

@history_item n=36 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (the buffered render's same-render limit) a task polled during a render acts on the document as the previous render left it; an error from such a task is attributed against that state, a limit owned by an open residual row.

@history_item n=37 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the read-only UI graph reader `DioxusDocument::graph`) one node per snapshot node; ids, component, state and handler event names, closed flag words and booleans, never a value; no served id holds `///` (`decision/ui-graph-one-node-per-snapshot-node`, `decision/ui-graph-carries-no-values`, `contract/node-id/reserved-separator`); the handler identity (owner id and event name) this chunk ties errors to, and its no-value rule.

@history_item n=38 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (graph state registration and the bridge dispatch's write record) where a `SignalRegistry` is registered, `DioxusEventHandler::handle_event` sets the dispatch (chain and event) around `runtime.handle_event` and clears it after; the natural seam, and a trap, for attributing a step's panic or log to its handler, since a panic must not leave the dispatch set.

@history_item n=39 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the «reacts to a click» snapshot field built as `NodeState.clickable`) `NODE_FIELDS` 13 → 14 and `Reading::ALL` 11 → 12, read from current listeners; a precedent for how a new answer or snapshot field grows the closed field tables and the `help` text.

@history_item n=40 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the one actionable rule and the check's listener leg) `unkeyed_actionable` and `actionable_nodes` share the crate-private `Readings`, its listener leg the exact current listeners other than `mounted`, the mark serving event dispatch alone (`decision/one-actionable-rule`).

@history_item n=41 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actionable-node reader) `actionable_nodes` added behind `accessibility`, beside the ungated `signal_registry` (`decision/agent-readers-behind-accessibility`); the wasm build compiles dioxus-native-dom without that feature, so any capture on the dispatch path must decide its gate.

@history_item n=42 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actions verb as a read-only reader) the `actions` answer writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, `MASKED_VALUE` or `NodeId` (`contract/actions-answer/fields`), its crossing on every carrier the founder's word; the template for the error field's closed content and the founder's fork if authored text crosses.

@history_item n=43 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the keyed reading, provisional on the carriers) a new field on the carriers stood PROVISIONAL as a delegate's call until the founder's word; how a new answer field is marked when the founder is away, as now.

@history_item n=44 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form` provisional → standing on the founder's «Утвердить keyed»; the closing step of the keyed precedent.

@history_item n=45 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the ungated reached record on `handle_event`'s path) `record_reached`/`reached_handlers` record (id, event) per dispatch before `runtime.handle_event`, ungated, a removed element still counting, one `has_context` lookup per dispatch elsewhere (`contract/dioxus-document/reached-handlers`, `contract/dioxus-document/record-reached`); one of the scope's named candidates for attributing an error to a handler.

@history_item n=46 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (`stand_handler_coverage`'s keyboard-activation check) a refused step exercises nothing and a keyboard activation its control's click, in both modes, in `stand-repeat` by prefix and no a11y leg list; the same refused and keyboard cases bound what errors a step can carry.

@history_item n=47 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (`handler_coverage`, a further reader of the UI graph) reads the graph at boot and after each replayed step, `mounted` left out, answer of ids, owner ids, event names and two counts only (`contract/handler-coverage/answer`); the kept-check replay path the new field must also ride.

@history_item n=48 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (429 records → 241 rows) on the founder's approval; every picked row's wording and its guards date from this conversion, so a row this chunk extends is amended by handle (`+`/`~`/`−`).

@history_item n=49 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (2. A11y Strategy) `~ decision/changed-set-marks` takes the select-list scroll qualifier; the changed set a step's diff reads, which the new capture must leave as is.
